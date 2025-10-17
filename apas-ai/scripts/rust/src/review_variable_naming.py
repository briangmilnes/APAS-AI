#!/usr/bin/env python3
"""
Review: Variable naming discipline.

RustRules.md Lines 22-26:
- No "temp" variables: Never use temp_vec, temp_data, temp_result, etc.
- No rock band/song names: Never use led_zeppelin, pink_floyd, stairway_to_heaven, etc.
- Use descriptive names that relate to the code's purpose.

Checks src/ for prohibited variable naming patterns.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


TEMP_PATTERN = re.compile(r'\btemp_\w+\b')
ROCK_BANDS = [
    'led_zeppelin', 'pink_floyd', 'the_beatles', 'rolling_stones',
    'queen', 'ac_dc', 'metallica', 'nirvana', 'radiohead',
    'stairway_to_heaven', 'bohemian_rhapsody', 'hotel_california',
]


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check a single file for prohibited variable names."""
    violations = []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        for line_num, line in enumerate(f, start=1):
            stripped = line.strip()
            if stripped.startswith('//') or stripped.startswith('/*') or stripped.startswith('*'):
                continue
            
            # Check for temp_ pattern
            temp_matches = TEMP_PATTERN.findall(line)
            for match in temp_matches:
                rel_path = context.relative_path(file_path)
                violations.append(
                    f"  {rel_path}:{line_num} - temp variable: {match}\n    {stripped}"
                )
            
            # Check for rock band names
            line_lower = line.lower()
            for band in ROCK_BANDS:
                if re.search(rf'\b{band}\b', line_lower):
                    rel_path = context.relative_path(file_path)
                    violations.append(
                        f"  {rel_path}:{line_num} - rock band name: {band}\n    {stripped}"
                    )
                    break
    
    return violations


def main():
    parser = create_review_parser(__doc__)
    args = parser.parse_args()
    context = ReviewContext(args)
    
    src_dir = context.repo_root / "src"
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    if context.dry_run:
        files = context.find_files([src_dir])
        print(f"Would check {len(files)} file(s) for prohibited variable names")
        return 0
    
    all_violations = []
    files = context.find_files([src_dir])
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if not all_violations:
        print("✓ No prohibited variable names found")
        return 0
    
    print(f"✗ Found prohibited variable names (RustRules.md Lines 22-26):\n")
    for violation in all_violations:
        print(violation)
    print(f"\nTotal violations: {len(all_violations)}")
    print("\nFix: Use descriptive names like 'entries', 'result_vec', 'filtered_data'.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
