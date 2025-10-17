#!/usr/bin/env python3
"""
Fix: MT module discipline - use MtT instead of StT + Send + Sync.

APASRules.md Lines 44-47: Files with Mt in their name must use MtT for element types,
not StT with threading bounds bolted on.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def fix_file(file_path, dry_run=False):
    """Fix MT discipline violations in a single file."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Strategy: Replace "StT + Send + Sync" with "StTInMtT"
    # But preserve other bounds like Clone, 'static, Ord that aren't part of StTInMtT
    # StTInMtT = StT + Send + Sync (includes Eq + Clone + Display + Debug + Sized + Send + Sync)
    
    # Pattern: Match "StT + Send + Sync" but capture what comes after
    # We need to handle different orderings: StT + Send + Sync, StT + Sync + Send, etc.
    
    def replace_stt_mt(match):
        """Replace StT + Send + Sync with StTInMtT, preserving other bounds."""
        # Get the full match
        full_text = match.group(0)
        
        # Remove StT, Send, Sync from the text (these are subsumed by MtT)
        cleaned = full_text
        for to_remove in ['StT', 'Send', 'Sync']:
            # Remove "word + " pattern
            cleaned = re.sub(r'\b' + to_remove + r'\s*\+\s*', '', cleaned)
            # Remove " + word" pattern
            cleaned = re.sub(r'\s*\+\s*\b' + to_remove + r'\b', '', cleaned)
            # Remove standalone word
            cleaned = re.sub(r'^\s*' + to_remove + r'\s*$', '', cleaned)
        
        # Clean up any leftover multiple + signs
        cleaned = re.sub(r'\+\s*\+', '+', cleaned)
        # Clean up leading/trailing + and whitespace
        cleaned = re.sub(r'^\s*\+\s*', '', cleaned)
        cleaned = re.sub(r'\s*\+\s*$', '', cleaned)
        cleaned = cleaned.strip()
        
        if cleaned:
            # There are other bounds to preserve
            return f'StTInMtT + {cleaned}'
        else:
            return 'StTInMtT'
    
    # Match StT with Send and Sync (with other bounds possibly in between)
    # Pattern needs to handle: StT + Ord + Send + Sync, StT + Send + Sync, StT + Send, etc.
    # We match StT followed by any combination of bounds that includes Send (and optionally Sync)
    
    # Pattern 1: StT with both Send and Sync
    pattern1 = re.compile(
        r'StT(?:\s*\+\s*\w+)*?\s*\+\s*(?:Send|Sync)(?:\s*\+\s*\w+)*?\s*\+\s*(?:Sync|Send)'
        r'(?:\s*\+\s*(?:Clone|\'static|Ord|Hash|Copy|Default|PartialOrd))*'
    )
    content = pattern1.sub(replace_stt_mt, content)
    
    # Pattern 2: StT with Send but not Sync (add Sync)
    # Match: StT + ... + Send (but NOT followed by Sync)
    pattern2 = re.compile(
        r'(StT(?:\s*\+\s*(?:Clone|\'static|Ord|Hash|Copy|Default|PartialOrd))*?\s*\+\s*Send)'
        r'(?!\s*\+\s*Sync)'  # Negative lookahead: not followed by Sync
    )
    # For these, we need to add + Sync before replacing
    def add_sync(match):
        return match.group(1) + ' + Sync'
    content = pattern2.sub(add_sync, content)
    
    # Now run pattern1 again to catch the newly fixed ones
    content = pattern1.sub(replace_stt_mt, content)
    
    changed = content != original_content
    
    if changed and not dry_run:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
    
    return changed


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Fix MT discipline violations.")
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
    
    # Fix all *Mt* files
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    fixed_count = 0
    
    for src_file in src_dir.rglob("*.rs"):
        # Only process *Mt* files
        if 'Mt' not in src_file.name:
            continue
            
        if fix_file(src_file, args.dry_run):
            fixed_count += 1
            rel_path = src_file.relative_to(repo_root)
            if not args.dry_run:
                print(f"✓ Fixed: {rel_path}")
    
    if args.dry_run:
        print(f"\nWould fix {fixed_count} files")
    else:
        print(f"\n✓ Fixed {fixed_count} *Mt* files with MT discipline violations")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

