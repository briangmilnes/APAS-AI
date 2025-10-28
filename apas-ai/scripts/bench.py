#!/usr/bin/env python3
"""Run project benchmarks using cargo bench."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Running benchmarks with cargo bench -j 10...")
    print("=" * 60)
    
    # Run cargo bench with parallel jobs, filtering out terminal control characters
    # sed removes ANSI escape sequences: \x1b[ followed by any number of digits/semicolons and a letter
    result = subprocess.run(
        "cargo bench -j 10 2>&1 | sed 's/\\x1b\\[[0-9;]*[mGKHJA-Z]//g'",
        cwd=project_root,
        shell=True
    )
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

