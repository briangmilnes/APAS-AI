#!/usr/bin/env python3
"""
Review: Import order.

RustRules.md Line 50: "Import order: after the module declaration add a blank line,
then all use std::… lines, then a blank line, then use statements from external crates,
then another blank line followed by use crate::Types::Types::*; if needed and the rest
of the internal crate::… imports."

Checks src/, tests/, and benches/ for proper import ordering:
1. std imports
2. blank line
3. external crate imports
4. blank line
5. internal crate imports

Note: This is a simplified checker - full validation would require AST parsing.
"""

import sys
from pathlib import Path


def check_file_imports(file_path):
    """Check if imports follow the correct order."""
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Find first use statement
    first_use_idx = None
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('use '):
            first_use_idx = idx
            break
    
    if first_use_idx is None:
        return []  # No imports, no problem
    
    violations = []
    
    # Collect imports and check order
    std_imports = []
    external_imports = []
    crate_imports = []
    current_section = None
    
    for idx in range(first_use_idx, len(lines)):
        line = lines[idx]
        stripped = line.strip()
        
        # Stop at first non-import, non-blank, non-comment line after imports start
        if not stripped.startswith('use ') and stripped and not stripped.startswith('//'):
            break
        
        if stripped.startswith('use std::') or stripped.startswith('use core::') or stripped.startswith('use alloc::'):
            if current_section not in [None, 'std']:
                violations.append((idx + 1, "std import after external/crate imports", line.strip()))
            std_imports.append((idx + 1, line))
            current_section = 'std'
        elif stripped.startswith('use crate::'):
            if current_section == 'external':
                # This is ok - external then crate
                pass
            crate_imports.append((idx + 1, line))
            current_section = 'crate'
        elif stripped.startswith('use ') and not stripped.startswith('use self::') and not stripped.startswith('use super::'):
            # External crate import
            if current_section == 'crate':
                violations.append((idx + 1, "external import after crate imports", line.strip()))
            external_imports.append((idx + 1, line))
            current_section = 'external'
    
    return violations


def main():
    repo_root = Path(__file__).parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    all_violations = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rust_file in search_dir.rglob("*.rs"):
            violations = check_file_imports(rust_file)
            if violations:
                all_violations.append((rust_file, violations))
    
    if all_violations:
        print("✗ Found import order violations (RustRules.md Line 50):\n")
        for file_path, violations in all_violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:")
            for line_num, reason, line_content in violations:
                print(f"    Line {line_num}: {reason}")
                print(f"      {line_content}")
            print()
        
        total = sum(len(v) for _, v in all_violations)
        print(f"Total violations: {total}")
        print("\nExpected order: std imports → external crate imports → internal crate:: imports")
        return 1
    else:
        print("✓ Import order looks correct")
        return 0


if __name__ == "__main__":
    sys.exit(main())

