#!/usr/bin/env python3
"""
Fix SetLit! patterns in TestJohnsonStEphFloat.rs
Converts (num, num, OrderedF64::from(...)) to Triple(num, num, OrderedF64::from(...))
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

from pathlib import Path
import re

fp = Path('tests/Chap59/TestJohnsonStEphFloat.rs')
content = fp.read_text()

# Convert multiline SetLit patterns
lines = content.split('\n')
result = []
in_setlit = False

for line in lines:
    if 'SetLit![' in line:
        in_setlit = True
    
    if in_setlit:
        # OrderedF64::from patterns
        line = re.sub(r'^\s*\((\d+),\s*(\d+),\s*(OrderedF64::from\([^)]+\))\)', r'        Triple(\1, \2, \3)', line)
        # Inline patterns
        line = re.sub(r'SetLit!\[\((\d+),\s*(\d+),\s*(OrderedF64::from\([^)]+\))\)', r'SetLit![Triple(\1, \2, \3)', line)
    
    result.append(line)
    
    if in_setlit and '];' in line:
        in_setlit = False

fp.write_text('\n'.join(result))
print(f'✓ Fixed: {fp.name}')


