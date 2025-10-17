#!/usr/bin/env python3
"""
Fix explicit type annotations in test files.
Converts SetStEph<(N, N, i32)> -> SetStEph<Triple<N, N, i32>>
Converts SetStEph<(N, i32)> -> SetStEph<Pair<N, i32>>
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

from pathlib import Path
import re

fp = Path('tests/Chap06/TestWeightedDirGraphStEphInt.rs')
content = fp.read_text()

# Fix type annotations: SetStEph<(N, N, i32)> -> SetStEph<Triple<N, N, i32>>
content = re.sub(r'SetStEph<\(N, N, i32\)>', r'SetStEph<Triple<N, N, i32>>', content)

# Fix type annotations: SetStEph<(N, i32)> -> SetStEph<Pair<N, i32>>
content = re.sub(r'SetStEph<\(N, i32\)>', r'SetStEph<Pair<N, i32>>', content)

fp.write_text(content)
print(f'✓ Fixed type annotations: {fp.name}')


