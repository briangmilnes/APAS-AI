#!/usr/bin/env python3
"""
Analyze which inherent impls can be safely removed - VERSION 2 with forwarding detection.

An inherent impl is safe to remove if:
1. Struct has a custom trait impl
2. ALL public methods in inherent impl are also in the trait impl
3. Trait impl does NOT forward to inherent impl (no infinite recursion)
4. Private/helper methods can be moved to module-level functions

This version detects when trait impls forward to inherent impls.
"""
# Git commit: TBD
# Date: 2025-10-18

import re
import sys
from pathlib import Path
from collections import defaultdict

def extract_method_signature(line):
    """Extract method name and visibility from a method signature."""
    line = re.sub(r'//.*$', '', line).strip()
    
    # Match: [pub] fn method_name
    match = re.search(r'\b(pub)?\s*fn\s+(\w+)', line)
    if match:
        is_public = match.group(1) == 'pub'
        method_name = match.group(2)
        return (method_name, is_public)
    return None

def find_methods_in_impl(lines, start, end):
    """Find all methods in an impl block."""
    methods = []
    
    i = start + 1  # Skip the impl line itself
    while i < end:
        line = lines[i].strip()
        
        # Skip comments and empty lines
        if not line or line.startswith('//'):
            i += 1
            continue
        
        # Check for method signature
        method_info = extract_method_signature(line)
        if method_info:
            method_name, is_public = method_info
            methods.append({
                'name': method_name,
                'public': is_public,
                'line': i + 1,
            })
        
        i += 1
    
    return methods

def check_forwarding_in_trait_impl(lines, trait_block, struct_name):
    """
    Check if trait impl methods forward to inherent impl.
    Returns list of method names that are forwarding.
    """
    forwarding_methods = []
    
    start = trait_block['start']
    end = trait_block['end']
    
    # Look for patterns like: StructName::method_name
    for i in range(start, end):
        line = lines[i]
        
        # Skip comments
        if '//' in line:
            line = line[:line.index('//')]
        
        # Check for method definition
        method_match = re.search(r'\bfn\s+(\w+)', line)
        if method_match:
            method_name = method_match.group(1)
            
            # Now look ahead in the method body for forwarding pattern
            # Pattern: StructName::method_name(
            j = i
            brace_count = 0
            in_method = False
            
            while j < end:
                current_line = lines[j]
                
                # Track braces to know when we're in/out of method body
                if '{' in current_line:
                    in_method = True
                    brace_count += current_line.count('{')
                if '}' in current_line:
                    brace_count -= current_line.count('}')
                    if brace_count == 0 and in_method:
                        break  # End of this method
                
                # Check for forwarding pattern
                if in_method:
                    # Remove comments
                    check_line = current_line
                    if '//' in check_line:
                        check_line = check_line[:check_line.index('//')]
                    
                    # Look for: StructName::method_name(
                    forward_pattern = rf'{struct_name}::{method_name}\s*\('
                    if re.search(forward_pattern, check_line):
                        forwarding_methods.append(method_name)
                        break
                
                j += 1
    
    return forwarding_methods

def count_braces_in_line(line):
    """Count braces in a line, ignoring those in strings and comments."""
    in_string = False
    in_char = False
    escape = False
    open_count = 0
    close_count = 0
    
    i = 0
    while i < len(line):
        c = line[i]
        
        if escape:
            escape = False
            i += 1
            continue
        
        if c == '\\':
            escape = True
            i += 1
            continue
        
        if c == '"' and not in_char:
            in_string = not in_string
            i += 1
            continue
        
        if c == "'" and not in_string:
            if i + 1 < len(line) and (line[i+1].isalpha() or line[i+1] == '_'):
                i += 1
                continue
            in_char = not in_char
            i += 1
            continue
        
        if not in_string and not in_char:
            if c == '{':
                open_count += 1
            elif c == '}':
                close_count += 1
        
        i += 1
    
    return (open_count, close_count)

def extract_impl_info(line):
    """Extract information from an impl line."""
    line = re.sub(r'//.*$', '', line).strip()
    
    # Standard traits to ignore
    STANDARD_TRAITS = {
        'Eq', 'PartialEq', 'Ord', 'PartialOrd',
        'Debug', 'Display', 
        'Clone', 'Copy',
        'Hash', 
        'Default',
    }
    
    # Check for trait impl
    trait_match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)', line)
    if trait_match:
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        is_standard = trait_name in STANDARD_TRAITS
        return ('trait', struct_name, trait_name, is_standard)
    
    # Check for inherent impl
    inherent_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{', line)
    if inherent_match:
        struct_name = inherent_match.group(1)
        return ('inherent', struct_name, None, False)
    
    return None

def find_impl_blocks(lines):
    """Find all impl blocks with their methods."""
    impl_blocks = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        if not stripped.startswith('impl'):
            i += 1
            continue
        
        impl_info = extract_impl_info(stripped)
        if not impl_info:
            i += 1
            continue
        
        impl_type, struct_name, trait_name, is_standard = impl_info
        start_line = i
        
        # Count braces to find end
        open_b, close_b = count_braces_in_line(stripped)
        brace_count = open_b - close_b
        
        j = i + 1
        while j < len(lines) and brace_count > 0:
            open_b, close_b = count_braces_in_line(lines[j])
            brace_count += open_b - close_b
            j += 1
        
        end_line = j
        
        # Extract methods
        methods = find_methods_in_impl(lines, start_line, end_line)
        
        impl_blocks.append({
            'start': start_line,
            'end': end_line,
            'type': impl_type,
            'struct': struct_name,
            'trait': trait_name,
            'is_standard': is_standard,
            'methods': methods,
        })
        
        i = end_line
    
    return impl_blocks

def analyze_file(file_path):
    """Analyze a file for safe inherent impl removals."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        return None
    
    impl_blocks = find_impl_blocks(lines)
    
    # Group by struct
    struct_impls = defaultdict(lambda: {'inherent': [], 'custom_traits': [], 'standard_traits': []})
    
    for block in impl_blocks:
        struct_name = block['struct']
        if block['type'] == 'inherent':
            struct_impls[struct_name]['inherent'].append(block)
        elif block['is_standard']:
            struct_impls[struct_name]['standard_traits'].append(block)
        else:
            struct_impls[struct_name]['custom_traits'].append(block)
    
    results = []
    
    for struct_name, impls in struct_impls.items():
        if not impls['inherent'] or not impls['custom_traits']:
            continue
        
        # Analyze each inherent impl
        for inh_block in impls['inherent']:
            inherent_methods = inh_block['methods']
            
            # Get all trait method names
            trait_methods = set()
            for trait_block in impls['custom_traits']:
                for method in trait_block['methods']:
                    trait_methods.add(method['name'])
            
            # Check for forwarding in trait impls
            forwarding_methods = []
            for trait_block in impls['custom_traits']:
                forwarding = check_forwarding_in_trait_impl(lines, trait_block, struct_name)
                forwarding_methods.extend(forwarding)
            
            # Check for public methods not in trait
            public_only_in_inherent = [
                m for m in inherent_methods 
                if m['public'] and m['name'] not in trait_methods
            ]
            
            # Check for private methods
            private_methods = [m for m in inherent_methods if not m['public']]
            
            # Determine safety
            if not inherent_methods:
                safety = "SAFE_EMPTY"
                reason = "Inherent impl is empty"
            elif public_only_in_inherent:
                safety = "UNSAFE"
                reason = f"Has public methods not in trait: {', '.join(m['name'] for m in public_only_in_inherent)}"
            elif forwarding_methods:
                safety = "FORWARDING"
                reason = f"Trait impl forwards to inherent: {', '.join(set(forwarding_methods))}"
            elif private_methods:
                safety = "NEEDS_REVIEW"
                reason = f"Has {len(private_methods)} private method(s): {', '.join(m['name'] for m in private_methods)}"
            else:
                safety = "SAFE"
                reason = "All public methods are in trait"
            
            results.append({
                'file': str(file_path),
                'struct': struct_name,
                'safety': safety,
                'reason': reason,
                'inherent_methods': len(inherent_methods),
                'public_not_in_trait': len(public_only_in_inherent),
                'private_methods': len(private_methods),
                'forwarding_methods': len(forwarding_methods),
            })
    
    return results

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Analyze safe inherent impl removals (v2 with forwarding detection)")
    parser.add_argument('--file', type=str, help='Single file to analyze')
    parser.add_argument('--all', action='store_true', help='Analyze all src files')
    args = parser.parse_args()
    
    if args.file:
        results = analyze_file(Path(args.file))
        if results:
            for r in results:
                print(f"{r['file']}")
                print(f"  Struct: {r['struct']}")
                print(f"  Safety: {r['safety']}")
                print(f"  Reason: {r['reason']}")
                print()
    elif args.all:
        src_dir = Path('src')
        all_results = []
        
        for rs_file in sorted(src_dir.rglob('*.rs')):
            results = analyze_file(rs_file)
            if results:
                all_results.extend(results)
        
        # Group by safety
        by_safety = defaultdict(list)
        for r in all_results:
            by_safety[r['safety']].append(r)
        
        print("=" * 100)
        print("SAFE TO REMOVE (all public methods are in trait, no forwarding):")
        print("=" * 100)
        for r in by_safety.get('SAFE', []):
            print(f"  {r['file']:60} | {r['struct']:25} | {r['inherent_methods']} methods")
        
        print()
        print("=" * 100)
        print("SAFE TO REMOVE (empty inherent impl):")
        print("=" * 100)
        for r in by_safety.get('SAFE_EMPTY', []):
            print(f"  {r['file']:60} | {r['struct']:25} | {r['reason']}")
        
        print()
        print("=" * 100)
        print("FORWARDING (trait impl forwards to inherent impl - would cause infinite recursion):")
        print("=" * 100)
        for r in by_safety.get('FORWARDING', []):
            print(f"  {r['file']:60} | {r['struct']:25} | {r['reason']}")
        
        print()
        print("=" * 100)
        print("NEEDS REVIEW (has private helper methods):")
        print("=" * 100)
        for r in by_safety.get('NEEDS_REVIEW', []):
            print(f"  {r['file']:60} | {r['struct']:25} | {r['reason']}")
        
        print()
        print("=" * 100)
        print("UNSAFE TO REMOVE (has public methods not in trait):")
        print("=" * 100)
        for r in by_safety.get('UNSAFE', []):
            print(f"  {r['file']:60} | {r['struct']:25} | {r['reason']}")
        
        print()
        print(f"Summary:")
        print(f"  SAFE:        {len(by_safety.get('SAFE', []))}")
        print(f"  SAFE_EMPTY:  {len(by_safety.get('SAFE_EMPTY', []))}")
        print(f"  FORWARDING:  {len(by_safety.get('FORWARDING', []))}")
        print(f"  NEEDS_REVIEW: {len(by_safety.get('NEEDS_REVIEW', []))}")
        print(f"  UNSAFE:      {len(by_safety.get('UNSAFE', []))}")
        print(f"  TOTAL:       {len(all_results)}")
    else:
        print("Error: Use --file or --all", file=sys.stderr)
        return 1
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

