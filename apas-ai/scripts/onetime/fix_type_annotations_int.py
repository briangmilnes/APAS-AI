#!/usr/bin/env python3
"""
Fix explicit type annotations in test files.
Converts SetStEph<(N, N, i32)> -> SetStEph<Triple<N, N, i32>>
Converts SetStEph<(N, i32)> -> SetStEph<Pair<N, i32>>
"""

# Git commit: 14ee2e77f49944647f5422f672c90d5f831db070
# Date: 2025-10-16 15:57:53 -0700

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


