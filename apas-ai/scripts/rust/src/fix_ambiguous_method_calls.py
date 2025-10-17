#!/usr/bin/env python3
"""
Fix ambiguous method calls that arise when inherent impls are removed.

When a struct's inherent impl is removed, method calls become ambiguous if multiple
traits define the same method. This script makes those calls explicit using UFCS.
"""
# Git commit: 509549c
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def find_local_trait_name(content):
    """Find the local trait name defined in this file."""
    # Pattern: pub trait TraitName<
    pattern = r'pub trait (\w+)<'
    match = re.search(pattern, content)
    if match:
        return match.group(1)
    return None


def find_ambiguous_method_calls(content, methods):
    """Find calls like variable.method() that might be ambiguous."""
    # Pattern: identifier.method_name(
    ambiguous = []
    for method in methods:
        pattern = rf'\b(\w+)\.{method}\s*\('
        for match in re.finditer(pattern, content):
            var_name = match.group(1)
            # Skip 'self' - that's usually in impl blocks and OK
            if var_name != 'self':
                ambiguous.append({
                    'method': method,
                    'var': var_name,
                    'pos': match.start(),
                    'match': match.group(0)
                })
    return ambiguous


def fix_method_call(content, ambiguous, trait_name):
    """Fix a single ambiguous method call using UFCS."""
    old_call = ambiguous['match']  # e.g., "a.length("
    var_name = ambiguous['var']
    method = ambiguous['method']
    
    # Replace: a.length( -> TraitName::length(a,
    new_call = f"{trait_name}::{method}({var_name}, "
    
    # But need to handle case with no other args: a.length() -> TraitName::length(a)
    # Look ahead to see if there's a closing paren
    pos = ambiguous['pos']
    after = content[pos + len(old_call):pos + len(old_call) + 10]
    
    if after.startswith(')'):
        # No args: a.length() -> TraitName::length(a)
        new_call = f"{trait_name}::{method}({var_name})"
        old_call = old_call[:-1] + ')'  # Include the closing paren
    
    return content.replace(old_call, new_call, 1)


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Fix ambiguous method calls")
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--methods', required=True, help='Comma-separated list of method names')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    methods = [m.strip() for m in args.methods.split(',')]
    
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return 1
    
    # Find local trait name
    trait_name = find_local_trait_name(content)
    if not trait_name:
        print(f"No trait found in {file_path}")
        return 1
    
    print(f"Using trait: {trait_name}")
    
    # Find ambiguous calls
    ambiguous_calls = find_ambiguous_method_calls(content, methods)
    
    if not ambiguous_calls:
        print(f"No ambiguous calls found in {file_path}")
        return 0
    
    print(f"Found {len(ambiguous_calls)} ambiguous calls")
    
    if args.dry_run:
        for call in ambiguous_calls:
            print(f"  {call['var']}.{call['method']}(...)")
        return 0
    
    # Fix each call
    new_content = content
    # Sort by position in reverse to avoid position shifts
    for call in sorted(ambiguous_calls, key=lambda x: x['pos'], reverse=True):
        new_content = fix_method_call(new_content, call, trait_name)
        print(f"  Fixed: {call['var']}.{call['method']}(...)")
    
    try:
        with open(file_path, 'w') as f:
            f.write(new_content)
        print(f"✓ Fixed {file_path}")
        return 0
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())

