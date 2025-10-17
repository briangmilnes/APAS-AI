#!/usr/bin/env python3
"""
Detect trait bound mismatches between inherent impls and trait impls.

This script finds cases where:
- A method exists in both inherent impl and trait
- The inherent impl has WEAKER bounds than the trait
- Moving to trait defaults would expose stricter bounds

Example problem:
    impl<T: Eq + Hash> Struct<T> {        // Weaker: just Eq + Hash
        fn foo() -> Struct<T> { ... }
    }
    
    trait StructTrait<T: StT + Hash> {    // Stronger: StT = Eq + Clone + Display + ...
        fn foo() -> Self;
    }
    
When inherent is removed, callers must satisfy StT, not just Eq + Hash!
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path
from collections import defaultdict

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


def parse_bounds(bounds_str):
    """Parse trait bounds like 'T: Eq + Hash' into a set of traits."""
    if not bounds_str:
        return set()
    
    # Extract individual trait names
    # Handle: T: Eq + Hash, U: Clone
    traits = set()
    for bound in bounds_str.split(','):
        if ':' in bound:
            _, trait_list = bound.split(':', 1)
            for trait in trait_list.split('+'):
                trait = trait.strip()
                if trait and not trait.startswith('\''):  # Skip lifetimes
                    traits.add(trait)
    
    return traits


def extract_impl_bounds(impl_line):
    """Extract bounds from impl line: impl<T: Eq + Hash> Struct<T>"""
    match = re.search(r'impl\s*<([^>]+)>', impl_line)
    if match:
        return parse_bounds(match.group(1))
    return set()


def extract_trait_bounds(trait_line):
    """Extract bounds from trait line: pub trait MyTrait<T: StT + Hash>"""
    match = re.search(r'trait\s+\w+\s*<([^>]+)>', trait_line)
    if match:
        return parse_bounds(match.group(1))
    return set()


def find_method_names_in_block(lines, start_idx, end_idx):
    """Find all method names in a block."""
    methods = set()
    for i in range(start_idx, end_idx):
        match = re.search(r'\b(?:pub\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)', lines[i])
        if match and not lines[i].strip().startswith('//'):
            methods.add(match.group(1))
    return methods


def analyze_file(filepath, context):
    """Analyze a file for trait bound mismatches."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception:
        return None
    
    # Find trait definition
    trait_idx = None
    trait_bounds = set()
    trait_end_idx = None
    
    for i, line in enumerate(lines):
        if 'pub trait ' in line and '{' in line:
            trait_idx = i
            trait_bounds = extract_trait_bounds(line)
            
            # Find trait end
            brace_depth = line.count('{') - line.count('}')
            for j in range(i + 1, len(lines)):
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if brace_depth <= 0:
                    trait_end_idx = j
                    break
            break
    
    if trait_idx is None:
        return None
    
    # Find inherent impl
    inherent_idx = None
    inherent_bounds = set()
    inherent_end_idx = None
    
    for i, line in enumerate(lines):
        if re.match(r'\s*impl\s*<', line) and ' for ' not in line:
            inherent_idx = i
            inherent_bounds = extract_impl_bounds(line)
            
            # Find impl end
            brace_depth = line.count('{') - line.count('}')
            for j in range(i + 1, len(lines)):
                brace_depth += lines[j].count('{') - lines[j].count('}')
                if brace_depth <= 0:
                    inherent_end_idx = j
                    break
            break
    
    if inherent_idx is None:
        return None
    
    # Get method names from both
    trait_methods = find_method_names_in_block(lines, trait_idx, trait_end_idx)
    inherent_methods = find_method_names_in_block(lines, inherent_idx, inherent_end_idx)
    
    overlapping = trait_methods & inherent_methods
    
    if not overlapping:
        return None
    
    # Check if bounds differ
    if inherent_bounds == trait_bounds:
        return None
    
    # Inherent has weaker bounds if it's missing some from trait
    missing_in_inherent = trait_bounds - inherent_bounds
    
    if not missing_in_inherent:
        return None
    
    return {
        'file': filepath,
        'trait_line': trait_idx + 1,
        'inherent_line': inherent_idx + 1,
        'trait_bounds': trait_bounds,
        'inherent_bounds': inherent_bounds,
        'missing_bounds': missing_in_inherent,
        'overlapping_methods': overlapping
    }


def main():
    parser = create_review_parser(
        description="Detect trait bound mismatches between inherent and trait impls"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Only check src/ files
    src_dir = context.repo_root / 'src'
    if not src_dir.exists():
        print("✗ No src/ directory found")
        return 1
    
    files = list(src_dir.rglob('*.rs'))
    print(f"Analyzing {len(files)} source files for trait bound mismatches...")
    print("=" * 80)
    
    all_mismatches = []
    
    for filepath in sorted(files):
        result = analyze_file(filepath, context)
        if result:
            all_mismatches.append(result)
    
    if not all_mismatches:
        print("\n✓ No trait bound mismatches found!")
        return 0
    
    # Report
    print(f"\n✗ Found {len(all_mismatches)} file(s) with trait bound mismatches:\n")
    
    for result in sorted(all_mismatches, key=lambda x: len(x['missing_bounds']), reverse=True):
        rel_path = context.relative_path(result['file'])
        print(f"\n{rel_path}:")
        print(f"  Trait (line {result['trait_line']}):")
        print(f"    Bounds: {', '.join(sorted(result['trait_bounds']))}")
        print(f"  Inherent impl (line {result['inherent_line']}):")
        print(f"    Bounds: {', '.join(sorted(result['inherent_bounds']))}")
        print(f"  Missing in inherent: {', '.join(sorted(result['missing_bounds']))}")
        print(f"  Affected methods ({len(result['overlapping_methods'])}): {', '.join(sorted(list(result['overlapping_methods'])[:5]))}")
        if len(result['overlapping_methods']) > 5:
            print(f"    ... and {len(result['overlapping_methods']) - 5} more")
    
    print("\n" + "=" * 80)
    print(f"Summary:")
    print(f"  Files with bound mismatches: {len(all_mismatches)}")
    print(f"\nRecommendation:")
    print(f"  Add missing bounds to inherent impl blocks to match trait bounds.")
    print(f"  This ensures no surprises when moving methods to trait defaults.")
    
    return 1


if __name__ == '__main__':
    sys.exit(main())


