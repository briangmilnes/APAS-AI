#!/usr/bin/env python3
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re

def fix_duplicate_imports(file_path):
    """Remove duplicate macro imports from test files."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
        
        # Track seen imports
        seen_imports = set()
        fixed_lines = []
        
        for line in lines:
            # Check for any macro imports (both forms)
            if ('ArraySeqStPerSLit' in line or 'ArraySeqStEphSLit' in line or 'ArraySeqMtPerSLit' in line) and line.strip().startswith('use apas_ai'):
                if line.strip() in seen_imports:
                    print(f"Removing duplicate import: {line.strip()}")
                    continue
                seen_imports.add(line.strip())
            
            fixed_lines.append(line)
        
        with open(file_path, 'w') as f:
            f.writelines(fixed_lines)
        
        print(f"Fixed duplicate imports in {file_path}")
        return True
    except Exception as e:
        print(f"Error fixing duplicates in {file_path}: {e}")
    return False

def main():
    import subprocess
    
    # Get all test files
    result = subprocess.run(['find', 'tests/', '-name', '*.rs'], capture_output=True, text=True)
    test_files = result.stdout.strip().split('\n')
    
    for file_path in test_files:
        if os.path.exists(file_path):
            fix_duplicate_imports(file_path)

if __name__ == "__main__":
    main()
