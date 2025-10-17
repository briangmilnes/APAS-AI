#!/usr/bin/env python3
"""
Fix Triple patterns inside Lit! macro invocations.
Converts E: [Triple(a,b,c)] back to E: [(a,b,c)] because the macro wraps them.

The Lit! macros expect raw tuples and do the Triple wrapping internally:
  Macro definition: E: [ $( ($from, $to, $weight) ),* ]
  Macro expansion:  let edges = SetLit![ $( Triple($from, $to, $weight) ),* ];

Test files should use raw tuples in Lit! invocations, but Triple elsewhere.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

from pathlib import Path
import re

fp = Path('tests/Chap06/TestWeightedDirGraphStEphInt.rs')
content = fp.read_text()

# Convert Triple back to raw tuples inside Lit! macro invocations
# Pattern: E: [Triple(...), Triple(...)] or A: [Triple(...), Triple(...)]
content = re.sub(
    r'(E: \[[^\]]*?)Triple\(([^)]+)\)',
    r'\1(\2)',
    content
)

content = re.sub(
    r'(A: \[[^\]]*?)Triple\(([^)]+)\)',
    r'\1(\2)',
    content
)

fp.write_text(content)
print(f'✓ Fixed Lit! macro invocations in {fp.name}')

# Check for this pattern in other test files
for test_file in Path('tests/Chap06').glob('Test*.rs'):
    content = test_file.read_text()
    if re.search(r'E: \[[^\]]*?Triple\(', content) or re.search(r'A: \[[^\]]*?Triple\(', content):
        print(f'⚠ Also needs fix: {test_file.name}')


