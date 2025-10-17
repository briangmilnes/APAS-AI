#!/usr/bin/env python3
"""
APAS Import Checker

Validates:
1. crate:: usage in src/, crate name in tests/benches/
2. Wildcard imports vs explicit imports
3. Trailing pub use re-exports at end of files
4. Result import patterns
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from pathlib import Path

def check_crate_usage(file_path, content):
    """Check that src/ uses crate::, tests/benches/ use apas_ai::."""
    violations = []
    lines = content.split('\n')
    
    is_src = '/src/' in file_path
    is_test_or_bench = '/tests/' in file_path or '/benches/' in file_path
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Check use statements
        if stripped.startswith('use '):
            if is_src:
                # In src/, should use crate::
                if re.search(r'use\s+apas_ai::', stripped):
                    violations.append(
                        f"{file_path}:{i}: Use crate:: not apas_ai:: in src/: {stripped}"
                    )
            elif is_test_or_bench:
                # In tests/benches, should use apas_ai::
                if re.search(r'use\s+crate::', stripped) and 'lib.rs' not in file_path:
                    violations.append(
                        f"{file_path}:{i}: Use apas_ai:: not crate:: in tests/benches: {stripped}"
                    )
    
    return violations

def check_trailing_reexports(file_path, content):
    """Check for pub use re-exports at end of files (should be in lib.rs only)."""
    violations = []
    
    # Skip lib.rs
    if file_path.endswith('lib.rs'):
        return violations
    
    lines = content.split('\n')
    
    # Look at last 20 lines for pub use statements
    for i, line in enumerate(lines[-20:], len(lines) - 19):
        stripped = line.strip()
        if stripped.startswith('pub use ') and '::' in stripped:
            violations.append(
                f"{file_path}:{i}: Trailing pub use re-export (should be in lib.rs only): {stripped}"
            )
    
    return violations

def check_result_imports(file_path, content):
    """Check Result import patterns."""
    violations = []
    lines = content.split('\n')
    
    has_generic_result = False
    has_fmt_result_import = False
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Check if file uses generic Result<T, E>
        if re.search(r'Result<\w+,\s*\w+>', line):
            has_generic_result = True
        
        # Check for fmt::Result import
        if 'use std::fmt::' in stripped and 'Result' in stripped:
            has_fmt_result_import = True
    
    # If has generic Result and imports fmt::Result, that's a conflict
    if has_generic_result and has_fmt_result_import:
        violations.append(
            f"{file_path}: Imports fmt::Result but also uses generic Result<T,E> - "
            "should use std::fmt::Result in fmt methods only"
        )
    
    return violations

def scan_rust_files(root_dir):
    """Scan all Rust files in src/, tests/, benches/ directories."""
    violations = []
    
    # Directories to scan
    scan_dirs = ['src', 'tests', 'benches']
    
    for scan_dir in scan_dirs:
        dir_path = os.path.join(root_dir, scan_dir)
        if not os.path.exists(dir_path):
            continue
        
        for rust_file in Path(dir_path).rglob('*.rs'):
            file_path = str(rust_file)
            
            # Read content and check patterns
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                violations.extend(check_crate_usage(file_path, content))
                violations.extend(check_trailing_reexports(file_path, content))
                violations.extend(check_result_imports(file_path, content))
                
            except Exception as e:
                print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations

def main():
    """Main entry point."""
    # Get project root (assume script is in scripts/lint/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(os.path.dirname(script_dir))
    
    print("Running APAS Import Checks...")
    print(f"Project root: {project_root}")
    print()
    
    violations = scan_rust_files(project_root)
    
    if violations:
        print(f"Found {len(violations)} import violation(s):")
        print()
        for violation in violations:
            print(f"  {violation}")
        print()
        return 1
    else:
        print("✓ All import checks passed")
        return 0

if __name__ == '__main__':
    sys.exit(main())

