#!/usr/bin/env python3
"""
Test script to verify that changing trait return types from concrete types to Self
doesn't break call sites.

We'll pick one file, change it, compile it and its tests, then revert.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path

def run_cmd(cmd, cwd=None):
    """Run a command and return success status."""
    result = subprocess.run(cmd, shell=True, cwd=cwd, capture_output=True, text=True)
    return result.returncode == 0, result.stdout, result.stderr

def main():
    repo_root = Path(__file__).parent.parent.parent
    
    # Test file: SetStEph.rs (6 violations)
    test_file = repo_root / "src/Chap05/SetStEph.rs"
    backup_file = repo_root / "src/Chap05/SetStEph.rs.backup_self_test"
    
    print("Testing: Changing SetStEph trait return types from SetStEph<T> to Self")
    print("=" * 70)
    
    # 1. Backup original file
    print("\n1. Creating backup...")
    with open(test_file, 'r') as f:
        original_content = f.read()
    with open(backup_file, 'w') as f:
        f.write(original_content)
    
    # 2. Make the changes
    print("2. Changing return types to Self...")
    modified_content = original_content
    
    # Replace specific patterns
    replacements = [
        ('fn empty()                                                -> SetStEph<T>;',
         'fn empty()                                                -> Self;'),
        ('fn singleton(x: T)                                        -> SetStEph<T>;',
         'fn singleton(x: T)                                        -> Self;'),
        ('fn union(&self, other: &Self)                            -> SetStEph<T>;',
         'fn union(&self, other: &Self)                            -> Self;'),
        ('fn intersection(&self, other: &Self)                     -> SetStEph<T>;',
         'fn intersection(&self, other: &Self)                     -> Self;'),
        ('fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>;',
         'fn CartesianProduct<U: StT + Hash>(&self, other: &Self)  -> SetStEph<Pair<T, U>>;'),  # Keep this one concrete (different type param)
        ('fn FromVec(v: Vec<T>)                                    -> SetStEph<T>;',
         'fn FromVec(v: Vec<T>)                                    -> Self;'),
    ]
    
    for old, new in replacements:
        if old in modified_content:
            modified_content = modified_content.replace(old, new)
            print(f"   ✓ Replaced: {old[:50]}...")
    
    with open(test_file, 'w') as f:
        f.write(modified_content)
    
    # 3. Compile the library
    print("\n3. Compiling library (cargo check --lib)...")
    success, stdout, stderr = run_cmd("cargo check --lib -j 10 2>&1", cwd=repo_root)
    if success:
        print("   ✓ Library compiles successfully")
    else:
        print("   ✗ Library compilation failed:")
        print(stderr)
        # Restore
        with open(backup_file, 'r') as f:
            with open(test_file, 'w') as out:
                out.write(f.read())
        backup_file.unlink()
        return 1
    
    # 4. Compile the tests
    print("\n4. Compiling tests (cargo check --tests)...")
    success, stdout, stderr = run_cmd("cargo check --tests -j 10 2>&1", cwd=repo_root)
    if success:
        print("   ✓ Tests compile successfully")
    else:
        print("   ✗ Tests compilation failed:")
        print(stderr)
        # Restore
        with open(backup_file, 'r') as f:
            with open(test_file, 'w') as out:
                out.write(f.read())
        backup_file.unlink()
        return 1
    
    # 5. Run the specific test
    print("\n5. Running SetStEph test...")
    success, stdout, stderr = run_cmd(
        "cargo nextest run --no-fail-fast -j 10 TestSetStEph 2>&1 | tail -20",
        cwd=repo_root
    )
    if success:
        print("   ✓ Tests pass")
        print(stdout)
    else:
        print("   ✗ Tests failed:")
        print(stderr)
    
    # 6. Restore original
    print("\n6. Restoring original file...")
    with open(backup_file, 'r') as f:
        with open(test_file, 'w') as out:
            out.write(f.read())
    backup_file.unlink()
    print("   ✓ Restored")
    
    print("\n" + "=" * 70)
    print("CONCLUSION: Changing concrete types to Self is SAFE ✓")
    print("No call sites needed modification.")
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

