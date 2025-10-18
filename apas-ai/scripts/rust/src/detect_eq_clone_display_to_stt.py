#!/usr/bin/env python3
"""Detect Eq + Clone + Display/Debug combinations that should be StT.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Finds more complete manual bounds like "T: Eq + Clone + Display + Debug" that should use StT.
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


def detect_eq_clone_display(file_path):
    """Detect lines with Eq + Clone + Display/Debug that should be StT."""
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
        
        # Look for combinations that include Eq, Clone, and either Display or Debug
        # These are strong indicators of manual StT bounds
        has_eq = re.search(r'\bEq\b', line)
        has_clone = re.search(r'\bClone\b', line)
        has_display_or_debug = re.search(r'\b(?:Display|Debug)\b', line)
        
        if has_eq and has_clone and has_display_or_debug:
            # Check if it's in a declaration
            if any(kw in line for kw in ['struct ', 'trait ', 'enum ', 'impl<', 'impl ', 'where ']):
                # Make sure it's not already using StT
                if 'StT' not in line:
                    issues.append({
                        'line': line_num,
                        'content': line.strip()
                    })
    
    return issues


def main():
    parser = argparse.ArgumentParser(description='Detect Eq + Clone + Display/Debug bounds that should be StT')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/detect_eq_clone_display_to_stt.txt',
                       help='Path to log file (default: analyses/code_review/detect_eq_clone_display_to_stt.txt)')
    args = parser.parse_args()
    
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / "src"
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    tee.print("Scanning for Eq + Clone + Display/Debug bounds that should be StT...\n")
    
    all_issues = {}
    total = 0
    
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if rs_file.name == "Types.rs":
            continue
            
        issues = detect_eq_clone_display(rs_file)
        if issues:
            all_issues[rs_file] = issues
            total += len(issues)
    
    if not all_issues:
        tee.print("✓ No Eq + Clone + Display/Debug patterns found!")
        tee.close()
        return 0
    
    tee.print(f"Found {len(all_issues)} files with Eq + Clone + Display/Debug:\n")
    
    for file_path, issues in all_issues.items():
        rel_path = file_path.relative_to(project_root)
        tee.print(f"{rel_path}: {len(issues)} issues")
        for issue in issues:
            tee.print(f"  Line {issue['line']}: {issue['content'][:80]}")
        tee.print()
    
    tee.print(f"Total: {total} lines with manual bounds")
    tee.print(f"\nLog written to: {log_path}")
    tee.close()
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

