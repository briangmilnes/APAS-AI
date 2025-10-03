#!/usr/bin/env python3

import os
import re
import subprocess

def fix_macro_imports(file_path):
    """Add missing macro imports to test files."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check if file needs macro imports
        macros_needed = []
        if 'ArraySeqStPerSLit!' in content and 'use apas_ai::{ArraySeqStPerSLit}' not in content:
            macros_needed.append('ArraySeqStPerSLit')
        if 'ArraySeqStEphSLit!' in content and 'use apas_ai::{ArraySeqStEphSLit}' not in content:
            macros_needed.append('ArraySeqStEphSLit')
        if 'ArraySeqMtPerSLit!' in content and 'use apas_ai::{ArraySeqMtPerSLit}' not in content:
            macros_needed.append('ArraySeqMtPerSLit')
        if 'UnsortedListPQLit!' in content and 'use apas_ai::{UnsortedListPQLit}' not in content:
            macros_needed.append('UnsortedListPQLit')
        if 'SortedListPQLit!' in content and 'use apas_ai::{SortedListPQLit}' not in content:
            macros_needed.append('SortedListPQLit')
        if 'BalancedTreePQLit!' in content and 'use apas_ai::{BalancedTreePQLit}' not in content:
            macros_needed.append('BalancedTreePQLit')
        
        if macros_needed:
            # Find the last use statement
            lines = content.split('\n')
            last_use_line = -1
            for i, line in enumerate(lines):
                if line.strip().startswith('use '):
                    last_use_line = i
            
            if last_use_line >= 0:
                # Insert macro imports after last use statement
                for macro in macros_needed:
                    lines.insert(last_use_line + 1, f'use apas_ai::{{{macro}}};')
                    last_use_line += 1
                
                with open(file_path, 'w') as f:
                    f.write('\n'.join(lines))
                
                print(f"Added macro imports {macros_needed} to {file_path}")
                return True
    except Exception as e:
        print(f"Error fixing macros in {file_path}: {e}")
    return False

def fix_type_annotations(file_path):
    """Fix type annotation issues in AugOrderedTable tests."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix AugOrderedTable type annotations
        patterns = [
            (r'let table = AugOrderedTableStPer::empty\(([^,]+), ([^)]+)\);',
             r'let table: AugOrderedTableStPer<i32, i32, _> = AugOrderedTableStPer::empty(\1, \2);'),
            (r'let table = AugOrderedTableStEph::empty\(([^,]+), ([^)]+)\);',
             r'let table: AugOrderedTableStEph<i32, i32, _> = AugOrderedTableStEph::empty(\1, \2);'),
            (r'let table = AugOrderedTableMtEph::empty\(([^,]+), ([^)]+)\);',
             r'let table: AugOrderedTableMtEph<i32, i32, _> = AugOrderedTableMtEph::empty(\1, \2);'),
        ]
        
        modified = False
        for pattern, replacement in patterns:
            if re.search(pattern, content):
                content = re.sub(pattern, replacement, content)
                modified = True
        
        if modified:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed type annotations in {file_path}")
            return True
    except Exception as e:
        print(f"Error fixing type annotations in {file_path}: {e}")
    return False

def fix_method_calls(file_path):
    """Fix common method call issues."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Fix from_vec calls
        content = re.sub(r'ArraySeqStPerTrait<[^>]+>::from_vec', 'ArraySeqStPerS::from_vec', content)
        content = re.sub(r'<[^>]+>::from_vec', 'ArraySeqStPerS::from_vec', content)
        
        # Fix size() vs length() calls for FlatHashTable
        if 'FlatHashTable' in content:
            content = re.sub(r'\.size\(\)', '.length()', content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        print(f"Fixed method calls in {file_path}")
        return True
    except Exception as e:
        print(f"Error fixing method calls in {file_path}: {e}")
    return False

def main():
    # Get all test files
    result = subprocess.run(['find', 'tests/', '-name', '*.rs'], capture_output=True, text=True)
    test_files = result.stdout.strip().split('\n')
    
    for file_path in test_files:
        if os.path.exists(file_path):
            print(f"Processing {file_path}...")
            fix_macro_imports(file_path)
            fix_type_annotations(file_path)
            fix_method_calls(file_path)

if __name__ == "__main__":
    main()
