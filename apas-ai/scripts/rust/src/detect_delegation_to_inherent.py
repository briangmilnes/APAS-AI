#!/usr/bin/env python3
"""
Detect trait impl methods that delegate to inherent impl methods.

Looks for pattern: fn method(...) { StructName::method(...) }
"""
# Git commit: e8e8f18
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Detect delegation to inherent impls")
    parser.add_argument('--file', required=True, help='File to analyze')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find trait impl blocks
    trait_impl_pattern = r'impl<[^>]*>\s+(\w+)<[^>]*>\s+for\s+(\w+)<[^>]*>'
    
    found_delegations = False
    for match in re.finditer(trait_impl_pattern, content):
        trait_name = match.group(1)
        struct_name = match.group(2)
        
        # Find the impl block content
        start = match.start()
        brace_count = 0
        i = content.find('{', match.end())
        if i == -1:
            continue
        
        impl_start = i
        while i < len(content):
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
                if brace_count == 0:
                    break
            i += 1
        
        impl_content = content[impl_start:i+1]
        
        # Look for delegation pattern: StructName::method_name(
        # Exclude enum variants (PascalCase) - only match methods (snake_case or camelCase starting lowercase)
        delegation_pattern = rf'{struct_name}::(\w+)\('
        delegations = set()
        
        for deleg_match in re.finditer(delegation_pattern, impl_content):
            method_name = deleg_match.group(1)
            # Filter out PascalCase identifiers (enum variants like Node, Leaf, etc.)
            # Methods start with lowercase or underscore
            if method_name[0].islower() or method_name[0] == '_':
                delegations.add(method_name)
        
        if delegations:
            found_delegations = True
            print(f"\n{file_path}:")
            print(f"  Trait impl: {trait_name} for {struct_name}")
            print(f"  Methods delegating to inherent impl: {', '.join(sorted(delegations))}")
            print(f"  Count: {len(delegations)}")
    
    return 0 if found_delegations else 1


if __name__ == '__main__':
    sys.exit(main())

