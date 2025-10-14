#!/usr/bin/env python3
"""
Sort coverage data by region coverage percentage (high to low).
Reads from sorted_by_coverage.txt and writes to coverage_sorted_by_region_high_to_low.txt
"""

import re
import sys
from pathlib import Path

def parse_coverage_line(line):
    """Parse a coverage line and extract filename and region coverage percentage."""
    # Skip empty lines
    if not line.strip():
        return None, None
    
    # The format appears to be:
    # Filename    RegionsTotal RegionsMissed RegionsCovered%  FunctionsTotal FunctionsMissed ...
    # We want to extract the filename and the third percentage (region coverage)
    
    parts = line.split()
    if len(parts) < 7:
        return None, None
    
    filename = parts[0]
    
    # Find region coverage percentage (should be around index 3-4)
    # Format: XX.XX%
    region_pct = None
    for i, part in enumerate(parts[1:7]):
        if '%' in part and i >= 2:  # Region percentage is typically 3rd numeric group
            try:
                region_pct = float(part.rstrip('%'))
                break
            except ValueError:
                continue
    
    return filename, region_pct

def main():
    input_file = Path(__file__).parent / "sorted_by_coverage.txt"
    output_file = Path(__file__).parent / "coverage_sorted_by_region_high_to_low.txt"
    
    if not input_file.exists():
        print(f"Error: {input_file} not found", file=sys.stderr)
        sys.exit(1)
    
    # Read and parse all lines
    coverage_data = []
    with open(input_file, 'r') as f:
        for line in f:
            filename, region_pct = parse_coverage_line(line)
            if filename and region_pct is not None:
                coverage_data.append((filename, region_pct, line.strip()))
    
    # Sort by region coverage percentage (high to low)
    coverage_data.sort(key=lambda x: x[1], reverse=True)
    
    # Write sorted data
    with open(output_file, 'w') as f:
        f.write("# Coverage data sorted by Region Coverage % (High to Low)\n")
        f.write("# Format: Filename RegionsTotal RegionsMissed RegionsCovered% ...\n")
        f.write("#" + "=" * 150 + "\n\n")
        
        for filename, region_pct, original_line in coverage_data:
            f.write(original_line + '\n')
    
    print(f"✓ Sorted {len(coverage_data)} files by region coverage (high to low)")
    print(f"✓ Output written to: {output_file}")
    print(f"\nTop 10 files by region coverage:")
    for i, (filename, region_pct, _) in enumerate(coverage_data[:10], 1):
        print(f"  {i:2d}. {region_pct:6.2f}%  {filename}")
    
    print(f"\nBottom 10 files by region coverage:")
    for i, (filename, region_pct, _) in enumerate(coverage_data[-10:], 1):
        print(f"  {i:2d}. {region_pct:6.2f}%  {filename}")

if __name__ == "__main__":
    main()

