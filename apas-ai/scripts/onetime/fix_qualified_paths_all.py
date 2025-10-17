#!/usr/bin/env python3
"""
Batch process all files with qualified path violations (src, tests, benches).

Applies fix_qualified_paths.py to files in batches, compiling after each batch.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys

def get_remaining_files(dir_prefix):
    """Get list of files with violations in given directory."""
    result = subprocess.run(
        ['python3', 'scripts/rust/src/review_qualified_paths.py'],
        capture_output=True, text=True
    )
    
    files = set()
    for line in result.stdout.split('\n'):
        line = line.strip()
        if line.startswith(dir_prefix) and ':' in line:
            file_path = line.split(':')[0]
            files.add(file_path)
    
    return sorted(files)

def fix_files(file_list):
    """Apply fix script to list of files."""
    for file_path in file_list:
        result = subprocess.run(
            ['python3', 'scripts/rust/src/fix_qualified_paths.py', '--file', file_path],
            capture_output=True, text=True
        )
        if result.returncode != 0:
            print(f"    ERROR fixing {file_path}: {result.stderr}")
            return False
        # Print summary line
        for line in result.stdout.split('\n'):
            if line.startswith('✓') or line.startswith('ERROR') or 'No qualified paths' in line:
                print(f"    {file_path}: {line}")
                break
    return True

def compile_target(target):
    """Compile the specified target."""
    print(f"  Compiling {target}...")
    result = subprocess.run(
        ['cargo', 'check', f'--{target}'],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        print(f"  COMPILATION FAILED for {target}:")
        print(result.stdout[-2000:] if len(result.stdout) > 2000 else result.stdout)
        return False
    print(f"  ✓ {target} compilation succeeded")
    return True

def main():
    batch_size = 10
    
    # Process src/
    print("=" * 60)
    print("Processing src/ files...")
    print("=" * 60)
    src_files = get_remaining_files('src/')
    print(f"Found {len(src_files)} src/ files\n")
    
    for i in range(0, len(src_files), batch_size):
        batch = src_files[i:i+batch_size]
        print(f"  Batch {(i//batch_size)+1} ({len(batch)} files)")
        if not fix_files(batch):
            return 1
        if not compile_target('lib'):
            return 1
        print()
    
    # Process tests/
    print("\n" + "=" * 60)
    print("Processing tests/ files...")
    print("=" * 60)
    test_files = get_remaining_files('tests/')
    print(f"Found {len(test_files)} test files\n")
    
    for i in range(0, len(test_files), batch_size):
        batch = test_files[i:i+batch_size]
        print(f"  Batch {(i//batch_size)+1} ({len(batch)} files)")
        if not fix_files(batch):
            return 1
        if not compile_target('tests'):
            return 1
        print()
    
    # Process benches/
    print("\n" + "=" * 60)
    print("Processing benches/ files...")
    print("=" * 60)
    bench_files = get_remaining_files('benches/')
    print(f"Found {len(bench_files)} bench files\n")
    
    for i in range(0, len(bench_files), batch_size):
        batch = bench_files[i:i+batch_size]
        print(f"  Batch {(i//batch_size)+1} ({len(batch)} files)")
        if not fix_files(batch):
            return 1
        if not compile_target('benches'):
            return 1
        print()
    
    print("\n" + "=" * 60)
    print(f"✓ All files fixed and compiled successfully!")
    print(f"   src/: {len(src_files)} files")
    print(f"   tests/: {len(test_files)} files")
    print(f"   benches/: {len(bench_files)} files")
    print("=" * 60)
    return 0

if __name__ == '__main__':
    sys.exit(main())

