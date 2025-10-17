#!/usr/bin/env python3
"""
Builds all benchmarks without running them using cargo bench --no-run.
"""
# Git commit: bbb60b54de5c4f85b06c43d0b4173e9d4a66bbad
# Date: 2025-10-14 15:17:28 -0700


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

