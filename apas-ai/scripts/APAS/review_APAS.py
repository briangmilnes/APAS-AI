#!/usr/bin/env python3
"""Run all APAS code reviews."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    
    suites = [
        ("APAS src", "src/review_APAS_src.py"),
        ("APAS tests", "tests/review_APAS_tests.py"),
        ("APAS benches", "benches/review_APAS_benches.py"),
    ]
    
    print("Running APAS Code Review\n")
    
    for name, script in suites:
        script_path = script_dir / script
        if not script_path.exists():
            continue
        print(f"[{name}]")
        try:
            subprocess.run([sys.executable, str(script_path)], check=True)
            print()
        except subprocess.CalledProcessError:
            print(f"\nFAILED: {name}")
            return 1
    
    print("✓ All APAS reviews passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
