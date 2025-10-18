#!/usr/bin/env python3
"""Detect manual bounds (Copy + Debug, etc.) instead of trait aliases (StT, MtT, etc.)
Git commit: 08cec0603b305aa07307724314ae2656d8597279
Date: 2025-10-18

Finds impl blocks, struct defs, and trait defs that use manual combinations of
standard bounds instead of the project's trait aliases (StT, MtT, StTInMtT).
"""

import re
import sys
from pathlib import Path
from collections import defaultdict


def detect_manual_bounds(file_path):
    """Detect lines with manual bounds that should use trait aliases."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return []

    issues = []
    lines = content.split('\n')
    
    # Patterns that suggest manual bounds instead of trait aliases
    # StT = Eq + Clone + Display + Debug + Sized
    # MtT = Send + Sync + Clone + Display + Debug + Sized + 'static
    manual_patterns = [
        # Common combinations that should be StT
        (r'\b(?:Copy|Clone)\s*\+\s*Debug\b', 'Copy/Clone + Debug (should use StT or MtT)'),
        (r'\bDebug\s*\+\s*(?:Copy|Clone)\b', 'Debug + Copy/Clone (should use StT or MtT)'),
        (r'\bClone\s*\+\s*Display\b', 'Clone + Display (should use StT or MtT)'),
        (r'\bDisplay\s*\+\s*Clone\b', 'Display + Clone (should use StT or MtT)'),
        (r'\bEq\s*\+\s*Clone\s*\+\s*(?:Debug|Display)\b', 'Eq + Clone + Debug/Display (should use StT)'),
        (r'\bClone\s*\+\s*Eq\s*\+\s*(?:Debug|Display)\b', 'Clone + Eq + Debug/Display (should use StT)'),
        # Common combinations that should be MtT
        (r'\bSend\s*\+\s*Sync\b', 'Send + Sync (likely should use MtT)'),
        (r'\bSync\s*\+\s*Send\b', 'Sync + Send (likely should use MtT)'),
    ]
    
    for line_num, line in enumerate(lines, start=1):
        # Skip comments
        if line.strip().startswith('//'):
            continue
            
        # Check if it's in a struct, trait, enum, or impl declaration
        is_declaration = any(keyword in line for keyword in [
            'struct ', 'trait ', 'enum ', 'impl<', 'impl ', 'fn ', 'pub fn ', 'where '
        ])
        
        if not is_declaration:
            continue
        
        # Check for manual bound patterns
        for pattern, description in manual_patterns:
            if re.search(pattern, line):
                issues.append({
                    'line': line_num,
                    'content': line.strip(),
                    'pattern': description
                })
                break  # Only report once per line
    
    return issues


def main():
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / "src"
    
    print("Scanning for manual bounds instead of trait aliases (StT, MtT)...\n")
    
    all_issues = {}
    pattern_counts = defaultdict(int)
    
    # Find all .rs files
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if rs_file.name == "Types.rs":
            continue
            
        issues = detect_manual_bounds(rs_file)
        if issues:
            all_issues[rs_file] = issues
            for issue in issues:
                pattern_counts[issue['pattern']] += 1
    
    if not all_issues:
        print("✓ No manual bounds found!")
        return 0
    
    # Report findings
    print(f"Found {len(all_issues)} files with manual bounds:\n")
    
    for file_path, issues in all_issues.items():
        rel_path = file_path.relative_to(project_root)
        print(f"{rel_path}")
        
        for issue in issues:
            print(f"  Line {issue['line']}: {issue['pattern']}")
            print(f"    {issue['content']}")
        print()
    
    print(f"\nPattern breakdown:")
    for pattern, count in sorted(pattern_counts.items(), key=lambda x: -x[1]):
        print(f"  {count:3d}x {pattern}")
    
    print(f"\nTotal: {len(all_issues)} files")
    print(f"Total: {sum(len(issues) for issues in all_issues.values())} problematic lines")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

