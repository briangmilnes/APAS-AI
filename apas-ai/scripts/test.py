#!/usr/bin/env python3
"""Run project tests using cargo nextest."""
# Git commit: 143f8b97182512ad358b60e1842552c21e868167
# Date: 2025-10-17 05:14:32 -0700


import subprocess
import sys
import re
from pathlib import Path


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean emacs compile mode output."""
    # Remove color codes: \x1b[...m
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    # Remove cursor control: \x1b[...letter
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def main():
    # Change to project root
    project_root = Path(__file__).parent.parent
    
    print("Running tests with cargo nextest...")
    print("=" * 60)
    
    # Run cargo nextest with --no-fail-fast, capture output for stripping
    result = subprocess.run(
        ["cargo", "nextest", "run", "--no-fail-fast"],
        cwd=project_root,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True
    )
    
    # Strip ANSI codes and print for emacs compile mode
    clean_output = strip_ansi_codes(result.stdout)
    print(clean_output, end='')
    
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())

