#!/usr/bin/env python3
"""
Fix imports for ArraySeqStEph to use wildcard import to bring trait into scope.
Changes:
  use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
to:
  use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
"""
# Git commit: 25ae22c50a0fcef6ba643cf969f9c755e1f73eab
# Date: 2025-10-18

import re
import sys
from pathlib import Path


def fix_import(file_path):
    """Fix ArraySeqStEph import to use wildcard."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False
    
    original = content
    
    # Pattern: use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
    pattern = r'use\s+crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS\s*;'
    replacement = 'use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;'
    
    if re.search(pattern, content):
        content = re.sub(pattern, replacement, content)
        
        try:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"✓ Fixed {file_path}")
            return True
        except Exception as e:
            with open(file_path, 'w') as f:
                f.write(original)
            print(f"Error writing {file_path}: {e}")
            return False
    else:
        print(f"- Skipped {file_path} (no matching import)")
        return False


def main():
    if len(sys.argv) < 2:
        print("Usage: fix_arrayseqsteph_imports.py <file1> [file2] ...")
        return 1
    
    files = sys.argv[1:]
    fixed = 0
    
    for file_path in files:
        if fix_import(Path(file_path)):
            fixed += 1
    
    print(f"\nFixed {fixed}/{len(files)} file(s)")
    return 0


if __name__ == '__main__':
    sys.exit(main())

