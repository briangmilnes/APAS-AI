#!/usr/bin/env python3
"""Review general Rust source code."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    my_name = Path(__file__).name
    
    # Find all review_*.py and find_*.py scripts (but NOT find_and_fix_* or fix_*)
    review_scripts = sorted([
        f for f in script_dir.glob("review_*.py")
        if f.name != my_name
    ])
    
    find_scripts = sorted([
        f for f in script_dir.glob("find_*.py")
        if not f.name.startswith("find_and_fix_") and not f.name.startswith("fix_")
    ])
    
    all_scripts = review_scripts + find_scripts
    
    if not all_scripts:
        print("✓ No Rust src review/find scripts configured")
        return 0
    
    print(f"Running {len(all_scripts)} Rust src check(s)\n")
    
    passed = 0
    failed = 0
    for script_path in all_scripts:
        name = script_path.stem.replace('review_', '').replace('find_', '').replace('_', ' ').title()
        prefix = "Review" if script_path.name.startswith("review_") else "Find"
        print(f"[{prefix}: {name}]")
        try:
            subprocess.run([sys.executable, str(script_path)], check=True)
            print()
            passed += 1
        except subprocess.CalledProcessError:
            print(f"FAILED: {name}\n")
            failed += 1
    
    if failed > 0:
        print(f"✗ Rust src: {passed} passed, {failed} failed")
        return 1
    else:
        print(f"✓ All Rust src checks passed ({passed}/{len(all_scripts)})")
        return 0


if __name__ == "__main__":
    sys.exit(main())
