#!/usr/bin/env python3
"""Run project tests using cargo nextest."""
# Git commit: a481a74d358eb5791997213bbdaa7318418fbb7c
# Date: 2025-10-14 18:16:19 -0700


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

