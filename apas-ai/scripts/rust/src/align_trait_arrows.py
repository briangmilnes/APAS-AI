#!/usr/bin/env python3
"""
Align -> arrows in trait method signatures for improved readability.

Finds trait blocks and aligns all method signature arrows vertically.
Only aligns within each trait block independently.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def find_trait_blocks(lines):
    """Find all trait blocks and their line ranges."""
    traits = []
    in_trait = False
    trait_start = 0
    trait_name = None
    brace_depth = 0
    
    for i, line in enumerate(lines):
        # Detect trait declaration
        trait_match = re.match(r'^(\s*)pub\s+trait\s+(\w+)', line)
        if trait_match:
            in_trait = True
            trait_start = i
            trait_name = trait_match.group(2)
            brace_depth = 0
        
        if in_trait:
            brace_depth += line.count('{') - line.count('}')
            if brace_depth == 0 and '{' in lines[trait_start]:
                # Trait block closed
                traits.append({
                    'name': trait_name,
                    'start': trait_start,
                    'end': i
                })
                in_trait = False
    
    return traits


def align_trait(lines, trait_info):
    """Align arrows in a single trait block."""
    # Find all method lines with arrows
    method_lines = []
    
    for i in range(trait_info['start'], trait_info['end'] + 1):
        line = lines[i]
        
        # Check if this is a method signature with fn and ->
        if re.search(r'^\s*fn\s+\w+', line) and '->' in line:
            # Skip lines that have arrows inside generic bounds (Fn/FnOnce/FnMut patterns)
            # These have fundamentally different structure and shouldn't be aligned
            if re.search(r'Fn(?:Once|Mut)?\s*\([^)]*\)\s*->', line):
                continue
            
            # Find position of method return ->
            arrow_pos = line.find('->')
            if arrow_pos == -1:
                continue
            
            method_lines.append({
                'line_num': i,
                'arrow_pos': arrow_pos
            })
    
    if not method_lines:
        return lines
    
    # Find the maximum arrow position
    max_arrow_pos = max(m['arrow_pos'] for m in method_lines)
    
    # Align each method
    for method in method_lines:
        line_num = method['line_num']
        line = lines[line_num]
        current_arrow_pos = method['arrow_pos']
        
        if current_arrow_pos < max_arrow_pos:
            # Add spaces before the arrow
            spaces_needed = max_arrow_pos - current_arrow_pos
            # Split at arrow, add spaces, rejoin
            before_arrow = line[:current_arrow_pos]
            after_arrow = line[current_arrow_pos:]
            lines[line_num] = before_arrow + (' ' * spaces_needed) + after_arrow
    
    return lines


def align_file(file_path):
    """Align all trait arrows in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"ERROR: Could not read {file_path}: {e}", file=sys.stderr)
        return False
    
    # Remove trailing newlines for processing
    lines = [line.rstrip('\n') for line in lines]
    
    # Find all trait blocks
    traits = find_trait_blocks(lines)
    
    if not traits:
        print(f"No traits found in {file_path}")
        return True
    
    print(f"Found {len(traits)} trait(s) in {file_path}:")
    for trait in traits:
        print(f"  - {trait['name']} (lines {trait['start']+1}-{trait['end']+1})")
    
    # Align each trait
    for trait in traits:
        lines = align_trait(lines, trait)
    
    # Write back
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            for line in lines:
                f.write(line + '\n')
        print(f"✓ Aligned {file_path}")
        return True
    except Exception as e:
        print(f"ERROR: Could not write {file_path}: {e}", file=sys.stderr)
        return False


def main():
    if len(sys.argv) < 2:
        print("Usage: align_trait_arrows.py <file1.rs> [file2.rs ...]", file=sys.stderr)
        return 1
    
    success = True
    for file_path in sys.argv[1:]:
        path = Path(file_path)
        if not path.exists():
            print(f"ERROR: File not found: {file_path}", file=sys.stderr)
            success = False
            continue
        
        if not align_file(path):
            success = False
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
