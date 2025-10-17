#!/usr/bin/env python3
"""
Check all benchmark files for duplicate benchmark IDs.

A duplicate occurs when the same BenchmarkId::new(name, param) combination
appears multiple times within the same benchmark group function.

Usage:
    python3 scripts/benches/check_duplicate_ids.py
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from collections import defaultdict


def check_file_for_duplicates(filepath):
    """Check a single benchmark file for duplicate benchmark IDs with same name and parameter."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Find all benchmark group functions
    # Pattern: fn bench_something(c: &mut Criterion) { ... }
    func_pattern = r'fn\s+(\w+)\s*\([^)]*Criterion[^)]*\)\s*\{((?:[^{}]|\{(?:[^{}]|\{[^{}]*\})*\})*)\}'
    
    duplicates = []
    functions = re.finditer(func_pattern, content, re.MULTILINE | re.DOTALL)
    
    for func_match in functions:
        func_name = func_match.group(1)
        func_body = func_match.group(2)
        
        # Find all BenchmarkId::new calls with both parameters
        # Pattern: BenchmarkId::new("name", param) where param could be a variable or literal
        bench_id_pattern = r'BenchmarkId::new\s*\(\s*"([^"]+)"\s*,\s*([^)]+)\)'
        bench_ids = re.findall(bench_id_pattern, func_body)
        
        if len(bench_ids) > 0:
            # Group by name
            by_name = defaultdict(list)
            for name, param in bench_ids:
                by_name[name].append(param.strip())
            
            # Check for duplicates within same name
            for name, params in by_name.items():
                if len(params) != len(set(params)):
                    param_counts = defaultdict(int)
                    for param in params:
                        param_counts[param] += 1
                    
                    for param, count in param_counts.items():
                        if count > 1:
                            duplicates.append((func_name, name, param, count))
    
    return duplicates


def main():
    # Find all benchmark files
    bench_files = []
    for root, dirs, files in os.walk('benches'):
        for file in files:
            if file.endswith('.rs') and file.startswith('Bench'):
                bench_files.append(os.path.join(root, file))

    bench_files.sort()

    # Check each file
    all_duplicates = {}
    for filepath in bench_files:
        duplicates = check_file_for_duplicates(filepath)
        if duplicates:
            all_duplicates[filepath] = duplicates

    # Report results
    if all_duplicates:
        print(f"\nERROR: Found duplicate benchmark IDs in {len(all_duplicates)} file(s):\n")
        for filepath, duplicates in all_duplicates.items():
            print(f"  {filepath}:")
            for func_name, name, param, count in duplicates:
                print(f"    - Function '{func_name}': BenchmarkId::new(\"{name}\", {param}) appears {count} times")
        print(f"\nTotal files with duplicates: {len(all_duplicates)}")
        print(f"Total files scanned: {len(bench_files)}")
        print("\nFix: Ensure each BenchmarkId::new(name, param) combination is unique within its group.")
        sys.exit(1)
    else:
        print(f"✓ No duplicate benchmark IDs found!")
        print(f"  Total files scanned: {len(bench_files)}")
        sys.exit(0)


if __name__ == "__main__":
    main()



