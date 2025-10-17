#!/usr/bin/env python3
"""
Detect which data structures don't implement Clone + Display + Debug (StT requirements).

StT (Single-Threaded Type) = Eq + Clone + Display + Debug + Sized
StTInMtT = StT + Send + Sync
MtT (Multi-Threaded Type) = Sized + Send + Sync + has Inner: StT

This script finds public structs that DON'T satisfy StT requirements.
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


def extract_derives(lines, struct_line_idx):
    """Extract derives from the lines before a struct definition."""
    derives = set()
    
    # Look backwards for #[derive(...)]
    for i in range(struct_line_idx - 1, max(0, struct_line_idx - 5), -1):
        line = lines[i].strip()
        if line.startswith('#[derive('):
            # Extract traits from derive: #[derive(Debug, Clone, Copy, ...)]
            match = re.search(r'#\[derive\((.*?)\)\]', line)
            if match:
                traits_str = match.group(1)
                for trait in traits_str.split(','):
                    trait = trait.strip()
                    derives.add(trait)
        elif not line.startswith('#'):
            # Stop at non-attribute line
            break
    
    return derives


def has_manual_impl(lines, struct_name, trait_name):
    """Check if struct has a manual impl for trait."""
    # Look for: impl ... Trait for StructName
    pattern = rf'impl.*{trait_name}\s+for\s+{struct_name}'
    for line in lines:
        if re.search(pattern, line):
            return True
    return False


def analyze_file(filepath, context):
    """Analyze a file for structs that don't satisfy StT."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception:
        return []
    
    non_stt_structs = []
    
    for i, line in enumerate(lines):
        # Find pub struct definitions
        match = re.match(r'\s*pub\s+struct\s+(\w+)', line)
        if match:
            struct_name = match.group(1)
            
            # Extract derives
            derives = extract_derives(lines, i)
            
            # Check for required traits
            has_clone = 'Clone' in derives or has_manual_impl(lines, struct_name, 'Clone')
            has_display = 'Display' in derives or has_manual_impl(lines, struct_name, 'Display')
            has_debug = 'Debug' in derives or has_manual_impl(lines, struct_name, 'Debug')
            has_eq = 'Eq' in derives or has_manual_impl(lines, struct_name, 'Eq')
            
            missing = []
            if not has_clone:
                missing.append('Clone')
            if not has_display:
                missing.append('Display')
            if not has_debug:
                missing.append('Debug')
            if not has_eq:
                missing.append('Eq')
            
            if missing:
                non_stt_structs.append({
                    'name': struct_name,
                    'line': i + 1,
                    'derives': derives,
                    'missing': missing
                })
    
    return non_stt_structs


def main():
    parser = create_review_parser(
        description="Detect data structures that don't satisfy StT (Eq + Clone + Display + Debug)"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Only check src/ files
    src_dir = context.repo_root / 'src'
    if not src_dir.exists():
        print("✗ No src/ directory found")
        return 1
    
    files = list(src_dir.rglob('*.rs'))
    print(f"Analyzing {len(files)} source files for StT compliance...")
    print("=" * 80)
    print()
    print("StT requirements: Eq + Clone + Display + Debug + Sized")
    print()
    
    all_violations = []
    
    for filepath in sorted(files):
        violations = analyze_file(filepath, context)
        if violations:
            all_violations.append((filepath, violations))
    
    if not all_violations:
        print("\n✓ All public structs satisfy StT requirements!")
        return 0
    
    # Group by what's missing
    missing_clone = []
    missing_display = []
    missing_debug = []
    missing_eq = []
    
    for filepath, violations in all_violations:
        for v in violations:
            if 'Clone' in v['missing']:
                missing_clone.append((filepath, v))
            if 'Display' in v['missing']:
                missing_display.append((filepath, v))
            if 'Debug' in v['missing']:
                missing_debug.append((filepath, v))
            if 'Eq' in v['missing']:
                missing_eq.append((filepath, v))
    
    total_count = sum(len(v) for _, v in all_violations)
    
    print(f"✗ Found {total_count} struct(s) that don't satisfy StT:\n")
    
    print(f"Summary by missing trait:")
    print(f"  Missing Clone:   {len(missing_clone)}")
    print(f"  Missing Display: {len(missing_display)}")
    print(f"  Missing Debug:   {len(missing_debug)}")
    print(f"  Missing Eq:      {len(missing_eq)}")
    
    print(f"\n{'='*80}")
    print("Detailed list:\n")
    
    for filepath, violations in sorted(all_violations, key=lambda x: len(x[1]), reverse=True):
        rel_path = context.relative_path(filepath)
        print(f"{rel_path}:")
        for v in violations:
            missing_str = ', '.join(v['missing'])
            derives_str = ', '.join(sorted(v['derives'])) if v['derives'] else 'none'
            print(f"  Line {v['line']}: {v['name']}")
            print(f"    Has derives: {derives_str}")
            print(f"    Missing: {missing_str}")
        print()
    
    return 1


if __name__ == '__main__':
    sys.exit(main())


