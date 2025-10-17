#!/usr/bin/env python3
"""
Generate coverage report by source file showing line coverage.
This is more accurate than function counts.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path
import re


def parse_lcov(lcov_file):
    """Parse LCOV file to get coverage per source file."""
    coverage_data = []
    current_file = None
    lines_found = 0
    lines_hit = 0
    
    with open(lcov_file, 'r') as f:
        for line in f:
            line = line.strip()
            
            if line.startswith('SF:'):
                # Source file
                current_file = line[3:]
            elif line.startswith('LF:'):
                # Lines found
                lines_found = int(line[3:])
            elif line.startswith('LH:'):
                # Lines hit
                lines_hit = int(line[3:])
            elif line == 'end_of_record':
                if current_file and lines_found > 0:
                    # Only track src/ files
                    if '/src/' in current_file and not current_file.endswith('main.rs'):
                        coverage_pct = (lines_hit / lines_found * 100) if lines_found > 0 else 100
                        uncovered = lines_found - lines_hit
                        
                        # Extract module name from path
                        if '/src/' in current_file:
                            module = current_file.split('/src/')[1].replace('.rs', '').replace('/', '::')
                        else:
                            module = Path(current_file).stem
                        
                        coverage_data.append({
                            'module': module,
                            'file': current_file,
                            'lines_total': lines_found,
                            'lines_covered': lines_hit,
                            'lines_uncovered': uncovered,
                            'coverage_pct': coverage_pct
                        })
                
                # Reset for next file
                current_file = None
                lines_found = 0
                lines_hit = 0
    
    return coverage_data


def main():
    repo_root = Path(__file__).parent.parent.parent
    analyses_dir = repo_root / "analyses"
    lcov_file = analyses_dir / "lcov.info"
    
    if not lcov_file.exists():
        print("Error: lcov.info not found. Run ./scripts/llvm-cov.py first.")
        return 1
    
    print("Parsing line coverage data...\n")
    coverage_data = parse_lcov(lcov_file)
    
    # Sort by uncovered lines (most to least)
    coverage_data.sort(key=lambda x: x['lines_uncovered'], reverse=True)
    
    # Calculate totals
    total_lines = sum(d['lines_total'] for d in coverage_data)
    total_covered = sum(d['lines_covered'] for d in coverage_data)
    total_uncovered = total_lines - total_covered
    overall_pct = (total_covered / total_lines * 100) if total_lines > 0 else 100
    
    print("=" * 90)
    print(f"LINE COVERAGE: {total_covered}/{total_lines} lines ({overall_pct:.1f}%), {total_uncovered} uncovered")
    print("=" * 90)
    print()
    print(f"{'Module':<50} {'Coverage':>10} {'Uncov':>8} {'Total':>8}")
    print("─" * 90)
    
    for d in coverage_data:
        if d['lines_uncovered'] > 0:  # Only show files with uncovered lines
            print(f"{d['module']:<50} {d['coverage_pct']:>9.1f}% {d['lines_uncovered']:>8} {d['lines_total']:>8}")
    
    # Save report
    output_file = analyses_dir / "coverage_by_file.txt"
    with open(output_file, 'w') as f:
        f.write("=" * 90 + "\n")
        f.write(f"LINE COVERAGE: {total_covered}/{total_lines} lines ({overall_pct:.1f}%), {total_uncovered} uncovered\n")
        f.write("=" * 90 + "\n\n")
        f.write(f"{'Module':<50} {'Coverage':>10} {'Uncov':>8} {'Total':>8}\n")
        f.write("─" * 90 + "\n")
        
        for d in coverage_data:
            if d['lines_uncovered'] > 0:
                f.write(f"{d['module']:<50} {d['coverage_pct']:>9.1f}% {d['lines_uncovered']:>8} {d['lines_total']:>8}\n")
    
    print(f"\n✓ Coverage report saved to: {output_file.relative_to(repo_root)}")
    print(f"\nHTML report available at: target/llvm-cov/html/index.html")
    print("\nTo see specific uncovered lines, use:")
    print("  cargo llvm-cov --html --open")
    
    return 1 if total_uncovered > 0 else 0


if __name__ == "__main__":
    sys.exit(main())

