#!/usr/bin/env python3
"""
Detect traits that have multiple impl blocks (should have single implementation).

Pattern to find:
- Trait FooTrait is implemented multiple times for the same struct
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


def analyze_file(filepath, context):
    """Find traits with multiple implementations."""
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
    except Exception:
        return {}
    
    # Track: trait_name -> [(struct_name, line_num), ...]
    trait_impls = defaultdict(list)
    
    for i, line in enumerate(lines, 1):
        # Match: impl<...> TraitName<...> for StructName<...>
        # Examples:
        #   impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T>
        #   impl SetStEphTrait<i32> for SetStEph<i32>
        match = re.match(r'\s*impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]*>)?\s+for\s+(\w+)', line)
        if match:
            trait_name = match.group(1)
            struct_name = match.group(2)
            
            # Skip standard traits (Debug, Clone, Display, etc.)
            standard_traits = {
                'Debug', 'Clone', 'Copy', 'PartialEq', 'Eq', 'PartialOrd', 'Ord',
                'Hash', 'Display', 'Default', 'From', 'Into', 'AsRef', 'AsMut',
                'Deref', 'DerefMut', 'Drop', 'Iterator', 'IntoIterator',
                'Send', 'Sync', 'Sized', 'Unpin'
            }
            
            if trait_name not in standard_traits:
                trait_impls[trait_name].append((struct_name, i))
    
    # Find traits with multiple impls for the same struct
    violations = {}
    for trait_name, impls in trait_impls.items():
        # Group by struct name
        by_struct = defaultdict(list)
        for struct_name, line_num in impls:
            by_struct[struct_name].append(line_num)
        
        # Find structs with multiple impls
        for struct_name, line_nums in by_struct.items():
            if len(line_nums) > 1:
                if trait_name not in violations:
                    violations[trait_name] = []
                violations[trait_name].append({
                    'struct': struct_name,
                    'lines': line_nums
                })
    
    return violations


def main():
    parser = create_review_parser(
        description="Detect traits with multiple implementations (should have single impl)"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Only check src/ files
    src_dir = context.repo_root / 'src'
    if not src_dir.exists():
        print("✗ No src/ directory found")
        return 1
    
    files = list(src_dir.rglob('*.rs'))
    
    all_violations = {}
    
    for filepath in sorted(files):
        violations = analyze_file(filepath, context)
        if violations:
            all_violations[filepath] = violations
    
    if not all_violations:
        print("\n✓ All traits have single implementations!")
        return 0
    
    # Count total violations
    total_count = sum(
        len(structs) 
        for file_violations in all_violations.values() 
        for structs in file_violations.values()
    )
    
    print(f"✗ Multiple Trait Implementations: {total_count} violation(s)\n")
    print("Each trait should have only ONE impl block for each struct.\n")
    print("="*80)
    
    for filepath, file_violations in sorted(all_violations.items()):
        rel_path = context.relative_path(filepath)
        
        for trait_name, struct_violations in sorted(file_violations.items()):
            for violation in struct_violations:
                struct_name = violation['struct']
                lines = violation['lines']
                
                print(f"\n{rel_path}")
                print(f"  Trait: {trait_name}")
                print(f"  Struct: {struct_name}")
                print(f"  Multiple impl blocks at lines: {', '.join(map(str, lines))}")
    
    print(f"\n{'='*80}")
    print(f"Total violations: {total_count}")
    print("\nRecommendation: Consolidate into a single impl block per trait per struct.")
    
    return 1 if all_violations else 0


if __name__ == '__main__':
    sys.exit(main())


