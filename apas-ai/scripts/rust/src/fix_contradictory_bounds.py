#!/usr/bin/env python3
"""Fix contradictory trait bounds (StT + MtT together) based on file naming convention.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Usage:
  fix_contradictory_bounds.py <file.rs>          # Fix specific file
  fix_contradictory_bounds.py <file.rs> --dry-run  # Show changes without applying
  fix_contradictory_bounds.py <file.rs> --log_file <path>  # Custom log path
"""

import argparse
import re
import sys
from pathlib import Path


class TeeOutput:
    """Write to both stdout and a log file."""
    def __init__(self, log_path):
        self.log_path = Path(log_path)
        self.log_path.parent.mkdir(parents=True, exist_ok=True)
        self.log_file = open(self.log_path, 'a', encoding='utf-8')  # Append mode
    
    def write(self, text):
        print(text, end='', flush=True)
        self.log_file.write(text)
        self.log_file.flush()
    
    def print(self, text=''):
        self.write(text + '\n')
    
    def close(self):
        self.log_file.close()


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


def fix_file(file_path, dry_run=False, tee=None):
    """Fix contradictory bounds in a file."""
    def log(msg):
        if tee:
            tee.print(msg)
        else:
            print(msg)
    
    keep_bound, remove_bound = determine_correct_bound(file_path)
    
    if not keep_bound:
        log(f"Warning: Cannot determine correct bound for {file_path.name}")
        log("  File naming convention unclear (expected *MtEph*, *MtPer*, *StEph*, or *StPer*)")
        return False
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        log(f"Error reading {file_path}: {e}")
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
                    log(f"Line {line_num}:")
                    log(f"  - {line}")
                    log(f"  + {new_line}")
                changes_made = True
                new_lines.append(new_line)
            else:
                new_lines.append(line)
        else:
            new_lines.append(line)
    
    if not changes_made:
        if dry_run:
            log(f"No changes needed in {file_path.name}")
        return False
    
    if dry_run:
        log(f"\nWould fix {file_path.name} (keeping {keep_bound}, removing {remove_bound})")
        return True
    
    # Write back to file
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write('\n'.join(new_lines))
        log(f"✓ Fixed {file_path.name} (kept {keep_bound}, removed {remove_bound})")
        return True
    except Exception as e:
        log(f"Error writing {file_path}: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(description='Fix contradictory trait bounds (StT + MtT)')
    parser.add_argument('file', type=Path, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show changes without applying')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/fix_contradictory_bounds.txt',
                       help='Path to log file (default: analyses/code_review/fix_contradictory_bounds.txt)')
    args = parser.parse_args()
    
    file_path = args.file
    dry_run = args.dry_run
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}", file=sys.stderr)
        return 1
    
    project_root = Path(__file__).parent.parent.parent.parent
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    if dry_run:
        tee.print(f"Dry run mode - showing changes without applying...\n")
    
    tee.print(f"{'='*60}")
    tee.print(f"fix_contradictory_bounds.py: {file_path}")
    tee.print(f"{'='*60}")
    
    success = fix_file(file_path, dry_run=dry_run, tee=tee)
    
    tee.print(f"{'='*60}\n")
    tee.close()
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
