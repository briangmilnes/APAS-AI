#!/usr/bin/env python3
"""
Review Cargo.toml to ensure all test and benchmark files are registered.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path
import re

def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    cargo_toml = repo_root / "Cargo.toml"
    tests_dir = repo_root / "tests"
    benches_dir = repo_root / "benches"
    
    # Read Cargo.toml
    with open(cargo_toml) as f:
        cargo_content = f.read()
    
    # Extract registered tests
    test_pattern = re.compile(r'name\s*=\s*"([^"]+)".*?path\s*=\s*"tests/([^"]+)"', re.DOTALL)
    registered_tests = {match.group(2) for match in test_pattern.finditer(cargo_content)}
    
    # Extract registered benchmarks
    bench_pattern = re.compile(r'name\s*=\s*"([^"]+)".*?path\s*=\s*"benches/([^"]+)"', re.DOTALL)
    registered_benches = {match.group(2) for match in bench_pattern.finditer(cargo_content)}
    
    # Find all test files
    test_files = {f.relative_to(tests_dir).as_posix() 
                  for f in tests_dir.rglob("*.rs")}
    
    # Find all benchmark files
    bench_files = {f.relative_to(benches_dir).as_posix() 
                   for f in benches_dir.rglob("*.rs")}
    
    # Check for missing registrations
    missing_tests = test_files - registered_tests
    missing_benches = bench_files - registered_benches
    
    if missing_tests or missing_benches:
        if missing_tests:
            print("❌ Tests not registered in Cargo.toml:")
            for test in sorted(missing_tests):
                print(f"   tests/{test}")
        
        if missing_benches:
            print("❌ Benchmarks not registered in Cargo.toml:")
            for bench in sorted(missing_benches):
                print(f"   benches/{bench}")
        
        total_violations = len(missing_tests) + len(missing_benches)
        print(f"\nTotal violations: {total_violations}")
        return 1
    
    print(f"✓ All {len(test_files)} tests and {len(bench_files)} benchmarks registered in Cargo.toml")
    return 0

if __name__ == "__main__":
    sys.exit(main())

