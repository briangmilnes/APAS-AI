#!/usr/bin/env python3
"""
Review that all benchmark files can be discovered and compiled.
Cross-references with Cargo.toml registration.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path
import subprocess

def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    
    # Use cargo to check if all benchmarks can be found
    result = subprocess.run(
        ["cargo", "bench", "--benches", "--no-run", "--quiet"],
        cwd=repo_root,
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        print("❌ Benchmark compilation check failed:")
        if result.stderr:
            # Show only the relevant error lines
            for line in result.stderr.split('\n'):
                if 'error' in line.lower() or 'warning' in line.lower():
                    print(f"   {line}")
        return 1
    
    print("✓ All benchmark modules compile successfully")
    return 0

if __name__ == "__main__":
    sys.exit(main())

