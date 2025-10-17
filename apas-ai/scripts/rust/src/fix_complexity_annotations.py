#!/usr/bin/env python3
"""
Fix: Simplify double-line complexity annotations.

Converts:
    /// APAS: Work Θ(1), Span Θ(1)
    /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)

To:
    /// APAS: Work Θ(1), Span Θ(1), claude agrees

Or if they disagree:
    /// APAS: Work Θ(1), Span Θ(1), claude disagrees: Work Θ(x), Span Θ(y), Parallelism Θ(z)
"""

import re
import sys
from pathlib import Path


def extract_complexity(line):
    """
    Extract Work and Span from a complexity annotation line.
    Returns (work, span, full_line_content)
    """
    # Match Work complexity
    work_match = re.search(r'Work\s+([OΘΩ]\([^)]+\))', line)
    work = work_match.group(1) if work_match else None
    
    # Match Span complexity
    span_match = re.search(r'Span\s+([OΘΩ]\([^)]+\))', line)
    span = span_match.group(1) if span_match else None
    
    return (work, span, line.strip())


def simplify_annotations(lines, is_mt_file):
    """
    Simplify consecutive APAS and claude-4-sonet complexity annotations.
    Returns modified lines and count of changes.
    
    Args:
        lines: File lines to process
        is_mt_file: True if this is a *Mt* file (allows Parallelism annotations)
    """
    new_lines = []
    changes = 0
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if this is an APAS complexity line
        if '/// APAS:' in line and 'Work' in line:
            # Check if next line is claude-4-sonet
            if i + 1 < len(lines) and '/// claude-4-sonet:' in lines[i + 1]:
                apas_line = line
                claude_line = lines[i + 1]
                
                # Extract complexities
                apas_work, apas_span, _ = extract_complexity(apas_line)
                claude_work, claude_span, _ = extract_complexity(claude_line)
                
                # Get the indentation from APAS line
                indent_match = re.match(r'^(\s*)', apas_line)
                indent = indent_match.group(1) if indent_match else ''
                
                # Check if they agree (comparing Work and Span only)
                if apas_work == claude_work and apas_span == claude_span:
                    # They agree
                    simplified = f"{indent}/// APAS: Work {apas_work}, Span {apas_span}, claude agrees\n"
                else:
                    # They disagree - extract full claude complexity
                    # Get everything after "claude-4-sonet: " 
                    claude_content_match = re.search(r'claude-4-sonet:\s+(.+)$', claude_line)
                    if claude_content_match:
                        claude_full = claude_content_match.group(1).strip()
                        # Strip out trailing comments (everything after " - ")
                        claude_full = re.sub(r'\s+-\s+.*$', '', claude_full)
                        
                        # Parallelism annotations only allowed in *Mt* files
                        if not is_mt_file:
                            # Remove Parallelism annotation entirely from non-Mt files
                            claude_full = re.sub(r',\s*Parallelism\s+[OΘΩ]\([^)]+\)', '', claude_full)
                        else:
                            # In Mt files, abbreviate Parallelism to Par
                            claude_full = claude_full.replace('Parallelism', 'Par')
                        
                        simplified = f"{indent}/// APAS: Work {apas_work}, Span {apas_span}, claude disagrees: {claude_full}\n"
                    else:
                        # Fallback if pattern doesn't match
                        simplified = f"{indent}/// APAS: Work {apas_work}, Span {apas_span}, claude agrees\n"
                
                new_lines.append(simplified)
                changes += 1
                i += 2  # Skip both lines
                continue
        
        # Not a double annotation, keep the line as-is
        new_lines.append(line)
        i += 1
    
    return new_lines, changes


def fix_file(file_path, dry_run=False):
    """
    Fix complexity annotations in a file.
    Returns True if changes were made.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    # Check if this is a *Mt* file (allows Parallelism annotations)
    is_mt_file = 'Mt' in Path(file_path).stem
    
    new_lines, changes = simplify_annotations(lines, is_mt_file)
    
    if changes == 0:
        return False
    
    if dry_run:
        print(f"Would simplify {changes} annotation pair(s) in {file_path}")
        return True
    
    # Write back
    try:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.writelines(new_lines)
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description='Simplify double-line complexity annotations'
    )
    parser.add_argument(
        '--file',
        type=str,
        help='Specific file to fix'
    )
    parser.add_argument(
        '--dry-run',
        action='store_true',
        help='Show what would be fixed without making changes'
    )
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.is_absolute():
            file_path = repo_root / file_path
        
        if fix_file(file_path, dry_run=args.dry_run):
            print(f"{'Would fix' if args.dry_run else 'Fixed'}: {file_path.relative_to(repo_root)}")
        else:
            print(f"No changes needed: {file_path.relative_to(repo_root)}")
        return 0
    
    # Fix all files in src/
    search_dir = repo_root / "src"
    fixed_count = 0
    
    for rs_file in sorted(search_dir.rglob("*.rs")):
        if fix_file(rs_file, dry_run=args.dry_run):
            rel_path = rs_file.relative_to(repo_root)
            print(f"{'Would fix' if args.dry_run else 'Fixed'}: {rel_path}")
            fixed_count += 1
    
    if fixed_count > 0:
        print(f"\n{'Would fix' if args.dry_run else 'Fixed'} {fixed_count} file(s)")
    else:
        print("No files need fixing")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

