#!/usr/bin/env python3
"""
Comprehensive fix for graph test files - converts tuple patterns to Triple/Pair wrappers.
Handles:
- SetLit![(num, num, weight)] -> SetLit![Triple(num, num, weight)]
- edges.insert((X, Y, Z)) -> edges.insert(Triple(X, Y, Z))
- .mem(&(X, Y, Z)) -> .mem(&Triple(X, Y, Z))
- .mem(&(X, Y)) -> .mem(&Pair(X, Y))
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

from pathlib import Path
import re

# Apply comprehensive fixes to ALL graph and algorithm test files
files = list(Path('tests/Chap06').glob('TestWeighted*.rs')) + \
        list(Path('tests/Chap58').glob('Test*.rs')) + \
        list(Path('tests/Chap59').glob('TestJohnson*.rs')) + \
        list(Path('tests/Chap57').glob('TestDijkstra*.rs'))

for fp in files:
    content = fp.read_text()
    orig = content
    
    # Convert SetLit patterns with multiline support
    lines = content.split('\n')
    result = []
    in_setlit = False
    
    for line in lines:
        if 'SetLit![' in line:
            in_setlit = True
        
        # Convert tuples in SetLit (not LabEdge)
        if in_setlit and 'LabEdge' not in line:
            # Float patterns
            line = re.sub(r'(?<!Triple)\((\d+),\s*(\d+),\s*(Ordered(?:Float|F64)(?:::|)\([^)]+\))\)', r'Triple(\1, \2, \3)', line)
            # Int patterns
            line = re.sub(r'(?<!Triple)\((\d+),\s*(\d+),\s*(-?\d+)\)', r'Triple(\1, \2, \3)', line)
        
        # Fix .mem patterns outside of SetLit
        if not in_setlit and '.mem(&(' in line and 'LabEdge' not in line:
            line = re.sub(r'\.mem\(&\(([^)]+,[^)]+,[^)]+)\)\)', r'.mem(&Triple(\1))', line)
            line = re.sub(r'\.mem\(&\(([^,)]+),\s*([^)]+)\)\)', r'.mem(&Pair(\1, \2))', line)
        
        result.append(line)
        
        if in_setlit and '];' in line:
            in_setlit = False
    
    content = '\n'.join(result)
    
    if content != orig:
        fp.write_text(content)
        print(f'✓ {fp.name}')

print("\n✅ All graph test files processed")


