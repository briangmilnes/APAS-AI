#!/usr/bin/env python3
"""
Fix: Move trait definitions to appear before impl blocks.

Correct order:
1. Data structure (struct/enum)
2. Trait definition for that data structure <- ensure HERE
3. Inherent impl (impl Type { ... })
4. Custom trait impls
5. Standard trait impls
"""
# Git commit: e4850e1
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def is_struct_or_enum_line(line):
    """Check if line defines a struct or enum."""
    stripped = line.strip()
    return bool(re.match(r'(pub\s+)?(?:struct|enum)\s+\w+', stripped))


def is_trait_definition(line):
    """Check if line defines a trait."""
    stripped = line.strip()
    return bool(re.match(r'(pub\s+)?trait\s+\w+', stripped))


def is_impl_line(line):
    """Check if line starts an impl block."""
    stripped = line.strip()
    return bool(re.match(r'impl(?:<[^>]+>)?\s+', stripped))


def find_block_end(lines, start_idx):
    """
    Find the end of a block starting at start_idx.
    Returns the index of the line containing the closing brace.
    Handles nested braces.
    """
    brace_count = 0
    found_first_brace = False
    
    for i in range(start_idx, len(lines)):
        line = lines[i]
        
        for char in line:
            if char == '{':
                brace_count += 1
                found_first_brace = True
            elif char == '}':
                brace_count -= 1
                if found_first_brace and brace_count == 0:
                    return i
    
    return start_idx  # fallback


def extract_trait_block(lines, trait_line_idx):
    """
    Extract a trait definition block including doc comments before it and all lines until closing brace.
    Returns (start_idx, end_idx) - both inclusive, 0-based.
    """
    # Look backwards for doc comments and attributes
    start = trait_line_idx
    for i in range(trait_line_idx - 1, -1, -1):
        stripped = lines[i].strip()
        # Include doc comments (/// or //!), attributes (#[...]), and blank lines before trait
        if stripped.startswith('///') or stripped.startswith('//!') or stripped.startswith('#[') or not stripped:
            start = i
        else:
            # Stop at first non-doc/non-attribute line
            break
    
    # Skip leading blank lines
    while start < trait_line_idx and not lines[start].strip():
        start += 1
    
    # Find the closing brace
    for i in range(trait_line_idx, len(lines)):
        if '{' in lines[i]:
            end = find_block_end(lines, i)
            return (start, end)
    
    # No brace found, assume single-line trait (shouldn't happen in practice)
    return (start, trait_line_idx)


def fix_trait_order(file_path, dry_run=False):
    """
    Fix trait definition order by moving traits before impl blocks.
    Returns True if file was modified.
    
    Strategy: Make one pass, identify ONE violation, fix it, repeat until no violations.
    This avoids index confusion from multiple moves.
    """
    # Skip Types.rs - it has a different format
    if file_path.name == 'Types.rs':
        return False
    
    # Skip Chap47 (Claude abomination - will be replaced by Chap47clean)
    # Skip Chap47clean (different structure - needs interactive fixing)
    if 'Chap47' in str(file_path.parent):
        return False
    
    max_iterations = 20  # Prevent infinite loops
    any_fixed = False
    
    for iteration in range(max_iterations):
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                lines = f.readlines()
        except Exception as e:
            print(f"Error reading {file_path}: {e}", file=sys.stderr)
            return any_fixed
        
        # Find FIRST violation
        struct_name = None
        seen_impl_after_struct = False
        first_impl_line_idx = None
        trait_to_move = None
        
        i = 0
        while i < len(lines) and not trait_to_move:
            line = lines[i]
            stripped = line.strip()
            
            # Skip empty lines and comments
            if not stripped or stripped.startswith('//'):
                i += 1
                continue
            
            # Detect struct/enum - resets state
            if is_struct_or_enum_line(line):
                m = re.search(r'(?:struct|enum)\s+(\w+)', stripped)
                struct_name = m.group(1) if m else None
                seen_impl_after_struct = False
                first_impl_line_idx = None
                i += 1
                continue
            
            # Detect impl block (any kind)
            if struct_name and is_impl_line(line):
                if not seen_impl_after_struct:
                    seen_impl_after_struct = True
                    first_impl_line_idx = i
                i += 1
                continue
            
            # Detect trait definition after impl - THIS IS A VIOLATION
            if struct_name and seen_impl_after_struct and first_impl_line_idx and is_trait_definition(line):
                m = re.search(r'trait\s+(\w+)', stripped)
                trait_name = m.group(1) if m else 'Unknown'
                
                # Find the full trait block
                start_idx, end_idx = extract_trait_block(lines, i)
                trait_to_move = {
                    'name': trait_name,
                    'start': start_idx,
                    'end': end_idx,
                    'insert_before': first_impl_line_idx,
                }
                break
            
            i += 1
        
        # If no violation found, we're done
        if not trait_to_move:
            break
        
        # Fix this one violation
        start = trait_to_move['start']
        end = trait_to_move['end']
        insert_before = trait_to_move['insert_before']
        
        # Extract trait block
        trait_lines = lines[start:end+1]
        
        # Check if there's a blank line after the trait
        if end + 1 < len(lines) and not lines[end + 1].strip():
            trait_lines.append(lines[end + 1])
            end += 1
        
        # Remove trait from current position
        del lines[start:end+1]
        
        # Adjust insert position if necessary
        if start < insert_before:
            insert_before -= (end - start + 1)
        
        # Insert trait before first impl
        # Ensure proper spacing
        if insert_before > 0 and lines[insert_before - 1].strip():
            lines.insert(insert_before, '\n')
            insert_before += 1
        
        # Insert the trait lines
        for j, line in enumerate(trait_lines):
            lines.insert(insert_before + j, line if line.endswith('\n') or not line else line + '\n')
        
        # Add blank line after trait if needed
        insert_after = insert_before + len(trait_lines)
        if insert_after < len(lines) and lines[insert_after].strip():
            lines.insert(insert_after, '\n')
        
        # Write the fixed file
        if not dry_run:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(lines)
        
        any_fixed = True
    
    if any_fixed:
        if not dry_run:
            print(f"Fixed: {file_path}")
        else:
            print(f"Would fix: {file_path}")
    
    return any_fixed


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Fix trait definition order: move traits before impl blocks"
    )
    parser.add_argument('--file', help='Specific file to fix')
    parser.add_argument('--dry-run', action='store_true',
                        help='Show what would be fixed without making changes')
    
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    
    if args.file:
        files_to_check = [Path(args.file)]
    else:
        search_dirs = [repo_root / "src"]
        files_to_check = []
        for search_dir in search_dirs:
            if search_dir.exists():
                files_to_check.extend(search_dir.rglob("*.rs"))
    
    fixed_count = 0
    for file_path in files_to_check:
        if fix_trait_order(file_path, args.dry_run):
            fixed_count += 1
    
    if fixed_count > 0:
        print(f"\n{'Would fix' if args.dry_run else 'Fixed'} {fixed_count} file(s)")
    else:
        print("No files needed fixing")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

