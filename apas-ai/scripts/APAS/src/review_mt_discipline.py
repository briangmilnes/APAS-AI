#!/usr/bin/env python3
"""
Review: MT module discipline.

APASRules.md Lines 44-47: "*Mt* files must use MtT elements, not StT shortcuts."
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
    
    # Find all *Mt*.rs files
    mt_files = [f for f in src_dir.rglob("*.rs") if 'Mt' in f.stem]
    
    if not mt_files:
        print("✓ No *Mt* files found")
        return 0
    
    stt_pattern = re.compile(r'\b(T:\s*StT|<T:\s*StT>|where\s+T:\s*StT)\b')
    
    for mt_file in mt_files:
        with open(mt_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, start=1):
                stripped = line.strip()
                
                # Skip comments
                if stripped.startswith('//'):
                    continue
                
                # Check for StT usage in MT files
                if stt_pattern.search(line):
                    rel_path = mt_file.relative_to(repo_root)
                    violations.append((rel_path, line_num, stripped))
    
    if violations:
        print("✗ MT modules using StT instead of MtT (APASRules.md Lines 44-47):\n")
        for file_path, line_num, line_content in violations:
            print(f"  {file_path}:{line_num}")
            print(f"    {line_content[:80]}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: *Mt* files must use MtT (Send + Sync), not StT.")
        return 1
    
    print(f"✓ All {len(mt_files)} *Mt* files properly use MtT")
    return 0


if __name__ == "__main__":
    sys.exit(main())

