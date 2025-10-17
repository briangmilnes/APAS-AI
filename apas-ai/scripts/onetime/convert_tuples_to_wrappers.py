#!/usr/bin/env python3
"""
Convert raw Rust tuples to wrapped types (Pair/Triple) in weighted graph files.

Converts:
- (V, V, OrderedFloat<f64>) -> Triple<V, V, OrderedFloat<f64>>
- (V, OrderedFloat<f64>) -> Pair<V, OrderedFloat<f64>>
- (V, V, i32) -> Triple<V, V, i32>
- (V, i32) -> Pair<V, i32>
- (a, b, c) -> Triple(a, b, c) (construction)
- (a, b) -> Pair(a, b) (construction)
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

def convert_file(file_path):
    """Convert tuples to wrappers in a single file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original = content
    
    # Type signatures: (V, V, T) -> Triple<V, V, T>
    # Match tuple types in type positions
    content = re.sub(
        r'\(V,\s*V,\s*OrderedFloat<f64>\)',
        'Triple<V, V, OrderedFloat<f64>>',
        content
    )
    content = re.sub(
        r'\(V,\s*OrderedFloat<f64>\)',
        'Pair<V, OrderedFloat<f64>>',
        content
    )
    content = re.sub(
        r'\(V,\s*V,\s*i32\)',
        'Triple<V, V, i32>',
        content
    )
    content = re.sub(
        r'\(V,\s*i32\)',
        'Pair<V, i32>',
        content
    )
    
    # Also handle without spaces
    content = re.sub(
        r'\(V, V, OrderedFloat<f64>\)',
        'Triple<V, V, OrderedFloat<f64>>',
        content
    )
    content = re.sub(
        r'\(V, OrderedFloat<f64>\)',
        'Pair<V, OrderedFloat<f64>>',
        content
    )
    content = re.sub(
        r'\(V, V, i32\)',
        'Triple<V, V, i32>',
        content
    )
    content = re.sub(
        r'\(V, i32\)',
        'Pair<V, i32>',
        content
    )
    
    if content != original:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False

def main():
    workspace_root = Path(__file__).parent.parent.parent
    
    # Find all weighted graph files
    chap06 = workspace_root / 'src' / 'Chap06'
    files = [
        chap06 / 'WeightedDirGraphMtEphFloat.rs',
        chap06 / 'WeightedDirGraphMtEphInt.rs',
        chap06 / 'WeightedDirGraphStEphFloat.rs',
        chap06 / 'WeightedDirGraphStEphInt.rs',
        chap06 / 'WeightedUnDirGraphMtEphFloat.rs',
        chap06 / 'WeightedUnDirGraphMtEphInt.rs',
        chap06 / 'WeightedUnDirGraphStEphFloat.rs',
        chap06 / 'WeightedUnDirGraphStEphInt.rs',
    ]
    
    changed_count = 0
    for file_path in files:
        if file_path.exists():
            if convert_file(file_path):
                print(f"Converted: {file_path.relative_to(workspace_root)}")
                changed_count += 1
        else:
            print(f"Not found: {file_path.relative_to(workspace_root)}")
    
    print(f"\nTotal files changed: {changed_count}")

if __name__ == '__main__':
    main()


