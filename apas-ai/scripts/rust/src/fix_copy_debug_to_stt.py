#!/usr/bin/env python3
"""Fix Copy/Clone + Debug bounds to use StT.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Usage:
  fix_copy_debug_to_stt.py <file.rs>          # Fix specific file
  fix_copy_debug_to_stt.py <file.rs> --dry-run  # Show changes without applying
  fix_copy_debug_to_stt.py <file.rs> --log_file <path>  # Custom log path
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


def fix_copy_debug_to_stt(file_path, dry_run=False, tee=None):
    """Replace Copy/Clone + Debug with StT."""
    def log(msg):
        if tee:
            tee.print(msg)
        else:
            print(msg)
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        log(f"Error reading {file_path}: {e}")
        return False
    
    original = content
    
    # Pattern 1: "T: Copy + Debug" or "T: Clone + Debug" → "T: StT"
    content = re.sub(r'\b(\w+):\s*(?:Copy|Clone)\s*\+\s*Debug\b', r'\1: StT', content)
    
    # Pattern 2: "T: Debug + Copy" or "T: Debug + Clone" → "T: StT"
    content = re.sub(r'\b(\w+):\s*Debug\s*\+\s*(?:Copy|Clone)\b', r'\1: StT', content)
    
    # Pattern 3: Handle more complex cases with additional traits
    # "T: Copy + Debug + Eq" → "T: StT" (StT already includes Eq)
    content = re.sub(r'\b(\w+):\s*(?:Copy|Clone)\s*\+\s*Debug\s*\+\s*(?:Eq|Clone)\b', r'\1: StT', content)
    content = re.sub(r'\b(\w+):\s*Eq\s*\+\s*(?:Copy|Clone)\s*\+\s*Debug\b', r'\1: StT', content)
    
    # Pattern 4: "T: Display + Copy + Debug" → "T: StT"
    content = re.sub(r'\b(\w+):\s*Display\s*\+\s*(?:Copy|Clone)\s*\+\s*Debug\b', r'\1: StT', content)
    content = re.sub(r'\b(\w+):\s*(?:Copy|Clone)\s*\+\s*Debug\s*\+\s*Display\b', r'\1: StT', content)
    
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
    parser = argparse.ArgumentParser(description='Fix Copy/Clone + Debug bounds to use StT')
    parser.add_argument('file', type=Path, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show changes without applying')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/fix_copy_debug_to_stt.txt',
                       help='Path to log file (default: analyses/code_review/fix_copy_debug_to_stt.txt)')
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
    
    tee.print(f"{'='*60}")
    tee.print(f"fix_copy_debug_to_stt.py: {file_path}")
    tee.print(f"{'='*60}")
    
    success = fix_copy_debug_to_stt(file_path, dry_run=dry_run, tee=tee)
    
    tee.print(f"{'='*60}\n")
    tee.close()
    
    return 0 if success else 1


if __name__ == "__main__":
    sys.exit(main())

