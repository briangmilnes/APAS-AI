#!/usr/bin/env python3
"""
Review: Graph notation convention.

APASRules.md Lines 60-72: "Directed graphs use A: (arcs), undirected use E: (edges)."
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    violations = []
    
    # Patterns for graph macros
    # Directed graphs should use A: (not E:) - match DirGraph but NOT UnDirGraph
    # Use word boundary and explicit non-UnDir check
    dir_with_e = re.compile(r'(?<!Un)(DirGraph|WeightedDirGraph)(?!UnDir)\w*Lit!\s*\([^)]*\bE:\s*\[')
    # Undirected graphs should use E: (not A:)
    undir_with_a = re.compile(r'(UnDirGraph|WeightedUnDirGraph)\w*Lit!\s*\([^)]*\bA:\s*\[')
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rs_file in search_dir.rglob("*.rs"):
            with open(rs_file, 'r', encoding='utf-8') as f:
                content = f.read()
                lines = content.split('\n')
            
            for line_num, line in enumerate(lines, start=1):
                # Check directed graphs using E: instead of A:
                if dir_with_e.search(line):
                    rel_path = rs_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, line.strip(), "Directed graph using E: (should be A:)"))
                
                # Check undirected graphs using A: instead of E:
                if undir_with_a.search(line):
                    rel_path = rs_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, line.strip(), "Undirected graph using A: (should be E:)"))
    
    if violations:
        print("✗ Found graph notation violations (APASRules.md Lines 60-72):\n")
        for file_path, line_num, line_content, reason in violations:
            print(f"  {file_path}:{line_num}")
            print(f"    {reason}")
            print(f"    {line_content[:80]}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nConvention:")
        print("  - Directed graphs: use A: for arcs")
        print("  - Undirected graphs: use E: for edges")
        return 1
    
    print("✓ All graph macros use correct notation (A: for directed, E: for undirected)")
    return 0


if __name__ == "__main__":
    sys.exit(main())

