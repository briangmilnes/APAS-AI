#!/usr/bin/env python3
"""
Review PascalCase and camelCase function/method names that need conversion to snake_case.

Scans src/, tests/, and benches/ directories.
Shows what would be changed without modifying any files.
"""

import re
import sys
from pathlib import Path
from collections import defaultdict


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
            # Add underscore before uppercase if:
            # 1. Not at start
            # 2. Previous char was lowercase, OR
            # 3. Next char exists and is lowercase (end of acronym)
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
    # Has at least one uppercase letter
    return any(c.isupper() for c in name)


def extract_function_names(file_path):
    """
    Extract all PascalCase/camelCase function/method names from a file.
    Returns list of (line_num, original_name, snake_name) tuples.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return []
    
    results = []
    
    # Pattern for function definitions
    fn_pattern = re.compile(
        r'\b(?:pub(?:\([^)]*\))?\s+)?'
        r'(?:unsafe\s+)?'
        r'(?:async\s+)?'
        r'(?:const\s+)?'
        r'fn\s+([a-zA-Z_][a-zA-Z0-9_]*)'
    )
    
    for line_num, line in enumerate(lines, start=1):
        # Skip comments
        if line.strip().startswith('//'):
            continue
        
        code_part = re.sub(r'//.*$', '', line)
        
        match = fn_pattern.search(code_part)
        if match:
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
                results.append((line_num, fn_name, snake_name))
    
    return results


def scan_directory(directory, workspace_root):
    """Scan a directory for Rust files needing conversion."""
    if not directory.exists():
        return {}
    
    rust_files = sorted(directory.rglob('*.rs'))
    
    file_results = {}
    
    for file_path in rust_files:
        conversions = extract_function_names(file_path)
        if conversions:
            rel_path = file_path.relative_to(workspace_root)
            file_results[rel_path] = conversions
    
    return file_results


def main():
    # Find workspace root
    script_path = Path(__file__).resolve()
    workspace_root = script_path
    while workspace_root.parent != workspace_root:
        if (workspace_root / 'Cargo.toml').exists():
            break
        workspace_root = workspace_root.parent
    
    print("=" * 100)
    print("REVIEW: PascalCase/camelCase → snake_case Conversion")
    print("=" * 100)
    print()
    
    # Scan all three directories
    src_results = scan_directory(workspace_root / 'src', workspace_root)
    test_results = scan_directory(workspace_root / 'tests', workspace_root)
    bench_results = scan_directory(workspace_root / 'benches', workspace_root)
    
    all_results = {**src_results, **test_results, **bench_results}
    
    if not all_results:
        print("✅ No PascalCase or camelCase function names found!")
        print("   All function names already follow snake_case convention.")
        return 0
    
    # Build global conversion map
    global_conversions = {}
    for conversions in all_results.values():
        for _, old_name, new_name in conversions:
            global_conversions[old_name] = new_name
    
    # Print summary by directory
    print(f"📊 SUMMARY")
    print("=" * 100)
    print(f"Files in src/:     {len(src_results)}")
    print(f"Files in tests/:   {len(test_results)}")
    print(f"Files in benches/: {len(bench_results)}")
    print(f"Total files:       {len(all_results)}")
    print(f"Unique conversions: {len(global_conversions)}")
    print()
    
    # Print global conversion map
    print("📋 GLOBAL CONVERSION MAP")
    print("=" * 100)
    print(f"{'Original Name':<35} → {'snake_case Name':<35}")
    print("-" * 100)
    for old_name, new_name in sorted(global_conversions.items()):
        print(f"{old_name:<35} → {new_name:<35}")
    print()
    
    # Print detailed file-by-file breakdown
    print("📁 DETAILED FILE BREAKDOWN")
    print("=" * 100)
    
    for section_name, results in [
        ("src/", src_results),
        ("tests/", test_results),
        ("benches/", bench_results)
    ]:
        if not results:
            continue
        
        print(f"\n{section_name}")
        print("-" * 100)
        
        for rel_path, conversions in sorted(results.items()):
            print(f"\n  📄 {rel_path} ({len(conversions)} conversion{'' if len(conversions) == 1 else 's'})")
            for line_num, old_name, new_name in conversions:
                print(f"     Line {line_num:4}: {old_name:<30} → {new_name}")
    
    print()
    print("=" * 100)
    print("NEXT STEPS")
    print("=" * 100)
    print("1. Review the conversions above")
    print("2. If acceptable, run: python scripts/rust/fix_to_snake_case.py")
    print("3. After running fix script:")
    print("   - Run: cargo check")
    print("   - Run: cargo test")
    print("   - Review: git diff")
    print("4. Commit if successful, or revert if issues found")
    print()
    print(f"⚠️  Total changes: {sum(len(c) for c in all_results.values())} function definitions")
    print(f"   Plus all call sites in {len(all_results)} files")
    print("=" * 100)
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

