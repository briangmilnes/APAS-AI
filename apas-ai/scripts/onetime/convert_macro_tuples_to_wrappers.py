#!/usr/bin/env python3
"""
Convert raw tuples to Triple() wrappers in weighted graph macro invocations.

Patterns to convert:
- E: [(a, b, c), ...] → E: [Triple(a, b, c), ...]
- A: [(a, b, c), ...] → A: [Triple(a, b, c), ...]

For Float graph macros, also wraps weight with OrderedFloat:
- E: [(1, 2, 1.5)] → E: [Triple(1, 2, OrderedFloat(1.5))]

This enforces "Types are your friends" - no hidden tuple wrapping in macros.
"""

# Git commit: 3ca9dcff19371dfe2d4dce7d93788971c098ac93
# Date: 2025-10-17 05:23:25 -0700

# Git commit: <will be filled by add_git_metadata.py>
# Date: <will be filled by add_git_metadata.py>

import re
import sys
from pathlib import Path

# Add lib directory to path
sys.path.insert(0, str(Path(__file__).parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


FLOAT_MACROS = {
    'WeightedDirGraphStEphFloatLit',
    'WeightedDirGraphMtEphFloatLit',
    'WeightedUnDirGraphStEphFloatLit',
    'WeightedUnDirGraphMtEphFloatLit',
}

INT_MACROS = {
    'WeightedDirGraphStEphIntLit',
    'WeightedDirGraphMtEphIntLit',
    'WeightedUnDirGraphStEphIntLit',
    'WeightedUnDirGraphMtEphIntLit',
}

ALL_WEIGHT_MACROS = FLOAT_MACROS | INT_MACROS


def convert_tuple_to_triple(tuple_match, wrap_weight_with_ordered_float):
    """
    Convert a tuple pattern to Triple wrapper.
    
    Args:
        tuple_match: regex match object for tuple pattern
        wrap_weight_with_ordered_float: if True, wrap third element with OrderedFloat
    
    Returns:
        Converted string with Triple wrapper
    """
    # Extract the three elements
    elem1 = tuple_match.group(1).strip()
    elem2 = tuple_match.group(2).strip()
    elem3 = tuple_match.group(3).strip()
    
    # For Float macros, wrap weight with OrderedFloat if it's a numeric literal
    if wrap_weight_with_ordered_float:
        # Check if elem3 is already wrapped with OrderedFloat
        if not elem3.startswith('OrderedFloat'):
            # It's a raw numeric literal - wrap it
            elem3 = f'OrderedFloat({elem3})'
    
    return f'Triple({elem1}, {elem2}, {elem3})'


def process_file(filepath, context, dry_run=False):
    """
    Process a single file, converting raw tuples to Triple wrappers.
    
    Returns:
        Number of conversions made
    """
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return 0
    
    original_content = content
    
    # Collect all replacements (start_pos, end_pos, new_text)
    replacements = []
    
    # Find all weighted graph macro invocations
    # Pattern: MacroName! { ... } or MacroName! ( ... )
    macro_pattern = r'(' + '|'.join(re.escape(m) for m in ALL_WEIGHT_MACROS) + r')!\s*([{(])'
    
    for macro_match in re.finditer(macro_pattern, original_content):
        macro_name = macro_match.group(1)
        open_delim = macro_match.group(2)
        is_float_macro = macro_name in FLOAT_MACROS
        
        macro_start = macro_match.end()
        
        # Find the matching closing delimiter for this macro
        delim_depth = 1
        macro_end = macro_start
        for i in range(macro_start, len(original_content)):
            if original_content[i] in '{(':
                delim_depth += 1
            elif original_content[i] in '})':
                delim_depth -= 1
                if delim_depth == 0:
                    macro_end = i
                    break
        
        if macro_end == macro_start:
            continue  # Couldn't find matching delimiter
        
        # Find all tuple patterns in E: or A: blocks
        # Pattern: E: [...] or A: [...]
        edge_block_pattern = r'([EA]):\s*\[([^\]]+)\]'
        
        for edge_match in re.finditer(edge_block_pattern, original_content[macro_start:macro_end]):
            block_content = edge_match.group(2)
            block_content_start = macro_start + edge_match.start(2)
            
            # Find all tuple patterns: (expr, expr, expr)
            tuple_pattern = r'\(([^,]+),\s*([^,]+),\s*([^)]+)\)'
            
            for tuple_match in re.finditer(tuple_pattern, block_content):
                # Skip if already wrapped with Triple - check the 7 chars before the match
                prefix_start = max(0, tuple_match.start() - 7)
                prefix = block_content[prefix_start:tuple_match.start()]
                if 'Triple' in prefix:
                    continue
                
                # Also skip if first element contains "Triple" (nested case)
                first_elem = tuple_match.group(1).strip()
                if 'Triple' in first_elem:
                    continue
                
                # Convert tuple to Triple
                new_tuple = convert_tuple_to_triple(tuple_match, is_float_macro)
                
                # Record replacement
                tuple_start = block_content_start + tuple_match.start()
                tuple_end = block_content_start + tuple_match.end()
                replacements.append((tuple_start, tuple_end, new_tuple))
    
    # Apply replacements in reverse order (end to start) to preserve positions
    replacements.sort(reverse=True)
    
    new_content = original_content
    for start, end, new_text in replacements:
        new_content = new_content[:start] + new_text + new_content[end:]
    
    conversions = len(replacements)
    
    # Write back if changed
    if new_content != original_content and not dry_run:
        try:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
        except Exception as e:
            print(f"Error writing {filepath}: {e}", file=sys.stderr)
            return 0
    
    return conversions


def main():
    parser = create_review_parser(
        description="Convert raw tuples to Triple() wrappers in weighted graph macros"
    )
    parser.add_argument('files', nargs='*', 
                       help='Specific files to process (default: tests/Chap06, benches/Chap06)')
    
    args = parser.parse_args()
    context = ReviewContext(args)
    
    dry_run = getattr(args, 'dry_run', False)
    
    # Determine files to process
    if args.files:
        files = [Path(f) for f in args.files]
    else:
        # Default: all weighted graph test and benchmark files
        files = []
        test_dirs = [
            context.repo_root / 'tests' / 'Chap06',
            context.repo_root / 'tests' / 'Chap57',
            context.repo_root / 'tests' / 'Chap58',
            context.repo_root / 'tests' / 'Chap59',
        ]
        bench_dirs = [
            context.repo_root / 'benches' / 'Chap06',
            context.repo_root / 'benches' / 'Chap57',
            context.repo_root / 'benches' / 'Chap58',
            context.repo_root / 'benches' / 'Chap59',
        ]
        
        for dir_path in test_dirs + bench_dirs:
            if dir_path.exists():
                files.extend(sorted(dir_path.glob('*.rs')))
    
    if not files:
        print("No files found to process", file=sys.stderr)
        return 1
    
    print(f"Processing {len(files)} file(s)...")
    if dry_run:
        print("DRY RUN - no files will be modified\n")
    
    total_conversions = 0
    files_modified = 0
    
    for filepath in files:
        conversions = process_file(filepath, context, dry_run)
        if conversions > 0:
            files_modified += 1
            total_conversions += conversions
            status = "(would convert)" if dry_run else "✓"
            rel_path = context.relative_path(filepath)
            print(f"{status} {rel_path}: {conversions} tuple(s) → Triple()")
    
    print(f"\n{'='*70}")
    print(f"Total: {total_conversions} conversion(s) in {files_modified} file(s)")
    if dry_run:
        print("Run without --dry-run to apply changes")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

