#!/usr/bin/env python3
"""
Check if inherent impls and trait impls use different generic type parameter names.

This identifies cases where the inherent impl uses `A, B` but the trait uses `X, Y`,
which causes issues when copying method signatures.
"""
# Git commit: TBD
# Date: 2025-10-17

import re
import sys
from pathlib import Path
from collections import defaultdict

def extract_type_params(impl_line):
    """
    Extract generic type parameters from an impl line.
    Returns list of type param names.
    
    Examples:
        impl<A: Trait, B: Trait> Struct<A, B> -> ['A', 'B']
        impl<X: StT + Hash, Y: StT + Hash> StructTrait<X, Y> for Struct<X, Y> -> ['X', 'Y']
    """
    # Look for impl<...> part
    match = re.search(r'impl<([^>]+)>', impl_line)
    if not match:
        return []
    
    type_bounds = match.group(1)
    
    # Extract type parameter names (before the colon or comma)
    # Pattern: TypeName followed by : or , or >
    params = []
    for part in type_bounds.split(','):
        part = part.strip()
        # Get the identifier before any : or whitespace
        match = re.match(r'([A-Z]\w*)', part)
        if match:
            params.append(match.group(1))
    
    return params

def count_braces_in_line(line):
    """Count braces in a line, ignoring those in strings and comments."""
    in_string = False
    in_char = False
    escape = False
    open_count = 0
    close_count = 0
    
    i = 0
    while i < len(line):
        c = line[i]
        
        if escape:
            escape = False
            i += 1
            continue
        
        if c == '\\':
            escape = True
            i += 1
            continue
        
        if c == '"' and not in_char:
            in_string = not in_string
            i += 1
            continue
        
        if c == "'" and not in_string:
            if i + 1 < len(line) and (line[i+1].isalpha() or line[i+1] == '_'):
                i += 1
                continue
            in_char = not in_char
            i += 1
            continue
        
        if not in_string and not in_char:
            if c == '{':
                open_count += 1
            elif c == '}':
                close_count += 1
        
        i += 1
    
    return (open_count, close_count)

def extract_impl_info(line):
    """Extract information from an impl line including type parameters."""
    line = re.sub(r'//.*$', '', line).strip()
    
    STANDARD_TRAITS = {
        'Eq', 'PartialEq', 'Ord', 'PartialOrd',
        'Debug', 'Display', 
        'Clone', 'Copy',
        'Hash', 
        'Default',
        'Drop',
        'IntoIterator', 'Iterator',
    }
    
    # Check for trait impl
    trait_match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)', line)
    if trait_match:
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        is_standard = trait_name in STANDARD_TRAITS
        type_params = extract_type_params(line)
        return ('trait', struct_name, trait_name, is_standard, type_params)
    
    # Check for inherent impl
    inherent_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{', line)
    if inherent_match:
        struct_name = inherent_match.group(1)
        type_params = extract_type_params(line)
        return ('inherent', struct_name, None, False, type_params)
    
    return None

def find_impl_blocks(lines):
    """Find all impl blocks with their type parameters."""
    impl_blocks = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        if not stripped.startswith('impl'):
            i += 1
            continue
        
        impl_info = extract_impl_info(stripped)
        if not impl_info:
            i += 1
            continue
        
        impl_type, struct_name, trait_name, is_standard, type_params = impl_info
        start_line = i
        
        # Count braces to find end
        open_b, close_b = count_braces_in_line(stripped)
        brace_count = open_b - close_b
        
        j = i + 1
        while j < len(lines) and brace_count > 0:
            open_b, close_b = count_braces_in_line(lines[j])
            brace_count += open_b - close_b
            j += 1
        
        end_line = j
        
        impl_blocks.append({
            'start': start_line,
            'end': end_line,
            'type': impl_type,
            'struct': struct_name,
            'trait': trait_name,
            'is_standard': is_standard,
            'type_params': type_params,
            'impl_line': stripped,
        })
        
        i = end_line
    
    return impl_blocks

def analyze_file(file_path):
    """Analyze a file for type parameter mismatches."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        return None
    
    impl_blocks = find_impl_blocks(lines)
    
    # Group by struct
    struct_impls = defaultdict(lambda: {'inherent': [], 'custom_traits': [], 'standard_traits': []})
    
    for block in impl_blocks:
        struct_name = block['struct']
        if block['type'] == 'inherent':
            struct_impls[struct_name]['inherent'].append(block)
        elif block['is_standard']:
            struct_impls[struct_name]['standard_traits'].append(block)
        else:
            struct_impls[struct_name]['custom_traits'].append(block)
    
    results = []
    
    for struct_name, impls in struct_impls.items():
        if not impls['inherent'] or not impls['custom_traits']:
            continue
        
        for inh_block in impls['inherent']:
            inh_params = inh_block['type_params']
            
            for trait_block in impls['custom_traits']:
                trait_params = trait_block['type_params']
                
                # Check if type params differ
                params_match = (inh_params == trait_params)
                
                results.append({
                    'file': str(file_path),
                    'struct': struct_name,
                    'trait': trait_block['trait'],
                    'inherent_params': inh_params,
                    'trait_params': trait_params,
                    'params_match': params_match,
                    'inherent_line': inh_block['impl_line'][:80],
                    'trait_line': trait_block['impl_line'][:80],
                })
    
    return results

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Check impl type parameter consistency")
    parser.add_argument('--file', type=str, help='Single file to analyze')
    parser.add_argument('--all', action='store_true', help='Analyze all src files')
    args = parser.parse_args()
    
    if args.file:
        results = analyze_file(Path(args.file))
        if results:
            for r in results:
                match_str = "✓ MATCH" if r['params_match'] else "✗ MISMATCH"
                print(f"\n{match_str}")
                print(f"  File: {r['file']}")
                print(f"  Struct: {r['struct']}")
                print(f"  Trait: {r['trait']}")
                print(f"  Inherent params: {r['inherent_params']}")
                print(f"  Trait params:    {r['trait_params']}")
    elif args.all:
        src_dir = Path('src')
        all_results = []
        
        for rs_file in sorted(src_dir.rglob('*.rs')):
            results = analyze_file(rs_file)
            if results:
                all_results.extend(results)
        
        # Count matches and mismatches
        matches = [r for r in all_results if r['params_match']]
        mismatches = [r for r in all_results if not r['params_match']]
        
        print(f"{'='*100}")
        print(f"TYPE PARAMETER MISMATCH ANALYSIS")
        print(f"{'='*100}\n")
        
        print(f"Total: {len(all_results)} struct(s) with inherent + trait impls")
        print(f"  ✓ Matching params:    {len(matches)}")
        print(f"  ✗ Mismatched params:  {len(mismatches)}\n")
        
        if mismatches:
            print(f"{'='*100}")
            print(f"MISMATCHES (these will need generic param substitution):")
            print(f"{'='*100}\n")
            
            for r in mismatches:
                print(f"{r['file']:60} | {r['struct']:20}")
                print(f"  Inherent: {' '.join(r['inherent_params'])}")
                print(f"  Trait:    {' '.join(r['trait_params'])}")
                print()
        
        if matches:
            print(f"{'='*100}")
            print(f"MATCHES (these should be easier to fix):")
            print(f"{'='*100}\n")
            
            for r in matches:
                print(f"{r['file']:60} | {r['struct']:20} | {' '.join(r['inherent_params'])}")
    else:
        print("Error: Use --file or --all", file=sys.stderr)
        return 1
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

