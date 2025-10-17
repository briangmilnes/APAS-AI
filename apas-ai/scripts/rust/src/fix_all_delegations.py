#!/usr/bin/env python3
"""
Find and fix all delegation patterns in the codebase.
"""
# Git commit: e8e8f18
# Date: 2025-10-17

import subprocess
import sys
from pathlib import Path


def find_files_with_delegations():
    """Find all Rust source files with delegation patterns."""
    src_dir = Path("src")
    files_with_delegations = []
    
    for rs_file in src_dir.rglob("*.rs"):
        result = subprocess.run(
            ["python3", "scripts/rust/src/detect_delegation_to_inherent.py", "--file", str(rs_file)],
            capture_output=True,
            text=True
        )
        
        if result.returncode == 0:  # Found delegations
            # Parse output to get count
            for line in result.stdout.splitlines():
                if "Count:" in line:
                    count = int(line.split(":")[-1].strip())
                    files_with_delegations.append((str(rs_file), count))
                    break
    
    return files_with_delegations


def fix_file(file_path, dry_run=False):
    """Fix delegation patterns in a file."""
    cmd = ["python3", "scripts/rust/src/fix_move_inherent_to_trait.py", "--file", file_path]
    if dry_run:
        cmd.append("--dry-run")
    
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    if result.returncode == 0:
        return True, result.stdout
    else:
        return False, result.stderr


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Fix all delegation patterns")
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    print("Scanning for files with delegation patterns...")
    files = find_files_with_delegations()
    
    if not files:
        print("No delegation patterns found.")
        return 0
    
    print(f"\nFound {len(files)} files with delegation patterns:")
    for file_path, count in files:
        print(f"  {file_path}: {count} delegations")
    
    if args.dry_run:
        print("\nDry run mode - no changes will be made.")
        return 0
    
    print("\nFixing delegation patterns...")
    fixed_count = 0
    failed_files = []
    
    for file_path, count in files:
        success, output = fix_file(file_path, dry_run=False)
        if success:
            print(f"✓ Fixed {file_path}")
            fixed_count += 1
        else:
            print(f"✗ Failed to fix {file_path}: {output}")
            failed_files.append(file_path)
    
    print(f"\nFixed {fixed_count}/{len(files)} files.")
    
    if failed_files:
        print("\nFailed files:")
        for file_path in failed_files:
            print(f"  {file_path}")
        return 1
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

