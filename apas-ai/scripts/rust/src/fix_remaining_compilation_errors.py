#!/usr/bin/env python3

import subprocess
import re
import os

def get_compilation_errors():
    """Get compilation errors from cargo build."""
    try:
        result = subprocess.run(
            ["timeout", "120", "cargo", "build", "--tests"],
            capture_output=True,
            text=True,
            cwd="/home/milnes/APASVERUS/APAS-AI/apas-ai"
        )
        return result.stderr
    except Exception as e:
        print(f"Error running cargo build: {e}")
        return ""

def fix_type_annotations():
    """Fix remaining type annotation issues."""
    errors = get_compilation_errors()
    
    # Pattern: let var: /* Type */ = ...
    type_annotation_pattern = r'let\s+(\w+):\s*/\*\s*Type\s*\*/'
    
    for line in errors.split('\n'):
        if 'consider giving' in line and 'an explicit type' in line:
            print(f"Type annotation needed: {line}")
    
    # Fix specific patterns we know about
    fixes = [
        # SortedListPQ type annotations
        ("tests/Chap45/TestSortedListPQ.rs", "let pq = SortedListPQTrait::", "let pq: SortedListPQ<i32> = SortedListPQTrait::"),
        ("tests/Chap45/TestSortedListPQ.rs", "let mut pq = SortedListPQTrait::", "let mut pq: SortedListPQ<i32> = SortedListPQTrait::"),
        
        # BalancedTreePQ type annotations  
        ("tests/Chap45/TestBalancedTreePQ.rs", "let pq = BalancedTreePQTrait::", "let pq: BalancedTreePQ<i32> = BalancedTreePQTrait::"),
        ("tests/Chap45/TestBalancedTreePQ.rs", "let mut pq = BalancedTreePQTrait::", "let mut pq: BalancedTreePQ<i32> = BalancedTreePQTrait::"),
        
        # UnsortedListPQ type annotations
        ("tests/Chap45/TestUnsortedListPQ.rs", "let pq = UnsortedListPQTrait::", "let pq: UnsortedListPQ<i32> = UnsortedListPQTrait::"),
        ("tests/Chap45/TestUnsortedListPQ.rs", "let mut pq = UnsortedListPQTrait::", "let mut pq: UnsortedListPQ<i32> = UnsortedListPQTrait::"),
    ]
    
    for file_path, old_pattern, new_pattern in fixes:
        if os.path.exists(file_path):
            try:
                with open(file_path, 'r') as f:
                    content = f.read()
                
                if old_pattern in content:
                    content = content.replace(old_pattern, new_pattern)
                    
                    with open(file_path, 'w') as f:
                        f.write(content)
                    
                    print(f"Fixed type annotations in {file_path}")
            except Exception as e:
                print(f"Error fixing {file_path}: {e}")

def main():
    print("Fixing remaining compilation errors...")
    fix_type_annotations()
    print("Done.")

if __name__ == "__main__":
    main()
