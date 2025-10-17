#!/usr/bin/env python3
"""
Review: Mandatory module encapsulation.

RustRules.md Lines 117-123: "ALL CODE MUST BE WITHIN pub mod M{...}: Every function,
struct, enum, type alias, macro, and implementation must be defined inside the module
block. Exceptions: src/main.rs (fn main), src/lib.rs (module declarations)."

Checks src/ for code outside module blocks.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def check_file(file_path: Path, context: ReviewContext) -> list:
    """Check if all code is inside pub mod blocks."""
    
    # Skip lib.rs and main.rs
    if file_path.name in ['lib.rs', 'main.rs']:
        return []
    
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    violations = []
    in_module = False
    module_depth = 0
    
    item_keywords = ['fn ', 'struct ', 'enum ', 'type ', 'trait ', 'impl ', 'const ', 'static ']
    
    for idx, line in enumerate(lines, start=1):
        stripped = line.strip()
        
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
                    # Allow macro_rules! at file level
                    if 'macro_rules!' in stripped or stripped.startswith('macro_rules!'):
                        continue
                    
                    rel_path = context.relative_path(file_path)
                    violations.append(
                        f"  {rel_path}:{idx}\n    {keyword.strip()} outside pub mod\n      {stripped[:80]}"
                    )
                    break
    
    return violations


def main():
    parser = create_review_parser(__doc__)
    args = parser.parse_args()
    context = ReviewContext(args)
    
    src_dir = context.repo_root / "src"
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    if context.dry_run:
        files = context.find_files([src_dir])
        print(f"Would check {len(files)} file(s) for module encapsulation")
        return 0
    
    all_violations = []
    files = context.find_files([src_dir])
    
    for file_path in files:
        violations = check_file(file_path, context)
        all_violations.extend(violations)
    
    if not all_violations:
        print("✓ All code properly encapsulated in modules")
        return 0
    
    print(f"✗ Found code outside module blocks (RustRules.md Lines 117-123):\n")
    for violation in all_violations:
        print(violation)
    print(f"\nTotal violations: {len(all_violations)}")
    print("\nFix: Move all definitions inside 'pub mod ModuleName { ... }' block.")
    return 1


if __name__ == "__main__":
    sys.exit(main())
