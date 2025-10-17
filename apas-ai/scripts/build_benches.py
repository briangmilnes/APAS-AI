#!/usr/bin/env python3
"""
Builds all benchmarks without running them using cargo bench --no-run.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys

def main():
    print("Building benchmarks with 'cargo bench --no-run -j 10'...")
    try:
        subprocess.run(["cargo", "bench", "--no-run", "-j", "10"], check=True)
        print("✅ Benchmark build successful!")
        return 0
    except subprocess.CalledProcessError as e:
        print(f"❌ Benchmark build failed: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())

