#!/usr/bin/env python3
"""Run cargo llvm-cov to generate test coverage reports."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import subprocess
import sys
from pathlib import Path


def main():
    project_root = Path(__file__).parent.parent
    analyses_dir = project_root / "analyses"
    analyses_dir.mkdir(exist_ok=True)
    
    print("Running cargo llvm-cov for test coverage analysis...")
    print("=" * 60)
    
    # Run tests once and generate JSON (most detailed format)
    print("Running tests with coverage instrumentation...\n")
    result = subprocess.run(
        [
            "cargo", "llvm-cov",
            "--all-features",
            "--workspace",
            "-j", "10",
            "--json",
            "--output-path", str(analyses_dir / "coverage.json")
        ],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    
    if result.returncode != 0:
        print("Error running cargo llvm-cov:")
        print(result.stderr)
        return result.returncode
    
    print(result.stdout)
    if result.stderr:
        print(result.stderr)
    
    print(f"\n✓ Coverage data saved to: {analyses_dir / 'coverage.json'}")
    
    # Now generate other formats WITHOUT re-running tests
    print("\nGenerating additional report formats (no re-run)...")
    
    # Generate LCOV format
    result2 = subprocess.run(
        [
            "cargo", "llvm-cov",
            "--no-run",
            "--lcov",
            "--output-path", str(analyses_dir / "lcov.info")
        ],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    
    if result2.returncode == 0:
        print(f"  - {analyses_dir / 'lcov.info'} (LCOV format)")
    
    # Generate human-readable summary
    result3 = subprocess.run(
        [
            "cargo", "llvm-cov",
            "--no-run",
            "--summary-only"
        ],
        cwd=project_root,
        capture_output=True,
        text=True
    )
    
    if result3.returncode == 0:
        # Save summary to file
        summary_file = analyses_dir / "coverage_summary.txt"
        with open(summary_file, 'w') as f:
            f.write(result3.stdout)
            if result3.stderr:
                f.write("\n\nErrors/Warnings:\n")
                f.write(result3.stderr)
        
        print(f"  - {analyses_dir / 'coverage_summary.txt'} (summary)")
        print("\n" + result3.stdout)
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

