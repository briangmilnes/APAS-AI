#!/usr/bin/env python3
"""Review APAS benchmark code."""

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
    ])
    
    if not scripts:
        print("✓ No APAS benchmark review scripts configured")
        return 0
    
    print(f"Running {len(scripts)} APAS benchmark check(s)\n")
    
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
        print(f"✗ APAS benches: {passed} passed, {failed} failed")
        return 1
    else:
        print(f"✓ All APAS benchmark checks passed ({passed}/{len(scripts)})")
        return 0


if __name__ == "__main__":
    sys.exit(main())
