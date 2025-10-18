#!/usr/bin/env python3
"""
Find all inherent impl blocks in the codebase (excluding Types.rs)

Inherent impls are: impl StructName { ... }
NOT trait impls: impl Trait for StructName { ... }
"""

import re
import os
import sys

def find_inherent_impls(src_dir="src"):
    """Find all inherent impl blocks"""
    
    with_generics = []
    without_generics = []
    
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if not file.endswith(".rs") or file == "Types.rs":
                continue
            
            filepath = os.path.join(root, file)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    for i, line in enumerate(f, 1):
                        # Match: impl<...> TypeName { or impl TypeName {
                        # But NOT: impl ... for ...
                        if re.match(r'^\s+impl', line) and ' for ' not in line:
                            stripped = line.strip()
                            # Check if it has generics
                            if '<' in stripped and '>' in stripped:
                                with_generics.append(f"{filepath}:{i}:{line.rstrip()}")
                            else:
                                without_generics.append(f"{filepath}:{i}:{line.rstrip()}")
            except Exception as e:
                print(f"Error reading {filepath}: {e}", file=sys.stderr)
    
    return with_generics, without_generics

def main():
    print("INHERENT IMPL BLOCKS (excluding Types.rs)")
    print("=" * 60)
    print()
    
    with_generics, without_generics = find_inherent_impls()
    
    print("WITH GENERICS (custom trait bounds):")
    print("-" * 40)
    for item in sorted(with_generics):
        print(item)
    
    print()
    print("WITHOUT GENERICS (likely utility types):")
    print("-" * 40)
    for item in sorted(without_generics):
        print(item)
    
    print()
    print(f"TOTAL: {len(with_generics) + len(without_generics)} inherent impl blocks")
    
    # Also output just file list
    print()
    print("=" * 60)
    print("FILES WITH INHERENT IMPLS:")
    print("-" * 40)
    all_files = set()
    for item in with_generics + without_generics:
        filepath = item.split(':')[0]
        all_files.add(filepath)
    
    for filepath in sorted(all_files):
        print(filepath)
    
    print()
    print(f"TOTAL FILES: {len(all_files)}")

if __name__ == "__main__":
    main()

