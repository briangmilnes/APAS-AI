#!/usr/bin/env python3
"""
Review: Suspicious Vec usage (for manual review).

APASRules.md Lines 3-16: "Callers must never gain new Vec usage. Inside modules,
Vec only for temporary builders or internal scratch space."

Note: This check identifies candidates for review, not automatic violations.
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
    
    candidates = []
    
    # Patterns that might indicate problematic Vec usage
    vec_new = re.compile(r'\bVec::new\(')
    vec_with_capacity = re.compile(r'\bVec::with_capacity\(')
    vec_macro = re.compile(r'\bvec!\[')
    to_vec = re.compile(r'\.to_vec\(\)')
    into_vec = re.compile(r'\.into_vec\(\)')
    vec_return = re.compile(r'->\s*Vec<')
    vec_param = re.compile(r':\s*Vec<')
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, start=1):
                stripped = line.strip()
                
                # Skip comments
                if stripped.startswith('//'):
                    continue
                
                # Check for Vec usage patterns
                if (vec_new.search(line) or vec_with_capacity.search(line) or 
                    vec_macro.search(line) or to_vec.search(line) or into_vec.search(line)):
                    # Skip if it's clearly for from_vec/to_vec conversions
                    if 'from_vec' in line or 'ArraySeq' in line or 'convert' in line.lower():
                        continue
                    
                    rel_path = src_file.relative_to(repo_root)
                    candidates.append((rel_path, line_num, stripped, "Vec construction/conversion"))
                
                # Check for Vec in return types or parameters (public API exposure)
                if vec_return.search(line) or vec_param.search(line):
                    if 'pub fn' in line:
                        rel_path = src_file.relative_to(repo_root)
                        candidates.append((rel_path, line_num, stripped, "Vec in public API"))
    
    if candidates:
        print("⚠ Found Vec usage candidates for review (APASRules.md Lines 3-16):\n")
        print("Note: These need manual review - some may be legitimate.\n")
        for file_path, line_num, line_content, reason in candidates:
            print(f"  {file_path}:{line_num} - {reason}")
            print(f"    {line_content[:80]}")
            print()
        print(f"Total candidates: {len(candidates)}")
        print(f"Total violations: {len(candidates)}")
        print("\nReview: Vec allowed only for temporary builders or internal scratch space.")
        return 1
    
    print("✓ No suspicious Vec usage found")
    return 0


if __name__ == "__main__":
    sys.exit(main())

