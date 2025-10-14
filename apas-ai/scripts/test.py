#!/usr/bin/env python3
"""Run project tests using cargo nextest."""

import subprocess
import sys
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Running tests with cargo nextest...")
    print("=" * 60)
    
    # Run cargo nextest with --no-fail-fast
    result = subprocess.run(
        ["cargo", "nextest", "run", "--no-fail-fast"],
        cwd=project_root
    )
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

