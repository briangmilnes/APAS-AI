#!/usr/bin/env python3
"""
Fix imports for ArraySeqStEph and ArraySeqStPer to include traits.
Changes specific imports to ::* imports.
"""

import subprocess
import re

def find_files():
    """Find all source files that import ArraySeqStEph or ArraySeqStPer"""
    result = subprocess.run(
        ['grep', '-rl', '--include=*.rs', 'use.*ArraySeqSt', 'src/'],
        capture_output=True,
        text=True,
        cwd='/home/milnes/APASVERUS/APAS-AI/apas-ai'
    )
    return [f.strip() for f in result.stdout.strip().split('\n') if f.strip()]

def fix_file(filepath):
    """Fix imports in a single file"""
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    # Pattern: use ...::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
    # Replace with: use ...::ArraySeqStEph::ArraySeqStEph::*;
    content = re.sub(
        r'(use .*::ArraySeqStEph::ArraySeqStEph::)ArraySeqStEphS;',
        r'\1*;',
        content
    )
    
    # Pattern: use ...::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;
    # Replace with: use ...::ArraySeqStPer::ArraySeqStPer::*;
    content = re.sub(
        r'(use .*::ArraySeqStPer::ArraySeqStPer::)ArraySeqStPerS;',
        r'\1*;',
        content
    )
    
    # Pattern: use ...::ArraySeqStEph::ArraySeqStEph::{...};
    # Replace with: use ...::ArraySeqStEph::ArraySeqStEph::*;
    content = re.sub(
        r'(use .*::ArraySeqStEph::ArraySeqStEph::)\{[^}]+\};',
        r'\1*;',
        content
    )
    
    # Pattern: use ...::ArraySeqStPer::ArraySeqStPer::{...};
    # Replace with: use ...::ArraySeqStPer::ArraySeqStPer::)\{[^}]+\};
    content = re.sub(
        r'(use .*::ArraySeqStPer::ArraySeqStPer::)\{[^}]+\};',
        r'\1*;',
        content
    )
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    files = find_files()
    fixed_count = 0
    
    for filepath in files:
        if fix_file(filepath):
            print(f"✓ Fixed: {filepath}")
            fixed_count += 1
        else:
            print(f"  Skipped: {filepath}")
    
    print(f"\n✓ Fixed {fixed_count}/{len(files)} files")

if __name__ == "__main__":
    main()

