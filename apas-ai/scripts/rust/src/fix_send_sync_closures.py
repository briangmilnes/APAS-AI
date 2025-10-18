#!/usr/bin/env python3
"""Remove Send + Sync from closure bounds (use only for St* files).
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

WARNING: Only use this on single-threaded (StEph/StPer) files!
Send + Sync is CORRECT for multi-threaded (MtEph/MtPer) files.

Usage:
  fix_send_sync_closures.py <file.rs>          # Fix specific file
  fix_send_sync_closures.py <file.rs> --dry-run  # Show changes without applying
  fix_send_sync_closures.py <file.rs> --log_file <path>  # Custom log path
"""

import argparse
import re
import sys
from pathlib import Path


class TeeOutput:
    """Write to both stdout and a log file."""
    def __init__(self, log_path):
        self.log_path = Path(log_path)
        self.log_path.parent.mkdir(parents=True, exist_ok=True)
        self.log_file = open(self.log_path, 'a', encoding='utf-8')  # Append mode
    
    def write(self, text):
        print(text, end='', flush=True)
        self.log_file.write(text)
        self.log_file.flush()
    
    def print(self, text=''):
        self.write(text + '\n')
    
    def close(self):
        self.log_file.close()


def fix_send_sync_closures(file_path, dry_run=False, tee=None):
    """Remove Send + Sync from closure bounds."""
    def log(msg):
        if tee:
            tee.print(msg)
        else:
            print(msg)
    
    # Safety check
    if any(x in file_path.name for x in ['MtEph', 'MtPer']):
        log(f"WARNING: {file_path.name} is a multi-threaded file (MtEph/MtPer)!")
        log(f"Send + Sync is likely CORRECT here. Skipping for safety.")
        return False
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        log(f"Error reading {file_path}: {e}")
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
            log(f"No changes needed in {file_path.name}")
        return False
    
    if dry_run:
        # Show diff
        lines_old = original.split('\n')
        lines_new = content.split('\n')
        for i, (old, new) in enumerate(zip(lines_old, lines_new), start=1):
            if old != new:
                log(f"Line {i}:")
                log(f"  - {old}")
                log(f"  + {new}")
        log(f"\nWould fix {file_path.name}")
        return True
    
    # Write back
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        log(f"✓ Fixed {file_path.name}")
        return True
    except Exception as e:
        log(f"Error writing {file_path}: {e}")
        return False


def main():
    parser = argparse.ArgumentParser(description='Remove Send + Sync from closure bounds')
    parser.add_argument('file', type=Path, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show changes without applying')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/fix_send_sync_closures.txt',
                       help='Path to log file (default: analyses/code_review/fix_send_sync_closures.txt)')
    args = parser.parse_args()
    
    file_path = args.file
    dry_run = args.dry_run
    
    if not file_path.exists():
        print(f"Error: File not found: {file_path}", file=sys.stderr)
        return 1
    
    project_root = Path(__file__).parent.parent.parent.parent
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    if dry_run:
        tee.print(f"Dry run mode - showing changes without applying...\n")
    
    tee.print("WARNING: Only use on single-threaded (StEph/StPer) files!")
    tee.print("Send + Sync is CORRECT for multi-threaded (MtEph/MtPer) files.\n")
    
    tee.print(f"{'='*60}")
    tee.print(f"fix_send_sync_closures.py: {file_path}")
    tee.print(f"{'='*60}")
    
    success = fix_send_sync_closures(file_path, dry_run=dry_run, tee=tee)
    
    tee.print(f"{'='*60}\n")
    tee.close()
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())
