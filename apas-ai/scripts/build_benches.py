#!/usr/bin/env python3
"""
Builds all benchmarks without running them using cargo bench --no-run.
"""

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

