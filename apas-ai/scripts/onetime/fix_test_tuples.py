#!/usr/bin/env python3
"""Fix tuple insertions in test files to use Triple wrapper."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

def fix_file(filepath):
    """Fix tuple constructions in a test file."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # Pattern 1: edges.insert((a, b, OrderedFloat(...)))
    content = re.sub(
        r'edges\.insert\(\((\d+), (\d+), (OrderedFloat\([^)]+\))\)\)',
        r'edges.insert(Triple(\1, \2, \3))',
        content
    )
    
    # Pattern 2: edges.insert((a, b, c)) where c is a numeric literal
    content = re.sub(
        r'edges\.insert\(\((\d+), (\d+), (-?\d+(?:\.\d+)?)\)\)',
        r'edges.insert(Triple(\1, \2, \3))',
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
        "tests/Chap57/TestDijkstraStEphFloat.rs",
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


