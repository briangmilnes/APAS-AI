#!/usr/bin/env python3
"""
Fix trait method duplication by removing inherent methods that duplicate trait methods.

WARNING: This makes potentially breaking changes. Always:
1. Run in --dry-run mode first
2. Compile after each file
3. Have git ready to revert

Usage:
  ./fix_trait_method_duplication.py              # Fix all files (dry-run)
  ./fix_trait_method_duplication.py --execute    # Actually fix
  ./fix_trait_method_duplication.py --file src/Chap18/ArraySeqStEph.rs --execute
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import run_review, get_repo_root


def extract_impl_blocks(content: str) -> list[dict]:
    """Extract all impl blocks from Rust source."""
    impl_blocks = []
    lines = content.splitlines()
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Match impl block start
        impl_match = re.match(r'^(\s*)impl<(.*)>\s+(\w+)', line)
        if impl_match:
            indent = impl_match.group(1)
            generics = impl_match.group(2)
            
            # Check if it's a trait impl or inherent impl
            # Trait impl: "impl<T> Trait for Type"
            # Inherent impl: "impl<T> Type"
            is_trait_impl = ' for ' in line
            
            # Find the impl block extent
            start_line = i
            brace_depth = 0
            found_brace = False
            j = i
            
            while j < len(lines):
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if '{' in lines[j]:
                    found_brace = True
                if found_brace and brace_depth == 0:
                    break
                j += 1
            
            end_line = j
            
            if found_brace:
                impl_blocks.append({
                    'start': start_line,
                    'end': end_line,
                    'is_trait_impl': is_trait_impl,
                    'indent': indent
                })
            
            i = end_line + 1
        else:
            i += 1
    
    return impl_blocks


def extract_method_line_ranges(lines: list[str], block_start: int, block_end: int) -> list[dict]:
    """Extract method definitions and their line ranges within an impl block."""
    methods = []
    
    i = block_start + 1  # Skip the impl line itself
    while i <= block_end:
        line = lines[i]
        
        # Match method definition
        method_match = re.match(r'^\s*(pub\s+)?fn\s+(\w+)', line)
        if method_match:
            method_name = method_match.group(2)
            method_start = i
            
            # Find the end of this method
            brace_depth = 0
            found_brace = False
            j = i
            
            while j <= block_end:
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if '{' in lines[j]:
                    found_brace = True
                if found_brace and brace_depth == 0:
                    break
                j += 1
            
            method_end = j
            
            methods.append({
                'name': method_name,
                'start': method_start,
                'end': method_end
            })
            
            i = method_end + 1
        else:
            i += 1
    
    return methods


def find_duplicates(file_path: Path) -> list[dict]:
    """Find inherent methods that duplicate trait methods."""
    try:
        content = file_path.read_text(encoding='utf-8')
    except Exception as e:
        print(f"ERROR: Could not read {file_path}: {e}", file=sys.stderr)
        return []
    
    lines = content.splitlines()
    impl_blocks = extract_impl_blocks(content)
    
    # Separate inherent and trait impls
    inherent_impls = [b for b in impl_blocks if not b['is_trait_impl']]
    trait_impls = [b for b in impl_blocks if b['is_trait_impl']]
    
    duplicates = []
    
    for inherent in inherent_impls:
        inherent_methods = extract_method_line_ranges(lines, inherent['start'], inherent['end'])
        
        for trait in trait_impls:
            trait_methods = extract_method_line_ranges(lines, trait['start'], trait['end'])
            trait_method_names = {m['name'] for m in trait_methods}
            
            for method in inherent_methods:
                if method['name'] in trait_method_names:
                    duplicates.append({
                        'name': method['name'],
                        'start': method['start'],
                        'end': method['end'],
                        'inherent_block_start': inherent['start'],
                        'trait_block_start': trait['start']
                    })
    
    return duplicates


def fix_file(file_path: Path, execute: bool = False) -> dict:
    """Remove duplicate inherent methods from a file."""
    duplicates = find_duplicates(file_path)
    
    if not duplicates:
        return {'fixed': 0, 'errors': []}
    
    try:
        lines = file_path.read_text(encoding='utf-8').splitlines()
    except Exception as e:
        return {'fixed': 0, 'errors': [f"Could not read file: {e}"]}
    
    # Sort duplicates by line number (descending) so we can delete from bottom up
    duplicates.sort(key=lambda d: d['start'], reverse=True)
    
    deleted_count = 0
    for dup in duplicates:
        if execute:
            # Delete lines from start to end (inclusive)
            del lines[dup['start']:dup['end'] + 1]
        else:
            print(f"  Would delete '{dup['name']}' (lines {dup['start'] + 1}-{dup['end'] + 1})")
        deleted_count += 1
    
    if execute and deleted_count > 0:
        try:
            file_path.write_text('\n'.join(lines) + '\n', encoding='utf-8')
        except Exception as e:
            return {'fixed': 0, 'errors': [f"Could not write file: {e}"]}
    
    return {'fixed': deleted_count, 'errors': []}


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='Fix trait method duplication')
    parser.add_argument('--file', type=str, help='Fix a specific file')
    parser.add_argument('--execute', action='store_true', help='Actually make changes (default is dry-run)')
    args = parser.parse_args()
    
    repo_root = get_repo_root()
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.is_absolute():
            file_path = repo_root / file_path
        files = [file_path]
    else:
        files = list((repo_root / "src").rglob("*.rs"))
    
    mode = "EXECUTING" if args.execute else "DRY-RUN"
    print(f"🔧 {mode}: Fixing trait method duplication in {len(files)} file(s)")
    print()
    
    total_fixed = 0
    files_with_changes = 0
    
    for file_path in sorted(files):
        duplicates = find_duplicates(file_path)
        
        if duplicates:
            rel_path = file_path.relative_to(repo_root)
            print(f"📝 {rel_path} - {len(duplicates)} duplicate(s)")
            
            result = fix_file(file_path, execute=args.execute)
            
            if result['errors']:
                for error in result['errors']:
                    print(f"  ❌ ERROR: {error}")
            else:
                total_fixed += result['fixed']
                if result['fixed'] > 0:
                    files_with_changes += 1
                    if args.execute:
                        print(f"  ✓ Deleted {result['fixed']} duplicate method(s)")
            print()
    
    print("=" * 60)
    if args.execute:
        print(f"✅ Fixed {total_fixed} duplicate(s) in {files_with_changes} file(s)")
        print()
        print("⚠️  IMPORTANT: Run cargo check to verify changes!")
    else:
        print(f"📊 Would fix {total_fixed} duplicate(s) in {files_with_changes} file(s)")
        print()
        print("💡 Run with --execute to actually make changes")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

