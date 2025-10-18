#!/usr/bin/env python3
"""
Automatically convert inherent impl to trait impl.

Handles the mechanical transformation:
1. Find inherent impl with generics
2. Extract all public methods (signatures only for trait)
3. Create trait definition before impl
4. Convert impl to trait impl (remove 'pub' from methods)

Git commit: eb9a2676c4e7f5e0c3e8f0e6e5d5e5d5
"""

import re
import sys
from pathlib import Path
import argparse

def has_existing_trait_for_struct(lines, struct_name):
    """
    Check if a trait already exists for this struct.
    Returns True if we find both:
    1. A pub trait definition with similar name pattern
    2. A trait impl for the struct (impl Trait for Struct)
    """
    # Match traits with struct name prefix (e.g., ArraySeqStEphTrait for ArraySeqStEphS)
    # Extract base name without suffix (remove S/M/T/etc at end)
    base_name = re.sub(r'[A-Z]$', '', struct_name)  # ArraySeqStEphS -> ArraySeqStEph
    
    trait_pattern = re.compile(rf'pub\s+trait\s+{re.escape(base_name)}\w*Trait')
    impl_pattern = re.compile(rf'impl<[^>]*>\s+\w*Trait<[^>]*>\s+for\s+{re.escape(struct_name)}')
    
    has_trait = False
    has_impl = False
    
    for line in lines:
        if trait_pattern.search(line):
            has_trait = True
        if impl_pattern.search(line):
            has_impl = True
    
    return has_trait and has_impl

def is_struct_public(lines, struct_name):
    """
    Check if a struct is public (has 'pub' keyword).
    Returns True if struct is public, False if private.
    """
    struct_pattern = re.compile(rf'^\s*pub\s+struct\s+{re.escape(struct_name)}\b')
    
    for line in lines:
        if struct_pattern.search(line):
            return True
    
    # If we find a private struct definition
    private_pattern = re.compile(rf'^\s*struct\s+{re.escape(struct_name)}\b')
    for line in lines:
        if private_pattern.search(line):
            return False
    
    # If struct not found, assume public (safer default)
    return True

def find_impl_block(lines, start_line):
    """Find the complete impl block starting at start_line (0-indexed)."""
    brace_count = 0
    impl_lines = []
    
    for i in range(start_line, len(lines)):
        line = lines[i]
        impl_lines.append((i, line))
        
        for char in line:
            if char == '{':
                brace_count += 1
            elif char == '}':
                brace_count -= 1
                if brace_count == 0:
                    return impl_lines, i
    
    return impl_lines, len(lines) - 1

def extract_public_methods(impl_lines):
    """Extract public method signatures from impl block."""
    methods = []
    current_method = []
    in_method = False
    brace_count = 0
    
    for line_num, line in impl_lines[1:]:  # Skip first line (impl header)
        stripped = line.strip()
        
        if stripped.startswith('pub fn '):
            if current_method and in_method:
                methods.append(current_method)
            current_method = [(line_num, line)]
            in_method = True
            brace_count = line.count('{') - line.count('}')
            
            # Handle one-liner methods (open and close brace on same line)
            if brace_count == 0 and '{' in line:
                methods.append(current_method)
                current_method = []
                in_method = False
        elif in_method:
            current_method.append((line_num, line))
            brace_count += line.count('{') - line.count('}')
            
            if brace_count == 0 and '{' in ''.join([l for _, l in current_method]):
                methods.append(current_method)
                current_method = []
                in_method = False
    
    return methods

def method_signature(method_lines):
    """Extract method signature (up to first '{') from method lines."""
    signature_parts = []
    for _, line in method_lines:
        if '{' in line:
            signature_parts.append(line.split('{')[0].strip())
            break
        signature_parts.append(line.rstrip())
    
    return ' '.join(signature_parts)

def create_trait_def(struct_name, generics, methods, impl_line_num, is_public=True):
    """Create trait definition lines."""
    trait_name = f"{struct_name}Trait"
    
    trait_lines = []
    visibility = "pub " if is_public else ""
    trait_lines.append((impl_line_num, f"    {visibility}trait {trait_name}<{generics}> {{"))
    
    for method in methods:
        sig = method_signature(method)
        # Remove 'pub' and add semicolon
        sig = sig.replace('pub fn ', 'fn ')
        if not sig.endswith(';'):
            sig += ';'
        
        # Add method doc comments if they exist
        indent = 8  # Default indent for trait methods
        for line_num, line in method:
            if line.strip().startswith('///') or line.strip().startswith('//'):
                indent = len(line) - len(line.lstrip())
                trait_lines.append((line_num, line))
            elif line.strip().startswith('pub fn '):
                # Use indent from method line if no comments
                method_indent = len(line) - len(line.lstrip())
                trait_lines.append((line_num, ' ' * method_indent + sig))
                break
    
    trait_lines.append((impl_line_num, "    }"))
    trait_lines.append((impl_line_num, ""))
    
    return trait_lines, trait_name

def main():
    parser = argparse.ArgumentParser(description='Auto-convert inherent impl to trait impl')
    parser.add_argument('file', help='Source file to convert')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    filepath = Path(args.file)
    if not filepath.exists():
        print(f"ERROR: File not found: {filepath}")
        return 1
    
    lines = filepath.read_text().split('\n')
    
    # Find inherent impl with generics (not trait impl - no 'for')
    impl_pattern = re.compile(r'^\s*impl<([^>]+)>\s+(\w+)<([^>]+)>')
    
    impl_line_num = None
    struct_name = None
    generics = None  # Full generic bounds
    type_params = None  # Just the type parameter names
    
    for i, line in enumerate(lines):
        match = impl_pattern.match(line)
        if match and ' for ' not in line and '{' in line:
            generics = match.group(1)
            struct_name = match.group(2)
            type_params = match.group(3)  # Type params on struct (may have bounds)
            impl_line_num = i
            break
    
    if not impl_line_num:
        print(f"No inherent impl found in {filepath}")
        return 0
    
    # Check if trait already exists for this struct
    if has_existing_trait_for_struct(lines, struct_name):
        print(f"✗ SKIPPED {filepath}")
        print(f"  {struct_name} already has a trait impl (complex refactoring needed)")
        return 1  # Non-zero to indicate skip
    
    # Check if struct is public or private
    is_public = is_struct_public(lines, struct_name)
    visibility_str = "public" if is_public else "private"
    
    print(f"Found inherent impl at line {impl_line_num + 1}: impl<{generics}> {struct_name}")
    print(f"  Struct is {visibility_str}")
    
    # Extract impl block
    impl_block, impl_end_line = find_impl_block(lines, impl_line_num)
    
    # Extract public methods
    public_methods = extract_public_methods(impl_block)
    
    print(f"Found {len(public_methods)} public methods")
    
    if not public_methods:
        print("No public methods to convert")
        return 0
    
    if args.dry_run:
        print("\n[DRY RUN] Would create trait and convert impl")
        return 0
    
    # Create trait definition
    trait_lines, trait_name = create_trait_def(struct_name, generics, public_methods, impl_line_num, is_public)
    
    # Modify impl header to be trait impl
    # Extract all param names including lifetimes (e.g., "'a, V" from "'a, V: StT + Hash")
    all_param_names = []
    for param in type_params.split(','):
        # Get just the name before any colon
        name = param.split(':')[0].strip()
        if name:
            all_param_names.append(name)
    
    type_params_only = ', '.join(all_param_names)
    
    impl_header = lines[impl_line_num]
    new_impl_header = impl_header.replace(
        f"impl<{generics}> {struct_name}<{type_params}>",
        f"impl<{generics}> {trait_name}<{type_params_only}> for {struct_name}<{type_params_only}>"
    )
    
    # Remove 'pub' from all methods in impl
    new_lines = lines[:impl_line_num]
    
    # Insert trait definition
    for _, line in trait_lines:
        new_lines.append(line)
    
    # Add modified impl
    new_lines.append(new_impl_header)
    
    for line_num, line in impl_block[1:]:
        # Remove 'pub' from method definitions
        if line.strip().startswith('pub fn '):
            indent = len(line) - len(line.lstrip())
            new_line = ' ' * indent + line.lstrip().replace('pub fn ', 'fn ', 1)
            new_lines.append(new_line)
        else:
            new_lines.append(line)
    
    # Add all lines after the impl block
    new_lines.extend(lines[impl_end_line + 1:])
    
    # Write back
    filepath.write_text('\n'.join(new_lines))
    trait_visibility = "pub " if is_public else ""
    print(f"✓ Converted {filepath}")
    print(f"  Created: {trait_visibility}trait {trait_name}")
    print(f"  Converted: impl<{generics}> {struct_name} -> trait impl")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())

