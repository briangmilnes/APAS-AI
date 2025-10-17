#!/usr/bin/env python3
"""
Review: No duplicate benchmark names in Cargo.toml.

Each [[bench]] entry must have a unique name.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path
from collections import defaultdict


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    cargo_toml = repo_root / "Cargo.toml"
    
    with open(cargo_toml, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Find all [[bench]] entries with name and path
    # Pattern: [[bench]]\nname = "X"\npath = "Y"
    bench_pattern = re.compile(
        r'\[\[bench\]\]\s*name\s*=\s*"([^"]+)"\s*path\s*=\s*"([^"]+)"',
        re.MULTILINE
    )
    
    bench_entries = bench_pattern.findall(content)
    
    # Group by name to find duplicates
    by_name = defaultdict(list)
    for name, path in bench_entries:
        by_name[name].append(path)
    
    # Find duplicates
    duplicates = {name: paths for name, paths in by_name.items() if len(paths) > 1}
    
    if duplicates:
        print("✗ Found duplicate benchmark names in Cargo.toml:\n")
        for name, paths in sorted(duplicates.items()):
            print(f"  name = \"{name}\" appears {len(paths)} times:")
            for path in paths:
                print(f"    - {path}")
            print()
        
        total_violations = sum(len(paths) - 1 for paths in duplicates.values())
        print(f"Total violations: {total_violations}")
        print("\nFix: Each benchmark must have a unique name.")
        print("Suggestion: Add chapter suffix like 'BenchFooChap37' and 'BenchFooChap38'")
        return 1
    
    print(f"✓ All {len(bench_entries)} benchmark names are unique in Cargo.toml")
    return 0


if __name__ == "__main__":
    sys.exit(main())

