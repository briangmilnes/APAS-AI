#!/usr/bin/env python3
"""
Review ALL files to detect trait impls that forward to inherent impls.

Reports files where trait implementations are NOT stand-alone.
Stand-alone means: trait impl has its own logic, doesn't call inherent impl.
"""
# Git commit: TBD
# Date: 2025-10-18

import sys
from pathlib import Path
import subprocess


def main():
    src_dir = Path('src')
    
    print("Scanning all files for trait impl forwarding to inherent impl...\n")
    
    results = []
    
    for rs_file in sorted(src_dir.rglob('*.rs')):
        # Run the existing detect script on each file
        result = subprocess.run(
            ['python3', 'scripts/rust/src/detect_delegation_to_inherent.py', '--file', str(rs_file)],
            capture_output=True,
            text=True
        )
        
        if result.returncode == 0:  # Found forwarding
            results.append((str(rs_file), result.stdout.strip()))
    
    if not results:
        print("✓ All trait impls are stand-alone (no forwarding detected)")
        return 0
    
    print("=" * 100)
    print("TRAIT IMPLS THAT FORWARD TO INHERENT IMPLS (NOT STAND-ALONE):")
    print("=" * 100)
    print()
    
    for file, output in results:
        print(output)
        print()
    
    print("=" * 100)
    print(f"\nSummary: Found {len(results)} file(s) with trait impl forwarding")
    print("\nThese need to be fixed BEFORE removing inherent impls:")
    print("  1. Move implementation from inherent impl to trait impl")
    print("  2. Make trait impl stand-alone (no calls to inherent methods)")
    print("  3. Then inherent impl can be safely removed")
    
    return 1


if __name__ == '__main__':
    sys.exit(main())

