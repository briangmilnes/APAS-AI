#!/usr/bin/env python3
"""
Convert all PascalCase and camelCase function/method names to snake_case.

Scans and modifies src/, tests/, and benches/ directories.

This script updates:
1. Function definitions (pub fn, fn)
2. Method definitions in traits
3. Method implementations
4. Function/method calls
5. UFCS-style calls
6. Macro invocations

USE WITH CAUTION - makes extensive changes across the codebase.
"""

import re
import sys
from pathlib import Path
from collections import defaultdict
import argparse


def pascal_or_camel_to_snake(name):
    """
    Convert PascalCase or camelCase to snake_case.
    
    Examples:
        FromVec → from_vec
        CartesianProduct → cartesian_product
        NPlus → n_plus
        NGOfVertices → ng_of_vertices
        isEmpty → is_empty
        isSingleton → is_singleton
        createTable → create_table
    """
    result = []
    prev_upper = False
    
    for i, char in enumerate(name):
        if char.isupper():
            if i > 0:
                if not prev_upper:
                    result.append('_')
                elif i + 1 < len(name) and name[i + 1].islower():
                    result.append('_')
            result.append(char.lower())
            prev_upper = True
        else:
            result.append(char)
            prev_upper = False
    
    return ''.join(result)


def is_pascal_or_camel_case(name):
    """Check if name is PascalCase or camelCase (not snake_case)."""
    if not name or '_' in name:
        return False
    if name.isupper() or name.islower():
        return False
    return any(c.isupper() for c in name)


def extract_function_names(file_path):
    """
    Extract all PascalCase/camelCase function/method names from a file.
    Returns dict mapping original name -> snake_case name.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return {}
    
    conversions = {}
    
    fn_pattern = re.compile(
        r'\b(?:pub(?:\([^)]*\))?\s+)?'
        r'(?:unsafe\s+)?'
        r'(?:async\s+)?'
        r'(?:const\s+)?'
        r'fn\s+([a-zA-Z_][a-zA-Z0-9_]*)'
    )
    
    for match in fn_pattern.finditer(content):
        fn_name = match.group(1)
        
        # Skip standard Rust names
        if fn_name in ['new', 'default', 'drop', 'clone', 'fmt', 'eq', 'cmp',
                       'hash', 'from', 'into', 'try_from', 'try_into',
                       'as_ref', 'as_mut', 'deref', 'deref_mut',
                       'index', 'next', 'into_iter', 'iter', 'iter_mut',
                       'add', 'sub', 'mul', 'div', 'rem', 'neg',
                       'not', 'main', 'test', 'empty', 'size', 'mem',
                       'union', 'intersection', 'difference', 'insert',
                       'remove', 'contains', 'len', 'capacity', 'clear',
                       'push', 'pop', 'get', 'set', 'nth', 'length']:
            continue
        
        if is_pascal_or_camel_case(fn_name):
            snake_name = pascal_or_camel_to_snake(fn_name)
            conversions[fn_name] = snake_name
    
    return conversions


def build_global_conversion_map(directories):
    """
    Build a global map of all PascalCase/camelCase names -> snake_case
    across all specified directories.
    """
    global_map = {}
    
    for directory in directories:
        if not directory.exists():
            continue
        
        rust_files = sorted(directory.rglob('*.rs'))
        
        for file_path in rust_files:
            conversions = extract_function_names(file_path)
            global_map.update(conversions)
    
    return global_map


def convert_file_content(content, conversion_map):
    """
    Apply conversions to file content.
    Handles function definitions, method calls, UFCS, traits, macros.
    """
    # Sort by length (longest first) to avoid partial replacements
    sorted_names = sorted(conversion_map.keys(), key=len, reverse=True)
    
    for old_name in sorted_names:
        new_name = conversion_map[old_name]
        
        # Pattern 1: fn definitions
        content = re.sub(
            r'\bfn\s+' + re.escape(old_name) + r'\b',
            f'fn {new_name}',
            content
        )
        
        # Pattern 2: Method calls .OldName(...)
        content = re.sub(
            r'\.(' + re.escape(old_name) + r')\s*\(',
            f'.{new_name}(',
            content
        )
        
        # Pattern 3: UFCS calls Type::OldName(...)
        content = re.sub(
            r'::(' + re.escape(old_name) + r')\s*\(',
            f'::{new_name}(',
            content
        )
        
        # Pattern 4: Trait method definitions (as ...)
        content = re.sub(
            r'>::(' + re.escape(old_name) + r')\s*\(',
            f'>::{new_name}(',
            content
        )
        
        # Pattern 5: Function calls without prefix
        content = re.sub(
            r'\b' + re.escape(old_name) + r'\s*\(',
            f'{new_name}(',
            content
        )
        
        # Pattern 6: In macro paths (without call parentheses)
        content = re.sub(
            r'::' + re.escape(old_name) + r'\b(?!\()',
            f'::{new_name}',
            content
        )
    
    return content


def convert_file(file_path, global_map, dry_run=True):
    """Convert a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            original_content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    new_content = convert_file_content(original_content, global_map)
    
    if new_content != original_content:
        if dry_run:
            print(f"Would modify: {file_path}")
        else:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            print(f"✓ Modified: {file_path}")
        return True
    
    return False


def main():
    parser = argparse.ArgumentParser(
        description='Convert PascalCase/camelCase function names to snake_case in src/, tests/, and benches/'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Show what would be changed without modifying files'
    )
    
    args = parser.parse_args()
    
    # Find workspace root
    script_path = Path(__file__).resolve()
    workspace_root = script_path
    while workspace_root.parent != workspace_root:
        if (workspace_root / 'Cargo.toml').exists():
            break
        workspace_root = workspace_root.parent
    
    directories = [
        workspace_root / 'src',
        workspace_root / 'tests',
        workspace_root / 'benches'
    ]
    
    print("=" * 100)
    print("FIX: Convert PascalCase/camelCase → snake_case")
    print("=" * 100)
    print()
    
    if args.dry_run:
        print("⚠️  DRY RUN MODE - No files will be modified")
        print()
    else:
        print("🔨 APPLYING CHANGES - Files will be modified")
        print()
        response = input("Are you sure you want to continue? [y/N]: ")
        if response.lower() != 'y':
            print("Cancelled.")
            return 1
        print()
    
    # Build global conversion map
    print("Building conversion map...")
    global_map = build_global_conversion_map(directories)
    
    if not global_map:
        print("✅ No conversions needed - all function names already use snake_case!")
        return 0
    
    print(f"Found {len(global_map)} unique function names to convert\n")
    
    # Convert all Rust files
    modified_count = 0
    total_files = 0
    
    for directory in directories:
        if not directory.exists():
            continue
        
        rust_files = sorted(directory.rglob('*.rs'))
        
        for file_path in rust_files:
            total_files += 1
            if convert_file(file_path, global_map, dry_run=args.dry_run):
                modified_count += 1
    
    print()
    print("=" * 100)
    print("SUMMARY")
    print("=" * 100)
    print(f"Total Rust files scanned: {total_files}")
    print(f"Unique conversions: {len(global_map)}")
    
    if args.dry_run:
        print(f"Files that would be modified: {modified_count}")
        print()
        print("To apply changes, run without --dry-run:")
        print("  python scripts/rust/fix_to_snake_case.py")
    else:
        print(f"Files modified: {modified_count}")
        print()
        print("✅ Conversion complete!")
        print()
        print("Next steps:")
        print("  1. cargo check     # Verify compilation")
        print("  2. cargo test      # Run tests")
        print("  3. git diff        # Review changes")
        print("  4. git add -A && git commit -m 'Convert to snake_case naming'")
        print("  5. Or: git reset --hard HEAD  # If issues found")
    
    print("=" * 100)
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

