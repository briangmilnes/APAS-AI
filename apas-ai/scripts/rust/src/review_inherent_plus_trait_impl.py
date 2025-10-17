#!/usr/bin/env python3
"""
Detect structs that have BOTH inherent impl AND trait impl blocks.

Pattern to find:
- impl<T> StructName<T> { ... }         // Inherent impl
- impl<T> TraitName<T> for StructName   // Trait impl

Most structs should have ONLY trait impl, not both.
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
    """Find structs with both inherent and trait impls."""
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
    except Exception:
        return []
    
    # Track struct_name -> {'inherent': [line_nums], 'traits': {trait_name: [line_nums]}}
    struct_impls = defaultdict(lambda: {'inherent': [], 'traits': defaultdict(list)})
    
    for i, line in enumerate(lines, 1):
        # Match inherent impl: impl<...> StructName<...> {
        inherent_match = re.match(r'\s*impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]*>)?\s*(?:where\s+|\{)', line)
        if inherent_match and ' for ' not in line:
            struct_name = inherent_match.group(1)
            struct_impls[struct_name]['inherent'].append(i)
        
        # Match trait impl: impl<...> TraitName<...> for StructName<...>
        trait_match = re.match(r'\s*impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]*>)?\s+for\s+(\w+)', line)
        if trait_match:
            trait_name = trait_match.group(1)
            struct_name = trait_match.group(2)
            
            # Skip standard traits
            standard_traits = {
                'Debug', 'Clone', 'Copy', 'PartialEq', 'Eq', 'PartialOrd', 'Ord',
                'Hash', 'Display', 'Default', 'From', 'Into', 'AsRef', 'AsMut',
                'Deref', 'DerefMut', 'Drop', 'Iterator', 'IntoIterator',
                'Send', 'Sync', 'Sized', 'Unpin'
            }
            
            if trait_name not in standard_traits:
                struct_impls[struct_name]['traits'][trait_name].append(i)
    
    # Find structs with BOTH inherent AND trait impls
    violations = []
    for struct_name, impls in struct_impls.items():
        if impls['inherent'] and impls['traits']:
            violations.append({
                'struct': struct_name,
                'inherent_lines': impls['inherent'],
                'traits': dict(impls['traits'])
            })
    
    return violations


def main():
    parser = create_review_parser(
        description="Detect structs with both inherent impl and trait impl (should have trait impl only)"
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
        print("\n✓ All structs use trait impl only (no inherent+trait duplication)!")
        return 0
    
    # Count total violations
    total_count = sum(len(v) for v in all_violations.values())
    
    print(f"✗ Inherent + Trait Impl Pattern: {total_count} struct(s)\n")
    print("Structs should use TRAIT impl only, not both inherent impl and trait impl.\n")
    print("="*80)
    
    for filepath, file_violations in sorted(all_violations.items()):
        rel_path = context.relative_path(filepath)
        print(f"\n{rel_path}:")
        
        for violation in file_violations:
            struct_name = violation['struct']
            inherent_lines = violation['inherent_lines']
            traits = violation['traits']
            
            print(f"  {struct_name}:")
            print(f"    Inherent impl at line(s): {', '.join(map(str, inherent_lines))}")
            for trait_name, trait_lines in sorted(traits.items()):
                print(f"    Trait impl {trait_name} at line(s): {', '.join(map(str, trait_lines))}")
    
    print(f"\n{'='*80}")
    print(f"Total: {total_count} struct(s) with both inherent and trait impls")
    print("\nRecommendation: Remove inherent impl, keep only trait impl.")
    
    return 1 if all_violations else 0


if __name__ == '__main__':
    sys.exit(main())


