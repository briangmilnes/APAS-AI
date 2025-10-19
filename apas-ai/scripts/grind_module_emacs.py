#!/usr/bin/env python3
"""GRIND MODULE EMACS: Build, test, and bench check with emacs-friendly error output.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Usage:
  grind_module_emacs.py <module_name>  # e.g., AVLTreeSeq, LabDirGraphStEph

Outputs errors in emacs-friendly format: file.rs:line:col: error[EXXX]: message
Strips cargo's verbose multiline output, showing only concise error lines.
"""

import subprocess
import sys
import re
import os
from pathlib import Path


def strip_ansi_codes(text):
    """Strip ANSI escape codes for clean output."""
    text = re.sub(r'\x1b\[[0-9;]*m', '', text)
    text = re.sub(r'\x1b\[[0-9]*[ABCDEFGHJKST]', '', text)
    return text


def parse_cargo_errors_streaming(lines_buffer, module_name, show_all=False):
    """Parse cargo output and print emacs-friendly error lines as they're found.
    
    Cargo format:
        error[E0599]: no method named `foo` found
          --> src/file.rs:123:45
           |
        123 |     something.foo()
        
    Emacs format:
        src/file.rs:123:45: error[E0599]: no method named `foo` found
        
    If show_all=False: Only includes errors from files matching the module name.
    If show_all=True: Shows ALL errors regardless of module.
    Returns count of errors found.
    """
    error_count = 0
    
    i = 0
    while i < len(lines_buffer):
        line = strip_ansi_codes(lines_buffer[i])
        
        # Look for error/warning lines
        if line.startswith('error') or line.startswith('warning'):
            error_msg = line.strip()
            
            # Look ahead for the --> line with file location
            for j in range(i+1, min(i+5, len(lines_buffer))):
                next_line = strip_ansi_codes(lines_buffer[j])
                if '-->' in next_line:
                    # Extract file:line:col from "--> src/file.rs:123:45"
                    match = re.search(r'-->\s+(.+):(\d+):(\d+)', next_line)
                    if match:
                        filepath = match.group(1)
                        line_num = match.group(2)
                        col_num = match.group(3)
                        
                        # Check if we should include this error
                        filename = Path(filepath).stem  # Get filename without extension
                        include_error = show_all or (module_name.lower() in filename.lower())
                        
                        if include_error:
                            # Format for emacs: file:line:col: error message
                            emacs_line = f"{filepath}:{line_num}:{col_num}: {error_msg}"
                            print(emacs_line, flush=True)
                            error_count += 1
                    break
        
        i += 1
    
    return error_count


def run_step(name, command, cwd, module_name, show_output=False):
    """Run a single step, return (success, error_count, output_lines)."""
    process = subprocess.Popen(
        command,
        cwd=cwd,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        bufsize=1
    )
    
    output_lines = []
    test_ran = False
    for line in process.stdout:
        output_lines.append(line)
        # Show output for test runs - but filter out compilation errors
        if show_output:
            # Only show test execution lines, not compilation noise
            stripped = strip_ansi_codes(line)
            if 'running' in stripped.lower() or stripped.strip().startswith('test '):
                print(line, end='', flush=True)
                test_ran = True
            elif 'passed' in stripped or 'failed' in stripped or 'result:' in stripped.lower():
                print(line, end='', flush=True)
    
    returncode = process.wait()
    
    if returncode != 0:
        error_count = parse_cargo_errors_streaming(output_lines, module_name)
        # For source compilation: if no errors in this module, it's clean (other files broken)
        # For test runs: if cargo failed and test didn't run, it's a failure
        if name.startswith("Run test:") and not test_ran:
            return False, error_count, output_lines  # Test didn't run due to compilation failures
        # If no errors in THIS module, consider it success (other files might be broken)
        return (error_count == 0), error_count, output_lines
    
    return True, 0, output_lines


def find_tests_and_benches(project_root, module_name):
    """Find test and benchmark names for a module from Cargo.toml."""
    test_names = []
    bench_names = []
    
    cargo_toml = project_root / "Cargo.toml"
    if not cargo_toml.exists():
        return test_names, bench_names
    
    with open(cargo_toml, 'r') as f:
        content = f.read()
    
    # Find [[test]] sections with names containing module_name
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
        print("Usage: grind_module_emacs.py <module_name_or_file>")
        print("Example: grind_module_emacs.py AVLTreeSeq")
        print("Example: grind_module_emacs.py src/Chap18/ArraySeqStEph.rs")
        return 1
    
    # Extract module name from argument (could be module name or file path)
    arg = sys.argv[1]
    if '/' in arg or arg.endswith('.rs'):
        # It's a file path - extract the filename without extension
        module_name = Path(arg).stem
    else:
        module_name = arg
    
    project_root = Path(__file__).parent.parent.resolve()
    
    # Change to project root if not already there
    os.chdir(project_root)
    
    # Find associated tests and benchmarks
    test_files, bench_files = find_tests_and_benches(project_root, module_name)
    
    steps = []
    
    # Step 1: Compile source
    steps.append(("Compile source", ["cargo", "check", "--lib", "-j", "10"]))
    
    # Step 2: Compile each test
    for test_name in test_files:
        steps.append((f"Compile test: {test_name}", 
                     ["cargo", "test", "--test", test_name, "--no-run", "-j", "10"]))
    
    # Step 3: Compile each benchmark
    for bench_name in bench_files:
        steps.append((f"Compile benchmark: {bench_name}", 
                     ["cargo", "bench", "--bench", bench_name, "--no-run", "-j", "10"]))
    
    # Step 4: Run each test (after all compilation passes)
    for test_name in test_files:
        steps.append((f"Run test: {test_name}", 
                     ["cargo", "test", "--test", test_name, "-j", "10"]))
    
    # Run all steps - STOP if source compilation fails with module-specific errors
    total_errors = 0
    all_steps_passed = True
    lib_failed_output = None
    
    for i, (name, command) in enumerate(steps):
        # Show output for test runs
        show_output = name.startswith("Run test:")
        success, error_count, output_lines = run_step(name, command, project_root, module_name, show_output)
        total_errors += error_count
        
        if success:
            print(f"✓ {name}", flush=True)
        else:
            all_steps_passed = False
            # If source compilation fails WITH module errors, stop - tests/benches will have same errors
            if i == 0 and error_count > 0:  # First step is source compilation
                print(f"\n{total_errors} error(s) in {module_name} - source failed to compile", flush=True)
                return 1
            # If step failed but no module errors - other files broke, can't run this step
            if error_count == 0:
                print(f"✗ {name} (not run - lib failed to compile)", flush=True)
                # Save output for showing all lib errors later
                if lib_failed_output is None:
                    lib_failed_output = output_lines
            # For test/bench failures with module errors, continue to see all issues
    
    # If tests couldn't run, show ALL lib errors
    if lib_failed_output is not None:
        print(f"\n{'='*60}", flush=True)
        print(f"FULL LIB COMPILATION ERRORS (preventing test execution):", flush=True)
        print(f"{'='*60}", flush=True)
        parse_cargo_errors_streaming(lib_failed_output, module_name, show_all=True)
        print(f"{'='*60}\n", flush=True)
    
    # Output final summary
    if total_errors > 0:
        print(f"\n{total_errors} error(s) in {module_name}", flush=True)
        return 1
    
    if all_steps_passed:
        print(f"\n✓ {module_name}: All checks passed", flush=True)
        return 0
    else:
        print(f"\n✓ {module_name}: Module is clean (but lib has errors in other files)", flush=True)
        return 0


if __name__ == "__main__":
    sys.exit(main())

