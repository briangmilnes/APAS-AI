#!/usr/bin/env python3
"""
Count actual benchmark runs in a file (including multiple input sizes).
"""

# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700

import sys
import re

if len(sys.argv) != 2:
    print("Usage: count_benchmark_runs.py <bench_file.rs>", file=sys.stderr)
    sys.exit(1)

bench_file = sys.argv[1]

with open(bench_file, 'r') as f:
    content = f.read()

# Count bench_with_input and bench_function calls
bench_with_input = len(re.findall(r'\.bench_with_input\s*\(', content))
bench_function = len(re.findall(r'\.bench_function\s*\(', content))

base_count = bench_with_input + bench_function

# Check for size multipliers like "for size in [10, 1000]" or "for &n in &[32, 64, 128]"
size_patterns = re.findall(r'for\s+[&\w]+\s+in\s+[&\[\]]*\[([^\]]+)\]', content)
if size_patterns:
    # Count elements in the first size array (assumes all are similar)
    first_pattern = size_patterns[0]
    num_sizes = len([x.strip() for x in first_pattern.split(',') if x.strip()])
    # Multiply base count by number of sizes
    total = base_count * num_sizes
else:
    total = base_count

print(total)
