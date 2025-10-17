#!/usr/bin/env python3
"""
Review: Import order.

RustRules.md Line 50: "Import order: after the module declaration add a blank line,
then all use std::… lines, then a blank line, then use statements from external crates,
then another blank line followed by use crate::Types::Types::*; if needed and the rest
of the internal crate::… imports."

RustRules.md Lines 75-86: "Inside src/ use crate::, outside src/ (tests/benches) use apas_ai::"

Checks:
1. Import ordering: std → external → internal (crate:: or apas_ai::)
2. Blank lines between sections
3. crate:: in src/ vs apas_ai:: in tests/benches/
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def check_file_imports(file_path, repo_root):
    """Check if imports follow the correct order."""
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Determine if file is in src/, tests/, or benches/
    relative_path = file_path.relative_to(repo_root)
    in_src = relative_path.parts[0] == 'src'
    in_tests_benches = relative_path.parts[0] in ('tests', 'benches')
    
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
    
    # Track import sections
    std_section = []
    external_section = []
    internal_section = []
    
    # Parse imports
    i = first_use_idx
    current_section = None
    blank_after_std = False
    blank_after_external = False
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Stop at first non-import, non-blank, non-comment line
        if not stripped.startswith('use ') and stripped and not stripped.startswith('//'):
            break
        
        if stripped.startswith('use std::') or stripped.startswith('use core::') or stripped.startswith('use alloc::'):
            if current_section == 'external':
                violations.append((i + 1, "std import after external imports", line.strip()))
            elif current_section == 'internal':
                violations.append((i + 1, "std import after internal imports", line.strip()))
            std_section.append((i + 1, line))
            current_section = 'std'
            
        elif stripped.startswith('use crate::') or stripped.startswith('use apas_ai::'):
            # Check crate:: vs apas_ai::
            if in_src and stripped.startswith('use apas_ai::'):
                violations.append((i + 1, "use apas_ai:: in src/ (should be crate::)", line.strip()))
            elif in_tests_benches and stripped.startswith('use crate::'):
                violations.append((i + 1, "use crate:: in tests/benches (should be apas_ai::)", line.strip()))
            
            if current_section == 'external' and not blank_after_external:
                violations.append((i + 1, "missing blank line before internal imports", line.strip()))
            
            internal_section.append((i + 1, line))
            current_section = 'internal'
            
        elif stripped.startswith('use ') and not stripped.startswith('use self::') and not stripped.startswith('use super::'):
            # External crate import
            if current_section == 'std' and not blank_after_std:
                violations.append((i + 1, "missing blank line before external imports", line.strip()))
            if current_section == 'internal':
                violations.append((i + 1, "external import after internal imports", line.strip()))
                
            external_section.append((i + 1, line))
            current_section = 'external'
            
        elif not stripped:  # Blank line
            if current_section == 'std' and not blank_after_std:
                blank_after_std = True
            elif current_section == 'external' and not blank_after_external:
                blank_after_external = True
        
        i += 1
    
    # Check for required blank lines
    if std_section and external_section and not blank_after_std:
        violations.append((std_section[-1][0], "missing blank line after std imports", ""))
    if external_section and internal_section and not blank_after_external:
        violations.append((external_section[-1][0], "missing blank line after external imports", ""))
    
    return violations


def main():
    parser = create_review_parser(__doc__)
    args = parser.parse_args()
    context = ReviewContext(args)

    search_dirs = [
        context.repo_root / "src",
        context.repo_root / "tests",
        context.repo_root / "benches",
    ]

    if context.dry_run:
        files = context.find_files(search_dirs)
        print(f"Would check {len(files)} file(s) for import order")
        return 0

    all_violations = []
    files = context.find_files(search_dirs)

    for rust_file in files:
        violations = check_file_imports(rust_file, context.repo_root)
        if violations:
            all_violations.append((rust_file, violations))
    
    if all_violations:
        print("✗ Found import order violations (RustRules.md Lines 50, 75-86):\n")
        for file_path, violations in all_violations:
            rel_path = file_path.relative_to(context.repo_root)
            print(f"  {rel_path}:")
            for line_num, reason, line_content in violations:
                print(f"    Line {line_num}: {reason}")
                if line_content:
                    print(f"      {line_content}")
            print()
        
        total = sum(len(v) for _, v in all_violations)
        print(f"Total violations: {total}")
        print("\nExpected:")
        print("  - Order: std imports → [blank] → external imports → [blank] → internal imports")
        print("  - In src/: use crate::")
        print("  - In tests/benches/: use apas_ai::")
        return 1
    else:
        print("✓ Import order correct: std → external → internal, with blank lines")
        return 0


if __name__ == "__main__":
    sys.exit(main())

