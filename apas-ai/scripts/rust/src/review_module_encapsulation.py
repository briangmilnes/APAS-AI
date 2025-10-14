#!/usr/bin/env python3
"""
Review: Mandatory module encapsulation.

RustRules.md Lines 117-123: "ALL CODE MUST BE WITHIN pub mod M{...}: Every function,
struct, enum, type alias, macro, and implementation must be defined inside the module
block. Exceptions: src/main.rs (fn main), src/lib.rs (module declarations)."

Checks src/ for code outside module blocks.
"""

import re
import sys
from pathlib import Path


def check_file_encapsulation(file_path):
    """Check if all code is inside pub mod blocks."""
    
    # Skip lib.rs and main.rs
    if file_path.name in ['lib.rs', 'main.rs']:
        return []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    violations = []
    in_module = False
    module_depth = 0
    
    # Keywords that define items (must be inside modules)
    item_keywords = ['fn ', 'struct ', 'enum ', 'type ', 'trait ', 'impl ', 'const ', 'static ']
    
    for idx, line in enumerate(lines, start=1):
        stripped = line.strip()
        
        # Skip empty lines and comments
        if not stripped or stripped.startswith('//') or stripped.startswith('/*') or stripped.startswith('*'):
            continue
        
        # Track module blocks
        if stripped.startswith('pub mod ') or stripped.startswith('mod '):
            in_module = True
            module_depth = 0
        
        # Track braces
        module_depth += line.count('{') - line.count('}')
        
        # If we close all braces, we're outside the module
        if in_module and module_depth <= 0 and '}' in line:
            in_module = False
        
        # Check for item definitions outside modules
        if not in_module:
            for keyword in item_keywords:
                if keyword in stripped and not stripped.startswith('use ') and not stripped.startswith('#['):
                    # Allow macro_rules! and #[macro_export] at file level
                    if 'macro_rules!' in stripped or stripped.startswith('macro_rules!'):
                        continue
                    
                    violations.append((idx, stripped, keyword.strip()))
                    break
    
    return violations


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    src_dir = repo_root / "src"
    
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    all_violations = []
    
    for src_file in src_dir.rglob("*.rs"):
        violations = check_file_encapsulation(src_file)
        if violations:
            all_violations.append((src_file, violations))
    
    if all_violations:
        print("✗ Found code outside module blocks (RustRules.md Lines 117-123):\n")
        for file_path, violations in all_violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:")
            for line_num, line_content, keyword in violations:
                print(f"    Line {line_num}: {keyword} outside pub mod")
                print(f"      {line_content[:80]}")
            print()
        
        total = sum(len(v) for _, v in all_violations)
        print(f"Total violations: {total}")
        print("\nFix: Move all definitions inside 'pub mod ModuleName { ... }' block.")
        return 1
    else:
        print("✓ All code properly encapsulated in modules")
        return 0


if __name__ == "__main__":
    sys.exit(main())

