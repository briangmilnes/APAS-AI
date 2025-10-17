#!/usr/bin/env python3
"""
Identify the next N files with lowest coverage for test improvement.
Excludes files already at 100% and example files.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import json
import sys
from pathlib import Path


def main(n=30):
    """Identify next N files with lowest coverage."""
    if len(sys.argv) > 1:
        n = int(sys.argv[1])
    
    project_root = Path(__file__).parent.parent.parent
    coverage_file = project_root / "analyses" / "coverage.json"
    
    if not coverage_file.exists():
        print("Error: coverage.json not found. Run ./scripts/llvm-cov.py first.")
        return 1
    
    with open(coverage_file) as f:
        data = json.load(f)
    
    files = data['data'][0]['files']
    
    # Extract src/ files with their coverage
    candidates = []
    for file_data in files:
        filename = file_data['filename']
        
        # Only src/ files, skip examples
        if '/src/Chap' not in filename:
            continue
        if 'Example' in filename:
            continue
        
        summary = file_data['summary']
        lines_percent = summary['lines']['percent']
        lines_covered = summary['lines']['covered']
        lines_total = summary['lines']['count']
        uncovered = lines_total - lines_covered
        
        # Skip files already at 100%
        if lines_percent >= 100.0:
            continue
        
        # Extract short name
        short_name = filename.split('/src/')[-1]
        
        candidates.append({
            'name': short_name,
            'coverage': lines_percent,
            'uncovered': uncovered,
            'total': lines_total,
        })
    
    # Sort by coverage percentage (lowest first)
    candidates.sort(key=lambda x: x['coverage'])
    
    # Take top N
    targets = candidates[:n]
    
    print("═" * 79)
    print(f"              NEXT {n} LOWEST-COVERAGE FILES FOR TEST IMPROVEMENT")
    print("═" * 79)
    print()
    
    for i, file in enumerate(targets, 1):
        print(f"{i:2}. {file['coverage']:5.1f}% | {file['uncovered']:4} uncov / "
              f"{file['total']:4} total | {file['name']}")
    
    print()
    print(f"Total uncovered lines in these {len(targets)} files: "
          f"{sum(f['uncovered'] for f in targets):,}")
    print("═" * 79)
    
    return 0


if __name__ == "__main__":
    sys.exit(main())


