#!/usr/bin/env python3
"""Master code review runner."""

import subprocess
import sys
from pathlib import Path


def main():
    script_dir = Path(__file__).parent
    
    suites = [
        ("APAS", "APAS/review_APAS.py"),
        ("Rust", "rust/review_rust.py"),
    ]
    
    print("=" * 60)
    print("APAS Project Code Review")
    print("=" * 60)
    print()
    
    for name, script in suites:
        script_path = script_dir / script
        if not script_path.exists():
            continue
        try:
            subprocess.run([sys.executable, str(script_path)], check=True)
        except subprocess.CalledProcessError:
            print(f"\n❌ FAILED: {name} review suite")
            return 1
    
    print("=" * 60)
    print("✅ SUCCESS: All code reviews passed!")
    print("=" * 60)
    return 0


if __name__ == "__main__":
    sys.exit(main())
