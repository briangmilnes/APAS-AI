#!/usr/bin/env python3
"""
Batch process files with qualified path violations.

Applies fix_qualified_paths.py to files in batches, compiling after each batch.
Used for the qualified paths refactoring task.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys

def get_remaining_files():
    """Get list of files with violations."""
    result = subprocess.run(
        ['python3', 'scripts/rust/src/review_qualified_paths.py'],
        capture_output=True, text=True
    )
    
    files = set()
    for line in result.stdout.split('\n'):
        line = line.strip()
        if line.startswith('src/') and ':' in line:
            file_path = line.split(':')[0]
            files.add(file_path)
    
    return sorted(files)

def fix_files(file_list):
    """Apply fix script to list of files."""
    for file_path in file_list:
        print(f"  Fixing {file_path}...")
        result = subprocess.run(
            ['python3', 'scripts/rust/src/fix_qualified_paths.py', '--file', file_path],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"    ERROR: {result.stderr}")
            return False
        # Print summary line
        for line in result.stdout.split('\n'):
            if line.startswith('✓') or line.startswith('ERROR'):
                print(f"    {line}")
    return True

def compile_lib():
    """Compile the library."""
    print("  Compiling...")
    result = subprocess.run(
        ['cargo', 'check', '--lib'],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        print("  COMPILATION FAILED:")
        print(result.stdout)
        print(result.stderr)
        return False
    print("  ✓ Compilation succeeded")
    return True

def main():
    batch_size = 5
    
    files = get_remaining_files()
    total = len(files)
    print(f"Found {total} src/ files with violations\n")
    
    for i in range(0, total, batch_size):
        batch = files[i:i+batch_size]
        batch_num = (i // batch_size) + 1
        total_batches = (total + batch_size - 1) // batch_size
        
        print(f"Batch {batch_num}/{total_batches} ({len(batch)} files):")
        
        if not fix_files(batch):
            print("\nFix failed, stopping")
            return 1
        
        if not compile_lib():
            print("\nCompilation failed, stopping")
            return 1
        
        print()
    
    print(f"✓ All {total} files fixed and compiled successfully!")
    return 0

if __name__ == '__main__':
    sys.exit(main())

