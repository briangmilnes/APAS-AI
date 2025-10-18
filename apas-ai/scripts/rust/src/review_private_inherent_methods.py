#!/usr/bin/env python3
"""
Review private methods in inherent impls that should be module-level functions.

Identifies private helper methods (like BalBinNode::new) that are only used
internally and could be module-level functions instead.
"""
# Git commit: TBD
# Date: 2025-10-18

import re
import sys
from pathlib import Path
from collections import defaultdict


def extract_method_info(line):
    """Extract method name and visibility from a method signature."""
    line = re.sub(r'//.*$', '', line).strip()
    
    # Match: [pub] fn method_name
    match = re.search(r'\b(pub)?\s*fn\s+(\w+)', line)
    if match:
        is_public = match.group(1) == 'pub'
        method_name = match.group(2)
        return (method_name, is_public)
    return None


def count_braces_in_line(line):
    """Count braces in a line, ignoring those in strings and comments."""
    # Simple version - good enough for most cases
    if '//' in line:
        line = line[:line.index('//')]
    
    # Remove string literals (rough approximation)
    line = re.sub(r'"[^"]*"', '', line)
    line = re.sub(r"'[^']*'", '', line)
    
    open_count = line.count('{')
    close_count = line.count('}')
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
        'From', 'Into', 'TryFrom', 'TryInto',
        'AsRef', 'AsMut',
        'Deref', 'DerefMut',
        'Iterator', 'IntoIterator',
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
        method_info = extract_method_info(line)
        if method_info:
            method_name, is_public = method_info
            methods.append({
                'name': method_name,
                'public': is_public,
                'line': i + 1,
            })
        
        i += 1
    
    return methods


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
    """Analyze a file for private helper methods in inherent impls."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        return None
    
    impl_blocks = find_impl_blocks(lines)
    
    # Find inherent impls with private methods
    results = []
    
    for block in impl_blocks:
        if block['type'] != 'inherent':
            continue
        
        private_methods = [m for m in block['methods'] if not m['public']]
        
        if private_methods:
            results.append({
                'file': str(file_path),
                'struct': block['struct'],
                'private_methods': private_methods,
                'count': len(private_methods),
            })
    
    return results


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Review private methods in inherent impls")
    parser.add_argument('--file', type=str, help='Single file to analyze')
    parser.add_argument('--all', action='store_true', help='Analyze all src files')
    args = parser.parse_args()
    
    if args.file:
        results = analyze_file(Path(args.file))
        if results:
            for r in results:
                print(f"{r['file']}")
                print(f"  Struct: {r['struct']}")
                print(f"  Private methods ({r['count']}):")
                for m in r['private_methods']:
                    print(f"    - {m['name']} (line {m['line']})")
                print()
    elif args.all:
        src_dir = Path('src')
        all_results = []
        
        for rs_file in sorted(src_dir.rglob('*.rs')):
            results = analyze_file(rs_file)
            if results:
                all_results.extend(results)
        
        print("=" * 100)
        print("PRIVATE HELPER METHODS IN INHERENT IMPLS:")
        print("=" * 100)
        
        for r in all_results:
            print(f"\n{r['file']}")
            print(f"  Struct: {r['struct']}")
            print(f"  Private methods ({r['count']}):")
            for m in r['private_methods']:
                print(f"    - {m['name']} (line {m['line']})")
        
        print()
        print(f"Summary: Found {len(all_results)} inherent impls with private methods")
    else:
        print("Error: Use --file or --all", file=sys.stderr)
        return 1
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

