#!/usr/bin/env python3
"""
Fix imports for converted ArraySeq types.
Changes ArraySeqXXX::ArraySeqXXX to ArraySeqXXX::* to import traits.
"""

import sys
import subprocess

# Find all files that use ArraySeqStEph or ArraySeqStPer
def find_files_using_array_seq():
    result = subprocess.run(
        ['grep', '-rl', '--include=*.rs', 'use.*::Chap18::', 'src/', 'tests/', 'benches/'],
        capture_output=True,
        text=True,
        cwd='/home/milnes/APASVERUS/APAS-AI/apas-ai'
    )
    return [f.strip() for f in result.stdout.strip().split('\n') if f.strip()]

array_seq_types = [
    "ArraySeqStEph",
    "ArraySeqStPer",
]

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    for typ in array_seq_types:
        # Pattern: use ...::Chap18::TypeName::TypeName::TypeName;
        # Replace with: use ...::Chap18::TypeName::TypeName::*;
        import_pattern_1 = f"::Chap18::{typ}::{typ}::{typ};"
        import_replacement_1 = f"::Chap18::{typ}::{typ}::*;"
        content = content.replace(import_pattern_1, import_replacement_1)
        
        # Pattern for crate imports: use crate::Chap18::...
        import_pattern_2 = f"use crate::Chap18::{typ}::{typ}::{typ};"
        import_replacement_2 = f"use crate::Chap18::{typ}::{typ}::*;"
        content = content.replace(import_pattern_2, import_replacement_2)
        
        # Pattern for apas_ai imports: use apas_ai::Chap18::...
        import_pattern_3 = f"use apas_ai::Chap18::{typ}::{typ}::{typ};"
        import_replacement_3 = f"use apas_ai::Chap18::{typ}::{typ}::*;"
        content = content.replace(import_pattern_3, import_replacement_3)
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    else:
        return False

def main():
    files = find_files_using_array_seq()
    fixed_count = 0
    
    for filepath in files:
        if fix_file(filepath):
            print(f"✓ Fixed: {filepath}")
            fixed_count += 1
    
    print(f"\n✓ Fixed {fixed_count}/{len(files)} files")

if __name__ == "__main__":
    main()


