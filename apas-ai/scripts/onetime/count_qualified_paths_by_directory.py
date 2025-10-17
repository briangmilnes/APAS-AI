#!/usr/bin/env python3
"""
Count files with qualified path violations by directory.

Runs review_qualified_paths.py and counts unique files per directory.
Used to plan the fix_qualified_paths batch processing.
"""
# Git commit: 36369123a0ba7c655eae6bf92acf6645f75a8ee6
# Date: 2025-10-16 10:25:14 -0700


import subprocess

output = subprocess.run(
    ['python3', 'scripts/rust/src/review_qualified_paths.py'],
    capture_output=True, text=True
).stdout

src_files = set()
test_files = set()
bench_files = set()

for line in output.split('\n'):
    if line.strip().startswith('src/'):
        src_files.add(line.split(':')[0].strip())
    elif line.strip().startswith('tests/'):
        test_files.add(line.split(':')[0].strip())
    elif line.strip().startswith('benches/'):
        bench_files.add(line.split(':')[0].strip())

print(f"src files: {len(src_files)}")
print(f"test files: {len(test_files)}")
print(f"bench files: {len(bench_files)}")
print(f"TOTAL: {len(src_files) + len(test_files) + len(bench_files)}")

