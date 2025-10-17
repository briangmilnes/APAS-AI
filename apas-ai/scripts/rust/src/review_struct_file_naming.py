#!/usr/bin/env python3
"""
Review: Struct naming must match file name.

Finds cases where a struct name doesn't match the file name pattern.

Example violation:
  File: src/Chap05/RelationStEph.rs
  Contains: pub struct Relation<A, B>  // Should be RelationStEph

Rule: The primary struct in a file should have the same base name as the file.
For files like FooStEph.rs, the struct should be FooStEph, not just Foo.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def check_file(file_path: Path, context: ReviewContext) -> list[str]:
    """
    Check if struct names match the file name pattern.
    Returns list of violation messages.
    """
    violations = []
    
    try:
        content = file_path.read_text()
        lines = content.split('\n')
        
        # Get the expected struct name from file name
        # e.g., RelationStEph.rs -> RelationStEph
        file_stem = file_path.stem  # Remove .rs extension
        
        # Pattern to match struct declarations
        # Matches: pub struct Foo, pub struct Foo<T>, pub(crate) struct Foo, etc.
        struct_pattern = re.compile(r'^\s*pub(?:\([^)]*\))?\s+struct\s+(\w+)')
        
        for line_num, line in enumerate(lines, 1):
            match = struct_pattern.match(line)
            if match:
                struct_name = match.group(1)
                
                # Check if struct name matches file name
                if struct_name != file_stem:
                    # Also check if it's the "S" suffix variant (FooS vs Foo)
                    if struct_name + 'S' == file_stem or struct_name == file_stem + 'S':
                        continue  # This is acceptable (FooS struct in Foo.rs or vice versa)
                    
                    rel_path = context.relative_path(file_path)
                    violations.append(
                        f"  {rel_path}:{line_num} - struct '{struct_name}' doesn't match file name '{file_stem}'\n"
                        f"    {line.strip()}"
                    )
    
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations


def main():
    parser = create_review_parser(
        description="Check that struct names match file names (RustRules.md - Naming conventions)"
    )
    args = parser.parse_args()
    
    context = ReviewContext(args)
    all_violations = []
    
    # Search in src/, tests/, and benches/
    for directory in ['src', 'tests', 'benches']:
        dir_path = context.repo_root / directory
        if not dir_path.exists():
            continue
        
        rust_files = list(dir_path.rglob('*.rs'))
        
        for file_path in rust_files:
            if args.file and context.relative_path(file_path) != args.file:
                continue
            
            violations = check_file(file_path, context)
            all_violations.extend(violations)
    
    if all_violations:
        print("✗ Struct/File Naming violations found:\n", file=sys.stderr)
        for violation in all_violations:
            print(violation, file=sys.stderr)
        print(f"\nTotal violations: {len(all_violations)}\n", file=sys.stderr)
        print("Struct names should match their file names (excluding .rs extension).", file=sys.stderr)
        return 1
    else:
        if not args.file:
            print("✓ Struct/File Naming: No violations found (RustRules.md)")
        else:
            print(f"✓ {args.file}: No violations found")
        return 0


if __name__ == '__main__':
    sys.exit(main())

