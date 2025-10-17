#!/usr/bin/env python3

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

import sys
import re

bench_file = sys.argv[1]
with open(bench_file, 'r') as f:
    content = f.read()

# Count bench_function calls
bench_function_count = len(re.findall(r'\.bench_function\(', content))

# Count bench_with_input calls  
bench_with_input_count = len(re.findall(r'\.bench_with_input\(', content))

# Look for loops with arrays like &[32, 64, 128]
for_loops = re.findall(r'for\s+&\w+\s+in\s+&\[([^\]]+)\]', content)
loop_iterations = sum(len(arr.split(',')) for arr in for_loops)

# Total is direct calls + loop iterations
total = bench_function_count + bench_with_input_count + loop_iterations

# If we found loops, subtract the single bench_with_input that's inside them
if loop_iterations > 0 and bench_with_input_count > 0:
    total = bench_function_count + loop_iterations

# Minimum 1 if file has criterion
if total == 0 and 'criterion' in content:
    total = 1

print(total)
