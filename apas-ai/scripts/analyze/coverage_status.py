#!/usr/bin/env python3
"""
Check overall project coverage status against target.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import json
import sys
from pathlib import Path


def main():
    project_root = Path(__file__).parent.parent.parent
    coverage_file = project_root / "analyses" / "coverage.json"
    
    if not coverage_file.exists():
        print("Error: coverage.json not found. Run ./scripts/llvm-cov.py first.")
        return 1
    
    with open(coverage_file) as f:
        data = json.load(f)
    
    # Get overall coverage
    summary = data['data'][0]['totals']
    lines_covered = summary['lines']['covered']
    lines_total = summary['lines']['count']
    lines_percent = summary['lines']['percent']
    lines_uncovered = lines_total - lines_covered
    
    target = 90.0
    gap = lines_percent - target
    
    print("═" * 79)
    print("                    PROJECT-WIDE COVERAGE STATUS")
    print("═" * 79)
    print()
    print(f"Overall Coverage:    {lines_percent:.2f}%")
    print(f"Lines Covered:       {lines_covered:,} / {lines_total:,}")
    print(f"Lines Uncovered:     {lines_uncovered:,}")
    print()
    print(f"Target:              {target:.2f}%")
    print(f"Gap:                 {gap:+.2f}%")
    print()
    
    if lines_percent >= target:
        print(f"✓ TARGET ACHIEVED! Coverage is {gap:.2f}% above target.")
        status = 0
    else:
        # Calculate lines needed
        lines_needed = int((target / 100.0) * lines_total) - lines_covered
        print(f"✗ Below target. Need {lines_needed:,} more lines covered.")
        status = 1
    
    print()
    print("═" * 79)
    
    return status


if __name__ == "__main__":
    sys.exit(main())

