#!/usr/bin/env python3
"""
Debug script to trace macro detection in HashFunctionTraits.rs
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

import sys
from pathlib import Path

# Add rust/src to path
sys.path.insert(0, str(Path(__file__).parent.parent / 'rust' / 'src'))

from fix_impl_order import count_braces_in_line

def main():
    file_path = Path(__file__).parent.parent.parent / 'src/Chap47/HashFunctionTraits.rs'
    
    with open(file_path) as f:
        lines = f.readlines()
    
    # Start from macro_rules! line
    print("Tracing macro from line 348:")
    i = 347  # Line 348 in 1-indexed, 347 in 0-indexed
    brace_count = 0
    started = False
    
    for j in range(i, min(i+35, len(lines))):
        line = lines[j]
        open_b, close_b = count_braces_in_line(line)
        if open_b > 0:
            started = True
        brace_count += open_b - close_b
        
        ends_semi = line.strip().endswith('};')
        print(f"Line {j+1}: open={open_b}, close={close_b}, brace_count={brace_count}, ends_semi={ends_semi}, started={started}")
        print(f"  Content: {line.rstrip()}")
        
        if started and brace_count == 1 and ends_semi:
            print(f"  --> MACRO ENDS AT LINE {j+1}")
            break

if __name__ == '__main__':
    main()

