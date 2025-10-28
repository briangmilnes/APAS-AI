#!/usr/bin/env python3
"""Run project benchmarks using cargo bench."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from datetime import datetime
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    # Log start time
    start_time = datetime.now()
    print(f"Benchmark started at: {start_time.strftime('%Y-%m-%d %H:%M:%S')}")
    print("Running benchmarks with cargo bench -j 10 --no-fail-fast...")
    print("=" * 60)
    
    # Run cargo bench with parallel jobs, filtering out terminal control characters
    # --no-fail-fast: Continue running all benchmarks even if one fails
    # sed removes ANSI escape sequences: \x1b[ followed by any number of digits/semicolons and a letter
    result = subprocess.run(
        "cargo bench -j 10 --no-fail-fast 2>&1 | sed 's/\\x1b\\[[0-9;]*[mGKHJA-Z]//g'",
        cwd=project_root,
        shell=True
    )
    
    # Log end time and duration
    end_time = datetime.now()
    elapsed = end_time - start_time
    print("=" * 60)
    print(f"Benchmark finished at: {end_time.strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Total elapsed time: {elapsed}")
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

