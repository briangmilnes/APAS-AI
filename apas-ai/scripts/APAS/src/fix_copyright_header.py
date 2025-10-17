#!/usr/bin/env python3
"""
Fix: Copyright header on line 1.

APASRules.md Lines 190-195: Ensures all .rs files have the correct copyright
on line 1.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path


REQUIRED_COPYRIGHT = "//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'."


def fix_file_copyright(file_path, dry_run=False):
    """Fix the copyright header in a single file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    if not lines:
        # Empty file - add copyright
        new_lines = [REQUIRED_COPYRIGHT + '\n']
        changed = True
    else:
        first_line = lines[0].rstrip()
        
        # Check if first line is already correct
        if first_line == REQUIRED_COPYRIGHT:
            return False  # No change needed
        
        # Check if first line is a copyright (wrong format)
        if 'Copyright' in first_line and ('2025' in first_line or '©' in first_line):
            # Replace wrong copyright
            new_lines = [REQUIRED_COPYRIGHT + '\n'] + lines[1:]
            changed = True
        else:
            # No copyright - insert at beginning
            new_lines = [REQUIRED_COPYRIGHT + '\n'] + lines
            changed = True
    
    if changed and not dry_run:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
    
    return changed


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Fix copyright headers in Rust files.")
    parser.add_argument('--file', type=str, help="Specify a single file to fix.")
    parser.add_argument('--dry-run', action='store_true', help="Show changes without writing.")
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.exists():
            print(f"Error: File not found at {file_path}")
            return 1
        
        if fix_file_copyright(file_path, args.dry_run):
            rel_path = file_path.relative_to(repo_root) if file_path.is_relative_to(repo_root) else file_path
            if args.dry_run:
                print(f"Would fix: {rel_path}")
            else:
                print(f"✓ Fixed: {rel_path}")
        else:
            print(f"✓ Already correct: {file_path}")
        return 0
    
    # Fix all files
    fixed_count = 0
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rs_file in search_dir.rglob("*.rs"):
            if fix_file_copyright(rs_file, args.dry_run):
                fixed_count += 1
                rel_path = rs_file.relative_to(repo_root)
                if not args.dry_run:
                    print(f"✓ Fixed: {rel_path}")
    
    if args.dry_run:
        print(f"\nWould fix {fixed_count} files")
    else:
        print(f"\n✓ Fixed {fixed_count} copyright headers")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

