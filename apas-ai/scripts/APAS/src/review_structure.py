#!/usr/bin/env python3
"""
APAS Structure Checker

Validates:
1. Code outside pub mod blocks (except lib.rs, main.rs)
2. #[cfg(test)] in integration tests (tests/ directory)
3. pub fields on structs
4. extern crate usage
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from pathlib import Path

def check_code_outside_modules(file_path, content):
    """Check for code definitions outside pub mod blocks."""
    violations = []
    
    # Skip lib.rs and main.rs - they can have top-level code
    filename = os.path.basename(file_path)
    if filename in ['lib.rs', 'main.rs']:
        return violations
    
    lines = content.split('\n')
    in_mod = False
    mod_depth = 0
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Track module depth
        if re.match(r'pub\s+mod\s+\w+\s*\{', stripped):
            in_mod = True
            mod_depth += stripped.count('{')
            continue
        
        if in_mod:
            mod_depth += stripped.count('{')
            mod_depth -= stripped.count('}')
            if mod_depth <= 0:
                in_mod = False
            continue
        
        # Skip empty lines, comments, attributes, use statements
        if not stripped or stripped.startswith('//') or stripped.startswith('/*'):
            continue
        if stripped.startswith('#[') or stripped.startswith('use ') or stripped.startswith('extern '):
            continue
        
        # Check for definitions outside modules
        if re.match(r'(pub\s+)?(struct|enum|trait|fn|impl|type|const|static)\s+', stripped):
            violations.append(
                f"{file_path}:{i}: Code outside pub mod block: {stripped[:60]}"
            )
    
    return violations

def check_cfg_test_in_integration(file_path, content):
    """Check for #[cfg(test)] in integration test files."""
    violations = []
    
    # Only check files in tests/ directory
    if '/tests/' not in file_path:
        return violations
    
    lines = content.split('\n')
    for i, line in enumerate(lines, 1):
        if '#[cfg(test)]' in line:
            violations.append(
                f"{file_path}:{i}: #[cfg(test)] not allowed in integration tests - "
                "prevents test discovery"
            )
    
    return violations

def check_pub_fields(file_path, content):
    """Check for pub fields on structs."""
    violations = []
    lines = content.split('\n')
    
    in_struct = False
    struct_name = None
    brace_depth = 0
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Detect struct definition
        struct_match = re.match(r'pub\s+struct\s+(\w+)', stripped)
        if struct_match:
            in_struct = True
            struct_name = struct_match.group(1)
            brace_depth = 0
            if '{' in stripped:
                brace_depth += 1
            continue
        
        if in_struct:
            brace_depth += stripped.count('{')
            brace_depth -= stripped.count('}')
            
            # Check for pub fields
            if re.match(r'pub\s+\w+\s*:', stripped):
                violations.append(
                    f"{file_path}:{i}: pub field in struct {struct_name}: {stripped[:60]}"
                )
            
            if brace_depth <= 0:
                in_struct = False
                struct_name = None
    
    return violations

def check_extern_crate(file_path, content):
    """Check for extern crate usage."""
    violations = []
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        if re.match(r'extern\s+crate\s+', line.strip()):
            violations.append(
                f"{file_path}:{i}: extern crate is banned: {line.strip()}"
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
                
                violations.extend(check_code_outside_modules(file_path, content))
                violations.extend(check_cfg_test_in_integration(file_path, content))
                violations.extend(check_pub_fields(file_path, content))
                violations.extend(check_extern_crate(file_path, content))
                
            except Exception as e:
                print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations

def main():
    """Main entry point."""
    # Get project root (assume script is in scripts/lint/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(os.path.dirname(script_dir))
    
    print("Running APAS Structure Checks...")
    print(f"Project root: {project_root}")
    print()
    
    violations = scan_rust_files(project_root)
    
    if violations:
        print(f"Found {len(violations)} structure violation(s):")
        print()
        for violation in violations:
            print(f"  {violation}")
        print()
        return 1
    else:
        print("✓ All structure checks passed")
        return 0

if __name__ == '__main__':
    sys.exit(main())

