#!/usr/bin/env python3

import os
import re
import subprocess

def fix_delete_tuples(file_path):
    """Fix delete method calls that return tuples."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Only process files that use FlatHashTable
        if 'FlatHashTable' not in content:
            return False
        
        # Fix delete calls that assign to same variable
        # Pattern: let table = table.delete(...);
        content = re.sub(
            r'let\s+(\w+)\s*=\s*\1\.delete\([^)]+\);',
            r'let (\1, _deleted) = \1.delete(\2);',
            content
        )
        
        # More specific pattern matching
        lines = content.split('\n')
        fixed_lines = []
        
        for line in lines:
            # Look for: let table = table.delete(...)
            if re.match(r'\s*let\s+(\w+)\s*=\s*\1\.delete\(', line):
                # Extract variable name and delete call
                match = re.match(r'(\s*)let\s+(\w+)\s*=\s*(\w+)\.delete\(([^)]+)\);', line)
                if match:
                    indent, var1, var2, args = match.groups()
                    if var1 == var2:  # Same variable being reassigned
                        fixed_line = f'{indent}let ({var1}, _deleted) = {var2}.delete({args});'
                        fixed_lines.append(fixed_line)
                        print(f"Fixed delete tuple in {file_path}: {line.strip()} -> {fixed_line.strip()}")
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
    # Get all test files that use FlatHashTable
    result = subprocess.run(['find', 'tests/', '-name', '*.rs'], capture_output=True, text=True)
    test_files = result.stdout.strip().split('\n')
    
    for file_path in test_files:
        if os.path.exists(file_path):
            fix_delete_tuples(file_path)

if __name__ == "__main__":
    main()
