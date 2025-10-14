#!/usr/bin/env python3

import os
import re

def fix_indentation(file_path):
    """Fix indentation in test files by removing leading spaces from all lines after imports."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        # Find where imports end (look for first #[test] or fn)
        import_end = 0
        for i, line in enumerate(lines):
            if line.strip().startswith('#[test]') or (line.strip().startswith('fn ') and 'test_' in line):
                import_end = i
                break
        
        # Fix indentation for all lines after imports
        fixed_lines = []
        for i, line in enumerate(lines):
            if i < import_end:
                fixed_lines.append(line)
            else:
                # Remove leading 4-space indentation
                if line.startswith('    '):
                    fixed_lines.append(line[4:])
                else:
                    fixed_lines.append(line)
        
        with open(file_path, 'w') as f:
            f.writelines(fixed_lines)
        
        print(f"Fixed indentation in {file_path}")
        return True
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
        return False

def main():
    # Get all test files that need fixing
    test_files = [
        "tests/Chap19/TestArraySeqStPer18.rs",
        "tests/Chap19/TestArraySeqMtPer18.rs", 
        "tests/Chap19/TestArraySeqStEph18.rs"
    ]
    
    for file_path in test_files:
        if os.path.exists(file_path):
            fix_indentation(file_path)

if __name__ == "__main__":
    main()