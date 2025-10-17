#!/usr/bin/env python3
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import subprocess

def fix_flathash_methods(file_path):
    """Fix FlatHashTable method calls in test files."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Only process files that use FlatHashTable
        if 'FlatHashTable' not in content:
            return False
        
        # Fix method calls
        content = re.sub(r'\.length\(\)', '.load_and_size().1', content)  # table size
        content = re.sub(r'\.size\(\)', '.load_and_size().0', content)    # num elements
        content = re.sub(r'\.is_empty\(\)', '.load_and_size().0 == 0', content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed FlatHashTable methods in {file_path}")
        return True
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
    return False

def main():
    # Get all test files
    result = subprocess.run(['find', 'tests/', '-name', '*.rs'], capture_output=True, text=True)
    test_files = result.stdout.strip().split('\n')
    
    for file_path in test_files:
        if os.path.exists(file_path):
            fix_flathash_methods(file_path)

if __name__ == "__main__":
    main()
