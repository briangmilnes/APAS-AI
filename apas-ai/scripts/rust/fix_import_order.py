#!/usr/bin/env python3
"""
Fix: Import order and blank lines.

Automatically fixes import ordering and adds blank lines between sections:
1. std imports
2. [blank line]
3. external imports
4. [blank line]
5. internal imports (crate:: or apas_ai::)

RustRules.md Lines 50, 75-86
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path


def fix_file_imports(file_path, repo_root, dry_run=False):
    """Fix import order in a single file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Determine if file is in src/, tests/, or benches/
    relative_path = file_path.relative_to(repo_root)
    in_src = relative_path.parts[0] == 'src'
    
    # Find first use statement
    first_use_idx = None
    for idx, line in enumerate(lines):
        stripped = line.strip()
        if stripped.startswith('use '):
            first_use_idx = idx
            break
    
    if first_use_idx is None:
        return False  # No imports
    
    # Find last use statement (end of import block)
    last_use_idx = first_use_idx
    i = first_use_idx
    while i < len(lines):
        stripped = lines[i].strip()
        # Stop at first non-import, non-blank, non-comment line
        if not stripped.startswith('use ') and stripped and not stripped.startswith('//'):
            break
        if stripped.startswith('use '):
            last_use_idx = i
        i += 1
    
    # Extract imports
    std_imports = []
    external_imports = []
    internal_imports = []
    
    for idx in range(first_use_idx, last_use_idx + 1):
        line = lines[idx]
        stripped = line.strip()
        
        if not stripped.startswith('use '):
            continue
        
        if stripped.startswith('use std::') or stripped.startswith('use core::') or stripped.startswith('use alloc::'):
            std_imports.append(line)
        elif stripped.startswith('use crate::') or stripped.startswith('use apas_ai::'):
            internal_imports.append(line)
        elif stripped.startswith('use ') and not stripped.startswith('use self::') and not stripped.startswith('use super::'):
            external_imports.append(line)
    
    # Check if already correct
    current_order = []
    for idx in range(first_use_idx, last_use_idx + 1):
        line = lines[idx]
        stripped = line.strip()
        if stripped.startswith('use std::') or stripped.startswith('use core::') or stripped.startswith('use alloc::'):
            current_order.append('std')
        elif stripped.startswith('use crate::') or stripped.startswith('use apas_ai::'):
            current_order.append('internal')
        elif stripped.startswith('use ') and not stripped.startswith('use self::') and not stripped.startswith('use super::'):
            current_order.append('external')
    
    # Check expected order
    expected_order = []
    if std_imports:
        expected_order.extend(['std'] * len(std_imports))
    if external_imports:
        expected_order.extend(['external'] * len(external_imports))
    if internal_imports:
        expected_order.extend(['internal'] * len(internal_imports))
    
    if current_order == expected_order:
        # Still need to check blank lines
        needs_fix = False
        check_idx = first_use_idx
        for idx in range(first_use_idx, last_use_idx + 1):
            line = lines[idx].strip()
            if line.startswith('use '):
                if line.startswith('use std::') or line.startswith('use core::') or line.startswith('use alloc::'):
                    section = 'std'
                elif line.startswith('use crate::') or line.startswith('use apas_ai::'):
                    section = 'internal'
                else:
                    section = 'external'
                
                # Check if previous section ended and we need blank line
                if idx > first_use_idx and not lines[idx - 1].strip():
                    # There's already a blank line
                    pass
                elif idx > first_use_idx:
                    prev_line = lines[idx - 1].strip()
                    if prev_line.startswith('use '):
                        # Determine previous section
                        if prev_line.startswith('use std::') or prev_line.startswith('use core::') or prev_line.startswith('use alloc::'):
                            prev_section = 'std'
                        elif prev_line.startswith('use crate::') or prev_line.startswith('use apas_ai::'):
                            prev_section = 'internal'
                        else:
                            prev_section = 'external'
                        
                        # If section changed, we need a blank line
                        if section != prev_section:
                            needs_fix = True
                            break
        
        if not needs_fix:
            return False  # Already correct
    
    # Build new import block
    new_import_lines = []
    
    # Add std imports
    if std_imports:
        new_import_lines.extend(std_imports)
        if external_imports or internal_imports:
            new_import_lines.append('\n')  # Blank line after std
    
    # Add external imports
    if external_imports:
        new_import_lines.extend(external_imports)
        if internal_imports:
            new_import_lines.append('\n')  # Blank line after external
    
    # Add internal imports (Types first, then rest)
    if internal_imports:
        types_imports = [imp for imp in internal_imports if '::Types::' in imp]
        other_imports = [imp for imp in internal_imports if '::Types::' not in imp]
        new_import_lines.extend(types_imports)
        new_import_lines.extend(other_imports)
    
    # Replace import block
    new_lines = lines[:first_use_idx] + new_import_lines + lines[last_use_idx + 1:]
    
    if dry_run:
        return True  # Would be fixed
    
    # Write back
    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(new_lines)
    
    return True  # Fixed


def main():
    import argparse
    parser = argparse.ArgumentParser(description='Fix import order and blank lines')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be fixed without changing files')
    parser.add_argument('--file', type=str, help='Fix specific file (for testing)')
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent
    
    if args.file:
        # Test mode - single file
        file_path = repo_root / args.file
        if not file_path.exists():
            print(f"✗ File not found: {args.file}")
            return 1
        
        print(f"{'Testing' if args.dry_run else 'Fixing'}: {args.file}")
        if fix_file_imports(file_path, repo_root, args.dry_run):
            print(f"✓ {'Would fix' if args.dry_run else 'Fixed'}: {args.file}")
        else:
            print(f"○ Already correct: {args.file}")
        return 0
    
    # Full run
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    fixed = []
    already_correct = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rust_file in search_dir.rglob("*.rs"):
            if fix_file_imports(rust_file, repo_root, args.dry_run):
                fixed.append(rust_file)
            else:
                already_correct.append(rust_file)
    
    if fixed:
        print(f"{'Would fix' if args.dry_run else 'Fixed'} {len(fixed)} files:")
        for f in fixed[:10]:  # Show first 10
            print(f"  {f.relative_to(repo_root)}")
        if len(fixed) > 10:
            print(f"  ... and {len(fixed) - 10} more")
    
    if already_correct:
        print(f"\n{len(already_correct)} files already correct")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

