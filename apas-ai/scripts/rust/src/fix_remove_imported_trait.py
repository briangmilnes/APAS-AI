#!/usr/bin/env python3
"""
Remove trait imports from Chap19 files and use fully qualified paths in UFCS.

Strategy:
1. Remove trait from use statement (keep only struct import)
2. Replace UFCS calls to use fully qualified path
   From: ArraySeqMtEphTraitChap18::length(self)
   To: crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphTrait::length(self)
"""
# Git commit: 509549c
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Remove imported trait to avoid ambiguity")
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Detect which chapter this is (Chap19/ArraySeqMtEph.rs -> ArraySeqMtEph)
    filename = file_path.stem  # ArraySeqMtEph
    
    # Pattern: use crate::Chap18::MODULE::MODULE::{STRUCT as ..., TRAIT as ...};
    # Remove the TRAIT import
    pattern = rf'(use crate::Chap18::{filename}::{filename}::\{{\s*{filename}S as {filename}SChap18),\s*{filename}Trait as {filename}TraitChap18,?\s*\}};'
    
    replacement = r'\1};'
    
    new_content = re.sub(pattern, replacement, content)
    
    if new_content == content:
        print(f"No changes needed in {file_path}")
        return 0
    
    # Now replace UFCS calls
    # From: ArraySeqMtEphTraitChap18::method
    # To: crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphTrait::method
    
    trait_alias = f"{filename}TraitChap18"
    full_path = f"crate::Chap18::{filename}::{filename}::{filename}Trait"
    
    new_content = new_content.replace(f"{trait_alias}::", f"{full_path}::")
    
    if args.dry_run:
        print(f"Would update {file_path}:")
        print(f"  - Remove trait import from use statement")
        print(f"  - Replace {trait_alias}:: with {full_path}::")
        return 0
    
    with open(file_path, 'w') as f:
        f.write(new_content)
    
    print(f"✓ Fixed {file_path}")
    return 0


if __name__ == '__main__':
    sys.exit(main())

