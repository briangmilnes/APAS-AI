#!/usr/bin/env python3
"""Run project tests using cargo nextest."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


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
    
    print("Running tests with cargo nextest...", flush=True)
    print("=" * 60, flush=True)
    
    # Run cargo nextest with --no-fail-fast, stream output line by line
    process = subprocess.Popen(
        ["cargo", "nextest", "run", "--no-fail-fast", "-j", "10"],
        cwd=project_root,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1  # Line buffered
    )
    
    # Stream output line by line, stripping ANSI codes
    for line in process.stdout:
        clean_line = strip_ansi_codes(line)
        print(clean_line, end='', flush=True)
    
    # Wait for process to complete
    returncode = process.wait()
    
    return returncode


if __name__ == "__main__":
    sys.exit(main())

