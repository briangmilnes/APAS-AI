#!/usr/bin/env python3
"""
Review: Copyright header on line 1.

APASRules.md Lines 190-195: "Always put this copyright in on line 1:
'//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.'"
"""
# Git commit: 27d5172c6a7bdef7ac1403a7f853f3de2d11fc1b
# Date: 2025-10-14 17:37:55 -0700


import sys
from pathlib import Path


REQUIRED_COPYRIGHT = "//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'."


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    violations = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rs_file in search_dir.rglob("*.rs"):
            with open(rs_file, 'r', encoding='utf-8') as f:
                lines = f.readlines()
            
            if not lines:
                continue
            
            first_line = lines[0].rstrip()
            
            # Check if first line is the copyright
            if first_line != REQUIRED_COPYRIGHT:
                rel_path = rs_file.relative_to(repo_root)
                violations.append((rel_path, first_line[:80] if first_line else "(empty)"))
    
    if violations:
        print("✗ Missing or incorrect copyright header (APASRules.md Lines 190-195):\n")
        for file_path, actual_line in violations:
            print(f"  {file_path}")
            print(f"    Found: {actual_line}")
            print()
        print(f"Total violations: {len(violations)}")
        print(f"\nRequired on line 1:")
        print(f"  {REQUIRED_COPYRIGHT}")
        return 1
    
    print("✓ All files have correct copyright header")
    return 0


if __name__ == "__main__":
    sys.exit(main())

