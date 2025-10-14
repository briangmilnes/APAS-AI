#!/usr/bin/env python3
"""
Review: CamelCase naming convention.

RustRules.md Lines 303-306:
- Functions/structures of more than one English word use CamelCase
- One-word functions may be all lower case
- File names should be in CamelCase and start with a capital

Checks file names in src/, tests/, and benches/ for CamelCase convention.
"""

import re
import sys
from pathlib import Path


def is_camelcase(name):
    """Check if a name follows CamelCase convention (starts with capital)."""
    # Remove .rs extension
    if name.endswith('.rs'):
        name = name[:-3]
    
    # Should start with capital letter
    if not name[0].isupper():
        return False
    
    return True


def main():
    repo_root = Path(__file__).parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    violations = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rust_file in search_dir.rglob("*.rs"):
            filename = rust_file.name
            
            # Skip special files
            if filename in ['lib.rs', 'main.rs', 'mod.rs']:
                continue
            
            # Check if filename is CamelCase
            if not is_camelcase(filename):
                violations.append((rust_file, filename))
    
    if violations:
        print("✗ Found non-CamelCase file names (RustRules.md Lines 303-306):\n")
        for file_path, filename in violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}")
            print(f"    File '{filename}' should start with capital letter")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Rename files to start with capital letter (e.g., 'myFile.rs' → 'MyFile.rs').")
        return 1
    else:
        print("✓ All file names follow CamelCase convention")
        return 0


if __name__ == "__main__":
    sys.exit(main())

