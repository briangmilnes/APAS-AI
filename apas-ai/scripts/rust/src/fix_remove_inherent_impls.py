#!/usr/bin/env python3
"""
Fix: Remove inherent impl blocks for structs that have custom trait impls.

Implements the "Single Implementation Pattern" rule from RustRules.md.
For structs with both inherent impl and custom trait impl, removes the
inherent impl block entirely, keeping only the trait impl.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

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


def extract_impl_info(line):
    """
    Extract information from an impl line.
    Returns (impl_type, struct_name, trait_name)
    """
    line = re.sub(r'//.*$', '', line).strip()
    
    # Check for trait impl: impl ... TraitName ... for StructName
    trait_match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)', line)
    if trait_match:
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        return ('trait', struct_name, trait_name)
    
    # Check for inherent impl: impl ... StructName
    inherent_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{', line)
    if inherent_match:
        struct_name = inherent_match.group(1)
        return ('inherent', struct_name, None)
    
    return None


def count_braces_in_line(line):
    """Count braces in a line, ignoring those in strings and comments."""
    # Simple version - count open and close braces
    in_string = False
    in_char = False
    escape = False
    open_count = 0
    close_count = 0
    
    i = 0
    while i < len(line):
        c = line[i]
        
        if escape:
            escape = False
            i += 1
            continue
        
        if c == '\\':
            escape = True
            i += 1
            continue
        
        if c == '"' and not in_char:
            in_string = not in_string
            i += 1
            continue
        
        if c == "'" and not in_string:
            # Simple check - if it looks like a lifetime, ignore it
            if i + 1 < len(line) and (line[i+1].isalpha() or line[i+1] == '_'):
                i += 1
                continue
            in_char = not in_char
            i += 1
            continue
        
        if not in_string and not in_char:
            if c == '{':
                open_count += 1
            elif c == '}':
                close_count += 1
        
        i += 1
    
    return (open_count, close_count)


def find_impl_blocks(lines):
    """
    Find all impl blocks in the file.
    Returns list of dicts with start, end, type, struct, trait info.
    """
    impl_blocks = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        if not stripped.startswith('impl'):
            i += 1
            continue
        
        impl_info = extract_impl_info(stripped)
        if not impl_info:
            i += 1
            continue
        
        impl_type, struct_name, trait_name = impl_info
        start_line = i
        
        # Count braces to find end of impl block
        open_b, close_b = count_braces_in_line(stripped)
        brace_count = open_b - close_b
        
        j = i + 1
        while j < len(lines) and brace_count > 0:
            open_b, close_b = count_braces_in_line(lines[j])
            brace_count += open_b - close_b
            j += 1
        
        end_line = j
        is_standard = trait_name in STANDARD_TRAITS if trait_name else False
        
        impl_blocks.append({
            'start': start_line,
            'end': end_line,
            'type': impl_type,
            'struct': struct_name,
            'trait': trait_name,
            'is_standard': is_standard,
        })
        
        i = end_line
    
    return impl_blocks


def fix_file(file_path, dry_run=False):
    """
    Remove inherent impl blocks for structs that have custom trait impls.
    Returns True if changes were made.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    impl_blocks = find_impl_blocks(lines)
    
    # Group impl blocks by struct
    struct_impls = {}
    for block in impl_blocks:
        struct_name = block['struct']
        if struct_name not in struct_impls:
            struct_impls[struct_name] = {'inherent': [], 'custom_traits': [], 'standard_traits': []}
        
        if block['type'] == 'inherent':
            struct_impls[struct_name]['inherent'].append(block)
        elif block['is_standard']:
            struct_impls[struct_name]['standard_traits'].append(block)
        else:
            struct_impls[struct_name]['custom_traits'].append(block)
    
    # Find structs with both inherent and custom trait impls
    blocks_to_remove = []
    for struct_name, impls in struct_impls.items():
        if impls['inherent'] and impls['custom_traits']:
            # Remove all inherent impl blocks for this struct
            blocks_to_remove.extend(impls['inherent'])
    
    if not blocks_to_remove:
        return False
    
    if dry_run:
        print(f"Would remove {len(blocks_to_remove)} inherent impl block(s) from {file_path}")
        for block in blocks_to_remove:
            print(f"  Lines {block['start']+1}-{block['end']}: impl {block['struct']}")
        return True
    
    # Remove blocks in reverse order to preserve line numbers
    new_lines = lines[:]
    for block in sorted(blocks_to_remove, key=lambda b: b['start'], reverse=True):
        # Also remove any blank lines immediately after the impl block
        end = block['end']
        while end < len(new_lines) and new_lines[end].strip() == '':
            end += 1
        
        del new_lines[block['start']:end]
    
    # Write back
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
        
        print(f"Fixed: {file_path}")
        print(f"  Removed {len(blocks_to_remove)} inherent impl block(s)")
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Remove inherent impl blocks for structs with custom trait impls"
    )
    parser.add_argument('--file', type=str, help='Single file to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.exists():
            print(f"Error: {file_path} not found", file=sys.stderr)
            return 1
        
        changed = fix_file(file_path, dry_run=args.dry_run)
        return 0 if changed else 1
    else:
        print("Error: --file argument required", file=sys.stderr)
        return 1


if __name__ == '__main__':
    sys.exit(main())


