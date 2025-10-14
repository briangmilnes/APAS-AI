#!/usr/bin/env python3
"""Review general Rust benchmark code."""

import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    
    # Add general Rust benchmark review scripts here when created
    checks = [
        # ("Benchmark Performance", "review_benchmark_perf.py"),
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
    
    if not checks:
        print("✓ No Rust benchmark checks configured")
    else:
        print(f"✓ All Rust benchmark checks passed ({passed}/{len(checks)})")
    return 0


if __name__ == "__main__":
    sys.exit(main())

