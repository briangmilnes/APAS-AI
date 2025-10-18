#!/usr/bin/env python3
"""Fix Clone + Display bounds to use StT.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Usage:
  fix_clone_display_to_stt.py <file.rs>          # Fix specific file
  fix_clone_display_to_stt.py <file.rs> --dry-run  # Show changes without applying
"""

import re
import sys
from pathlib import Path


def fix_clone_display_to_stt(file_path, dry_run=False):
    """Replace Clone + Display + Debug + Eq + Hash combinations with StT + Hash."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    original = content
    
    # Pattern: "L: Clone + Display + Debug + Eq + Hash" → "L: StT + Hash"
    # StT = Eq + Clone + Display + Debug + Sized, so we keep Hash
    content = re.sub(
        r'\b(\w+):\s*Clone\s*\+\s*Display\s*\+\s*Debug\s*\+\s*Eq\s*\+\s*Hash\b',
        r'\1: StT + Hash',
        content
    )
    
    # Other orders
    content = re.sub(
        r'\b(\w+):\s*Eq\s*\+\s*Clone\s*\+\s*Display\s*\+\s*Debug\s*\+\s*Hash\b',
        r'\1: StT + Hash',
        content
    )
    
    # Simpler: "L: Clone + Display + Debug + Eq" → "L: StT"
    content = re.sub(
        r'\b(\w+):\s*Clone\s*\+\s*Display\s*\+\s*Debug\s*\+\s*Eq\b',
        r'\1: StT',
        content
    )
    
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
        print("Usage: fix_clone_display_to_stt.py <file.rs> [--dry-run]")
        return 1
    
    file_path = Path(sys.argv[1])
    dry_run = '--dry-run' in sys.argv
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}", file=sys.stderr)
        return 1
    
    if dry_run:
        print(f"Dry run mode - showing changes without applying...\n")
    
    success = fix_clone_display_to_stt(file_path, dry_run=dry_run)
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())

