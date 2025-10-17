#!/usr/bin/env python3
"""Master code review runner.

Runs all review suites and outputs to both stdout and analyses/review.txt.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


class TeeWriter:
    """Write to both stdout and a file simultaneously."""
    
    def __init__(self, file_path):
        self.file = open(file_path, 'w', encoding='utf-8')
        self.stdout = sys.stdout
    
    def write(self, data):
        self.stdout.write(data)
        self.file.write(data)
        self.stdout.flush()
        self.file.flush()
    
    def flush(self):
        self.stdout.flush()
        self.file.flush()
    
    def close(self):
        self.file.close()


def main():
    script_dir = Path(__file__).parent
    repo_root = script_dir.parent
    analyses_dir = repo_root / "analyses"
    analyses_dir.mkdir(exist_ok=True)
    
    output_file = analyses_dir / "review.txt"
    
    # Set up tee output
    tee = TeeWriter(output_file)
    original_stdout = sys.stdout
    sys.stdout = tee
    
    try:
        suites = [
            ("APAS", "APAS/review_APAS.py"),
            ("Rust", "rust/review_rust.py"),
        ]
        
        print("=" * 60)
        print("APAS Project Code Review")
        print("=" * 60)
        print()
        
        failed = False
        for name, script in suites:
            script_path = script_dir / script
            if not script_path.exists():
                continue
            try:
                result = subprocess.run(
                    [sys.executable, str(script_path)],
                    capture_output=True,
                    text=True
                )
                # Print captured output
                if result.stdout:
                    print(result.stdout, end='')
                if result.stderr:
                    print(result.stderr, end='', file=sys.stderr)
                
                if result.returncode != 0:
                    print(f"\n❌ FAILED: {name} review suite")
                    failed = True
                    
            except subprocess.CalledProcessError as e:
                print(f"\n❌ FAILED: {name} review suite - {e}")
                failed = True
        
        print("=" * 60)
        if failed:
            print("❌ FAILED: Some code reviews failed!")
        else:
            print("✅ SUCCESS: All code reviews passed!")
        print("=" * 60)
        print(f"\nOutput saved to: {output_file.relative_to(repo_root)}")
        
        return 1 if failed else 0
        
    finally:
        sys.stdout = original_stdout
        tee.close()


if __name__ == "__main__":
    sys.exit(main())
