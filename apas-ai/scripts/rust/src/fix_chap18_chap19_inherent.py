#!/usr/bin/env python3
"""
Fix Chap18/Chap19 ArraySeq files by removing inherent impls and fixing ambiguous calls.

Strategy:
1. Remove inherent impl from Chap18 files
2. Fix Chap19 files that import Chap18 types to use explicit trait qualification
"""
# Git commit: 509549c
# Date: 2025-10-17

import subprocess
import sys
from pathlib import Path


# Map of Chap18 files to their Chap19 dependents
CHAP18_TO_CHAP19 = {
    'src/Chap18/ArraySeqMtEph.rs': 'src/Chap19/ArraySeqMtEph.rs',
    'src/Chap18/ArraySeqMtPer.rs': 'src/Chap19/ArraySeqMtPer.rs',
    'src/Chap18/ArraySeqStEph.rs': 'src/Chap19/ArraySeqStEph.rs',
    'src/Chap18/ArraySeqStPer.rs': 'src/Chap19/ArraySeqStPer.rs',
}

# Methods that are likely ambiguous
COMMON_METHODS = ['length', 'nth_cloned', 'nth', 'empty', 'singleton', 'from_vec']


def remove_inherent_impl(file_path):
    """Remove inherent impl from a file."""
    result = subprocess.run(
        ["python3", "scripts/rust/src/remove_redundant_inherent_impls.py", "--file", str(file_path)],
        capture_output=True,
        text=True
    )
    return result.returncode == 0, result.stdout + result.stderr


def fix_ambiguous_calls(file_path, methods):
    """Fix ambiguous method calls in a file."""
    result = subprocess.run(
        ["python3", "scripts/rust/src/fix_ambiguous_method_calls.py", 
         "--file", str(file_path), 
         "--methods", ','.join(methods)],
        capture_output=True,
        text=True
    )
    return result.returncode == 0, result.stdout + result.stderr


def check_compilation():
    """Check if the codebase compiles."""
    result = subprocess.run(
        ["cargo", "check", "--lib"],
        capture_output=True,
        text=True
    )
    return result.returncode == 0, result.stderr[:1000]


def revert_files(*file_paths):
    """Revert changes to files."""
    for fp in file_paths:
        subprocess.run(["git", "checkout", str(fp)], capture_output=True)


def main():
    print("Fixing Chap18/Chap19 ArraySeq inherent impls...\n")
    
    for chap18_file, chap19_file in CHAP18_TO_CHAP19.items():
        print(f"{'='*60}")
        print(f"Processing: {chap18_file}")
        print(f"Dependent:  {chap19_file}")
        print(f"{'='*60}\n")
        
        # Step 1: Remove inherent impl from Chap18
        print(f"1. Removing inherent impl from {chap18_file}...")
        success, output = remove_inherent_impl(chap18_file)
        if not success:
            print(f"   ✗ Failed: {output}")
            continue
        print(f"   ✓ Removed")
        
        # Step 2: Check if compilation fails
        print(f"2. Checking compilation...")
        compiles, errors = check_compilation()
        
        if compiles:
            print(f"   ✓ Compiles! No fixes needed for {chap19_file}")
            continue
        
        # Step 3: Fix ambiguous calls in Chap19
        print(f"   ✗ Compilation failed")
        print(f"3. Fixing ambiguous calls in {chap19_file}...")
        
        success, output = fix_ambiguous_calls(chap19_file, COMMON_METHODS)
        if not success:
            print(f"   ✗ Fix failed: {output}")
            print(f"   Reverting changes...")
            revert_files(chap18_file, chap19_file)
            continue
        
        print(output)
        
        # Step 4: Check compilation again
        print(f"4. Checking compilation...")
        compiles, errors = check_compilation()
        
        if compiles:
            print(f"   ✓ Success!")
        else:
            print(f"   ✗ Still failing")
            print(f"   Error preview: {errors}")
            print(f"   Reverting changes...")
            revert_files(chap18_file, chap19_file)
        
        print()
    
    # Final compilation check
    print(f"\n{'='*60}")
    print("Final compilation check...")
    compiles, errors = check_compilation()
    
    if compiles:
        print("✓ All changes successful!")
        return 0
    else:
        print("✗ Compilation failed")
        print(errors)
        return 1


if __name__ == '__main__':
    sys.exit(main())

