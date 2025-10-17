#!/usr/bin/env python3
"""Run all general Rust code reviews."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    
    # Cross-cutting checks (check all of src/, tests/, benches/)
    cross_cutting = [
        ("No extern crate", "review_no_extern_crate.py"),
        ("No UFCS at call sites", "review_no_ufcs_call_sites.py"),
        ("Import order", "review_import_order.py"),
        ("CamelCase file names", "review_camelcase.py"),
    ]
    
    # Directory-specific checks
    suites = [
        ("Rust src", "src/review_rust_src.py"),
        ("Rust tests", "tests/review_rust_tests.py"),
        ("Rust benches", "benches/review_rust_benches.py"),
    ]
    
    print("Running Rust Code Review\n")
    
    # Run cross-cutting checks first
    for name, script in cross_cutting:
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
    
    # Run directory-specific checks
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
    
    print("✓ All Rust reviews passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
