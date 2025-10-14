#!/usr/bin/env python3
"""Build project using cargo build."""

import subprocess
import sys
from pathlib import Path


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Building project with cargo build...")
    print("=" * 60)
    
    # Run cargo build
    result = subprocess.run(
        ["cargo", "build"],
        cwd=project_root
    )
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

