#!/usr/bin/env python3
"""Review APAS benchmark code."""

import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    
    checks = [
        ("Benchmark Timing Parameters", "review_timing_params.py"),
        ("Benchmark Duplicate IDs", "review_duplicate_ids.py"),
        ("Benchmark Cargo Registration", "review_cargo_bench_names.py"),
    ]
    
    passed = 0
    for name, script in checks:
        script_path = script_dir / script
        if not script_path.exists():
            continue
        print(f"Running {name}...")
        try:
            subprocess.run([sys.executable, str(script_path)], check=True)
            passed += 1
        except subprocess.CalledProcessError:
            print(f"FAILED: {name}\n")
            return 1
    
    print(f"✓ All APAS benchmark checks passed ({passed}/{len(checks)})")
    return 0


if __name__ == "__main__":
    sys.exit(main())

