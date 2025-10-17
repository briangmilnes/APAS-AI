#!/usr/bin/env python3
"""
Review: Parallel spawn/join model (no rayon, no thresholds).

APASRules.md Lines 39-42: "Use std::thread::spawn, avoid rayon, no PARALLEL_THRESHOLD."
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
    
    rayon_pattern = re.compile(r'\brayon\b')
    threshold_pattern = re.compile(r'\b(PARALLEL_THRESHOLD|parallel_threshold|threshold)\b', re.IGNORECASE)
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, start=1):
                stripped = line.strip()
                
                # Skip comments
                if stripped.startswith('//'):
                    continue
                
                # Check for rayon usage
                if rayon_pattern.search(line):
                    rel_path = src_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped, "rayon usage"))
                
                # Check for threshold checks
                if threshold_pattern.search(line):
                    # Skip if it's in a comment or string
                    if '//' not in line[:line.find('threshold')] if 'threshold' in line.lower() else True:
                        rel_path = src_file.relative_to(repo_root)
                        violations.append((rel_path, line_num, stripped, "threshold check"))
    
    if violations:
        print("✗ Found parallel model violations (APASRules.md Lines 39-42):\n")
        for file_path, line_num, line_content, reason in violations:
            print(f"  {file_path}:{line_num} - {reason}")
            print(f"    {line_content[:80]}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Use std::thread::spawn/join, no rayon, no thresholds.")
        print("APAS parallel algorithms should always use parallel structure.")
        return 1
    
    print("✓ No rayon or threshold patterns found")
    return 0


if __name__ == "__main__":
    sys.exit(main())

