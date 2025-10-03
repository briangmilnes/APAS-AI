#!/usr/bin/env python3

import re
import os

def fix_dereference_issues(file_path):
    """Fix dereference issues in test files."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix pattern: assert_eq!(*var, ArraySeqStEphSLit![...])
        content = re.sub(
            r'assert_eq!\(\*([^,]+), (ArraySeq\w+Lit!\[[^\]]*\])\);',
            r'assert_eq!(\1, \2);',
            content
        )
        
        # Fix pattern: assert_eq!(*var.field, ArraySeqStEphSLit![...])
        content = re.sub(
            r'assert_eq!\(\*([^,]+\.[^,]+), (ArraySeq\w+Lit!\[[^\]]*\])\);',
            r'assert_eq!(\1, \2);',
            content
        )
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed dereference issues in {file_path}")
        return True
    except Exception as e:
        print(f"Error fixing {file_path}: {e}")
    return False

def main():
    files_to_fix = [
        "tests/Chap19/TestArraySeqStEph18.rs"
    ]
    
    for file_path in files_to_fix:
        if os.path.exists(file_path):
            fix_dereference_issues(file_path)

if __name__ == "__main__":
    main()
