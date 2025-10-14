#!/usr/bin/env python3
"""
Review: Variable naming discipline.

RustRules.md Lines 22-26:
- No "temp" variables: Never use temp_vec, temp_data, temp_result, etc.
- No rock band/song names: Never use led_zeppelin, pink_floyd, stairway_to_heaven, etc.
- Use descriptive names that relate to the code's purpose.

Checks src/ for prohibited variable naming patterns.
"""

import re
import sys
from pathlib import Path


# Prohibited patterns
TEMP_PATTERN = re.compile(r'\btemp_\w+\b')
ROCK_BANDS = [
    'led_zeppelin', 'pink_floyd', 'the_beatles', 'rolling_stones',
    'queen', 'ac_dc', 'metallica', 'nirvana', 'radiohead',
    'stairway_to_heaven', 'bohemian_rhapsody', 'hotel_california',
]


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    src_dir = repo_root / "src"
    
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    violations = []
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            for line_num, line in enumerate(f, start=1):
                # Skip comments
                stripped = line.strip()
                if stripped.startswith('//'):
                    continue
                if stripped.startswith('/*') or stripped.startswith('*'):
                    continue
                
                # Check for temp_ pattern
                temp_matches = TEMP_PATTERN.findall(line)
                for match in temp_matches:
                    violations.append((
                        src_file, line_num, line.strip(),
                        f"temp variable: {match}"
                    ))
                
                # Check for rock band names
                line_lower = line.lower()
                for band in ROCK_BANDS:
                    if band in line_lower:
                        # Make sure it's a whole word
                        if re.search(rf'\b{band}\b', line_lower):
                            violations.append((
                                src_file, line_num, line.strip(),
                                f"rock band name: {band}"
                            ))
    
    if violations:
        print("✗ Found prohibited variable names (RustRules.md Lines 22-26):\n")
        for file_path, line_num, line_content, reason in violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:{line_num} - {reason}")
            print(f"    {line_content}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Use descriptive names like 'entries', 'result_vec', 'filtered_data'.")
        return 1
    else:
        print("✓ No prohibited variable names found")
        return 0


if __name__ == "__main__":
    sys.exit(main())

