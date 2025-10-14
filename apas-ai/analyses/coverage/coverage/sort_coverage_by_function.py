#!/usr/bin/env python3
"""
Sort coverage data by function coverage percentage (high to low).
Reads from sorted_by_coverage.txt and writes to coverage_sorted_by_function_high_to_low.txt
"""

import re
import sys
from pathlib import Path

def parse_coverage_line(line):
    """Parse a coverage line and extract filename and function coverage percentage."""
    # Skip empty lines
    if not line.strip():
        return None, None
    
    # The format appears to be:
    # Filename    RegionsTotal RegionsMissed RegionsCovered%  FunctionsTotal FunctionsMissed FunctionsCovered% ...
    # We want to extract the filename and the function coverage percentage (6th numeric percentage)
    
    parts = line.split()
    if len(parts) < 7:
        return None, None
    
    filename = parts[0]
    
    # Find function coverage percentage (should be around index 6)
    # Format: XX.XX%
    function_pct = None
    pct_count = 0
    for part in parts[1:]:
        if '%' in part:
            pct_count += 1
            if pct_count == 2:  # Second percentage is function coverage
                try:
                    function_pct = float(part.rstrip('%'))
                    break
                except ValueError:
                    continue
    
    return filename, function_pct

def main():
    input_file = Path(__file__).parent / "sorted_by_coverage.txt"
    output_file = Path(__file__).parent / "coverage_sorted_by_function_high_to_low.txt"
    
    if not input_file.exists():
        print(f"Error: {input_file} not found", file=sys.stderr)
        sys.exit(1)
    
    # Read and parse all lines
    coverage_data = []
    with open(input_file, 'r') as f:
        for line in f:
            filename, function_pct = parse_coverage_line(line)
            if filename and function_pct is not None:
                coverage_data.append((filename, function_pct, line.strip()))
    
    # Sort by function coverage percentage (high to low)
    coverage_data.sort(key=lambda x: x[1], reverse=True)
    
    # Write sorted data
    with open(output_file, 'w') as f:
        f.write("# Coverage data sorted by Function Coverage % (High to Low)\n")
        f.write("# Format: Filename RegionsTotal RegionsMissed RegionsCovered% FunctionsTotal FunctionsMissed FunctionsCovered% ...\n")
        f.write("#" + "=" * 150 + "\n\n")
        
        for filename, function_pct, original_line in coverage_data:
            f.write(original_line + '\n')
    
    print(f"✓ Sorted {len(coverage_data)} files by function coverage (high to low)")
    print(f"✓ Output written to: {output_file}")
    print(f"\nTop 10 files by function coverage:")
    for i, (filename, function_pct, _) in enumerate(coverage_data[:10], 1):
        print(f"  {i:2d}. {function_pct:6.2f}%  {filename}")
    
    print(f"\nBottom 10 files by function coverage:")
    for i, (filename, function_pct, _) in enumerate(coverage_data[-10:], 1):
        print(f"  {i:2d}. {function_pct:6.2f}%  {filename}")

if __name__ == "__main__":
    main()

