#!/usr/bin/env python3
"""
Fix: Implementation order - move standard traits to the bottom.

Automatically reorders trait implementations so that standard trait impls
(Eq, PartialEq, Debug, Display, etc.) come after custom trait impls.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


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


def find_macro_definitions(lines):
    """
    Find all macro definition blocks in the file.
    Returns list of (start_line, end_line) for macro_rules! definitions.
    """
    macros = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Check if this is a macro definition
        if 'macro_rules!' in stripped or '#[macro_export]' in stripped:
            # Find the macro_rules! line
            start_line = i
            while i < len(lines) and 'macro_rules!' not in lines[i]:
                i += 1
            
            if i >= len(lines):
                break
            
            # Find the closing }; for the macro
            # Macros end with }; (the outermost brace + semicolon)
            brace_count = 0
            started = False
            
            while i < len(lines):
                line_content = lines[i]
                
                # Count braces
                open_b, close_b = count_braces_in_line(line_content)
                if open_b > 0:
                    started = True
                brace_count += open_b - close_b
                
                # Check if we've closed the macro
                # The macro ends when we close the outermost brace with };
                if started and brace_count == 1 and lines[i].strip().endswith('};'):
                    macros.append({
                        'start': start_line,
                        'end': i + 1,
                    })
                    i += 1
                    break
                
                i += 1
        else:
            i += 1
    
    return macros


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
    macros = find_macro_definitions(lines)
    
    if not impl_blocks:
        return False
    
    # Find ALL standard trait impls that come before ANY custom trait impl
    # Find the position of the last custom trait impl
    last_custom_line = 0
    for block in impl_blocks:
        if not block['is_standard']:
            last_custom_line = max(last_custom_line, block['end'])
    
    if last_custom_line == 0:
        # No custom traits found, nothing to do
        return False
    
    # Collect all standard trait impls that come before the last custom impl
    standard_impls_to_move = []
    for block in impl_blocks:
        if block['is_standard'] and block['start'] < last_custom_line:
            standard_impls_to_move.append(block)
    
    if not standard_impls_to_move:
        return False
    
    # Find insertion point: before first macro or at end of module
    insertion_point = len(lines) - 1  # Default: before last line (module closing brace)
    
    # Look for #[macro_export] or macro_rules! after last custom impl
    for i in range(last_custom_line, len(lines)):
        stripped = lines[i].strip()
        if stripped.startswith('#[macro_export]') or stripped.startswith('macro_rules!'):
            insertion_point = i
            break
    
    # If no macro found, insert before closing module brace
    if insertion_point == len(lines) - 1:
        # Find the last closing brace
        for i in range(len(lines) - 1, last_custom_line, -1):
            if lines[i].strip() == '}':
                insertion_point = i
                break
    
    if dry_run:
        print(f"Would move {len(standard_impls_to_move)} standard trait impl(s) to bottom in {file_path}")
        return True
    
    # Extract all standard impl blocks
    standard_impl_code = []
    for impl_block in standard_impls_to_move:
        standard_impl_code.extend(impl_block['lines'])
    
    # Remove standard impls from original positions (work backwards to preserve indices)
    new_lines = lines[:]
    for impl_block in reversed(standard_impls_to_move):
        del new_lines[impl_block['start']:impl_block['end']]
    
    # Recalculate insertion point after deletions
    # Count how many lines were deleted before the original insertion point
    deleted_before_insertion = sum(
        impl_block['end'] - impl_block['start']
        for impl_block in standard_impls_to_move
        if impl_block['start'] < insertion_point
    )
    adjusted_insertion_point = insertion_point - deleted_before_insertion
    
    # Add blank line separator if needed
    if adjusted_insertion_point > 0 and adjusted_insertion_point < len(new_lines):
        if new_lines[adjusted_insertion_point - 1].strip():
            standard_impl_code.insert(0, '\n')
        # Also add blank line after if the next line isn't blank
        if adjusted_insertion_point < len(new_lines) and new_lines[adjusted_insertion_point].strip():
            standard_impl_code.append('\n')
    
    # Insert all standard impls at calculated position
    new_lines[adjusted_insertion_point:adjusted_insertion_point] = standard_impl_code
    
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

