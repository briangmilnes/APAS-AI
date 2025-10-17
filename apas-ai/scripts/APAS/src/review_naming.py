#!/usr/bin/env python3
"""
APAS Naming Convention Checker

Validates:
1. Factory pattern ban - no "Factory" in struct/trait/function names
2. CamlCase for multi-word items (not snake_case)
3. Prohibited variable names (temp_, rock band names)
4. File names start with capital letter
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from pathlib import Path

# Prohibited variable name patterns
PROHIBITED_PATTERNS = [
    (r'\btemp_\w+\b', 'temp_ prefix'),
    (r'\b(led_zeppelin|pink_floyd|stairway_to_heaven|bohemian_rhapsody)\b', 'rock band/song names'),
]

# Rock band/song names to check
BAND_NAMES = [
    'led_zeppelin', 'pink_floyd', 'beatles', 'rolling_stones',
    'stairway_to_heaven', 'bohemian_rhapsody', 'hotel_california'
]

def check_factory_ban(file_path, content):
    """Check for 'Factory' in struct/trait/function names."""
    violations = []
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        # Check for Factory in type/trait/function definitions
        if re.search(r'\b(struct|trait|fn|enum|type)\s+\w*Factory\w*', line):
            violations.append(f"{file_path}:{i}: Factory pattern banned: {line.strip()}")
    
    return violations

def check_camlcase(file_path, content):
    """Check for CamlCase vs snake_case in multi-word items."""
    violations = []
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        # Skip comments and strings
        if line.strip().startswith('//') or line.strip().startswith('/*'):
            continue
            
        # Check for snake_case in struct/trait/enum/type names (multi-word)
        match = re.search(r'\b(struct|trait|enum|type)\s+([a-z][a-z0-9]*_[a-z0-9_]+)\b', line)
        if match:
            name = match.group(2)
            # Allow acronyms and single words
            if '_' in name:
                violations.append(f"{file_path}:{i}: Use CamlCase not snake_case for types: {name}")
    
    return violations

def check_prohibited_variables(file_path, content):
    """Check for prohibited variable name patterns."""
    violations = []
    lines = content.split('\n')
    
    for i, line in enumerate(lines, 1):
        # Skip comments
        if line.strip().startswith('//') or line.strip().startswith('/*'):
            continue
        
        # Check prohibited patterns
        for pattern, description in PROHIBITED_PATTERNS:
            if re.search(pattern, line, re.IGNORECASE):
                violations.append(f"{file_path}:{i}: Prohibited variable name ({description}): {line.strip()}")
    
    return violations

def check_file_capitalization(file_path):
    """Check that file names start with capital letter."""
    violations = []
    filename = os.path.basename(file_path)
    
    # Skip special files
    if filename in ['lib.rs', 'main.rs', 'mod.rs']:
        return violations
    
    # Check .rs files
    if filename.endswith('.rs'):
        if filename[0].islower():
            violations.append(f"{file_path}: File name should start with capital: {filename}")
    
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
            
            # Check file capitalization
            violations.extend(check_file_capitalization(file_path))
            
            # Read content and check patterns
            try:
                with open(file_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                
                violations.extend(check_factory_ban(file_path, content))
                violations.extend(check_camlcase(file_path, content))
                violations.extend(check_prohibited_variables(file_path, content))
                
            except Exception as e:
                print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations

def main():
    """Main entry point."""
    # Get project root (assume script is in scripts/lint/)
    script_dir = os.path.dirname(os.path.abspath(__file__))
    project_root = os.path.dirname(os.path.dirname(script_dir))
    
    print("Running APAS Naming Convention Checks...")
    print(f"Project root: {project_root}")
    print()
    
    violations = scan_rust_files(project_root)
    
    if violations:
        print(f"Found {len(violations)} naming violation(s):")
        print()
        for violation in violations:
            print(f"  {violation}")
        print()
        return 1
    else:
        print("✓ All naming convention checks passed")
        return 0

if __name__ == '__main__':
    sys.exit(main())

