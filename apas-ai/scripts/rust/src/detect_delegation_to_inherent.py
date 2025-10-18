#!/usr/bin/env python3
"""
Detect trait impl methods that delegate to inherent impl methods.

Looks for pattern: fn method(...) { StructName::method(...) }
Only reports if the method actually exists in an inherent impl.
"""
# Git commit: e8e8f18
# Date: 2025-10-17
# Updated: 2025-10-18 - Fixed false positives

import re
import sys
from pathlib import Path


def find_inherent_methods(content, struct_name):
    """Find all methods defined in inherent impl blocks for struct_name."""
    inherent_methods = set()
    
    # Find inherent impl blocks: impl<...> StructName<...> {
    # NOT: impl<...> Trait for StructName
    inherent_pattern = rf'impl(?:<[^>]*>)?\s+{struct_name}(?:<[^>]*>)?\s*{{'
    
    for match in re.finditer(inherent_pattern, content):
        # Make sure it's not a trait impl (no "for" keyword before the struct)
        before_match = content[max(0, match.start() - 100):match.start()]
        if ' for ' in before_match.split('\n')[-1]:
            continue
            
        # Find the impl block content
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
        
        # Find all method definitions in this inherent impl
        method_pattern = r'\bfn\s+(\w+)\s*(?:<[^>]*>)?\s*\('
        for method_match in re.finditer(method_pattern, impl_content):
            method_name = method_match.group(1)
            inherent_methods.add(method_name)
    
    return inherent_methods


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
        
        # Find methods in inherent impl for this struct
        inherent_methods = find_inherent_methods(content, struct_name)
        
        if not inherent_methods:
            # No inherent impl, so no possible delegations
            continue
        
        # Find the trait impl block content
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
        # Only report if method_name is in inherent_methods
        delegation_pattern = rf'{struct_name}::(\w+)\('
        delegations = set()
        
        for deleg_match in re.finditer(delegation_pattern, impl_content):
            method_name = deleg_match.group(1)
            # Check if this method exists in inherent impl
            if method_name in inherent_methods:
                # Filter out PascalCase identifiers (enum variants)
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
