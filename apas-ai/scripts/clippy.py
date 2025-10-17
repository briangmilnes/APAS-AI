#!/usr/bin/env python3
"""
Runs Clippy on the project and saves output to analyses/clippy.txt.
Output is also displayed to stdout for Emacs compile mode integration.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path

def main():
    repo_root = Path(__file__).parent.parent
    output_file = repo_root / "analyses" / "clippy.txt"
    
    print("Running Clippy analysis...")
    print(f"Output will be saved to: {output_file}")
    print("-" * 70)
    
    # Run clippy with all targets and features
    # Use Popen to stream output in real-time
    process = subprocess.Popen(
        ["cargo", "clippy", "--all-targets", "--all-features"],
        cwd=repo_root,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1
    )
    
    # Stream output to both stdout and file
    with open(output_file, 'w') as f:
        for line in process.stdout:
            print(line, end='')  # Print to stdout for Emacs
            f.write(line)        # Write to file for analysis
    
    return_code = process.wait()
    
    print("-" * 70)
    if return_code == 0:
        print("✅ Clippy completed successfully!")
    else:
        print(f"⚠️  Clippy completed with warnings/errors (exit code: {return_code})")
    
    print(f"\nRun './scripts/counting/review_clippy.py' for Pareto analysis")
    
    return return_code

if __name__ == "__main__":
    sys.exit(main())

