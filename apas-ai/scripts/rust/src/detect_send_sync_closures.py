#!/usr/bin/env python3
"""Detect Send + Sync on closure bounds (often correct, but good to review).
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Finds closure/function bounds like "F: Fn(...) -> T + Send + Sync".
These are often correct for multi-threaded code, but worth documenting.
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
        self.log_file = open(self.log_path, 'w', encoding='utf-8')
    
    def write(self, text):
        print(text, end='', flush=True)
        self.log_file.write(text)
        self.log_file.flush()
    
    def print(self, text=''):
        self.write(text + '\n')
    
    def close(self):
        self.log_file.close()


def detect_send_sync_closures(file_path):
    """Detect lines with Send + Sync on closure bounds."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return []

    issues = []
    lines = content.split('\n')
    
    for line_num, line in enumerate(lines, start=1):
        # Skip comments
        if line.strip().startswith('//'):
            continue
        
        # Look for "Fn(...) -> ... + Send + Sync" pattern
        if re.search(r'\bFn\s*\([^)]*\)\s*->\s*\w+\s*\+\s*Send\s*\+\s*Sync\b', line) or \
           re.search(r'\bFn\s*\([^)]*\)\s*->\s*\w+\s*\+\s*Sync\s*\+\s*Send\b', line):
            # Check if it's in a declaration
            if any(kw in line for kw in ['fn ', 'pub fn ', 'trait ', 'where ']):
                issues.append({
                    'line': line_num,
                    'content': line.strip(),
                    'note': 'Send + Sync on closures (usually correct for MtT code)'
                })
    
    return issues


def main():
    parser = argparse.ArgumentParser(description='Detect Send + Sync on closure bounds')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/detect_send_sync_closures.txt',
                       help='Path to log file (default: analyses/code_review/detect_send_sync_closures.txt)')
    args = parser.parse_args()
    
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / "src"
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    tee.print("Scanning for Send + Sync on closure bounds...\n")
    tee.print("Note: These are often CORRECT for multi-threaded code.")
    tee.print("Only fix if the file is single-threaded (StEph, StPer).\n")
    
    all_issues = {}
    total = 0
    mt_files = 0
    st_files = 0
    
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if rs_file.name == "Types.rs":
            continue
            
        issues = detect_send_sync_closures(rs_file)
        if issues:
            all_issues[rs_file] = issues
            total += len(issues)
            
            # Classify file
            if 'MtEph' in rs_file.name or 'MtPer' in rs_file.name:
                mt_files += 1
            elif 'StEph' in rs_file.name or 'StPer' in rs_file.name:
                st_files += 1
    
    if not all_issues:
        tee.print("✓ No Send + Sync on closures found!")
        tee.close()
        return 0
    
    tee.print(f"Found {len(all_issues)} files with Send + Sync on closures:\n")
    tee.print(f"  {mt_files} multi-threaded files (MtEph/MtPer) - likely CORRECT")
    tee.print(f"  {st_files} single-threaded files (StEph/StPer) - may need review")
    tee.print(f"  {len(all_issues) - mt_files - st_files} other files\n")
    
    for file_path, issues in all_issues.items():
        rel_path = file_path.relative_to(project_root)
        file_type = "Mt" if any(x in file_path.name for x in ['MtEph', 'MtPer']) else "St" if any(x in file_path.name for x in ['StEph', 'StPer']) else "?"
        tee.print(f"{rel_path} [{file_type}]: {len(issues)} closures")
    
    tee.print(f"\nTotal: {total} closures with Send + Sync")
    tee.print(f"\nRecommendation: Send + Sync on closures is usually correct for Mt* files.")
    tee.print(f"Only remove from St* files if the parallel operations were converted to sequential.")
    tee.print(f"\nLog written to: {log_path}")
    tee.close()
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

