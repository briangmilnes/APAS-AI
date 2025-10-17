#!/usr/bin/env python3
"""
Check for mismatches between benchmark filenames and Cargo.toml registrations.
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

import re
import os

# Read Cargo.toml
with open('Cargo.toml', 'r') as f:
    cargo_content = f.read()

# Find all [[bench]] entries
bench_pattern = r'\[\[bench\]\]\s*name = "([^"]+)"\s*path = "([^"]+)"'
entries = re.findall(bench_pattern, cargo_content)

mismatches = []
for name, path in entries:
    # Expected name from filename
    filename = os.path.basename(path)
    expected_name = filename.replace('.rs', '')
    
    if name != expected_name:
        mismatches.append({
            'path': path,
            'registered_name': name,
            'expected_name': expected_name
        })

if mismatches:
    print(f"Found {len(mismatches)} mismatches:")
    print()
    for m in mismatches:
        print(f"Path: {m['path']}")
        print(f"  Registered as: {m['registered_name']}")
        print(f"  Should be:     {m['expected_name']}")
        print()
else:
    print("No mismatches found!")

