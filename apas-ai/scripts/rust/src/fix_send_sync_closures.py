#!/usr/bin/env python3
"""Remove Send + Sync from closure bounds (use only for St* files).
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

WARNING: Only use this on single-threaded (StEph/StPer) files!
Send + Sync is CORRECT for multi-threaded (MtEph/MtPer) files.

Usage:
  fix_send_sync_closures.py <file.rs>          # Fix specific file
  fix_send_sync_closures.py <file.rs> --dry-run  # Show changes without applying
"""

import re
import sys
from pathlib import Path


def fix_send_sync_closures(file_path, dry_run=False):
    """Remove Send + Sync from closure bounds."""
    # Safety check
    if any(x in file_path.name for x in ['MtEph', 'MtPer']):
        print(f"WARNING: {file_path.name} is a multi-threaded file (MtEph/MtPer)!")
        print(f"Send + Sync is likely CORRECT here. Skipping for safety.")
        return False
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    original = content
    
    # Remove "+ Send + Sync" from function bounds
    content = re.sub(r'\s*\+\s*Send\s*\+\s*Sync\b', '', content)
    content = re.sub(r'\s*\+\s*Sync\s*\+\s*Send\b', '', content)
    
    # Also handle "'static + Send + Sync"
    content = re.sub(r"\s*\+\s*Send\s*\+\s*Sync\s*\+\s*'static\b", " + 'static", content)
    content = re.sub(r"'static\s*\+\s*Send\s*\+\s*Sync\b", "'static", content)
    
    if content == original:
        if dry_run:
            print(f"No changes needed in {file_path.name}")
        return False
    
    if dry_run:
        # Show diff
        lines_old = original.split('\n')
        lines_new = content.split('\n')
        for i, (old, new) in enumerate(zip(lines_old, lines_new), start=1):
            if old != new:
                print(f"Line {i}:")
                print(f"  - {old}")
                print(f"  + {new}")
        print(f"\nWould fix {file_path.name}")
        return True
    
    # Write back
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"✓ Fixed {file_path.name}")
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    if len(sys.argv) < 2:
        print("Usage: fix_send_sync_closures.py <file.rs> [--dry-run]")
        print("\nWARNING: Only use on single-threaded (StEph/StPer) files!")
        print("Send + Sync is CORRECT for multi-threaded (MtEph/MtPer) files.")
        return 1
    
    file_path = Path(sys.argv[1])
    dry_run = '--dry-run' in sys.argv
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}", file=sys.stderr)
        return 1
    
    if dry_run:
        print(f"Dry run mode - showing changes without applying...\n")
    
    success = fix_send_sync_closures(file_path, dry_run=dry_run)
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())

