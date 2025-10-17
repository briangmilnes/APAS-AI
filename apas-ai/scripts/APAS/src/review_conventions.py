#!/usr/bin/env python3
"""
APAS Convention Checker

Validates:
1. Graph notation: A: for directed graphs, E: for undirected graphs
2. *Mt* files use MtT not StT
3. *Per files have no set/update methods
4. UFCS patterns <Type as Trait>::
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from pathlib import Path

def check_graph_notation(file_path, content):
    """Check graph notation conventions."""
    violations = []
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        # Check for DirGraph using E: instead of A:
        if 'DirGraph' in line and re.search(r'E:\s*\[', line):
            violations.append(
                f"{file_path}:{i}: Directed graphs should use A: not E:: {line.strip()}"
            )
        
        # Check for UnDirGraph using A: instead of E:
        if 'UnDirGraph' in line and re.search(r'A:\s*\[', line):
            violations.append(
                f"{file_path}:{i}: Undirected graphs should use E: not A:: {line.strip()}"
            )
    
    return violations

def check_mt_files_use_mtt(file_path, content):
    """Check that *Mt* files use MtT not StT."""
    violations = []
    
    # Only check files with Mt in the name
    filename = os.path.basename(file_path)
    if 'Mt' not in filename:
        return violations
    
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        # Check for StT usage in trait/impl bounds
        if re.search(r'(trait|impl).*:\s*StT', line):
            violations.append(
                f"{file_path}:{i}: Mt files should use MtT not StT: {line.strip()}"
            )
        
        # Check for T: StT in where clauses
        if re.search(r'where.*T:\s*StT', line):
            violations.append(
                f"{file_path}:{i}: Mt files should use MtT not StT: {line.strip()}"
            )
    
    return violations

def check_per_files_no_mutation(file_path, content):
    """Check that *Per files have no set/update/insert_in_place methods."""
    violations = []
    
    # Only check files with Per in the name
    filename = os.path.basename(file_path)
    if 'Per' not in filename:
        return violations
    
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Check for mutating method definitions
        if re.search(r'fn\s+(set|update|insert_in_place)\s*\(', stripped):
            violations.append(
                f"{file_path}:{i}: Persistent files should not have mutating methods: {stripped}"
            )
    
    return violations

def check_ufcs_patterns(file_path, content):
    """Check for UFCS patterns at call sites."""
    violations = []
    
    # Skip if this is in src/ (UFCS is allowed in implementations)
    # Focus on tests/ and benches/ where it should be avoided
    if '/tests/' not in file_path and '/benches/' not in file_path:
        return violations
    
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        # Check for UFCS call syntax
        if re.search(r'<\w+\s+as\s+\w+>::', line):
            violations.append(
                f"{file_path}:{i}: Avoid UFCS at call sites, use method syntax: {line.strip()[:60]}"
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
                
                violations.extend(check_graph_notation(file_path, content))
                violations.extend(check_mt_files_use_mtt(file_path, content))
                violations.extend(check_per_files_no_mutation(file_path, content))
                violations.extend(check_ufcs_patterns(file_path, content))
                
            except Exception as e:
                print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations

def main():
    """Main entry point."""
    # Get project root (assume script is in scripts/lint/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(os.path.dirname(script_dir))
    
    print("Running APAS Convention Checks...")
    print(f"Project root: {project_root}")
    print()
    
    violations = scan_rust_files(project_root)
    
    if violations:
        print(f"Found {len(violations)} convention violation(s):")
        print()
        for violation in violations:
            print(f"  {violation}")
        print()
        return 1
    else:
        print("✓ All convention checks passed")
        return 0

if __name__ == '__main__':
    sys.exit(main())

