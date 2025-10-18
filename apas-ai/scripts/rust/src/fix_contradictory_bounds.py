#!/usr/bin/env python3
"""Fix contradictory trait bounds (StT + MtT together) based on file naming convention.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Usage:
  fix_contradictory_bounds.py <file.rs>          # Fix specific file
  fix_contradictory_bounds.py <file.rs> --dry-run  # Show changes without applying
"""

import re
import sys
from pathlib import Path


def determine_correct_bound(file_path):
    """Determine the correct bound based on file naming convention."""
    file_name = file_path.name
    
    if 'MtEph' in file_name or 'MtPer' in file_name:
        return 'MtT', 'StT'  # keep MtT, remove StT
    elif 'StEph' in file_name or 'StPer' in file_name:
        return 'StT', 'MtT'  # keep StT, remove MtT
    else:
        return None, None


def fix_bounds_in_line(line, keep_bound, remove_bound):
    """Fix a single line by removing the incorrect bound."""
    # Pattern 1: "remove_bound + keep_bound" -> "keep_bound"
    line = re.sub(
        rf'\b{remove_bound}\s*\+\s*{keep_bound}\b',
        keep_bound,
        line
    )
    
    # Pattern 2: "keep_bound + remove_bound" -> "keep_bound"
    line = re.sub(
        rf'\b{keep_bound}\s*\+\s*{remove_bound}\b',
        keep_bound,
        line
    )
    
    # Pattern 3: "remove_bound + something + keep_bound" -> "something + keep_bound"
    line = re.sub(
        rf'\b{remove_bound}\s*\+\s*(\w+\s*\+\s*)*{keep_bound}\b',
        lambda m: m.group(0).replace(f'{remove_bound} + ', ''),
        line
    )
    
    # Pattern 4: "keep_bound + something + remove_bound" -> "keep_bound + something"
    line = re.sub(
        rf'\b{keep_bound}(\s*\+\s*\w+)*\s*\+\s*{remove_bound}\b',
        lambda m: m.group(0).replace(f' + {remove_bound}', ''),
        line
    )
    
    # Clean up "StTInMtT" patterns (these should become just the keep_bound if it's MtT)
    if remove_bound == 'StT' and keep_bound == 'MtT':
        # "StTInMtT + MtT" -> "MtT" (StTInMtT already implies MtT)
        line = re.sub(r'\bStTInMtT\s*\+\s*MtT\b', 'StTInMtT', line)
        line = re.sub(r'\bMtT\s*\+\s*StTInMtT\b', 'StTInMtT', line)
    
    return line


def fix_file(file_path, dry_run=False):
    """Fix contradictory bounds in a file."""
    keep_bound, remove_bound = determine_correct_bound(file_path)
    
    if not keep_bound:
        print(f"Warning: Cannot determine correct bound for {file_path.name}", file=sys.stderr)
        print("  File naming convention unclear (expected *MtEph*, *MtPer*, *StEph*, or *StPer*)", file=sys.stderr)
        return False
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    lines = content.split('\n')
    new_lines = []
    changes_made = False
    
    for line_num, line in enumerate(lines, start=1):
        # Check if line has both bounds
        has_both = (
            (re.search(r'\bStT\b', line) or re.search(r'\bStTInMtT\b', line)) and
            re.search(r'\bMtT\b', line)
        )
        
        if has_both:
            new_line = fix_bounds_in_line(line, keep_bound, remove_bound)
            if new_line != line:
                if dry_run:
                    print(f"Line {line_num}:")
                    print(f"  - {line}")
                    print(f"  + {new_line}")
                changes_made = True
                new_lines.append(new_line)
            else:
                new_lines.append(line)
        else:
            new_lines.append(line)
    
    if not changes_made:
        if dry_run:
            print(f"No changes needed in {file_path.name}")
        return False
    
    if dry_run:
        print(f"\nWould fix {file_path.name} (keeping {keep_bound}, removing {remove_bound})")
        return True
    
    # Write back to file
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(new_lines))
        print(f"✓ Fixed {file_path.name} (kept {keep_bound}, removed {remove_bound})")
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    if len(sys.argv) < 2:
        print("Usage: fix_contradictory_bounds.py <file.rs> [--dry-run]")
        return 1
    
    file_path = Path(sys.argv[1])
    dry_run = '--dry-run' in sys.argv
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}", file=sys.stderr)
        return 1
    
    if dry_run:
        print(f"Dry run mode - showing changes without applying...\n")
    
    success = fix_file(file_path, dry_run=dry_run)
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())

