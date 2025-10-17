#!/usr/bin/env python3
"""
Move methods from inherent impl blocks to trait default implementations.

This script:
1. Identifies methods in inherent impl blocks that are duplicated in trait impls
2. Removes the methods from the inherent impl block
3. Adds default implementations to the trait definition

Strategy: Remove duplication by making trait methods have default impls,
removing the redundant inherent methods.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


def find_trait_block(lines):
    """Find the trait definition block."""
    for i, line in enumerate(lines):
        if 'pub trait ' in line and '{' in line:
            return i
    return None


def find_inherent_impl_block(lines):
    """Find the inherent impl block (not trait impl)."""
    for i, line in enumerate(lines):
        if re.match(r'\s*impl\s*(<[^>]*>)?\s+\w+', line):
            if ' for ' not in line:
                return i
    return None


def extract_method_from_inherent(lines, start_idx):
    """Extract a complete method from inherent impl starting at start_idx."""
    method_lines = []
    brace_depth = 0
    
    for i in range(start_idx, len(lines)):
        line = lines[i]
        method_lines.append(line)
        
        # Track braces
        brace_depth += line.count('{') - line.count('}')
        
        # Method ends when braces balance
        if brace_depth == 0 and '{' in ''.join(method_lines):
            return method_lines, i + 1
    
    return method_lines, len(lines)


def convert_to_trait_default(method_lines, trait_indent="        "):
    """Convert inherent method to trait default implementation."""
    # Remove 'pub' from first line
    first_line = method_lines[0]
    first_line = re.sub(r'\bpub\s+', '', first_line)
    
    # Keep same indentation structure
    result = [first_line]
    result.extend(method_lines[1:])
    
    return result


def fix_file(filepath, context):
    """Remove inherent methods that duplicate trait methods, move to trait defaults."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return 0
    
    # Find trait and inherent impl blocks
    trait_idx = find_trait_block(lines)
    inherent_idx = find_inherent_impl_block(lines)
    
    if trait_idx is None:
        print(f"  No trait found in {filepath}")
        return 0
    
    if inherent_idx is None:
        print(f"  No inherent impl found in {filepath}")
        return 0
    
    print(f"  Found trait at line {trait_idx + 1}, inherent impl at line {inherent_idx + 1}")
    
    # For now, just report - actual implementation would:
    # 1. Extract methods from inherent impl
    # 2. Check if they exist in trait (by signature)
    # 3. If in trait but no default impl, add default impl
    # 4. Remove from inherent impl
    
    print(f"  [Script needs completion - would move methods from inherent to trait defaults]")
    
    return 0


def main():
    parser = create_review_parser(
        description="Move methods from inherent impl to trait default implementations"
    )
    parser.add_argument(
        'files',
        nargs='*',
        help='Specific files to fix'
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    if args.files:
        files_to_fix = [Path(f) if Path(f).is_absolute() else context.repo_root / f for f in args.files]
    else:
        print("Usage: fix_move_methods_to_trait_defaults.py file1.rs file2.rs ...")
        return 1

    total_fixes = 0
    for filepath in files_to_fix:
        if not filepath.exists():
            print(f"✗ File not found: {filepath}")
            continue
        
        print(f"\nFixing {context.relative_path(filepath)}...")
        fixes = fix_file(filepath, context)
        total_fixes += fixes

    print(f"\n{'='*70}")
    print(f"Total: Fixed {total_fixes} file(s)")
    return 0


if __name__ == '__main__':
    sys.exit(main())


