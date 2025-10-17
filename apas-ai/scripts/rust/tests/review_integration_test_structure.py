#!/usr/bin/env python3
"""
Review: Integration test structure.

RustRules.md Lines 292-298: "Integration tests must have test functions at the 
root level of the file. NEVER use #[cfg(test)] modules in integration test files
- this prevents test discovery."

Checks all files in tests/ directory for #[cfg(test)] module usage.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check a single test file for #[cfg(test)] usage."""
    violations = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        in_multiline_comment = False
        for line_num, line in enumerate(f, start=1):
            # Handle multi-line comments
            if '/*' in line:
                in_multiline_comment = True
            if '*/' in line:
                in_multiline_comment = False
                continue
            if in_multiline_comment:
                continue
            
            stripped = line.strip()
            if stripped.startswith('//'):
                continue
            
            # Check for #[cfg(test)]
            if '#[cfg(test)]' in line:
                rel_path = context.relative_path(file_path)
                violations.append(f"  {rel_path}:{line_num}\n    {stripped}")
    
    return violations


def main():
    parser = create_review_parser(__doc__)
    args = parser.parse_args()
    context = ReviewContext(args)
    
    tests_dir = context.repo_root / "tests"
    if not tests_dir.exists():
        print("✓ No tests/ directory found")
        return 0
    
    if context.dry_run:
        files = context.find_files([tests_dir])
        print(f"Would check {len(files)} file(s) for #[cfg(test)] usage")
        return 0
    
    all_violations = []
    files = context.find_files([tests_dir])
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if not all_violations:
        print("✓ No #[cfg(test)] modules in integration tests")
        return 0
    
    print(f"✗ Found #[cfg(test)] in integration tests (RustRules.md Lines 292-298):\n")
    for violation in all_violations:
        print(violation)
    print(f"\nTotal violations: {len(all_violations)}")
    print("\nFix: Remove #[cfg(test)] modules from integration tests.")
    print("Integration tests should have #[test] functions at root level.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
