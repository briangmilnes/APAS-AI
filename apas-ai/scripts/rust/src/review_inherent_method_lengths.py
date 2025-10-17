#!/usr/bin/env python3
"""
Analyze inherent impl methods to plan trait default refactor.

This script identifies:
1. Methods in inherent impl blocks (not trait impls)
2. Whether they're already in the corresponding trait
3. Their line length (for <120 char threshold)
4. Classification: short (move to trait default) vs long (signature only in trait)

Goal: Move short methods to trait defaults, add signatures for long methods.
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


def extract_trait_methods(lines):
    """Extract method names from trait definitions."""
    trait_methods = set()
    in_trait = False
    brace_depth = 0
    
    for line in lines:
        stripped = line.strip()
        if stripped.startswith('//'):
            continue
        
        if not in_trait and 'pub trait ' in line and '{' in line:
            in_trait = True
            brace_depth = line.count('{') - line.count('}')
            continue
        
        if in_trait:
            brace_depth += line.count('{') - line.count('}')
            
            if brace_depth <= 0:
                in_trait = False
                continue
            
            match = re.search(r'\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)', stripped)
            if match and not stripped.startswith('//'):
                trait_methods.add(match.group(1))
    
    return trait_methods


def analyze_inherent_impl(lines):
    """Analyze inherent impl blocks to find methods to refactor."""
    trait_methods = extract_trait_methods(lines)
    
    inherent_methods = []
    in_impl = False
    in_trait_impl = False
    impl_start = 0
    brace_depth = 0
    method_start = None
    method_lines = []
    method_name = None
    
    for line_num, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Skip comments
        if stripped.startswith('//'):
            continue
        
        # Detect impl start
        if not in_impl and not in_trait_impl:
            # Inherent impl: impl<T> StructName<T> {
            if re.match(r'\s*impl\s*(<[^>]*>)?\s+\w+', line):
                # Check if it's a trait impl
                if ' for ' in line:
                    in_trait_impl = True
                    brace_depth = line.count('{') - line.count('}')
                else:
                    in_impl = True
                    impl_start = line_num
                    brace_depth = line.count('{') - line.count('}')
                continue
        
        # Track trait impl (skip it)
        if in_trait_impl:
            brace_depth += line.count('{') - line.count('}')
            if brace_depth <= 0:
                in_trait_impl = False
            continue
        
        # Analyze inherent impl
        if in_impl:
            brace_depth += line.count('{') - line.count('}')
            
            if brace_depth <= 0:
                in_impl = False
                continue
            
            # Detect method start
            if method_start is None:
                match = re.search(r'^\s*pub\s+fn\s+([a-zA-Z_][a-zA-Z0-9_]*)', line)
                if match:
                    method_start = line_num
                    method_name = match.group(1)
                    method_lines = [line]
                    continue
            
            # Collect method lines
            if method_start is not None:
                method_lines.append(line)
                
                # Check if method ends (simplified - looks for closing brace)
                # This is heuristic - may need refinement
                if stripped == '}' and line.count('}') > line.count('{'):
                    # Calculate method length
                    method_text = ''.join(method_lines)
                    line_length = len(method_text.strip())
                    num_lines = len(method_lines)
                    
                    # Check if already in trait
                    in_trait = method_name in trait_methods
                    
                    inherent_methods.append({
                        'name': method_name,
                        'start_line': method_start,
                        'end_line': line_num,
                        'num_lines': num_lines,
                        'char_length': line_length,
                        'in_trait': in_trait,
                        'text': method_text
                    })
                    
                    method_start = None
                    method_lines = []
                    method_name = None
    
    return inherent_methods


def review_file(filepath, context):
    """Review a single file for inherent methods."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception:
        return None
    
    methods = analyze_inherent_impl(lines)
    
    if not methods:
        return None
    
    return {
        'file': filepath,
        'methods': methods
    }


def main():
    parser = create_review_parser(
        description="Analyze inherent impl methods for trait default refactor"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Only check src/ files
    src_dir = context.repo_root / 'src'
    if not src_dir.exists():
        print("✗ No src/ directory found")
        return 1
    
    files = list(src_dir.rglob('*.rs'))
    print(f"Analyzing {len(files)} source files for inherent methods...")
    print("=" * 80)
    
    all_results = []
    
    for filepath in sorted(files):
        result = review_file(filepath, context)
        if result:
            all_results.append(result)
    
    if not all_results:
        print("\n✓ No inherent impl methods found")
        return 0
    
    # Categorize methods
    short_not_in_trait = []  # <120 chars, not in trait - MOVE to trait default
    long_not_in_trait = []   # >=120 chars, not in trait - ADD signature to trait
    already_in_trait = []    # Already in trait - might be duplication
    
    for result in all_results:
        for method in result['methods']:
            if method['in_trait']:
                already_in_trait.append((result['file'], method))
            elif method['char_length'] < 120:
                short_not_in_trait.append((result['file'], method))
            else:
                long_not_in_trait.append((result['file'], method))
    
    # Report
    print(f"\n📊 Analysis Summary:")
    print("=" * 80)
    print(f"Files with inherent methods: {len(all_results)}")
    print(f"\nMethod categorization:")
    print(f"  ✓ Already in trait (keep as-is): {len(already_in_trait)}")
    print(f"  → Short (<120 chars, not in trait) - MOVE to trait default: {len(short_not_in_trait)}")
    print(f"  → Long (≥120 chars, not in trait) - ADD signature to trait: {len(long_not_in_trait)}")
    
    total_to_refactor = len(short_not_in_trait) + len(long_not_in_trait)
    print(f"\nTotal methods to refactor: {total_to_refactor}")
    
    # Show examples
    if short_not_in_trait:
        print(f"\n📝 Short methods to move to trait defaults (showing first 10):")
        for filepath, method in short_not_in_trait[:10]:
            rel_path = context.relative_path(filepath)
            print(f"  {rel_path}:{method['start_line']}")
            print(f"    {method['name']}() - {method['num_lines']} lines, {method['char_length']} chars")
    
    if long_not_in_trait:
        print(f"\n📝 Long methods to add signatures (showing first 10):")
        for filepath, method in long_not_in_trait[:10]:
            rel_path = context.relative_path(filepath)
            print(f"  {rel_path}:{method['start_line']}")
            print(f"    {method['name']}() - {method['num_lines']} lines, {method['char_length']} chars")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())


