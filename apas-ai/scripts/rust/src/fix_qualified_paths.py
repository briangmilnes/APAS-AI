#!/usr/bin/env python3
"""
Fix script: Replace fully-qualified paths with imports.

Finds fully-qualified paths (std::collections::HashMap, std::fmt::Debug, etc.)
in code bodies and:
1. Adds appropriate use statements at the top of the module
2. Replaces the qualified paths with short names
"""

import re
import sys
from pathlib import Path
from collections import defaultdict

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def extract_qualified_paths(content: str) -> dict[str, list[tuple[int, str]]]:
    """
    Extract all qualified paths from content.
    Returns dict of {qualified_path: [(line_num, context), ...]}
    """
    lines = content.split('\n')
    paths = defaultdict(list)
    
    # Pattern to match qualified paths (at least 2 :: separators)
    qualified_path_pattern = re.compile(
        r'\b(std::\w+::\w+(?:::\w+)*)'
        r'|'
        r'\b(core::\w+::\w+(?:::\w+)*)'
    )
    
    in_comment = False
    
    for line_num, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Skip comments
        if stripped.startswith('//'):
            continue
        if '/*' in stripped:
            in_comment = True
        if '*/' in stripped:
            in_comment = False
            continue
        if in_comment:
            continue
        
        # Skip use statements (these are imports, not usage)
        if stripped.startswith('use '):
            continue
        
        # Skip pub mod and mod statements
        if stripped.startswith('pub mod ') or stripped.startswith('mod '):
            continue
        
        # Find qualified paths in this line
        matches = qualified_path_pattern.finditer(line)
        for match in matches:
            full_path = match.group(1) or match.group(2)
            
            # Skip attribute macros
            if '#[' in line[:match.start()]:
                continue
            
            paths[full_path].append((line_num, line))
    
    return paths


def find_use_block_end(lines: list[str]) -> int:
    """
    Find the line index (0-based) where new use statements should be inserted.
    Returns the index after the last existing use statement.
    """
    last_use_idx = -1
    inside_module = False
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Find pub mod declaration
        if stripped.startswith('pub mod '):
            inside_module = True
            continue
        
        # Track last use statement after module start
        if inside_module and stripped.startswith('use '):
            last_use_idx = i
    
    # Return the index after the last use statement
    # If no use statements found, return -1
    return last_use_idx + 1 if last_use_idx >= 0 else -1


def fix_file(file_path: Path, context: ReviewContext, dry_run: bool = False) -> tuple[bool, str]:
    """
    Fix qualified paths in a single file.
    Returns (changed, message).
    """
    try:
        content = file_path.read_text()
        original_content = content
        lines = content.split('\n')
        
        # Extract all qualified paths
        paths = extract_qualified_paths(content)
        
        if not paths:
            return False, "No qualified paths found"
        
        # Determine what needs to be imported
        imports_to_add = set()
        replacements = {}  # {old_path: short_name}
        
        for full_path in paths.keys():
            # Extract the short name (last component)
            parts = full_path.split('::')
            short_name = parts[-1]
            
            # Store the import and replacement
            imports_to_add.add(full_path)
            replacements[full_path] = short_name
        
        # Find where to insert use statements
        use_insert_idx = find_use_block_end(lines)
        
        if use_insert_idx < 0:
            return False, "Could not find use block insertion point"
        
        if not dry_run:
            print(f"  DEBUG: Inserting at line index {use_insert_idx} (line {use_insert_idx + 1} in editor)")
            print(f"  DEBUG: Line before insert: {lines[use_insert_idx - 1] if use_insert_idx > 0 else 'N/A'}")
        
        # Build new use statements
        new_use_lines = []
        for import_path in sorted(imports_to_add):
            new_use_lines.append(f"    use {import_path};")
        
        if not dry_run:
            print(f"  DEBUG: Adding {len(new_use_lines)} use statements:")
            for line in new_use_lines:
                print(f"    {line}")
        
        # Insert new use statements (one line at a time at the same position)
        if new_use_lines:
            for i, use_line in enumerate(new_use_lines):
                lines.insert(use_insert_idx + i, use_line)
        
        # Rebuild content and apply replacements
        content = '\n'.join(lines)
        
        # Replace qualified paths with short names
        for full_path, short_name in replacements.items():
            # Use word boundaries to avoid partial matches
            pattern = re.compile(r'\b' + re.escape(full_path) + r'\b')
            content = pattern.sub(short_name, content)
        
        if content == original_content:
            return False, "No changes needed"
        
        if dry_run:
            rel_path = context.relative_path(file_path)
            return True, f"Would add {len(imports_to_add)} imports and fix {sum(len(v) for v in paths.values())} uses"
        
        # Write the fixed content
        file_path.write_text(content)
        
        rel_path = context.relative_path(file_path)
        return True, f"Added {len(imports_to_add)} imports, fixed {sum(len(v) for v in paths.values())} uses"
        
    except Exception as e:
        return False, f"Error: {e}"


def main():
    parser = create_review_parser(__doc__)
    args = parser.parse_args()
    context = ReviewContext(args)
    
    if args.file:
        # Single file mode
        file_path = Path(args.file)
        if not file_path.exists():
            file_path = context.repo_root / args.file
        
        if not file_path.exists():
            print(f"Error: File not found: {args.file}")
            return 1
        
        changed, message = fix_file(file_path, context, args.dry_run)
        rel_path = context.relative_path(file_path)
        
        if changed:
            print(f"✓ {rel_path}: {message}")
            return 0
        else:
            print(f"  {rel_path}: {message}")
            return 0
    
    else:
        # Directory mode - check all three directories
        dirs_to_check = []
        for dir_name in ["src", "tests", "benches"]:
            dir_path = context.repo_root / dir_name
            if dir_path.exists():
                dirs_to_check.append(dir_path)
        
        if not dirs_to_check:
            print("✓ No src/, tests/, or benches/ directories found")
            return 0
        
        files = context.find_files(dirs_to_check)
        fixed_count = 0
        
        for file_path in files:
            changed, message = fix_file(file_path, context, args.dry_run)
            rel_path = context.relative_path(file_path)
            
            if changed:
                print(f"✓ {rel_path}: {message}")
                fixed_count += 1
        
        if fixed_count > 0:
            if args.dry_run:
                print(f"\nWould fix {fixed_count} file(s)")
            else:
                print(f"\nFixed {fixed_count} file(s)")
            return 0
        else:
            print("No files needed fixing")
            return 0


if __name__ == '__main__':
    sys.exit(main())

