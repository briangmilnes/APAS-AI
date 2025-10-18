#!/usr/bin/env python3
"""Detect Clone + Display bounds that should be StT.
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Finds type bounds like "L: Clone + Display + Debug + Eq" that should use StT.
"""

import re
import sys
from pathlib import Path


def detect_clone_display(file_path):
    """Detect lines with Clone + Display that should be StT."""
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
        
        # Look for Clone + Display patterns (both orders)
        if re.search(r'\bClone\s*\+\s*Display\b', line) or \
           re.search(r'\bDisplay\s*\+\s*Clone\b', line):
            # Check if it's in a declaration
            if any(kw in line for kw in ['struct ', 'trait ', 'enum ', 'impl<', 'impl ']):
                issues.append({
                    'line': line_num,
                    'content': line.strip()
                })
    
    return issues


def main():
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / "src"
    
    print("Scanning for Clone + Display bounds that should be StT...\n")
    
    all_issues = {}
    total = 0
    
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if rs_file.name == "Types.rs":
            continue
            
        issues = detect_clone_display(rs_file)
        if issues:
            all_issues[rs_file] = issues
            total += len(issues)
    
    if not all_issues:
        print("✓ No Clone + Display patterns found!")
        return 0
    
    print(f"Found {len(all_issues)} files with Clone + Display:\n")
    
    for file_path, issues in all_issues.items():
        rel_path = file_path.relative_to(project_root)
        print(f"{rel_path}: {len(issues)} issues")
        for issue in issues:
            print(f"  Line {issue['line']}: {issue['content'][:80]}")
        print()
    
    print(f"Total: {total} lines with Clone + Display")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

