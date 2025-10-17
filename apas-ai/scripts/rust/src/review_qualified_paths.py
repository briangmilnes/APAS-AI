#!/usr/bin/env python3
"""
Review: Qualified path organization.

Finds fully-qualified paths in code bodies that should be imported.

Violations: Using std::collections::hash_set::Iter or similar long paths in trait/impl
bodies instead of importing at the top and using the short name.

Examples:
  BAD:  fn iter(&self) -> std::collections::hash_set::Iter<'_, T>
  GOOD: use std::collections::hash_set::Iter;
        fn iter(&self) -> Iter<'_, T>
  
  BAD:  let map: std::collections::HashMap<K, V> = ...;
  GOOD: use std::collections::HashMap;
        let map: HashMap<K, V> = ...;
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check a single file for qualified paths that should be imported."""
    violations = []
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        # Pattern to match qualified paths (at least 2 :: separators)
        # Matches things like: std::collections::HashMap, std::collections::hash_set::Iter
        # But NOT crate:: or apas_ai:: or Type::method (single ::)
        qualified_path_pattern = re.compile(
            r'\b(std::\w+::\w+(?:::\w+)*)'  # std::module::Type or deeper
            r'|'
            r'\b(core::\w+::\w+(?:::\w+)*)'  # core::module::Type or deeper
        )
        
        in_comment = False
        in_macro = False
        
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
            
            # Track macro_rules! blocks
            if 'macro_rules!' in stripped:
                in_macro = True
            if in_macro and stripped == '}':
                in_macro = False
                continue
            if in_macro:
                continue
            
            # Skip use and pub use statements (these are imports/re-exports, not usage)
            if stripped.startswith('use ') or stripped.startswith('pub use '):
                continue
            
            # Skip pub mod and mod statements
            if stripped.startswith('pub mod ') or stripped.startswith('mod '):
                continue
            
            # Find qualified paths in this line
            matches = qualified_path_pattern.finditer(line)
            for match in matches:
                full_path = match.group(1) or match.group(2)
                
                # Skip some common acceptable cases:
                # - Attribute macros like #[derive(...)]
                if '#[' in line[:match.start()]:
                    continue
                
                # - Function/method calls (path followed by :: or ( or ::<)
                # This includes associated functions like HashSet::new() and UFCS like Debug::fmt(...)
                end_pos = match.end()
                if end_pos < len(line):
                    next_chars = line[end_pos:end_pos+3]
                    if next_chars.startswith('::') or next_chars.startswith('(') or next_chars.startswith('::<'):
                        continue
                
                # - std::fmt::Result conflicts with prelude Result<T, E>, keep it qualified
                if full_path == 'std::fmt::Result':
                    continue
                
                # Extract short context around the match
                start_col = max(0, match.start() - 20)
                end_col = min(len(line), match.end() + 20)
                ctx = line[start_col:end_col].strip()
                
                rel_path = context.relative_path(file_path)
                violations.append(
                    f"  {rel_path}:{line_num} - '{full_path}' should be imported\n    {stripped}"
                )
    
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
    
    return violations


def main():
    parser = create_review_parser(__doc__)
    args = parser.parse_args()
    context = ReviewContext(args)
    
    # Check all three directories: src, tests, benches
    dirs_to_check = []
    for dir_name in ["src", "tests", "benches"]:
        dir_path = context.repo_root / dir_name
        if dir_path.exists():
            dirs_to_check.append(dir_path)
    
    if not dirs_to_check:
        print("✓ No src/, tests/, or benches/ directories found")
        return 0
    
    if context.dry_run:
        files = context.find_files(dirs_to_check)
        print(f"Would check {len(files)} file(s) for qualified paths in {len(dirs_to_check)} directories")
        return 0
    
    all_violations = []
    files = context.find_files(dirs_to_check)
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if all_violations:
        print("✗ Qualified Path Organization violations found:\n")
        for v in all_violations:
            print(v)
        print(f"\nTotal violations: {len(all_violations)}")
        print("\nUse 'use' statements at the top to import types, then use short names.")
        return 1
    else:
        print("✓ Qualified Path Organization: No violations found (RustRules.md)")
        return 0


if __name__ == '__main__':
    sys.exit(main())

