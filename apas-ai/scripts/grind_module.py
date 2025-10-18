#!/usr/bin/env python3
"""GRIND MODULE: Build, test, and bench check for a specific module.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Usage:
  grind_module.py <module_name>  # e.g., AVLTreeSeq, LabDirGraphStEph

Runs (stops at first failure):
1. Compile source: cargo check --lib -j 10
2. Compile tests: cargo test --test Test<module> --no-run -j 10 (for each test)
3. Run tests: cargo nextest run --test Test<module> -j 10 (for each test)
4. Compile benchmarks: cargo bench --bench Bench<module> --no-run -j 10 (for each bench)
"""

import subprocess
import sys
import re
from pathlib import Path


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean output."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def run_step(name, command, cwd):
    """Run a single step, return True if successful."""
    print(f"\n{'=' * 70}", flush=True)
    print(f"GRIND SINGLE: {name}", flush=True)
    print(f"{'=' * 70}", flush=True)
    
    process = subprocess.Popen(
        command,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1
    )
    
    # Stream output line by line
    for line in process.stdout:
        clean_line = strip_ansi_codes(line)
        print(clean_line, end='', flush=True)
    
    returncode = process.wait()
    
    if returncode != 0:
        print(f"\n✗ GRIND SINGLE FAILED at: {name}", flush=True)
        return False
    
    print(f"✓ {name} passed", flush=True)
    return True


def find_tests_and_benches(project_root, module_name):
    """Find test and benchmark names for a module from Cargo.toml."""
    import re
    
    test_names = []
    bench_names = []
    
    cargo_toml = project_root / "Cargo.toml"
    if not cargo_toml.exists():
        return test_names, bench_names
    
    with open(cargo_toml, 'r') as f:
        content = f.read()
    
    # Find [[test]] sections with names containing module_name
    # Pattern: [[test]]\nname = "TestXXX"\npath = "..."
    test_pattern = r'\[\[test\]\]\s*name\s*=\s*"([^"]+)"\s*path\s*=\s*"[^"]*' + re.escape(module_name) + r'[^"]*"'
    test_matches = re.finditer(test_pattern, content, re.MULTILINE | re.IGNORECASE)
    for match in test_matches:
        test_names.append(match.group(1))
    
    # Find [[bench]] sections with names containing module_name
    bench_pattern = r'\[\[bench\]\]\s*name\s*=\s*"([^"]+)"\s*path\s*=\s*"[^"]*' + re.escape(module_name) + r'[^"]*"'
    bench_matches = re.finditer(bench_pattern, content, re.MULTILINE | re.IGNORECASE)
    for match in bench_matches:
        bench_names.append(match.group(1))
    
    return sorted(test_names), sorted(bench_names)


def main():
    if len(sys.argv) < 2:
        print("Usage: grind_module.py <module_name>")
        print("Example: grind_module.py AVLTreeSeq")
        return 1
    
    module_name = sys.argv[1]
    project_root = Path(__file__).parent.parent
    
    print("=" * 70, flush=True)
    print(f"GRIND SINGLE: {module_name}", flush=True)
    print("=" * 70, flush=True)
    
    # Find associated tests and benchmarks
    test_files, bench_files = find_tests_and_benches(project_root, module_name)
    
    print(f"\nFound {len(test_files)} test(s): {', '.join(test_files)}", flush=True)
    print(f"Found {len(bench_files)} benchmark(s): {', '.join(bench_files)}", flush=True)
    
    steps = []
    
    # Step 1: Compile source
    steps.append(("Compile source", ["cargo", "check", "--lib", "-j", "10"]))
    
    # Step 2-3: Compile and run each test
    for test_name in test_files:
        steps.append((f"Compile test: {test_name}", 
                     ["cargo", "test", "--test", test_name, "--no-run", "-j", "10"]))
        steps.append((f"Run test: {test_name}", 
                     ["cargo", "nextest", "run", "--test", test_name, "-j", "10"]))
    
    # Step 4: Compile each benchmark
    for bench_name in bench_files:
        steps.append((f"Compile benchmark: {bench_name}", 
                     ["cargo", "bench", "--bench", bench_name, "--no-run", "-j", "10"]))
    
    # Run all steps
    for name, command in steps:
        if not run_step(name, command, project_root):
            return 1
    
    print(f"\n{'=' * 70}", flush=True)
    print(f"✓ GRIND SINGLE COMPLETE for {module_name}: All steps passed!", flush=True)
    print(f"{'=' * 70}", flush=True)
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

