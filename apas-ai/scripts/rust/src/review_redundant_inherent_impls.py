#!/usr/bin/env python3
"""
Review files that have redundant inherent impls after inlining.

These files have both:
1. An inherent impl with methods
2. A trait impl that was inlined (contains actual implementations, not just delegation)

The inherent impl is now redundant and should be removed.
"""
# Git commit: 509549c
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def has_inherent_impl(content, struct_name):
    """Check if file has an inherent impl for struct_name."""
    pattern = rf'impl<[^>]*>\s+{struct_name}<[^>]*>\s*\{{'
    return re.search(pattern, content) is not None


def has_trait_impl(content, struct_name):
    """Check if file has a trait impl for struct_name."""
    pattern = rf'impl<[^>]*>\s+\w+<[^>]*>\s+for\s+{struct_name}<[^>]*>\s*\{{'
    return re.search(pattern, content) is not None


def find_struct_name(content):
    """Find the main struct name in a file."""
    pattern = r'pub\s+struct\s+(\w+)<'
    match = re.search(pattern, content)
    if match:
        return match.group(1)
    return None


def main():
    src_dir = Path("src")
    files_with_redundant_impls = []
    
    for rs_file in src_dir.rglob("*.rs"):
        try:
            with open(rs_file, 'r') as f:
                content = f.read()
        except Exception:
            continue
        
        struct_name = find_struct_name(content)
        if not struct_name:
            continue
        
        has_inherent = has_inherent_impl(content, struct_name)
        has_trait = has_trait_impl(content, struct_name)
        
        if has_inherent and has_trait:
            files_with_redundant_impls.append((str(rs_file), struct_name))
    
    if not files_with_redundant_impls:
        print("No redundant inherent impls found.")
        return 1
    
    print(f"Found {len(files_with_redundant_impls)} files with redundant inherent impls:\n")
    
    for file_path, struct_name in sorted(files_with_redundant_impls):
        print(f"{file_path}: {struct_name}")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

