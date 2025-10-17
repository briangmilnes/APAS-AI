#!/usr/bin/env python3
"""
Detect trait impls that delegate to inherent impls.

Example delegation pattern:
    impl Struct {
        pub fn foo() -> Self { /* actual implementation */ }
    }
    
    impl Trait for Struct {
        fn foo() -> Self { Struct::foo() }  // <-- delegation
    }
"""
# Git commit: e8e8f18
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def find_impl_blocks(content):
    """Find all impl blocks and extract their method calls."""
    impl_blocks = []
    
    # Find impl blocks
    impl_pattern = r'impl(?:<[^>]*>)?\s+(?:(\w+)\s+for\s+)?(\w+)(?:<[^>]*>)?\s*\{'
    
    for match in re.finditer(impl_pattern, content):
        trait_name = match.group(1)  # None for inherent impl
        struct_name = match.group(2)
        impl_type = 'trait' if trait_name else 'inherent'
        
        # Find the impl block content
        start = match.start()
        brace_count = 0
        i = match.end() - 1  # Start at the opening brace
        
        while i < len(content):
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
                if brace_count == 0:
                    break
            i += 1
        
        impl_content = content[match.start():i+1]
        
        # Extract method definitions
        method_pattern = r'fn\s+(\w+)\s*[<(]'
        methods = [m.group(1) for m in re.finditer(method_pattern, impl_content)]
        
        # Check for delegation pattern in each method
        delegations = []
        for method in methods:
            # Look for pattern: fn method(...) { StructName::method(...) }
            delegation_pattern = rf'fn\s+{method}\s*[^{{]*\{{\s*{struct_name}::{method}\s*\('
            if re.search(delegation_pattern, impl_content):
                delegations.append(method)
        
        impl_blocks.append({
            'type': impl_type,
            'struct': struct_name,
            'trait': trait_name,
            'methods': methods,
            'delegations': delegations,
            'content': impl_content,
        })
    
    return impl_blocks


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Detect delegation patterns in trait impls")
    parser.add_argument('--file', required=True, help='File to analyze')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    impl_blocks = find_impl_blocks(content)
    
    # Group by struct
    by_struct = {}
    for block in impl_blocks:
        struct = block['struct']
        if struct not in by_struct:
            by_struct[struct] = {'inherent': [], 'traits': []}
        
        if block['type'] == 'inherent':
            by_struct[struct]['inherent'].append(block)
        else:
            by_struct[struct]['traits'].append(block)
    
    # Find delegation patterns
    found_delegations = False
    for struct, impls in by_struct.items():
        if not impls['inherent']:
            continue
        
        inherent_methods = set()
        for inherent in impls['inherent']:
            inherent_methods.update(inherent['methods'])
        
        for trait_impl in impls['traits']:
            if trait_impl['delegations']:
                found_delegations = True
                print(f"\n{file_path}:")
                print(f"  Struct: {struct}")
                print(f"  Trait: {trait_impl['trait']}")
                print(f"  Delegating methods: {', '.join(trait_impl['delegations'])}")
    
    return 0 if found_delegations else 1


if __name__ == '__main__':
    sys.exit(main())

