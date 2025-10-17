#!/usr/bin/env python3
"""
Review: Where clause simplification.

RustRules.md Lines 322-329: "Replace fn method<F>(...) where F: Fn(...); with
fn method<F: Fn(...)>(...); for simple bounds. Minimize where clauses across
codebase by inlining bounds."

Checks src/ for simple where clauses that could be inlined into the generic parameters.
Handles multi-line function signatures and where clauses.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import ReviewContext, create_review_parser


def parse_function_with_where(lines, start_idx):
    """
    Parse a function signature that may span multiple lines.
    Returns: (fn_name, generics, where_bounds, end_idx) or None
    """
    # Look for function declaration
    fn_match = re.search(r'(pub\s+)?fn\s+(\w+)\s*<([^>]+)>', lines[start_idx])
    if not fn_match:
        return None
    
    fn_name = fn_match.group(2)
    generics_str = fn_match.group(3)
    
    # Parse generic parameters (simplified - just get names without existing bounds)
    generic_names = []
    for gen in generics_str.split(','):
        # Extract just the name (before any :)
        name = gen.strip().split(':')[0].strip()
        if name and name[0].isupper():  # Type parameters are uppercase
            generic_names.append(name)
    
    if not generic_names:
        return None
    
    # Look for 'where' on following lines
    where_line_idx = None
    for i in range(start_idx, min(start_idx + 10, len(lines))):
        if lines[i].strip() == 'where':
            where_line_idx = i
            break
        if '{' in lines[i] and 'where' not in lines[i]:
            # Opening brace before where - no where clause
            return None
    
    if where_line_idx is None:
        return None
    
    # Parse where clause (lines after 'where' until '{')
    where_bounds = {}
    i = where_line_idx + 1
    while i < len(lines) and '{' not in lines[i]:
        line = lines[i].strip()
        if line and not line.startswith('//'):
            # Parse bound: "T: SomeTrait," or "T: Trait1 + Trait2,"
            match = re.match(r'(\w+):\s*(.+?),?\s*$', line)
            if match:
                param = match.group(1)
                bounds = match.group(2).rstrip(',').strip()
                if param in generic_names:
                    if param not in where_bounds:
                        where_bounds[param] = []
                    where_bounds[param].append(bounds)
        i += 1
    
    return (fn_name, generic_names, where_bounds, where_line_idx, i)


def is_simple_bound(bound):
    """Check if a bound is simple enough to inline."""
    # Simple: single trait name, possibly with path (Clone, std::fmt::Display)
    # Not simple: multiple traits (Clone + Send), function traits with complex signatures
    
    # Count '+' - multiple bounds
    if bound.count('+') > 0:
        return False
    
    # Check for function trait complexity (Fn with complex args)
    if 'Fn' in bound and '(' in bound:
        # FnMut, FnOnce with complex signatures might be too complex
        if bound.count(',') > 1:  # Multiple parameters
            return False
    
    return True


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
        print(f"Would check {len(files)} file(s) for simplifiable where clauses")
        return 0

    violations = []
    files = context.find_files([src_dir])

    for src_file in files:
        with open(src_file, 'r', encoding='utf-8') as f:
            lines = f.readlines()
        
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            
            # Look for function with generics
            if line.startswith('pub fn ') or line.startswith('fn '):
                if '<' in line and '>' in line:
                    result = parse_function_with_where(lines, i)
                    if result:
                        fn_name, generic_names, where_bounds, where_idx, end_idx = result
                        
                        # Check if where bounds are simple and could be inlined
                        for param, bounds_list in where_bounds.items():
                            if len(bounds_list) == 1:  # Single bound
                                bound = bounds_list[0]
                                if is_simple_bound(bound):
                                    violations.append((
                                        src_file,
                                        i + 1,  # fn line
                                        where_idx + 1,  # where line
                                        fn_name,
                                        param,
                                        bound
                                    ))
                        
                        i = end_idx
                        continue
            
            i += 1
    
    if violations:
        print("✗ Found simplifiable where clauses (RustRules.md Lines 322-329):\n")
        for file_path, fn_line, where_line, fn_name, param, bound in violations:
            rel_path = file_path.relative_to(context.repo_root)
            print(f"  {rel_path}:{fn_line}")
            print(f"    fn {fn_name}<{param}>")
            print(f"    where {param}: {bound}  ← could be inlined as <{param}: {bound}>")
            print()
        print(f"Total simplifiable where clauses: {len(violations)}")
        print(f"Total violations: {len(violations)}")
        print("\nSuggestion: Inline simple single-bound where clauses into generic parameters.")
        return 1
    else:
        print("✓ No simple where clauses found that should be inlined")
        return 0


if __name__ == "__main__":
    sys.exit(main())

