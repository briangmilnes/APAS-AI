#!/usr/bin/env python3
"""Fix SetLit! tuple patterns in test files to use Triple wrapper."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

def fix_file(filepath):
    """Fix SetLit! tuple patterns in a test file."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # Pattern: (digit, digit, OrderedF64::from(...))
    content = re.sub(
        r'\((\d+), (\d+), (OrderedF64::from\([^)]+\))\)',
        r'Triple(\1, \2, \3)',
        content
    )
    
    # Pattern: (digit, digit, OrderedFloat(...))
    content = re.sub(
        r'\((\d+), (\d+), (OrderedFloat\([^)]+\))\)',
        r'Triple(\1, \2, \3)',
        content
    )
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Fixed: {filepath}")
        return True
    else:
        print(f"No changes: {filepath}")
        return False

if __name__ == "__main__":
    test_files = [
        "tests/Chap58/TestBellmanFordStEphFloat.rs",
        "tests/Chap59/TestJohnsonStEphFloat.rs",
    ]
    
    repo_root = Path(__file__).parent.parent.parent
    
    for test_file in test_files:
        filepath = repo_root / test_file
        if filepath.exists():
            fix_file(filepath)
        else:
            print(f"Not found: {filepath}")


