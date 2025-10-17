#!/usr/bin/env python3
"""
Extract list of files with qualified path violations.

Runs review_qualified_paths.py and extracts unique file paths,
sorted by directory (src, tests, benches) for batch processing.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


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

print("=== SRC FILES ===")
for f in sorted(src_files):
    print(f)

print("\n=== TEST FILES ===")
for f in sorted(test_files):
    print(f)

print("\n=== BENCH FILES ===")
for f in sorted(bench_files):
    print(f)

