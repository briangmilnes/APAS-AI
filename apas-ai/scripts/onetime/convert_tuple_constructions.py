#!/usr/bin/env python3
"""
Convert tuple construction/destructuring to Pair/Triple in weighted graph files.

Phase 2: Convert actual tuple constructions and destructurings.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

def convert_constructions(content):
    """Convert tuple constructions to Pair/Triple."""
    # Simple heuristic: when we see .insert((a, b, c)), convert to .insert(Triple(a, b, c))
    # when we see .insert((a, b)), convert to .insert(Pair(a, b))
    
    # Pattern: .insert((expr1, expr2, expr3))
    # Need to be careful about nested parentheses
    
    # For now, let's handle the specific patterns we see in the graph code
    # These are relatively simple cases
    
    # Triple construction: (x.clone(), y.clone(), z)
    content = re.sub(
        r'\.insert\(\(([^,]+),\s*([^,]+),\s*([^)]+)\)\)',
        r'.insert(Triple(\1, \2, \3))',
        content
    )
    
    # Pair construction after Triples are done: (x.clone(), y)
    content = re.sub(
        r'\.insert\(\(([^,]+),\s*([^)]+)\)\)',
        r'.insert(Pair(\1, \2))',
        content
    )
    
    # Also handle Some((a, b, c)) -> Some(Triple(a, b, c))
    content = re.sub(
        r'Some\(\(([^,]+),\s*([^,]+),\s*([^)]+)\)\)',
        r'Some(Triple(\1, \2, \3))',
        content
    )
    
    # And (a, b, c) in return position or variable assignment
    # This is trickier - let's be conservative
    
    return content

def convert_destructuring(content):
    """Convert tuple destructuring patterns."""
    # Pattern: for (u, v, w) in ... -> for Triple(u, v, w) in ...
    content = re.sub(
        r'for\s+\((\w+),\s*(\w+),\s*(\w+)\)\s+in',
        r'for Triple(\1, \2, \3) in',
        content
    )
    
    # Pattern: for (u, v) in ... -> for Pair(u, v) in ...
    content = re.sub(
        r'for\s+\((\w+),\s*(\w+)\)\s+in',
        r'for Pair(\1, \2) in',
        content
    )
    
    # Pattern: if let Some((u, v, w)) = ... -> if let Some(Triple(u, v, w)) = ...
    content = re.sub(
        r'if\s+let\s+Some\(\((\w+),\s*(\w+),\s*(\w+)\)\)',
        r'if let Some(Triple(\1, \2, \3))',
        content
    )
    
    # Pattern: let (u, v, w) = ... -> let Triple(u, v, w) = ...
    content = re.sub(
        r'let\s+\((\w+),\s*(\w+),\s*(\w+)\)\s*=',
        r'let Triple(\1, \2, \3) =',
        content
    )
    
    return content

def add_triple_import(content):
    """Add Triple import if not present."""
    if 'use crate::Types::Types::Triple;' not in content:
        # Find the imports section and add Triple
        if 'use crate::Types::Types::Pair;' in content:
            content = content.replace(
                'use crate::Types::Types::Pair;',
                'use crate::Types::Types::{Pair, Triple};'
            )
        elif 'use crate::Types::Types::*;' not in content:
            # Add after other imports
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if line.startswith('use crate::Types::Types'):
                    lines.insert(i+1, 'use crate::Types::Types::Triple;')
                    break
            content = '\n'.join(lines)
    return content

def convert_file(file_path):
    """Convert tuples to wrappers in a single file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original = content
    
    content = convert_constructions(content)
    content = convert_destructuring(content)
    content = add_triple_import(content)
    
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


