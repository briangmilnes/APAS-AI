#!/usr/bin/env python3
"""
Review calls to Type::method() where Type is from an external module.

These calls may be delegating to inherent impls that should use trait methods instead.
"""
# Git commit: 509549c
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def extract_imports(content):
    """Extract type imports from use statements."""
    imports = {}
    
    # Match: use crate::ChapXX::ModuleName::ModuleName::TypeName;
    # or: use crate::ChapXX::ModuleName::ModuleName::*;
    use_pattern = r'use\s+crate::([^;]+);'
    
    for match in re.finditer(use_pattern, content):
        use_path = match.group(1)
        parts = use_path.split('::')
        
        if parts[-1] == '*':
            # Wildcard import - extract module path
            module_path = '::'.join(parts[:-1])
            imports['*'] = module_path
        else:
            # Specific type import
            type_name = parts[-1]
            module_path = '::'.join(parts[:-1])
            imports[type_name] = module_path
    
    return imports


def find_type_method_calls(content):
    """Find calls like TypeName::method() in content."""
    # Pattern: TypeName::method_name(
    pattern = r'\b([A-Z][a-zA-Z0-9_]*?)::([a-z_][a-zA-Z0-9_]*)\s*\('
    
    calls = []
    for match in re.finditer(pattern, content):
        type_name = match.group(1)
        method_name = match.group(2)
        calls.append((type_name, method_name, match.start()))
    
    return calls


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Review external type method calls")
    parser.add_argument('--file', help='File to review')
    parser.add_argument('--dir', default='src', help='Directory to scan')
    args = parser.parse_args()
    
    files_to_check = []
    if args.file:
        files_to_check = [Path(args.file)]
    else:
        files_to_check = list(Path(args.dir).rglob('*.rs'))
    
    issues = []
    
    for file_path in files_to_check:
        try:
            with open(file_path, 'r') as f:
                content = f.read()
        except Exception as e:
            continue
        
        imports = extract_imports(content)
        calls = find_type_method_calls(content)
        
        # Find calls to imported types
        external_calls = []
        for type_name, method_name, pos in calls:
            if type_name in imports and imports[type_name]:
                external_calls.append((type_name, method_name, imports[type_name]))
        
        if external_calls:
            # Group by type
            by_type = {}
            for type_name, method_name, module in external_calls:
                if type_name not in by_type:
                    by_type[type_name] = {'module': module, 'methods': set()}
                by_type[type_name]['methods'].add(method_name)
            
            issues.append({
                'file': str(file_path),
                'types': by_type
            })
    
    if not issues:
        print("No external type method calls found.")
        return 1
    
    print(f"Found {len(issues)} files with external type method calls:\n")
    
    for issue in issues:
        print(f"{issue['file']}:")
        for type_name, info in issue['types'].items():
            methods = ', '.join(sorted(info['methods']))
            print(f"  {type_name} (from {info['module']}): {methods}")
        print()
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

