#!/usr/bin/env python3
"""
Review: APAS where clause simplification.

APASRules.md Lines 96-101: "Use Pred<T> instead of Fn(&T) -> B, apply type abbreviations."
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
    
    # Pattern for Fn(&T) -> B that should be Pred<T>
    fn_pred_pattern = re.compile(r'Fn\s*\(\s*&\s*\w+\s*\)\s*->\s*B\b')
    
    # Pattern for redundant 'static when already MtVal
    redundant_static = re.compile(r'where.*T:\s*MtVal.*T:\s*\'static|where.*T:\s*\'static.*T:\s*MtVal')
    
    for src_file in src_dir.rglob("*.rs"):
        # Skip Types.rs - it defines PredSt and PredMt
        if src_file.name == "Types.rs":
            continue
            
        with open(src_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, start=1):
                stripped = line.strip()
                
                # Skip comments
                if stripped.startswith('//'):
                    continue
                
                # Check for Fn(&T) -> B pattern
                if fn_pred_pattern.search(line):
                    rel_path = src_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped, "Use Pred<T> instead of Fn(&T) -> B"))
                
                # Check for redundant 'static
                if redundant_static.search(line):
                    rel_path = src_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped, "Redundant 'static (MtVal includes it)"))
    
    if violations:
        print("✗ Found APAS where clause simplification opportunities (APASRules.md Lines 96-101):\n")
        for file_path, line_num, line_content, reason in violations:
            print(f"  {file_path}:{line_num}")
            print(f"    {reason}")
            print(f"    {line_content[:80]}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nSimplifications:")
        print("  - Fn(&T) -> B → Pred<T>")
        print("  - Remove T: 'static when T: MtVal (already includes 'static)")
        return 1
    
    print("✓ No APAS where clause simplification opportunities found")
    return 0


if __name__ == "__main__":
    sys.exit(main())

