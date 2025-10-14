#!/usr/bin/env python3
"""
Builds all tests without running them using cargo test --no-run.
"""

import subprocess
import sys

def main():
    print("Building tests with 'cargo test --no-run -j 10'...")
    try:
        subprocess.run(["cargo", "test", "--no-run", "-j", "10"], check=True)
        print("✅ Test build successful!")
        return 0
    except subprocess.CalledProcessError as e:
        print(f"❌ Test build failed: {e}")
        return 1

if __name__ == "__main__":
    sys.exit(main())

