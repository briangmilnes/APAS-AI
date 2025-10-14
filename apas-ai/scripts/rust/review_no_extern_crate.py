#!/usr/bin/env python3
"""
Review: No 'extern crate' usage.

RustRules.md Line 86: "Never use extern crate. Do not add re-exports."

Checks all Rust source files in src/, tests/, and benches/ for 'extern crate' usage.
"""

import sys
from pathlib import Path


def main():
    repo_root = Path(__file__).parent.parent.parent
    
    # Directories to check
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
            with open(rust_file, 'r', encoding='utf-8') as f:
                for line_num, line in enumerate(f, start=1):
                    # Check for 'extern crate' (ignoring comments)
                    stripped = line.strip()
                    if stripped.startswith('//'):
                        continue
                    if 'extern crate' in line:
                        violations.append((rust_file, line_num, line.strip()))
    
    if violations:
        print("✗ Found 'extern crate' usage (RustRules.md Line 86):\n")
        for file_path, line_num, line_content in violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:{line_num}")
            print(f"    {line_content}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Remove 'extern crate' statements. Use 'use' statements instead.")
        return 1
    else:
        print("✓ No 'extern crate' usage found")
        return 0


if __name__ == "__main__":
    sys.exit(main())

