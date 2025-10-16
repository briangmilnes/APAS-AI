#!/usr/bin/env python3
"""
Fix: Implementation order - move standard traits to the bottom.

Automatically reorders trait implementations so that standard trait impls
(Eq, PartialEq, Debug, Display, etc.) come after custom trait impls.
"""

import re
import sys
from pathlib import Path

# Standard library traits that should come after custom impls
STANDARD_TRAITS = {
    'Eq', 'PartialEq', 'Ord', 'PartialOrd',
    'Debug', 'Display', 
    'Clone', 'Copy',
    'Hash', 
    'Default',
    'From', 'Into', 'TryFrom', 'TryInto',
    'AsRef', 'AsMut',
    'Deref', 'DerefMut',
    'Drop',
    'Iterator', 'IntoIterator',
    'Index', 'IndexMut',
    'Add', 'Sub', 'Mul', 'Div', 'Rem', 'Neg',
    'BitAnd', 'BitOr', 'BitXor', 'Shl', 'Shr',
    'Not',
    'Send', 'Sync',
    'Fn', 'FnMut', 'FnOnce',
    'Error',
}


def extract_trait_name(impl_line):
    """Extract trait name from an impl line."""
    impl_line = re.sub(r'//.*$', '', impl_line).strip()
    match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+', impl_line)
    if match:
        return match.group(1)
    return None


def count_braces_in_line(line):
    """
    Count braces in a line, ignoring those in strings and comments.
    Returns (open_braces, close_braces)
    """
    # Remove string literals and comments for accurate brace counting
    in_string = False
    in_char = False
    escape = False
    cleaned = []
    
    i = 0
    while i < len(line):
        c = line[i]
        
        # Handle escape sequences
        if escape:
            escape = False
            i += 1
            continue
        
        if c == '\\':
            escape = True
            i += 1
            continue
        
        # Handle strings
        if c == '"' and not in_char:
            in_string = not in_string
            i += 1
            continue
        
        # Handle char literals vs lifetimes
        # Char literal: 'x' where x is a character or escape sequence
        # Lifetime: 'ident where ident is an identifier or '_
        if c == "'" and not in_string:
            # Look ahead to distinguish char literal from lifetime
            if i + 2 < len(line):
                next_char = line[i + 1]
                after_next = line[i + 2]
                
                # Check if it's a lifetime (followed by identifier char or _)
                if next_char.isalpha() or next_char == '_':
                    # It's a lifetime, not a char literal - treat as regular code
                    cleaned.append(c)
                    i += 1
                    continue
                # Check for escaped char like '\n'
                elif next_char == '\\' and i + 3 < len(line) and line[i + 3] == "'":
                    # It's a char literal '\x' - skip it
                    i += 4
                    continue
                # Regular char like 'a'
                elif after_next == "'":
                    # It's a char literal 'x' - skip it
                    i += 3
                    continue
            
            # Default: treat as start of char literal for safety
            in_char = not in_char
            i += 1
            continue
        
        # If we're in a string or char, skip this character
        if in_string or in_char:
            i += 1
            continue
        
        # Handle line comments
        if i + 1 < len(line) and line[i:i+2] == '//':
            break  # Rest of line is comment
        
        cleaned.append(c)
        i += 1
    
    cleaned_line = ''.join(cleaned)
    open_braces = cleaned_line.count('{')
    close_braces = cleaned_line.count('}')
    
    return (open_braces, close_braces)


def find_impl_blocks(lines):
    """
    Find all trait impl blocks in the file.
    Returns list of (start_line, end_line, trait_name, is_standard, lines_content)
    """
    impl_blocks = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Check if this is a trait impl line
        if stripped.startswith('impl') and ' for ' in stripped:
            trait_name = extract_trait_name(stripped)
            
            if trait_name:
                # Find the closing brace for this impl block
                start_line = i
                
                # Count braces on the impl line itself
                open_b, close_b = count_braces_in_line(stripped)
                brace_count = open_b - close_b
                
                j = i + 1
                
                # Continue until braces balance
                while j < len(lines) and brace_count > 0:
                    open_b, close_b = count_braces_in_line(lines[j])
                    brace_count += open_b - close_b
                    j += 1
                
                end_line = j
                is_standard = trait_name in STANDARD_TRAITS
                impl_lines = lines[start_line:end_line]
                
                impl_blocks.append({
                    'start': start_line,
                    'end': end_line,
                    'trait': trait_name,
                    'is_standard': is_standard,
                    'lines': impl_lines,
                })
                
                i = end_line
                continue
        
        i += 1
    
    return impl_blocks


def fix_file(file_path, dry_run=False):
    """
    Fix implementation order in a file.
    Returns True if changes were made.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    impl_blocks = find_impl_blocks(lines)
    
    if not impl_blocks:
        return False
    
    # Group consecutive impl blocks that need reordering
    # We need to find groups where standard impls come before custom impls
    groups_to_fix = []
    processed_blocks = set()  # Track blocks already in a group
    
    for i in range(len(impl_blocks)):
        current = impl_blocks[i]
        
        # Skip if this block is already in a group
        if i in processed_blocks:
            continue
        
        # Check if this standard impl is followed by any custom impl
        if current['is_standard']:
            # Find all consecutive impls (with possible blank lines between)
            group = [current]
            group_indices = [i]
            needs_fix = False
            
            for j in range(i + 1, len(impl_blocks)):
                if j in processed_blocks:
                    continue
                    
                next_block = impl_blocks[j]
                
                # Check if blocks are consecutive (allowing for blank lines)
                gap = next_block['start'] - group[-1]['end']
                # Allow up to 5 lines of whitespace/comments between impls
                if gap <= 5:
                    group.append(next_block)
                    group_indices.append(j)
                    if not next_block['is_standard']:
                        needs_fix = True
                else:
                    break
            
            if needs_fix and len(group) > 1:
                groups_to_fix.append(group)
                # Mark all blocks in this group as processed
                for idx in group_indices:
                    processed_blocks.add(idx)
    
    if not groups_to_fix:
        return False
    
    if dry_run:
        print(f"Would fix {len(groups_to_fix)} impl block group(s) in {file_path}")
        return True
    
    # Apply fixes from bottom to top to preserve line numbers
    new_lines = lines[:]
    
    for group in reversed(groups_to_fix):
        start = group[0]['start']
        end = group[-1]['end']
        
        # Separate blocks into custom and standard
        custom_blocks = []
        standard_blocks = []
        
        for block in group:
            if block['is_standard']:
                standard_blocks.append(block)
            else:
                custom_blocks.append(block)
        
        # Build the reordered content by simply concatenating impl blocks
        reordered = []
        
        # Add all custom impls first
        for i, block in enumerate(custom_blocks):
            reordered.extend(block['lines'])
            # Add blank line between custom impls (but not after the last one)
            if i < len(custom_blocks) - 1:
                # Only add if next line isn't already blank
                if block['lines'] and block['lines'][-1].strip() != '':
                    reordered.append('\n')
        
        # Add blank line between custom and standard sections
        if custom_blocks and standard_blocks:
            reordered.append('\n')
        
        # Add all standard impls
        for i, block in enumerate(standard_blocks):
            reordered.extend(block['lines'])
            # Add blank line between standard impls (but not after the last one)
            if i < len(standard_blocks) - 1:
                if block['lines'] and block['lines'][-1].strip() != '':
                    reordered.append('\n')
        
        # Replace the lines
        new_lines[start:end] = reordered
    
    # Write back
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description='Fix implementation order: move standard traits to bottom'
    )
    parser.add_argument(
        '--file',
        type=str,
        help='Specific file to fix'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Show what would be fixed without making changes'
    )
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.is_absolute():
            file_path = repo_root / file_path
        
        if fix_file(file_path, dry_run=args.dry_run):
            print(f"{'Would fix' if args.dry_run else 'Fixed'}: {file_path.relative_to(repo_root)}")
        else:
            print(f"No changes needed: {file_path.relative_to(repo_root)}")
        return 0
    
    # Fix all files in src/
    search_dir = repo_root / "src"
    fixed_count = 0
    
    for rs_file in sorted(search_dir.rglob("*.rs")):
        if fix_file(rs_file, dry_run=args.dry_run):
            rel_path = rs_file.relative_to(repo_root)
            print(f"{'Would fix' if args.dry_run else 'Fixed'}: {rel_path}")
            fixed_count += 1
    
    if fixed_count > 0:
        print(f"\n{'Would fix' if args.dry_run else 'Fixed'} {fixed_count} file(s)")
    else:
        print("No files need fixing")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

