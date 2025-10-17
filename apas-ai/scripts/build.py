#!/usr/bin/env python3
"""Build project using cargo build."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Building project with cargo build...")
    print("=" * 60)
    
    # Run cargo build with -j 10 to keep computer responsive
    result = subprocess.run(
        ["cargo", "build", "-j", "10"],
        cwd=project_root
    )
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

