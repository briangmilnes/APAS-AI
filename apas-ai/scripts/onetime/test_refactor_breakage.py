#!/usr/bin/env python3
"""
Test the trait default refactor to find actual breakage.

Strategy:
1. Apply refactor to one file
2. Try to compile (lib, tests, benches)
3. Collect actual errors
4. Revert
5. Report which files/call-sites actually need fixes

This tells us the REAL impact, not hypothetical worst-case.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path
import shutil
import re


def run_cmd(cmd, cwd):
    """Run command and capture output."""
    result = subprocess.run(
        cmd, 
        shell=True, 
        cwd=cwd, 
        capture_output=True, 
        text=True
    )
    return result.returncode, result.stdout + result.stderr


def backup_file(filepath):
    """Create backup of file."""
    backup = filepath.with_suffix('.rs.backup_test')
    shutil.copy2(filepath, backup)
    return backup


def restore_file(filepath, backup):
    """Restore file from backup."""
    shutil.copy2(backup, filepath)
    backup.unlink()


def parse_compiler_errors(output):
    """Extract error locations from compiler output."""
    errors = []
    
    # Look for error lines with file:line:col
    for line in output.split('\n'):
        match = re.match(r'error.*?-->\s+([^:]+):(\d+):(\d+)', line)
        if match:
            filepath = match.group(1)
            line_num = match.group(2)
            col = match.group(3)
            errors.append({
                'file': filepath,
                'line': line_num,
                'col': col
            })
    
    return errors


def test_one_method_move(repo_root):
    """Test moving ONE short method to see what breaks."""
    
    # Example: Move ArraySeqStEph::from_vec to trait default
    test_file = repo_root / 'src/Chap18/ArraySeqStEph.rs'
    
    if not test_file.exists():
        print(f"Test file not found: {test_file}")
        return
    
    print(f"Testing: Moving from_vec() to trait default in {test_file.name}")
    print("=" * 80)
    
    # Backup
    backup = backup_file(test_file)
    
    try:
        # TODO: Apply actual refactor here (would need the fix script)
        # For now, just demonstrate the process
        
        print("\n1. Applying refactor...")
        print("   [Would move from_vec() from inherent impl to trait default]")
        
        print("\n2. Compiling library...")
        exitcode, output = run_cmd("cargo check --lib -j 10", repo_root)
        
        if exitcode != 0:
            print("   ✗ Library compilation failed")
            errors = parse_compiler_errors(output)
            print(f"   Found {len(errors)} error locations")
            for err in errors[:5]:
                print(f"     {err['file']}:{err['line']}:{err['col']}")
        else:
            print("   ✓ Library compiles")
        
        print("\n3. Compiling tests...")
        exitcode, output = run_cmd("cargo check --tests -j 10", repo_root)
        
        if exitcode != 0:
            print("   ✗ Test compilation failed")
            errors = parse_compiler_errors(output)
            print(f"   Found {len(errors)} error locations in tests")
        else:
            print("   ✓ Tests compile")
        
        print("\n4. Compiling benches...")
        exitcode, output = run_cmd("cargo check --benches -j 10", repo_root)
        
        if exitcode != 0:
            print("   ✗ Bench compilation failed")
            errors = parse_compiler_errors(output)
            print(f"   Found {len(errors)} error locations in benches")
        else:
            print("   ✓ Benches compile")
        
    finally:
        # Restore
        print("\n5. Restoring original file...")
        restore_file(test_file, backup)
        print("   ✓ Restored")
    
    print("\n" + "=" * 80)
    print("CONCLUSION:")
    print("This approach lets us see ACTUAL breakage, not hypothetical.")
    print("We only fix call sites that actually fail to compile.")


def main():
    repo_root = Path(__file__).parent.parent.parent
    
    print("Test Strategy: Find Actual Breakage from Trait Default Refactor")
    print("=" * 80)
    print()
    print("Instead of guessing which call sites need type annotations,")
    print("we test the refactor and collect ACTUAL compiler errors.")
    print()
    
    # For now, just demonstrate the concept
    print("NOTE: This is a proof-of-concept script.")
    print("It shows the PROCESS we should follow:")
    print()
    print("1. Pick a file with methods to move")
    print("2. Apply refactor (move to trait defaults)")
    print("3. Try to compile everything")
    print("4. Collect actual errors (if any)")
    print("5. Revert")
    print("6. Count how many call sites actually break")
    print()
    print("BENEFIT: We learn the TRUE cost before committing.")
    print("We might find only 10 call sites break, not 1000!")
    print()
    
    # Uncomment to actually test:
    # test_one_method_move(repo_root)
    
    return 0


if __name__ == '__main__':
    sys.exit(main())


