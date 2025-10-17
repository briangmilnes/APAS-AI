#!/usr/bin/env python3
"""
Fix missing trait methods by adding them to trait definition and trait impl.

This script:
1. Detects methods in inherent impl that are missing from trait
2. Adds missing method signatures to the trait definition
3. Copies method implementations from inherent impl to trait impl
4. Optionally removes the inherent impl (with --remove-inherent flag)
"""
# Git commit: TBD
# Date: 2025-10-17

import re
import sys
from pathlib import Path

# Import the detection logic
sys.path.insert(0, str(Path(__file__).parent))
from detect_missing_trait_methods import analyze_file, find_impl_blocks, find_trait_definition

def extract_method_body(lines, start_line, method_name):
    """
    Extract the full method implementation from inherent impl.
    Returns (signature_lines, body_lines, end_line).
    """
    i = start_line
    
    # Collect signature lines (until we hit '{')
    sig_lines = []
    while i < len(lines):
        line = lines[i]
        sig_lines.append(line)
        if '{' in line:
            break
        i += 1
    
    if i >= len(lines):
        return None, None, None
    
    # Now collect body lines (count braces to find end)
    brace_count = line.count('{') - line.count('}')
    body_start = i
    i += 1
    
    while i < len(lines) and brace_count > 0:
        line = lines[i]
        brace_count += line.count('{') - line.count('}')
        i += 1
    
    body_end = i
    body_lines = lines[body_start:body_end]
    
    return sig_lines, body_lines, body_end

def add_methods_to_trait_def(lines, trait_def, missing_methods):
    """
    Add missing method signatures to the trait definition.
    Returns modified lines.
    """
    if not trait_def or not missing_methods:
        return lines
    
    new_lines = lines[:]
    trait_end = trait_def['end']
    
    # Find the last non-empty, non-comment line before the closing brace
    insert_pos = trait_end - 1
    while insert_pos > trait_def['start']:
        line = new_lines[insert_pos].strip()
        if line and not line.startswith('//') and line != '}':
            insert_pos += 1
            break
        insert_pos -= 1
    
    # Prepare method signatures to add
    lines_to_add = ["\n"]
    lines_to_add.append("        // Methods added from inherent impl\n")
    
    for method in missing_methods:
        # Convert implementation signature to trait signature
        sig = method['signature']
        
        # Remove 'pub' keyword (traits don't use pub on method signatures)
        sig = re.sub(r'\bpub\s+', '', sig)
        
        # If it has a body, make it just a signature
        if '{' in sig:
            sig = sig[:sig.index('{')].strip()
        
        # Ensure it ends with semicolon
        if not sig.endswith(';'):
            sig += ';'
        
        lines_to_add.append(f"        {sig}\n")
    
    # Insert lines
    new_lines[insert_pos:insert_pos] = lines_to_add
    
    return new_lines

def add_methods_to_trait_impl(lines, trait_impl_location, inherent_impl_location, missing_methods):
    """
    Copy missing method implementations from inherent impl to trait impl.
    Returns modified lines and new inherent impl end location.
    """
    trait_impl_start, trait_impl_end = trait_impl_location
    inh_impl_start, inh_impl_end = inherent_impl_location
    
    new_lines = lines[:]
    
    # Find insert position in trait impl (before closing brace)
    insert_pos = trait_impl_end - 1
    while insert_pos > trait_impl_start:
        line = new_lines[insert_pos].strip()
        if line and not line.startswith('//') and line != '}':
            insert_pos += 1
            break
        insert_pos -= 1
    
    # Extract method bodies from inherent impl and prepare to add to trait impl
    methods_to_add = []
    methods_to_add.append("\n")
    methods_to_add.append("        // Methods moved from inherent impl\n")
    
    # Process each missing method
    for method in missing_methods:
        method_name = method['name']
        
        # Find the method in inherent impl section of lines
        for i in range(inh_impl_start + 1, inh_impl_end):
            line = new_lines[i]
            if f'fn {method_name}' in line:
                # Extract full method
                sig_lines, body_lines, method_end = extract_method_body(new_lines, i, method_name)
                
                if sig_lines and body_lines:
                    # Add the complete method
                    methods_to_add.extend(sig_lines)
                    if sig_lines[-1].strip().endswith('{'):
                        # Body is on following lines
                        methods_to_add.extend(body_lines[1:])  # Skip the line with just '{'
                    else:
                        methods_to_add.extend(body_lines)
                    methods_to_add.append("\n")
                break
    
    # Insert methods into trait impl
    new_lines[insert_pos:insert_pos] = methods_to_add
    
    # Update inherent impl end location (it shifted down)
    offset = len(methods_to_add)
    new_inh_impl_end = inh_impl_end + offset if inh_impl_end > insert_pos else inh_impl_end
    
    return new_lines, new_inh_impl_end

def remove_inherent_impl(lines, inherent_impl_location):
    """
    Remove the inherent impl block.
    Returns modified lines.
    """
    start, end = inherent_impl_location
    
    # Remove any blank lines after the impl block too
    while end < len(lines) and lines[end].strip() == '':
        end += 1
    
    new_lines = lines[:start] + lines[end:]
    return new_lines

def fix_file(file_path, dry_run=False, remove_inherent=False):
    """
    Fix a file by adding missing methods to trait and optionally removing inherent impl.
    """
    results = analyze_file(file_path)
    
    if not results:
        return False
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    modified = False
    
    for result in results:
        missing_methods = result['missing_from_trait_def']
        
        if not missing_methods:
            continue
        
        if dry_run:
            print(f"\nWould fix {file_path}:")
            print(f"  Struct: {result['struct']}")
            print(f"  Trait: {result['trait']}")
            print(f"  Adding {len(missing_methods)} method(s) to trait definition")
            print(f"  Moving {len(missing_methods)} method(s) to trait impl")
            if remove_inherent:
                print(f"  Removing inherent impl")
            for m in missing_methods:
                print(f"    - {m['name']} ({'public' if m['public'] else 'private'})")
            continue
        
        # Step 1: Add methods to trait definition
        lines = add_methods_to_trait_def(lines, result['trait_def_location'], missing_methods)
        
        # Step 2: Copy methods to trait impl
        lines, new_inh_end = add_methods_to_trait_impl(
            lines,
            result['trait_impl_location'],
            result['inherent_impl_location'],
            missing_methods
        )
        
        # Step 3: Optionally remove inherent impl
        if remove_inherent:
            lines = remove_inherent_impl(lines, (result['inherent_impl_location'][0], new_inh_end))
        
        modified = True
    
    if modified and not dry_run:
        try:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.writelines(lines)
            
            print(f"Fixed: {file_path}")
            for result in results:
                if result['missing_from_trait_def']:
                    print(f"  Added {len(result['missing_from_trait_def'])} method(s) to {result['trait']}")
                    if remove_inherent:
                        print(f"  Removed inherent impl for {result['struct']}")
            return True
        except Exception as e:
            print(f"Error writing {file_path}: {e}", file=sys.stderr)
            return False
    
    return modified

def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Fix missing trait methods by adding them from inherent impl"
    )
    parser.add_argument('--file', type=str, help='Single file to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    parser.add_argument('--remove-inherent', action='store_true', 
                       help='Remove inherent impl after moving methods')
    args = parser.parse_args()
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.exists():
            print(f"Error: {file_path} not found", file=sys.stderr)
            return 1
        
        changed = fix_file(file_path, dry_run=args.dry_run, remove_inherent=args.remove_inherent)
        return 0 if changed or args.dry_run else 1
    else:
        print("Error: --file argument required", file=sys.stderr)
        return 1

if __name__ == '__main__':
    sys.exit(main())

