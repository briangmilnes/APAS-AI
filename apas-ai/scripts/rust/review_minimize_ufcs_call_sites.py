#!/usr/bin/env python3
"""
Review: Minimize UFCS at call sites.

RustRules.md Lines 309-320: "Replace <Type as Trait>::method(...) at call sites
with method-call syntax wherever possible. Keep UFCS inside impls/traits for
disambiguation; minimize UFCS in callers."

Checks src/, tests/, and benches/ for UFCS usage outside of impl/trait blocks.

Note: Some UFCS usage may be legitimate (primitives, macros, disambiguation).
This check identifies candidates for review, not automatic violations.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


UFCS_PATTERN = re.compile(r'<[^>]+\s+as\s+[^>]+>::\w+')


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check a single file for UFCS usage at call sites."""
    violations = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        in_impl = False
        in_trait = False
        brace_depth = 0
        
        for line_num, line in enumerate(f, start=1):
            stripped = line.strip()
            
            if stripped.startswith('//'):
                continue
            
            # Track impl/trait blocks
            if stripped.startswith('impl ') or ' impl ' in stripped:
                in_impl = True
                brace_depth = 0
            elif stripped.startswith('trait ') or ' trait ' in stripped:
                in_trait = True
                brace_depth = 0
            
            # Track braces
            brace_depth += line.count('{') - line.count('}')
            
            # Exit impl/trait when braces close
            if (in_impl or in_trait) and brace_depth <= 0:
                in_impl = False
                in_trait = False
            
            # Check for UFCS outside impl/trait
            if not in_impl and not in_trait:
                ufcs_matches = UFCS_PATTERN.findall(line)
                if ufcs_matches:
                    rel_path = context.relative_path(file_path)
                    for match in ufcs_matches:
                        violations.append(
                            f"  {rel_path}:{line_num}\n    {stripped}\n    UFCS: {match}"
                        )
    
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
        print(f"Would check {len(files)} file(s) for UFCS at call sites")
        return 0
    
    all_violations = []
    files = context.find_files(search_dirs)
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if not all_violations:
        print("✓ No minimizable UFCS call sites found")
        return 0
    
    print(f"⚠ Found UFCS at call sites (RustRules.md Lines 309-320):\n")
    print("Note: Review these - some may be legitimate (primitives, macros).\n")
    for violation in all_violations:
        print(violation)
    print(f"\nTotal violations: {len(all_violations)}")
    print("\nFix: Use method-call syntax: value.method(...) instead of <Type as Trait>::method(value, ...)")
    return 1


if __name__ == "__main__":
    sys.exit(main())
