#!/usr/bin/env python3
"""
Review: CamelCase naming convention.

RustRules.md Lines 303-306:
- Functions/structures of more than one English word use CamelCase
- One-word functions may be all lower case
- File names should be in CamelCase and start with a capital

Checks file names in src/, tests/, and benches/ for CamelCase convention.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def is_camelcase(name):
    """Check if a name follows CamelCase convention (starts with capital)."""
    if name.endswith('.rs'):
        name = name[:-3]
    return name[0].isupper() if name else False


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check a single file's name for CamelCase convention."""
    filename = file_path.name
    
    # Skip special files
    if filename in ['lib.rs', 'main.rs', 'mod.rs']:
        return []
    
    if not is_camelcase(filename):
        rel_path = context.relative_path(file_path)
        return [f"  {rel_path}\n    File '{filename}' should start with capital letter"]
    
    return []


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
        print(f"Would check {len(files)} file(s) for CamelCase naming")
        return 0
    
    all_violations = []
    files = context.find_files(search_dirs)
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if not all_violations:
        print("✓ All file names follow CamelCase convention")
        return 0
    
    print(f"✗ Found non-CamelCase file names (RustRules.md Lines 303-306):\n")
    for violation in all_violations:
        print(violation)
    print(f"\nTotal violations: {len(all_violations)}")
    print("\nFix: Rename files to start with capital letter (e.g., 'myFile.rs' → 'MyFile.rs').")
    return 1


if __name__ == "__main__":
    sys.exit(main())

