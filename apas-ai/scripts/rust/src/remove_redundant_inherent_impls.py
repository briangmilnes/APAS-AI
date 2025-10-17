#!/usr/bin/env python3
"""
Remove inherent impl blocks that have been fully inlined into trait impls.

This script removes inherent impl blocks where all methods have been moved to trait impls.
"""
# Git commit: 509549c
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def find_inherent_impl_block(content, struct_name):
    """Find the inherent impl block for a struct."""
    # Pattern: impl<...> StructName<...> {
    pattern = rf'impl<[^>]*>\s+{struct_name}<[^>]*>\s*\{{'
    
    match = re.search(pattern, content)
    if not match:
        return None
    
    # Find the block boundaries
    start = match.start()
    brace_pos = content.find('{', match.end() - 1)
    brace_count = 1
    i = brace_pos + 1
    
    while i < len(content) and brace_count > 0:
        if content[i] == '{':
            brace_count += 1
        elif content[i] == '}':
            brace_count -= 1
        i += 1
    
    end = i
    
    return {
        'start': start,
        'end': end,
        'content': content[start:end]
    }


def find_trait_impl_struct_name(content):
    """Find the struct name from a trait impl."""
    # Pattern: impl<...> SomeTrait<...> for StructName<...>
    pattern = r'impl<[^>]*>\s+\w+<[^>]*>\s+for\s+(\w+)<'
    match = re.search(pattern, content)
    if match:
        return match.group(1)
    return None


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Remove redundant inherent impls")
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return 1
    
    # Find the struct name from trait impl
    struct_name = find_trait_impl_struct_name(content)
    if not struct_name:
        print(f"No trait impl found in {file_path}", file=sys.stderr)
        return 1
    
    # Find the inherent impl block
    inherent_impl = find_inherent_impl_block(content, struct_name)
    if not inherent_impl:
        print(f"No inherent impl found for {struct_name} in {file_path}")
        return 1
    
    if args.dry_run:
        print(f"Would remove inherent impl for {struct_name} from {file_path}")
        print(f"Lines to remove: {inherent_impl['start']}-{inherent_impl['end']}")
        return 0
    
    # Remove the inherent impl block (including surrounding blank lines)
    new_content = content[:inherent_impl['start']] + content[inherent_impl['end']:]
    
    # Clean up multiple blank lines
    new_content = re.sub(r'\n\n\n+', '\n\n', new_content)
    
    try:
        with open(file_path, 'w') as f:
            f.write(new_content)
        print(f"Removed inherent impl for {struct_name} from {file_path}")
        return 0
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())

