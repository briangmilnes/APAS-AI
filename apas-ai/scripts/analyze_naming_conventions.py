#!/usr/bin/env python3
"""
Analyze naming conventions for functions and methods in Rust files.
Reports PascalCase vs camelCase vs snake_case per file in table format.
"""

import re
import sys
from pathlib import Path
from collections import defaultdict


def classify_name(name):
    """
    Classify a name into PascalCase, camelCase, snake_case, or other.
    Returns: 'pascal', 'camel', 'snake', 'other'
    """
    if not name:
        return 'other'
    
    # All uppercase (constants) - skip
    if name.isupper():
        return 'other'
    
    has_underscore = '_' in name
    starts_upper = name[0].isupper()
    starts_lower = name[0].islower()
    has_uppercase = any(c.isupper() for c in name[1:])
    
    # PascalCase: starts with uppercase, no underscores, may have more uppercase
    if starts_upper and not has_underscore:
        return 'pascal'
    
    # camelCase: starts with lowercase, has uppercase letters, no underscores
    if starts_lower and not has_underscore and has_uppercase:
        return 'camel'
    
    # snake_case: all lowercase (or only lowercase letters), may have underscores
    if starts_lower and (not has_uppercase or name.replace('_', '').islower()):
        return 'snake'
    
    return 'other'


def extract_functions_and_methods(file_path):
    """
    Extract function and method names from a Rust file.
    Returns dict with 'pascal', 'camel', 'snake' lists of (line_num, name) tuples.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return None
    
    pascal_case = []
    camel_case = []
    snake_case = []
    other = []
    
    # Patterns for function/method definitions
    # fn name(...) or pub fn name(...) or pub(crate) fn name(...)
    fn_pattern = re.compile(
        r'^\s*(?:pub(?:\([^)]*\))?\s+)?'  # optional pub or pub(...)
        r'(?:unsafe\s+)?'                  # optional unsafe
        r'(?:async\s+)?'                   # optional async
        r'(?:const\s+)?'                   # optional const
        r'fn\s+([a-zA-Z_][a-zA-Z0-9_]*)'   # fn keyword and name
    )
    
    for i, line in enumerate(lines, start=1):
        # Skip comments
        stripped = line.strip()
        if stripped.startswith('//'):
            continue
        
        # Remove inline comments
        code_part = re.sub(r'//.*$', '', line)
        
        match = fn_pattern.search(code_part)
        if match:
            fn_name = match.group(1)
            
            # Skip standard Rust trait method names
            if fn_name in ['new', 'default', 'drop', 'clone', 'fmt', 'eq', 'cmp', 
                          'hash', 'from', 'into', 'try_from', 'try_into',
                          'as_ref', 'as_mut', 'deref', 'deref_mut',
                          'index', 'next', 'into_iter', 'iter', 'iter_mut',
                          'add', 'sub', 'mul', 'div', 'rem', 'neg',
                          'not', 'main', 'test']:
                continue
            
            case_type = classify_name(fn_name)
            
            if case_type == 'pascal':
                pascal_case.append((i, fn_name))
            elif case_type == 'camel':
                camel_case.append((i, fn_name))
            elif case_type == 'snake':
                snake_case.append((i, fn_name))
            else:
                other.append((i, fn_name))
    
    return {
        'pascal': pascal_case,
        'camel': camel_case,
        'snake': snake_case,
        'other': other
    }


def print_table_header():
    """Print the table header."""
    print("=" * 110)
    print(f"{'File':<55} {'Pascal':>10} {'camel':>10} {'snake':>10} {'Other':>10} {'Status':<10}")
    print("=" * 110)


def print_table_row(rel_path, pascal_count, camel_count, snake_count, other_count):
    """Print a single table row."""
    # Determine status
    non_zero = sum([1 for c in [pascal_count, camel_count, snake_count] if c > 0])
    
    if non_zero == 0:
        status = "-"
    elif non_zero == 1:
        if pascal_count > 0:
            status = "✓ Pascal"
        elif camel_count > 0:
            status = "✓ camel"
        else:
            status = "✓ snake"
    else:
        status = "⚠️ MIXED"
    
    # Truncate path if too long
    path_str = str(rel_path)
    if len(path_str) > 54:
        path_str = "..." + path_str[-51:]
    
    print(f"{path_str:<55} {pascal_count:>10} {camel_count:>10} {snake_count:>10} {other_count:>10} {status:<10}")


def main():
    # Find workspace root
    script_path = Path(__file__).resolve()
    workspace_root = script_path
    while workspace_root.parent != workspace_root:
        if (workspace_root / 'Cargo.toml').exists():
            break
        workspace_root = workspace_root.parent
    
    src_dir = workspace_root / 'src'
    if not src_dir.exists():
        print(f"Error: {src_dir} not found", file=sys.stderr)
        return 1
    
    rust_files = sorted(src_dir.rglob('*.rs'))
    
    # Statistics
    total_pascal = 0
    total_camel = 0
    total_snake = 0
    total_other = 0
    
    pascal_only_files = 0
    camel_only_files = 0
    snake_only_files = 0
    mixed_files = 0
    
    # Collect all results
    file_results = []
    
    for file_path in rust_files:
        result = extract_functions_and_methods(file_path)
        if result is None:
            continue
        
        pascal = result['pascal']
        camel = result['camel']
        snake = result['snake']
        other = result['other']
        
        pascal_count = len(pascal)
        camel_count = len(camel)
        snake_count = len(snake)
        other_count = len(other)
        
        # Skip files with no functions
        if pascal_count + camel_count + snake_count == 0:
            continue
        
        rel_path = file_path.relative_to(workspace_root)
        
        file_results.append({
            'path': rel_path,
            'pascal': pascal,
            'camel': camel,
            'snake': snake,
            'other': other,
            'pascal_count': pascal_count,
            'camel_count': camel_count,
            'snake_count': snake_count,
            'other_count': other_count
        })
        
        # Update totals
        total_pascal += pascal_count
        total_camel += camel_count
        total_snake += snake_count
        total_other += other_count
        
        # Count file categories
        non_zero = sum([1 for c in [pascal_count, camel_count, snake_count] if c > 0])
        
        if non_zero > 1:
            mixed_files += 1
        elif pascal_count > 0:
            pascal_only_files += 1
        elif camel_count > 0:
            camel_only_files += 1
        elif snake_count > 0:
            snake_only_files += 1
    
    # Print title
    print("\n" + "=" * 110)
    print("RUST FUNCTION/METHOD NAMING CONVENTION ANALYSIS - FULL TABLE")
    print("=" * 110)
    print()
    
    # Print table
    print_table_header()
    
    for result in file_results:
        print_table_row(
            result['path'],
            result['pascal_count'],
            result['camel_count'],
            result['snake_count'],
            result['other_count']
        )
    
    print("=" * 110)
    
    # Print detailed breakdown of mixed files
    mixed_results = [r for r in file_results if sum([1 for c in [r['pascal_count'], r['camel_count'], r['snake_count']] if c > 0]) > 1]
    
    if mixed_results:
        print("\n" + "=" * 110)
        print("DETAILED BREAKDOWN - MIXED NAMING FILES")
        print("=" * 110)
        
        for result in mixed_results:
            print(f"\n📁 {result['path']}")
            
            if result['pascal']:
                print(f"   PascalCase ({len(result['pascal'])} functions):")
                for line_num, name in result['pascal'][:10]:
                    print(f"      Line {line_num}: {name}")
                if len(result['pascal']) > 10:
                    print(f"      ... and {len(result['pascal']) - 10} more")
            
            if result['camel']:
                print(f"   camelCase ({len(result['camel'])} functions):")
                for line_num, name in result['camel'][:10]:
                    print(f"      Line {line_num}: {name}")
                if len(result['camel']) > 10:
                    print(f"      ... and {len(result['camel']) - 10} more")
            
            if result['snake']:
                print(f"   snake_case ({len(result['snake'])} functions):")
                for line_num, name in result['snake'][:10]:
                    print(f"      Line {line_num}: {name}")
                if len(result['snake']) > 10:
                    print(f"      ... and {len(result['snake']) - 10} more")
    
    # Summary
    print("\n" + "=" * 110)
    print("SUMMARY")
    print("=" * 110)
    print(f"Total files analyzed: {len(file_results)}")
    print(f"Total functions/methods: {total_pascal + total_camel + total_snake}")
    print()
    print(f"PascalCase functions:    {total_pascal:>6}  ({100*total_pascal/(total_pascal+total_camel+total_snake+0.001):.1f}%)")
    print(f"camelCase functions:     {total_camel:>6}  ({100*total_camel/(total_pascal+total_camel+total_snake+0.001):.1f}%)")
    print(f"snake_case functions:    {total_snake:>6}  ({100*total_snake/(total_pascal+total_camel+total_snake+0.001):.1f}%)")
    if total_other > 0:
        print(f"Other naming:            {total_other:>6}")
    print()
    print(f"Files with PascalCase only:  {pascal_only_files:>4}")
    print(f"Files with camelCase only:   {camel_only_files:>4}")
    print(f"Files with snake_case only:  {snake_only_files:>4}")
    print(f"Files with MIXED naming:     {mixed_files:>4}  {'⚠️' if mixed_files > 0 else '✅'}")
    print("=" * 110)
    print()
    
    if mixed_files > 0:
        return 1
    else:
        return 0


if __name__ == '__main__':
    sys.exit(main())
