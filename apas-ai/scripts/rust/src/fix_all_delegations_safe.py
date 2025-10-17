#!/usr/bin/env python3
"""
Find and fix all delegation patterns, verifying compilation after each fix.
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


def check_compilation():
    """Check if the codebase compiles."""
    result = subprocess.run(
        ["cargo", "check", "--lib"],
        capture_output=True,
        text=True
    )
    return result.returncode == 0


def fix_file(file_path):
    """Fix delegation patterns in a file."""
    result = subprocess.run(
        ["python3", "scripts/rust/src/fix_move_inherent_to_trait.py", "--file", file_path],
        capture_output=True,
        text=True
    )
    
    if result.returncode == 0:
        return True, result.stdout
    else:
        return False, result.stderr


def revert_file(file_path):
    """Revert changes to a file."""
    subprocess.run(["git", "checkout", file_path], capture_output=True)


def main():
    print("Scanning for files with delegation patterns...")
    files = find_files_with_delegations()
    
    if not files:
        print("No delegation patterns found.")
        return 0
    
    print(f"\nFound {len(files)} files with delegation patterns")
    print("\nFixing delegation patterns (with compilation check after each)...")
    
    fixed_count = 0
    failed_files = []
    
    for i, (file_path, count) in enumerate(files, 1):
        print(f"\n[{i}/{len(files)}] Fixing {file_path} ({count} delegations)...")
        
        success, output = fix_file(file_path)
        if not success:
            print(f"  ✗ Fix script failed: {output}")
            failed_files.append((file_path, "Fix script failed"))
            continue
        
        print(f"  ✓ Applied fixes")
        print(f"  Checking compilation...")
        
        if check_compilation():
            print(f"  ✓ Compilation successful")
            fixed_count += 1
        else:
            print(f"  ✗ Compilation failed - reverting")
            revert_file(file_path)
            failed_files.append((file_path, "Compilation failed"))
            print(f"  ✓ Reverted {file_path}")
    
    print(f"\n{'='*60}")
    print(f"Fixed {fixed_count}/{len(files)} files successfully.")
    
    if failed_files:
        print(f"\nFailed files ({len(failed_files)}):")
        for file_path, reason in failed_files:
            print(f"  {file_path}: {reason}")
        return 1
    
    print("\n✓ All delegation patterns fixed successfully!")
    return 0


if __name__ == '__main__':
    sys.exit(main())

