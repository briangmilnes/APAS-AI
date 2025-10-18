#!/usr/bin/env python3
"""Detect contradictory trait bounds (StT + MtT together) in struct/trait/impl definitions.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18
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


def detect_contradictory_bounds(file_path):
    """Detect lines with both StT and MtT bounds (contradictory)."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return []

    issues = []
    lines = content.split('\n')
    
    # Determine expected bound based on file naming convention
    file_name = file_path.name
    if 'MtEph' in file_name or 'MtPer' in file_name:
        expected = 'MtT'
        wrong = 'StT'
    elif 'StEph' in file_name or 'StPer' in file_name:
        expected = 'StT'
        wrong = 'MtT'
    else:
        expected = None
        wrong = None

    for line_num, line in enumerate(lines, start=1):
        # Look for trait bounds containing both StT and MtT
        # Match patterns like: "V: StT + MtT" or "T: MtT + StT + Hash"
        if re.search(r'\b(StT|StTInMtT)\b.*\bMtT\b', line) or re.search(r'\bMtT\b.*\b(StT|StTInMtT)\b', line):
            # Check if it's in a struct, trait, or impl declaration
            if any(keyword in line for keyword in ['struct ', 'trait ', 'impl<', 'impl ']):
                issues.append({
                    'line': line_num,
                    'content': line.strip(),
                    'expected': expected,
                    'wrong': wrong
                })
    
    return issues


def main():
    parser = argparse.ArgumentParser(description='Detect contradictory trait bounds (StT + MtT)')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/detect_contradictory_bounds.txt',
                       help='Path to log file (default: analyses/code_review/detect_contradictory_bounds.txt)')
    args = parser.parse_args()
    
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / "src"
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    tee.print("Scanning for contradictory trait bounds (StT + MtT)...\n")
    
    all_issues = {}
    
    # Find all .rs files
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if rs_file.name == "Types.rs":
            continue
            
        issues = detect_contradictory_bounds(rs_file)
        if issues:
            all_issues[rs_file] = issues
    
    if not all_issues:
        tee.print("✓ No contradictory bounds found!")
        tee.close()
        return 0
    
    # Report findings
    tee.print(f"Found {len(all_issues)} files with contradictory bounds:\n")
    
    for file_path, issues in all_issues.items():
        rel_path = file_path.relative_to(project_root)
        tee.print(f"{rel_path}")
        
        for issue in issues:
            tee.print(f"  Line {issue['line']}: {issue['content']}")
            if issue['expected']:
                tee.print(f"    → File naming suggests: use {issue['expected']} only, remove {issue['wrong']}")
        tee.print()
    
    tee.print(f"\nTotal: {len(all_issues)} files with contradictory bounds")
    tee.print(f"Total: {sum(len(issues) for issues in all_issues.values())} problematic lines")
    tee.print(f"\nLog written to: {log_path}")
    tee.close()
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

