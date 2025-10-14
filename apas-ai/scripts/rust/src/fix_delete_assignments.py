#!/usr/bin/env python3

import os
import re

def fix_delete_assignments(file_path):
    """Fix delete method assignments that return tuples."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix pattern: table = table.delete(...)
        lines = content.split('\n')
        fixed_lines = []
        
        for line in lines:
            # Look for: table = table.delete(...)
            if re.search(r'\s*(\w+)\s*=\s*\1\.delete\(', line):
                # Replace with tuple destructuring
                fixed_line = re.sub(
                    r'(\s*)(\w+)\s*=\s*(\w+)\.delete\(([^)]+)\);',
                    r'\1let (\2, _deleted) = \3.delete(\4);',
                    line
                )
                if fixed_line != line:
                    print(f"Fixed in {file_path}: {line.strip()} -> {fixed_line.strip()}")
                    fixed_lines.append(fixed_line)
                    continue
            
            fixed_lines.append(line)
        
        content = '\n'.join(fixed_lines)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        return True
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
    return False

def main():
    files_to_fix = [
        "tests/Chap47/TestDoubleHashing.rs",
        "tests/Chap47/TestNestedHashTable.rs",
        "tests/Chap47/TestLinearProbing.rs",
        "tests/Chap47/TestQuadraticProbing.rs"
    ]
    
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            fix_delete_assignments(file_path)

if __name__ == "__main__":
    main()
