#!/usr/bin/env python3
"""Review general Rust test code."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    my_name = Path(__file__).name
    
    # Find all review_*.py and find_*.py scripts in this directory (excluding this script)
    scripts = sorted([
        f for f in script_dir.glob("review_*.py")
        if f.name != my_name
    ] + list(script_dir.glob("find_*.py")))
    
    if not scripts:
        print("✓ No Rust test review/find scripts configured")
        return 0
    
    print(f"Running {len(scripts)} Rust test check(s)\n")
    
    passed = 0
    failed = 0
    for script_path in scripts:
        prefix = 'review_' if script_path.name.startswith('review_') else 'find_'
        name = script_path.stem.replace(prefix, '').replace('_', ' ').title()
        print(f"[{prefix.strip('_').title()}: {name}]")
        try:
            subprocess.run([sys.executable, str(script_path)], check=True)
            print()
            passed += 1
        except subprocess.CalledProcessError:
            print(f"FAILED: {name}\n")
            failed += 1
    
    if failed > 0:
        print(f"✗ Rust tests: {passed} passed, {failed} failed")
        return 1
    else:
        print(f"✓ All Rust test checks passed ({passed}/{len(scripts)})")
        return 0


if __name__ == "__main__":
    sys.exit(main())
