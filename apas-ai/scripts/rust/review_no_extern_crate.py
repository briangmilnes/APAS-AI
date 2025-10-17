#!/usr/bin/env python3
"""
Review: No 'extern crate' usage.

RustRules.md Line 86: "Never use extern crate. Do not add re-exports."

Checks all Rust source files in src/, tests/, and benches/ for 'extern crate' usage.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check a single file for extern crate usage."""
    violations = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        for line_num, line in enumerate(f, start=1):
            stripped = line.strip()
            if stripped.startswith('//'):
                continue
            if 'extern crate' in line:
                rel_path = context.relative_path(file_path)
                violations.append(f"  {rel_path}:{line_num}\n    {stripped}")
    
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
        print(f"Would check {len(files)} file(s) for 'extern crate' usage")
        return 0
    
    all_violations = []
    files = context.find_files(search_dirs)
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if not all_violations:
        print("✓ No 'extern crate' usage found")
        return 0
    
    print(f"✗ Found 'extern crate' usage (RustRules.md Line 86):\n")
    for violation in all_violations:
        print(violation)
    print(f"\nTotal violations: {len(all_violations)}")
    print("\nFix: Remove 'extern crate' statements. Use 'use' statements instead.")
    return 1


if __name__ == "__main__":
    sys.exit(main())

