#!/usr/bin/env python3
"""
Move method implementations from inherent impl to trait impl.

Replaces delegation pattern with actual implementation.
"""
# Git commit: e8e8f18
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def find_method_in_impl(content, method_name):
    """Find a method definition in content and return its full text and body."""
    # Match: fn method_name<generics>(params) -> return { body }
    # Need to handle multi-line, generics, where clauses, etc.
    
    pattern = rf'\b(pub\s+)?fn\s+{method_name}\s*(<[^>]*>)?\s*\('
    match = re.search(pattern, content)
    
    if not match:
        return None
    
    # Find the opening brace
    start = match.start()
    brace_search_start = match.end()
    brace_pos = content.find('{', brace_search_start)
    
    if brace_pos == -1:
        return None
    
    # Extract everything from method start to opening brace
    signature = content[start:brace_pos].strip()
    
    # Extract the body by counting braces
    brace_count = 1
    i = brace_pos + 1
    
    while i < len(content) and brace_count > 0:
        if content[i] == '{':
            brace_count += 1
        elif content[i] == '}':
            brace_count -= 1
        i += 1
    
    # Body is everything inside the outer braces
    body_with_braces = content[brace_pos:i]
    body = body_with_braces[1:-1]  # Strip outer braces
    
    # Full method text
    full = content[start:i]
    
    return {
        'full': full,
        'signature': signature,
        'body': body,
        'start': start,
        'end': i,
    }


def fix_file(file_path, dry_run=False):
    """Fix delegation patterns in a file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    # Find trait impl and inherent impl
    trait_impl_pattern = r'impl<([^>]*)>\s+(\w+)<([^>]*)>\s+for\s+(\w+)<([^>]*)>\s*\{'
    match = re.search(trait_impl_pattern, content)
    
    if not match:
        print(f"No trait impl found in {file_path}", file=sys.stderr)
        return False
    
    trait_name = match.group(2)
    struct_name = match.group(4)
    
    # Find trait impl block
    trait_impl_start = match.start()
    brace_pos = content.find('{', match.end() - 1)
    brace_count = 1
    i = brace_pos + 1
    
    while i < len(content) and brace_count > 0:
        if content[i] == '{':
            brace_count += 1
        elif content[i] == '}':
            brace_count -= 1
        i += 1
    
    trait_impl_end = i
    trait_impl_content = content[trait_impl_start:trait_impl_end]
    
    # Find inherent impl block
    inherent_pattern = rf'impl<[^>]*>\s+{struct_name}<[^>]*>\s*\{{'
    inherent_match = re.search(inherent_pattern, content)
    
    if not inherent_match:
        print(f"No inherent impl found for {struct_name} in {file_path}", file=sys.stderr)
        return False
    
    inherent_start = inherent_match.start()
    brace_pos = content.find('{', inherent_match.end() - 1)
    brace_count = 1
    i = brace_pos + 1
    
    while i < len(content) and brace_count > 0:
        if content[i] == '{':
            brace_count += 1
        elif content[i] == '}':
            brace_count -= 1
        i += 1
    
    inherent_end = i
    inherent_content = content[inherent_start:inherent_end]
    
    # Find delegating methods
    delegation_pattern = rf'{struct_name}::(\w+)\('
    delegations = set()
    
    for deleg_match in re.finditer(delegation_pattern, trait_impl_content):
        method_name = deleg_match.group(1)
        delegations.add(method_name)
    
    if not delegations:
        print(f"No delegations found in {file_path}")
        return False
    
    # Process each delegation
    new_trait_impl = trait_impl_content
    changes_made = False
    
    for method_name in sorted(delegations):
        # Find method in inherent impl
        inherent_method = find_method_in_impl(inherent_content, method_name)
        if not inherent_method:
            print(f"Warning: Could not find {method_name} in inherent impl", file=sys.stderr)
            continue
        
        # Find delegating call in trait impl
        # Pattern: fn method_name(...) { StructName::method_name(...) }
        # Need to handle multi-line
        trait_method_pattern = rf'(fn\s+{method_name}\s*(?:<[^>]*>)?\s*\([^)]*\)(?:\s*->\s*[^{{]*)?)\s*\{{\s*{struct_name}::{method_name}\([^)]*\)\s*\}}'
        
        trait_match = re.search(trait_method_pattern, new_trait_impl, re.DOTALL)
        
        if not trait_match:
            # Try multi-line variant
            trait_method_pattern_ml = rf'(fn\s+{method_name}\s*(?:<[^>]*>)?\s*\([^{{]*)\{{\s*{struct_name}::{method_name}\([^}}]*\)\s*\}}'
            trait_match = re.search(trait_method_pattern_ml, new_trait_impl, re.DOTALL)
        
        if not trait_match:
            print(f"Warning: Could not find delegation for {method_name} in trait impl", file=sys.stderr)
            continue
        
        # Extract just the signature from the trait method (everything before {)
        trait_method_text = trait_match.group(0)
        trait_sig_match = trait_match.group(1)
        
        # Build new method: trait signature + inherent body
        new_method = trait_sig_match.strip() + ' {' + inherent_method['body'] + '}'
        
        # Replace in trait impl
        new_trait_impl = new_trait_impl.replace(trait_method_text, new_method, 1)
        changes_made = True
        
        if dry_run:
            print(f"Would move {method_name} implementation from inherent to trait impl")
    
    if not changes_made:
        return False
    
    if dry_run:
        print(f"\nWould update {file_path}")
        return True
    
    # Replace trait impl in content
    new_content = content[:trait_impl_start] + new_trait_impl + content[trait_impl_end:]
    
    try:
        with open(file_path, 'w') as f:
            f.write(new_content)
        print(f"Fixed: {file_path}")
        print(f"Moved {len([m for m in delegations if find_method_in_impl(inherent_content, m)])} methods")
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Move inherent impl methods to trait impl")
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    changed = fix_file(file_path, dry_run=args.dry_run)
    return 0 if changed else 1


if __name__ == '__main__':
    sys.exit(main())

