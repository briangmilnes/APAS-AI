#!/usr/bin/env python3
"""Test the import fixing on a single file."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
import os
sys.path.append('scripts')
from fix_imports import fix_imports_in_file

# Test on a single file
file_path = 'src/Chap37/AVLTreeSeq.rs'
print(f"BEFORE:")
with open(file_path, 'r') as f:
    lines = f.readlines()
    for i, line in enumerate(lines[9:20], 10):
        print(f"{i:2}: {repr(line.rstrip())}")

changed, message = fix_imports_in_file(file_path)
print(f"\nFile: {file_path}")
print(f"Changed: {changed}")
print(f"Message: {message}")

print(f"\nAFTER:")
with open(file_path, 'r') as f:
    lines = f.readlines()
    for i, line in enumerate(lines[9:20], 10):
        print(f"{i:2}: {repr(line.rstrip())}")