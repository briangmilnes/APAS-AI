#!/usr/bin/env python3
"""
Review: Trait definition order - traits should appear before impl blocks.

Malpattern:
1. Data structure (struct/enum)
2. Inherent impl or trait impl <- WRONG: impl before trait definition
3. Trait definition for that data structure <- should be earlier

Correct order:
1. Data structure (struct/enum)
2. Trait definition for that data structure <- HERE
3. Inherent impl (impl Type { ... })
4. Custom trait impls
5. Standard trait impls

This ensures the trait interface is visible before any implementations.
"""
# Git commit: e4850e1
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def is_struct_or_enum_line(line):
    """Check if line defines a struct or enum."""
    stripped = line.strip()
    return bool(re.match(r'(pub\s+)?(?:struct|enum)\s+\w+', stripped))


def is_trait_definition(line):
    """Check if line defines a trait."""
    stripped = line.strip()
    return bool(re.match(r'(pub\s+)?trait\s+\w+', stripped))


def is_impl_line(line):
    """Check if line starts an impl block."""
    stripped = line.strip()
    return bool(re.match(r'impl(?:<[^>]+>)?\s+', stripped))


def check_trait_order(file_path):
    """
    Check for trait definitions appearing after impl blocks.
    
    Returns list of violations: (file_path, struct_name, trait_line, trait_name, first_impl_line)
    """
    # Skip Types.rs - it has a different format
    if file_path.name == 'Types.rs':
        return []
    
    # Skip Chap47 (Claude abomination - will be replaced by Chap47clean)
    # Skip Chap47clean (different structure - needs interactive fixing)
    if 'Chap47' in str(file_path.parent):
        return []
    
    violations = []
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return violations
    
    struct_name = None
    struct_line = None
    seen_impl_after_struct = False
    first_impl_line = None
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Skip empty lines and comments
        if not stripped or stripped.startswith('//'):
            continue
        
        # Detect struct/enum - resets state
        if is_struct_or_enum_line(line):
            m = re.search(r'(?:struct|enum)\s+(\w+)', stripped)
            struct_name = m.group(1) if m else None
            struct_line = i
            seen_impl_after_struct = False
            first_impl_line = None
            continue
        
        # Detect impl block (any kind)
        if struct_name and is_impl_line(line):
            if not seen_impl_after_struct:
                seen_impl_after_struct = True
                first_impl_line = i
            continue
        
        # Detect trait definition after impl
        if struct_name and seen_impl_after_struct and is_trait_definition(line):
            m = re.search(r'trait\s+(\w+)', stripped)
            trait_name = m.group(1) if m else 'Unknown'
            violations.append({
                'file': file_path,
                'struct': struct_name,
                'struct_line': struct_line,
                'trait_name': trait_name,
                'trait_line': i,
                'first_impl_line': first_impl_line,
            })
    
    return violations


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
    ]
    
    all_violations = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rs_file in search_dir.rglob("*.rs"):
            violations = check_trait_order(rs_file)
            all_violations.extend(violations)
    
    if all_violations:
        print("✗ Trait Definition Order Violations:\n")
        print("Trait definitions should appear BEFORE impl blocks (after struct/enum).\n")
        
        for v in all_violations:
            rel_path = v['file'].relative_to(repo_root)
            print(f"  {rel_path}:{v['struct_line']}")
            print(f"    Struct: {v['struct']}")
            print(f"    Line {v['first_impl_line']}: First impl block")
            print(f"    Line {v['trait_line']}: trait {v['trait_name']} definition")
            print(f"    → Trait {v['trait_name']} should move before line {v['first_impl_line']}")
            print()
        
        print(f"Total violations: {len(all_violations)}")
        print("\nCorrect order:")
        print("  1. Data structure (struct/enum)")
        print("  2. Trait definition <- SHOULD BE HERE")
        print("  3. Inherent impl (impl Type { ... })")
        print("  4. Custom trait implementations")
        print("  5. Standard trait implementations")
        return 1
    
    print("✓ All trait definitions are in correct order")
    return 0


if __name__ == "__main__":
    sys.exit(main())

