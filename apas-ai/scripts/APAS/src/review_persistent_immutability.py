#!/usr/bin/env python3
"""
Review: Persistent data structures must be immutable.

APASRules.md Lines 49-53: "*Per files must not expose in-place mutators like set/update.
No &mut self methods, no slices &[T] or &mut [T]."
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
    
    violations = []
    
    # Find all *Per.rs files
    per_files = list(src_dir.rglob("*Per.rs"))
    
    if not per_files:
        print("✓ No *Per files found")
        return 0
    
    for per_file in per_files:
        with open(per_file, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        in_impl = False
        in_pub_mod = False
        
        for line_num, line in enumerate(lines, start=1):
            stripped = line.strip()
            
            # Track pub mod and impl blocks
            if stripped.startswith('pub mod '):
                in_pub_mod = True
            if stripped.startswith('impl '):
                in_impl = True
            
            if in_impl or in_pub_mod:
                # Check for &mut self (in-place mutation)
                if '&mut self' in line:
                    rel_path = per_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped, "in-place mutation (&mut self)"))
                
                # Check for set/update methods
                if re.search(r'fn (set|update|insert_in_place)\s*\(', line):
                    rel_path = per_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped, "mutable method"))
                
                # Check for exposed slices
                if re.search(r'fn \w+.*->\s*&\s*\[\w+\]', line) or re.search(r'fn \w+.*->\s*&mut\s*\[\w+\]', line):
                    rel_path = per_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped, "exposed slice"))
            
            # Track end of blocks
            if '}' in line and not stripped.startswith('//'):
                if in_impl:
                    in_impl = False
                if in_pub_mod and line.count('}') > line.count('{'):
                    in_pub_mod = False
    
    if violations:
        print("✗ Persistent (*Per) files have mutability violations (APASRules.md Lines 49-53):\n")
        for file_path, line_num, line_content, reason in violations:
            print(f"  {file_path}:{line_num} - {reason}")
            print(f"    {line_content[:80]}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Persistent files must return new values, not mutate in place.")
        return 1
    
    print(f"✓ All {len(per_files)} *Per files are properly immutable")
    return 0


if __name__ == "__main__":
    sys.exit(main())

