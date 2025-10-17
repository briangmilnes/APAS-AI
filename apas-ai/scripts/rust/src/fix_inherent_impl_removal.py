#!/usr/bin/env python3
"""
Remove redundant inherent impls and fix resulting ambiguous method calls.

Strategy:
1. Remove inherent impl from Chap18-style files (where struct is defined)
2. Identify files that will have ambiguous calls (multiple traits in scope)
3. Fix ambiguous calls by using UFCS or importing only the needed trait
"""
# Git commit: 509549c
# Date: 2025-10-17

import subprocess
import sys
from pathlib import Path


def remove_inherent_impl(file_path):
    """Remove inherent impl from a file."""
    result = subprocess.run(
        ["python3", "scripts/rust/src/remove_redundant_inherent_impls.py", "--file", str(file_path)],
        capture_output=True,
        text=True
    )
    return result.returncode == 0, result.stdout + result.stderr


def check_compilation():
    """Check if the codebase compiles and return error details."""
    result = subprocess.run(
        ["cargo", "check", "--lib"],
        capture_output=True,
        text=True
    )
    return result.returncode == 0, result.stderr


def revert_file(file_path):
    """Revert changes to a file."""
    subprocess.run(["git", "checkout", file_path], capture_output=True)


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Fix inherent impl removal")
    parser.add_argument('--file', required=True, help='File to fix')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    
    print(f"Removing inherent impl from {file_path}...")
    success, output = remove_inherent_impl(file_path)
    
    if not success:
        print(f"Failed to remove inherent impl: {output}")
        return 1
    
    print(output)
    print("\nChecking compilation...")
    
    compiles, errors = check_compilation()
    
    if compiles:
        print("✓ Compilation successful!")
        return 0
    else:
        print("✗ Compilation failed:")
        print(errors[:2000])  # First 2000 chars
        print("\nReverting changes...")
        revert_file(file_path)
        print(f"✓ Reverted {file_path}")
        print("\nNeed to fix ambiguous method calls in dependent files first.")
        return 1


if __name__ == '__main__':
    sys.exit(main())

