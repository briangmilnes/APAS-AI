#!/usr/bin/env python3
"""
APAS Benchmark Timing Parameter Checker

Validates that all benchmark files use correct timing parameters:
- Warm-up time: 300ms (0.3 seconds)
- Measurement time: 1 second
- Sample size: 30
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from pathlib import Path

def check_timing_params(file_path, content):
    """Check for correct timing parameters in benchmark file."""
    violations = []
    lines = content.split('\n')
    
    has_warmup = False
    has_measurement = False
    has_sample_size = False
    
    correct_warmup = False
    correct_measurement = False
    correct_sample_size = False
    
    for i, line in enumerate(lines, 1):
        # Check warm_up_time
        if 'warm_up_time' in line:
            has_warmup = True
            # Should be Duration::from_millis(300)
            if 'from_millis(300)' in line:
                correct_warmup = True
            else:
                violations.append(
                    f"{file_path}:{i}: warm_up_time should be Duration::from_millis(300): {line.strip()}"
                )
        
        # Check measurement_time
        if 'measurement_time' in line:
            has_measurement = True
            # Should be Duration::from_secs(1)
            if 'from_secs(1)' in line:
                correct_measurement = True
            else:
                violations.append(
                    f"{file_path}:{i}: measurement_time should be Duration::from_secs(1): {line.strip()}"
                )
        
        # Check sample_size
        if 'sample_size' in line:
            has_sample_size = True
            # Should be 30
            if 'sample_size(30)' in line:
                correct_sample_size = True
            else:
                violations.append(
                    f"{file_path}:{i}: sample_size should be 30: {line.strip()}"
                )
    
    # Check if timing params are present at all
    if not has_warmup:
        violations.append(f"{file_path}: Missing warm_up_time configuration")
    if not has_measurement:
        violations.append(f"{file_path}: Missing measurement_time configuration")
    if not has_sample_size:
        violations.append(f"{file_path}: Missing sample_size configuration")
    
    return violations

def scan_benchmark_files(root_dir):
    """Scan all benchmark files in benches/ directory."""
    violations = []
    
    benches_dir = os.path.join(root_dir, 'benches')
    if not os.path.exists(benches_dir):
        print(f"Warning: benches directory not found at {benches_dir}", file=sys.stderr)
        return violations
    
    for bench_file in Path(benches_dir).rglob('*.rs'):
        file_path = str(bench_file)
        
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            violations.extend(check_timing_params(file_path, content))
            
        except Exception as e:
            print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations

def main():
    """Main entry point."""
    # Get project root (assume script is in scripts/benches/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(script_dir)
    
    print("Running APAS Benchmark Timing Parameter Checks...")
    print(f"Project root: {project_root}")
    print()
    
    violations = scan_benchmark_files(project_root)
    
    if violations:
        print(f"Found {len(violations)} timing parameter violation(s):")
        print()
        for violation in violations:
            print(f"  {violation}")
        print()
        return 1
    else:
        print("✓ All benchmark timing parameters are correct")
        return 0

if __name__ == '__main__':
    sys.exit(main())

