#!/usr/bin/env python3
"""GRIND CODEBASE: Comprehensive build, test, and benchmark compilation check.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Runs the full development cycle:
1. Compile source (cargo check --lib -j 10)
2. Compile tests (cargo test --no-run -j 10)
3. Run tests (cargo nextest run --no-fail-fast -j 10)
4. Compile benchmarks (cargo bench --no-run -j 10)

Stops at first failure for fast feedback.
"""

import subprocess
import sys
import re
from pathlib import Path


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean output."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def run_step(name, command, cwd):
    """Run a single step, return True if successful."""
    print(f"\n{'=' * 70}", flush=True)
    print(f"GRIND: {name}", flush=True)
    print(f"{'=' * 70}", flush=True)
    
    process = subprocess.Popen(
        command,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1
    )
    
    # Stream output line by line
    for line in process.stdout:
        clean_line = strip_ansi_codes(line)
        print(clean_line, end='', flush=True)
    
    returncode = process.wait()
    
    if returncode != 0:
        print(f"\n✗ GRIND FAILED at: {name}", flush=True)
        return False
    
    print(f"✓ {name} passed", flush=True)
    return True


def main():
    project_root = Path(__file__).parent.parent
    
    print("=" * 70, flush=True)
    print("GRIND: Comprehensive Build + Test + Bench Check", flush=True)
    print("=" * 70, flush=True)
    
    steps = [
        ("Compile source", ["cargo", "check", "--lib", "-j", "10"]),
        ("Compile tests", ["cargo", "test", "--no-run", "-j", "10"]),
        ("Run tests", ["cargo", "nextest", "run", "--no-fail-fast", "-j", "10"]),
        ("Compile benchmarks", ["cargo", "bench", "--no-run", "-j", "10"]),
    ]
    
    for name, command in steps:
        if not run_step(name, command, project_root):
            return 1
    
    print(f"\n{'=' * 70}", flush=True)
    print("✓ GRIND COMPLETE: All steps passed!", flush=True)
    print(f"{'=' * 70}", flush=True)
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

