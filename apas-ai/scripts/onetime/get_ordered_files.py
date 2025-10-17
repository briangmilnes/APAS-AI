#!/usr/bin/env python3
"""Get files with inherent+trait impls, ordered by chapter number."""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / 'rust' / 'src'))

from review_inherent_and_trait_impl import review_file

def extract_chapter_num(path_str):
    """Extract chapter number for sorting."""
    path = Path(path_str)
    parts = path.parts
    for part in parts:
        if part.startswith('Chap'):
            # Extract number from "Chap05" -> 5
            num_str = part[4:]
            if 'clean' in num_str:
                num_str = num_str.replace('clean', '')
            try:
                return int(num_str)
            except:
                return 999
    return 999

def main():
    workspace_root = Path(__file__).parent.parent.parent
    src_dir = workspace_root / 'src'
    
    rust_files = sorted(src_dir.rglob('*.rs'))
    
    files_with_violations = []
    
    for file_path in rust_files:
        violations = review_file(file_path)
        if violations:
            files_with_violations.append(str(file_path.relative_to(workspace_root)))
    
    # Sort by chapter number
    files_with_violations.sort(key=extract_chapter_num)
    
    for f in files_with_violations:
        print(f)

if __name__ == '__main__':
    main()


