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


def find_std_import_end(lines: list[str]) -> int:
    """
    Find the line index (0-based) where new std:: imports should be inserted.
    Returns the index after the last std/core import (before any blank line or crate imports).
    """
    last_std_idx = -1
    inside_module = False
    found_pub_mod = False
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Find pub mod declaration
        if stripped.startswith('pub mod '):
            inside_module = True
            found_pub_mod = True
            continue
        
        # Track last std/core import
        if stripped.startswith('use '):
            leading_spaces = len(line) - len(line.lstrip())
            
            # Check if it's a std or core import
            is_std_import = stripped.startswith('use std::') or stripped.startswith('use core::')
            
            if found_pub_mod:
                # Inside a module - only count module-level uses (< 12 spaces)
                if inside_module and leading_spaces < 12 and is_std_import:
                    last_std_idx = i
            else:
                # Top-level file (tests/benches) - count top-level std imports
                if leading_spaces == 0 and is_std_import:
                    last_std_idx = i
        
        # Stop if we hit a blank line after finding std imports
        if last_std_idx >= 0 and stripped == '':
            break
        
        # Stop if we hit a crate:: or apas_ai:: import
        if last_std_idx >= 0 and (stripped.startswith('use crate::') or stripped.startswith('use apas_ai::')):
            break
    
    # Return the index after the last std import
    # If no std imports found, return -1
    return last_std_idx + 1 if last_std_idx >= 0 else -1


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
        
        # Check what's already imported
        existing_imports = set()
        for line in lines:
            stripped = line.strip()
            if stripped.startswith('use '):
                # Extract imported items
                # Handle: use std::fmt::{Debug, Display};
                # and: use std::collections::HashSet;
                existing_imports.add(stripped)
        
        # Determine what needs to be imported
        imports_to_add = set()
        replacements = {}  # {old_path: short_name}
        
        for full_path in paths.keys():
            # Extract the short name (last component)
            parts = full_path.split('::')
            short_name = parts[-1]
            
            # Check if already imported (exact match or in a group import)
            already_imported = False
            for existing in existing_imports:
                if full_path in existing or short_name in existing:
                    already_imported = True
                    break
            
            if not already_imported:
                # Store the import
                imports_to_add.add(full_path)
            
            # Always store replacement (even if already imported, we still want to shorten)
            replacements[full_path] = short_name
        
        # Find where to insert use statements (after last std import)
        use_insert_idx = find_std_import_end(lines)
        
        if use_insert_idx < 0:
            # No existing std imports - insert before first use statement or after pub mod
            for i, line in enumerate(lines):
                stripped = line.strip()
                if stripped.startswith('pub mod '):
                    # Insert after pub mod with blank line
                    use_insert_idx = i + 1
                    # Skip blank lines after pub mod
                    while use_insert_idx < len(lines) and lines[use_insert_idx].strip() == '':
                        use_insert_idx += 1
                    break
                elif stripped.startswith('use '):
                    # Insert before first use statement
                    use_insert_idx = i
                    break
            
            if use_insert_idx < 0:
                return False, "Could not find insertion point"
        
        # Detect indentation of existing use statements
        indent = "    "  # Default 4 spaces for modules
        
        # Check if this is a top-level file (test/bench) or a module file
        found_pub_mod = False
        for line in lines[:20]:  # Check first 20 lines
            if line.strip().startswith('pub mod '):
                found_pub_mod = True
                break
        
        if not found_pub_mod:
            # Top-level file - no indentation
            indent = ""
        elif use_insert_idx > 0:
            # Module file - check existing use statement indentation
            prev_line = lines[use_insert_idx - 1]
            if prev_line.strip().startswith('use '):
                # Match the indentation of the previous use statement
                indent = prev_line[:len(prev_line) - len(prev_line.lstrip())]
        
        if not dry_run:
            print(f"  DEBUG: Inserting at line index {use_insert_idx} (line {use_insert_idx + 1} in editor)")
            print(f"  DEBUG: Line before insert: {lines[use_insert_idx - 1] if use_insert_idx > 0 else 'N/A'}")
            print(f"  DEBUG: Using indentation: '{indent}' ({len(indent)} spaces)")
        
        # Build new use statements
        new_use_lines = []
        for import_path in sorted(imports_to_add):
            new_use_lines.append(f"{indent}use {import_path};")
        
        if not dry_run:
            print(f"  DEBUG: Adding {len(new_use_lines)} use statements:")
            for line in new_use_lines:
                print(f"    {line}")
        
        # Insert new use statements (one line at a time at the same position)
        if new_use_lines:
            for i, use_line in enumerate(new_use_lines):
                lines.insert(use_insert_idx + i, use_line)
            
            # If we inserted before non-std imports, add a blank line separator
            next_line_idx = use_insert_idx + len(new_use_lines)
            if next_line_idx < len(lines):
                next_line = lines[next_line_idx].strip()
                if next_line.startswith('use ') and not (next_line.startswith('use std::') or next_line.startswith('use core::')):
                    # Add blank line after std imports
                    lines.insert(next_line_idx, '')
        
        # Rebuild content and apply replacements
        content = '\n'.join(lines)
        
        # Replace qualified paths with short names
        # BUT: skip use statements (don't replace in lines that start with "use")
        lines = content.split('\n')
        for i, line in enumerate(lines):
            stripped = line.strip()
            # Skip replacement in use statements
            if stripped.startswith('use '):
                continue
            
            # Apply replacements to this line
            for full_path, short_name in replacements.items():
                # Use word boundaries to avoid partial matches
                pattern = re.compile(r'\b' + re.escape(full_path) + r'\b')
                lines[i] = pattern.sub(short_name, lines[i])
        
        content = '\n'.join(lines)
        
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

