#!/usr/bin/env python3
"""Run project benchmarks using cargo bench."""
# Git commit: 143f8b97182512ad358b60e1842552c21e868167
# Date: 2025-10-17 05:14:32 -0700


import subprocess
import sys
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Running benchmarks with cargo bench -j 10...")
    print("=" * 60)
    
    # Run cargo bench with parallel jobs
    result = subprocess.run(
        ["cargo", "bench", "-j", "10"],
        cwd=project_root
    )
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

