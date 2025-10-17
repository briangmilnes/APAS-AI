#!/usr/bin/env python3
"""
Detect methods in inherent impl that are missing from the trait.

For structs with both inherent impl and custom trait impl, identifies methods
that exist in the inherent impl but are NOT present in the trait definition.
These must be added to the trait before the inherent impl can be removed.

Filters out standard library traits (IntoIterator, Debug, Clone, etc.) and
only reports on custom project traits (typically ending in "Trait").
"""
# Git commit: [New script - to be committed]
# Date: 2025-10-17
# Updated: 2025-10-17 - Added comprehensive stdlib trait filtering

import re
import sys
from pathlib import Path
from collections import defaultdict

def extract_method_signature(line):
    """Extract method name, visibility, and full signature from a method line."""
    line = re.sub(r'//.*$', '', line).strip()
    
    # Match: [pub] [unsafe] [async] fn method_name[<generics>](params) [-> return_type] [where ...]
    match = re.search(r'\b(pub)?\s*(unsafe)?\s*(async)?\s*fn\s+(\w+)', line)
    if match:
        is_public = match.group(1) == 'pub'
        method_name = match.group(4)
        return {
            'name': method_name,
            'public': is_public,
            'signature': line,
        }
    return None

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
    """Extract information from an impl line."""
    line = re.sub(r'//.*$', '', line).strip()
    
    # Standard library traits that we should NOT modify
    STANDARD_TRAITS = {
        # Comparison
        'Eq', 'PartialEq', 'Ord', 'PartialOrd',
        # Formatting
        'Debug', 'Display', 'Binary', 'Octal', 'LowerHex', 'UpperHex', 'LowerExp', 'UpperExp', 'Pointer',
        # Memory
        'Clone', 'Copy', 'Drop',
        # Conversion
        'From', 'Into', 'TryFrom', 'TryInto', 'AsRef', 'AsMut', 'Borrow', 'BorrowMut', 'ToOwned',
        # Iteration
        'Iterator', 'IntoIterator', 'DoubleEndedIterator', 'ExactSizeIterator', 'Extend', 'FromIterator',
        # Indexing
        'Index', 'IndexMut',
        # Operators
        'Add', 'Sub', 'Mul', 'Div', 'Rem', 'Neg', 'Not', 
        'BitAnd', 'BitOr', 'BitXor', 'Shl', 'Shr',
        'AddAssign', 'SubAssign', 'MulAssign', 'DivAssign', 'RemAssign',
        'BitAndAssign', 'BitOrAssign', 'BitXorAssign', 'ShlAssign', 'ShrAssign',
        # Smart pointers
        'Deref', 'DerefMut',
        # Hash
        'Hash', 'Hasher', 'BuildHasher',
        # Default
        'Default',
        # Concurrency
        'Send', 'Sync', 'Unpin',
        # Error handling
        'Error',
        # Fn traits
        'Fn', 'FnMut', 'FnOnce',
    }
    
    # Check for trait impl
    trait_match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)', line)
    if trait_match:
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        is_standard = trait_name in STANDARD_TRAITS
        return ('trait', struct_name, trait_name, is_standard)
    
    # Check for inherent impl
    inherent_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{', line)
    if inherent_match:
        struct_name = inherent_match.group(1)
        return ('inherent', struct_name, None, False)
    
    return None

def find_methods_in_impl(lines, start, end):
    """Find all methods in an impl block with full context."""
    methods = []
    
    i = start + 1
    while i < end:
        line = lines[i].strip()
        
        # Skip comments and empty lines
        if not line or line.startswith('//'):
            i += 1
            continue
        
        # Check for method signature
        method_info = extract_method_signature(line)
        if method_info:
            # Collect full method signature (may span multiple lines)
            full_sig_lines = [line]
            
            # Keep reading until we hit the opening brace or semicolon
            j = i + 1
            while j < end:
                next_line = lines[j].strip()
                full_sig_lines.append(next_line)
                if '{' in next_line or ';' in next_line:
                    break
                j += 1
            
            method_info['full_signature'] = ' '.join(full_sig_lines)
            method_info['line'] = i + 1
            methods.append(method_info)
        
        i += 1
    
    return methods

def find_impl_blocks(lines):
    """Find all impl blocks with their methods."""
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
        
        impl_type, struct_name, trait_name, is_standard = impl_info
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
        
        # Extract methods
        methods = find_methods_in_impl(lines, start_line, end_line)
        
        impl_blocks.append({
            'start': start_line,
            'end': end_line,
            'type': impl_type,
            'struct': struct_name,
            'trait': trait_name,
            'is_standard': is_standard,
            'methods': methods,
        })
        
        i = end_line
    
    return impl_blocks

def find_trait_definition(lines, trait_name):
    """Find the trait definition block."""
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        
        # Look for: pub trait TraitName
        if re.match(rf'\bpub\s+trait\s+{re.escape(trait_name)}\b', line):
            start_line = i
            
            # Find the end of the trait
            open_b, close_b = count_braces_in_line(line)
            brace_count = open_b - close_b
            
            j = i + 1
            while j < len(lines) and brace_count > 0:
                open_b, close_b = count_braces_in_line(lines[j])
                brace_count += open_b - close_b
                j += 1
            
            end_line = j
            
            # Extract method signatures from trait
            trait_methods = []
            for k in range(start_line + 1, end_line):
                method_info = extract_method_signature(lines[k])
                if method_info:
                    trait_methods.append(method_info['name'])
            
            return {
                'start': start_line,
                'end': end_line,
                'methods': trait_methods,
            }
        
        i += 1
    
    return None

def analyze_file(file_path):
    """Analyze a file for missing trait methods."""
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
            inherent_methods = inh_block['methods']
            
            for trait_block in impls['custom_traits']:
                trait_name = trait_block['trait']
                trait_impl_methods = {m['name'] for m in trait_block['methods']}
                
                # Find methods in inherent but not in trait impl
                missing_in_trait = [
                    m for m in inherent_methods
                    if m['name'] not in trait_impl_methods
                ]
                
                if missing_in_trait:
                    # Check if these methods are in the trait definition
                    trait_def = find_trait_definition(lines, trait_name)
                    trait_def_methods = set(trait_def['methods']) if trait_def else set()
                    
                    missing_from_trait_def = [
                        m for m in missing_in_trait
                        if m['name'] not in trait_def_methods
                    ]
                    
                    results.append({
                        'file': str(file_path),
                        'struct': struct_name,
                        'trait': trait_name,
                        'missing_methods': missing_in_trait,
                        'missing_from_trait_def': missing_from_trait_def,
                        'trait_def_location': trait_def,
                        'inherent_impl_location': (inh_block['start'], inh_block['end']),
                        'trait_impl_location': (trait_block['start'], trait_block['end']),
                    })
    
    return results

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Detect missing trait methods")
    parser.add_argument('--file', type=str, help='Single file to analyze')
    parser.add_argument('--all', action='store_true', help='Analyze all src files')
    args = parser.parse_args()
    
    if args.file:
        results = analyze_file(Path(args.file))
        if results:
            for r in results:
                print(f"\n{'='*80}")
                print(f"File: {r['file']}")
                print(f"Struct: {r['struct']}")
                print(f"Trait: {r['trait']}")
                print(f"\nMethods in inherent impl but NOT in trait impl:")
                for m in r['missing_methods']:
                    vis = "pub " if m['public'] else "    "
                    print(f"  {vis}{m['name']} (line {m['line']})")
                
                if r['missing_from_trait_def']:
                    print(f"\nMethods also missing from trait DEFINITION:")
                    for m in r['missing_from_trait_def']:
                        vis = "pub " if m['public'] else "    "
                        print(f"  {vis}{m['name']}")
                        print(f"      Signature: {m['signature'][:80]}...")
    elif args.all:
        src_dir = Path('src')
        all_results = []
        
        for rs_file in sorted(src_dir.rglob('*.rs')):
            results = analyze_file(rs_file)
            if results:
                all_results.extend(results)
        
        print(f"Found {len(all_results)} struct(s) with missing trait methods\n")
        
        for r in all_results:
            print(f"{r['file']:60} | {r['struct']:20} | {r['trait']:30} | {len(r['missing_methods'])} missing")
            for m in r['missing_from_trait_def']:
                print(f"    {'[PUB]' if m['public'] else '[priv]'} {m['name']}")
    else:
        print("Error: Use --file or --all", file=sys.stderr)
        return 1
    
    return 0

if __name__ == '__main__':
    sys.exit(main())

