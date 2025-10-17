#!/usr/bin/env python3
"""
Review: No Factory pattern names.

APASRules.md Lines 176-181: "NEVER use 'Factory' in struct, trait, or function names.
This is a Java anti-pattern."
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
    factory_pattern = re.compile(r'\b[Ff]actory\b')
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, start=1):
                # Skip comments
                stripped = line.strip()
                if stripped.startswith('//'):
                    continue
                
                # Check for Factory in code
                if factory_pattern.search(line):
                    rel_path = src_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped))
    
    if violations:
        print("✗ Found 'Factory' pattern usage (APASRules.md Lines 176-181):\n")
        for file_path, line_num, line_content in violations:
            print(f"  {file_path}:{line_num}")
            print(f"    {line_content}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Use direct constructors, builder patterns, or free functions.")
        print("Instead of: LinearProbingFactory::create() → Use: LinearProbing::new()")
        return 1
    
    print("✓ No 'Factory' pattern found")
    return 0


if __name__ == "__main__":
    sys.exit(main())

