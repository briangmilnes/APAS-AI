#!/usr/bin/env python3
"""
Review macros for method calls that may break without inherent impls.

Detects patterns like:
  macro_rules! Foo {
    ...
    StructName::method_name(...)  // Will break if only in trait
    ...
  }

Reports macros that use Type::method() syntax where method only exists
in trait impl, not inherent impl.
"""
# Git commit: TBD
# Date: 2025-10-18

import re
import sys
from pathlib import Path
from collections import defaultdict


STANDARD_TRAITS = {
    'Eq', 'PartialEq', 'Ord', 'PartialOrd',
    'Debug', 'Display', 
    'Clone', 'Copy',
    'Hash', 
    'Default',
    'From', 'Into', 'TryFrom', 'TryInto',
    'AsRef', 'AsMut',
    'Deref', 'DerefMut',
}


def find_macros(content):
    """Find all macro_rules! definitions and their bodies."""
    macros = []
    
    # Pattern: macro_rules! name { ... }
    pattern = r'#\[macro_export\]\s*macro_rules!\s+(\w+)\s*\{'
    
    for match in re.finditer(pattern, content):
        macro_name = match.group(1)
        start = match.end() - 1  # Start at opening brace
        
        # Find matching closing brace
        brace_count = 0
        i = start
        while i < len(content):
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
                if brace_count == 0:
                    break
            i += 1
        
        if i < len(content):
            macro_body = content[start:i+1]
            macros.append({
                'name': macro_name,
                'start': start,
                'end': i+1,
                'body': macro_body,
            })
    
    return macros


def find_type_method_calls(macro_body):
    """Find Type::method() patterns in macro body."""
    # Pattern: TypeName::method_name(
    # Exclude:
    #   - <Type as Trait>::method (already qualified)
    #   - TypeName::VariantName (PascalCase variants)
    
    calls = []
    
    # Look for patterns like: $crate::Path::To::TypeName::method_name(
    # or just: TypeName::method_name(
    pattern = r'(?:\$crate::)?(?:[\w:]+::)*(\w+)::(\w+)\s*\('
    
    for match in re.finditer(pattern, macro_body):
        type_name = match.group(1)
        method_name = match.group(2)
        
        # Skip if it's a qualified trait call
        before_match = macro_body[max(0, match.start()-20):match.start()]
        if '<' in before_match and ' as ' in before_match:
            continue
        
        # Skip PascalCase identifiers (likely enum variants)
        if method_name[0].isupper():
            continue
        
        calls.append({
            'type': type_name,
            'method': method_name,
            'full_match': match.group(0),
        })
    
    return calls


def find_impl_info(file_path):
    """Find all impl blocks and their methods for a file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception:
        return {}
    
    impl_info = defaultdict(lambda: {'inherent': set(), 'trait': set()})
    
    # Find inherent impls: impl TypeName { ... }
    inherent_pattern = r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]*>)?\s*\{'
    for match in re.finditer(inherent_pattern, content):
        type_name = match.group(1)
        # Find methods in this impl
        start = match.end()
        brace_count = 1
        i = start
        while i < len(content) and brace_count > 0:
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
            i += 1
        
        impl_body = content[start:i]
        methods = re.findall(r'\bfn\s+(\w+)', impl_body)
        impl_info[type_name]['inherent'].update(methods)
    
    # Find trait impls: impl Trait for TypeName { ... }
    trait_pattern = r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)'
    for match in re.finditer(trait_pattern, content):
        trait_name = match.group(1)
        type_name = match.group(2)
        
        # Skip standard traits
        if trait_name in STANDARD_TRAITS:
            continue
        
        # Find methods in this impl
        start_pos = match.end()
        brace_start = content.find('{', start_pos)
        if brace_start == -1:
            continue
        
        brace_count = 1
        i = brace_start + 1
        while i < len(content) and brace_count > 0:
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
            i += 1
        
        impl_body = content[brace_start:i]
        methods = re.findall(r'\bfn\s+(\w+)', impl_body)
        impl_info[type_name]['trait'].update(methods)
    
    return impl_info


def analyze_file(file_path, all_impl_info):
    """Analyze a file for broken macro calls."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception:
        return []
    
    macros = find_macros(content)
    if not macros:
        return []
    
    results = []
    
    for macro in macros:
        calls = find_type_method_calls(macro['body'])
        if not calls:
            continue
        
        broken_calls = []
        for call in calls:
            type_name = call['type']
            method_name = call['method']
            
            # Check if method exists in inherent impl
            has_inherent = False
            has_trait = False
            
            # Check all files for this type
            for impl_info in all_impl_info.values():
                if type_name in impl_info:
                    if method_name in impl_info[type_name]['inherent']:
                        has_inherent = True
                    if method_name in impl_info[type_name]['trait']:
                        has_trait = True
            
            # If only in trait (not inherent), it will break
            if has_trait and not has_inherent:
                broken_calls.append({
                    'type': type_name,
                    'method': method_name,
                    'full_match': call['full_match'],
                    'status': 'BROKEN: method only in trait, needs qualified syntax'
                })
            elif not has_inherent and not has_trait:
                # Might be from another crate or module
                broken_calls.append({
                    'type': type_name,
                    'method': method_name,
                    'full_match': call['full_match'],
                    'status': 'UNKNOWN: method not found in this codebase'
                })
        
        if broken_calls:
            results.append({
                'file': str(file_path),
                'macro': macro['name'],
                'broken_calls': broken_calls,
            })
    
    return results


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Review macros for method calls that break without inherent impls"
    )
    parser.add_argument('--file', type=str, help='Single file to analyze')
    parser.add_argument('--all', action='store_true', help='Analyze all src files')
    args = parser.parse_args()
    
    src_dir = Path('src')
    
    # First pass: collect all impl info from all files
    print("Scanning codebase for impl blocks...")
    all_impl_info = {}
    for rs_file in src_dir.rglob('*.rs'):
        impl_info = find_impl_info(rs_file)
        if impl_info:
            all_impl_info[str(rs_file)] = impl_info
    
    if args.file:
        results = analyze_file(Path(args.file), all_impl_info)
        if results:
            for r in results:
                print(f"\n{r['file']}")
                print(f"  Macro: {r['macro']}")
                for call in r['broken_calls']:
                    print(f"    {call['full_match']}")
                    print(f"      {call['status']}")
        else:
            print("No broken macro calls found")
    
    elif args.all:
        print("\nAnalyzing macros...")
        all_results = []
        
        for rs_file in sorted(src_dir.rglob('*.rs')):
            results = analyze_file(rs_file, all_impl_info)
            if results:
                all_results.extend(results)
        
        if not all_results:
            print("\n✓ No broken macro calls found")
            return 0
        
        print("\n" + "=" * 100)
        print("BROKEN MACRO CALLS (method only in trait, needs qualified syntax):")
        print("=" * 100)
        
        for r in all_results:
            print(f"\n{r['file']}")
            print(f"  Macro: {r['macro']}")
            for call in r['broken_calls']:
                print(f"    ❌ {call['full_match']}")
                print(f"       {call['status']}")
        
        print(f"\n\nSummary: Found {len(all_results)} macro(s) with broken calls")
        print(f"\nThese macros need to use qualified syntax:")
        print(f"  FROM: TypeName::method(...)")
        print(f"  TO:   <TypeName as TraitName>::method(...)")
        
        return 1
    
    else:
        print("Error: Use --file or --all", file=sys.stderr)
        return 1
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

