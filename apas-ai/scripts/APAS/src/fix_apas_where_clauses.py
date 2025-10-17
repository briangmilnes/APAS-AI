#!/usr/bin/env python3
"""
Fix: APAS where clause simplification.

APASRules.md Lines 96-101: Replace Fn(&T) -> B with Pred<T>.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def fix_file(file_path, dry_run=False):
    """Fix APAS where clauses in a single file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Determine if this is a single-threaded or multi-threaded file
    file_name = file_path.name
    is_mt = 'Mt' in file_name or 'Slice' in file_name  # MtEphSlice uses MT bounds
    pred_type = 'PredMt' if is_mt else 'PredSt'
    
    # Pattern: Fn(&T) -> B  followed by threading bounds (Send/Sync/'static)
    # We want to replace the entire pattern including threading bounds
    # But preserve other bounds like Clone
    #
    # Strategy: Match "Fn(&T) -> B" and then optionally match threading-specific bounds
    # in any order, but stop before we hit other bounds like Clone or >
    
    # For Mt files: Replace "Fn(&T) -> B + Send + Sync + 'static" with "PredMt<T>"
    # For St files: Replace "Fn(&T) -> B" with "PredSt<T>"
    
    if is_mt:
        # Match: Fn(&T) -> B followed by (in any order): Send, Sync, 'static
        # This pattern captures the threading bounds to remove them
        pattern = re.compile(
            r'\bFn\s*\(\s*&\s*(\w+)\s*\)\s*->\s*B'  # Core pattern
            r'(?:\s*\+\s*(?:Send|Sync|\'static))*'  # Threading bounds (greedy, but OK for Mt)
        )
    else:
        # For St files, just match the core pattern
        pattern = re.compile(r'\bFn\s*\(\s*&\s*(\w+)\s*\)\s*->\s*B\b')
    
    def replace_pred(match):
        type_param = match.group(1)
        return f'{pred_type}<{type_param}>'
    
    content = pattern.sub(replace_pred, content)
    
    # Cleanup: Remove redundant '+ 'static' after PredMt (which already includes it)
    if is_mt:
        # Remove "+ Clone + 'static" -> "+ Clone" (PredMt already has 'static)
        content = re.sub(r'(PredMt<\w+>)\s*\+\s*Clone\s*\+\s*\'static\b', r'\1 + Clone', content)
        # Remove just "+ 'static" after PredMt
        content = re.sub(r'(PredMt<\w+>)\s*\+\s*\'static\b', r'\1', content)
    
    changed = content != original_content
    
    if changed and not dry_run:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
    
    return changed


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Fix APAS where clauses.")
    parser.add_argument('--file', type=str, help="Specify a single file to fix.")
    parser.add_argument('--dry-run', action='store_true', help="Show changes without writing.")
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    src_dir = repo_root / "src"
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.exists():
            print(f"Error: File not found at {file_path}")
            return 1
        
        if fix_file(file_path, args.dry_run):
            rel_path = file_path.relative_to(repo_root) if file_path.is_relative_to(repo_root) else file_path
            if args.dry_run:
                print(f"Would fix: {rel_path}")
            else:
                print(f"✓ Fixed: {rel_path}")
        else:
            print(f"✓ No changes needed: {file_path}")
        return 0
    
    # Fix all files
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    fixed_count = 0
    
    for src_file in src_dir.rglob("*.rs"):
        # Skip Types.rs - it defines the traits
        if src_file.name == "Types.rs":
            continue
            
        if fix_file(src_file, args.dry_run):
            fixed_count += 1
            rel_path = src_file.relative_to(repo_root)
            if not args.dry_run:
                print(f"✓ Fixed: {rel_path}")
    
    if args.dry_run:
        print(f"\nWould fix {fixed_count} files")
    else:
        print(f"\n✓ Fixed {fixed_count} files with APAS where clause simplifications")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

