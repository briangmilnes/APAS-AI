#!/usr/bin/env python3
"""Compile benchmarks without running them using cargo bench --no-run."""
# Git commit: 08cec0603b305aa07307724314ae2656d8597279
# Date: 2025-10-18


import subprocess
import sys
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Compiling benchmarks with cargo bench --no-run -j 10...", flush=True)
    print("=" * 60, flush=True)
    
    # Run cargo bench --no-run with parallel jobs
    result = subprocess.run(
        ["cargo", "bench", "--no-run", "-j", "10"],
        cwd=project_root
    )
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

