#!/usr/bin/env python3
"""
Review: Functional modules need typeless traits.

APASRules.md Lines 103-175: "Purely functional modules (only free functions)
must define a typeless trait with signatures matching the free functions."
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    src_dir = repo_root / "src"
    
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    candidates = []
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check if this is a functional module (only pub fn, no struct/enum/type/impl)
        has_pub_fn = 'pub fn ' in content
        has_struct = re.search(r'\bstruct\s+\w+', content)
        has_enum = re.search(r'\benum\s+\w+', content)
        has_type_alias = re.search(r'\btype\s+\w+\s*=', content)
        has_impl = re.search(r'\bimpl\s+', content)
        has_trait = re.search(r'trait\s+\w+Trait', content)
        
        if has_pub_fn and not (has_struct or has_enum or has_type_alias or has_impl):
            # This is a functional module
            if not has_trait:
                rel_path = src_file.relative_to(repo_root)
                # Count public functions
                pub_fn_count = len(re.findall(r'pub fn \w+', content))
                candidates.append((rel_path, pub_fn_count))
    
    if candidates:
        print("⚠ Functional modules without typeless traits (APASRules.md Lines 103-175):\n")
        print("Note: These modules contain only free functions and should have a trait.\n")
        for file_path, fn_count in candidates:
            print(f"  {file_path}")
            print(f"    {fn_count} public function(s)")
            print()
        print(f"Total candidates: {len(candidates)}")
        print(f"Total violations: {len(candidates)}")
        print("\nRequired: Add trait with signatures matching all public functions.")
        print("Comment: // A dummy trait as a minimal type checking comment and space for algorithmic analysis.")
        return 1
    
    print("✓ All functional modules have appropriate traits")
    return 0


if __name__ == "__main__":
    sys.exit(main())

