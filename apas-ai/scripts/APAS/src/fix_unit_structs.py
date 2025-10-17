#!/usr/bin/env python3
"""
Fix: Convert unit structs with algorithmic impl blocks to modules with traits.

APASRules.md Lines 183-188: Unit structs with only methods should be converted
to modules with documentary traits + free functions.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def fix_unit_struct(file_path, struct_name, dry_run=False):
    """Convert a unit struct to module with trait + free functions."""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Find the unit struct definition
    struct_pattern = re.compile(rf'^\s*pub struct {struct_name};\s*$', re.MULTILINE)
    struct_match = struct_pattern.search(content)
    
    if not struct_match:
        return False, "Unit struct not found"
    
    # Find the impl block
    impl_pattern = re.compile(
        rf'^\s*impl {struct_name} \{{\s*$'
        r'(.*?)'
        r'^\s*\}\s*$',
        re.MULTILINE | re.DOTALL
    )
    impl_match = impl_pattern.search(content)
    
    if not impl_match:
        return False, "Impl block not found"
    
    impl_body = impl_match.group(1)
    
    # Extract method signatures for the trait
    method_pattern = re.compile(
        r'^\s*((?:\/\/\/.*\n\s*)*)'  # Doc comments
        r'pub fn (\w+)<?(.*?)>?\((.*?)\)(?: -> (.*?))?\s*\{',
        re.MULTILINE
    )
    
    methods = []
    for match in method_pattern.finditer(impl_body):
        doc_comment = match.group(1).strip()
        fn_name = match.group(2)
        generics = match.group(3)
        params = match.group(4)
        return_type = match.group(5)
        
        # Build trait signature
        trait_sig = f"        fn {fn_name}"
        if generics:
            trait_sig += f"<{generics}>"
        trait_sig += f"({params})"
        if return_type:
            trait_sig += f" -> {return_type}"
        trait_sig += ";"
        
        if doc_comment:
            methods.append(f"        {doc_comment}\n{trait_sig}")
        else:
            methods.append(trait_sig)
    
    if not methods:
        return False, "No public methods found"
    
    # Build the new module structure
    trait_methods = "\n\n".join(methods)
    
    # Convert impl body: replace Self:: with nothing (since they'll be free functions)
    new_impl_body = impl_body.replace('Self::', '')
    
    new_module = f'''    pub mod {struct_name} {{
        use crate::Types::Types::*;

        // A dummy trait as a minimal type checking comment and space for algorithmic analysis.
        pub trait {struct_name}Trait {{
{trait_methods}
        }}

{new_impl_body}    }}'''
    
    # Replace the struct definition and impl block with the module
    # First remove the impl block
    content = impl_pattern.sub('', content)
    # Then replace the struct definition with the module
    content = struct_pattern.sub(new_module, content)
    
    changed = content != original_content
    
    if changed and not dry_run:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
    
    return changed, "Fixed"


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Fix unit struct algorithmic patterns.")
    parser.add_argument('--file', type=str, help="Specify a single file to fix.")
    parser.add_argument('--struct', type=str, help="Specify struct name to fix.")
    parser.add_argument('--dry-run', action='store_true', help="Show changes without writing.")
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    
    # Known unit structs to fix
    unit_structs = [
        ('src/Chap47/AdvancedDoubleHashing.rs', 'RelativePrimeValidator'),
        ('src/Chap47/AdvancedQuadraticProbing.rs', 'PrimeValidator'),
        ('src/Chap47/HashFunctionTraits.rs', 'HashTableUtils'),
        ('src/Chap47/HashFunctionTraits.rs', 'HashFunctionTester'),
        ('src/Chap47clean/DoubleHashFlatHashTable.rs', 'DoubleHashFlatHashTableStEph'),
    ]
    
    if args.file and args.struct:
        file_path = repo_root / args.file
        if not file_path.exists():
            print(f"Error: File not found at {file_path}")
            return 1
        
        changed, msg = fix_unit_struct(file_path, args.struct, args.dry_run)
        if changed:
            if args.dry_run:
                print(f"Would fix {args.struct} in {args.file}: {msg}")
            else:
                print(f"✓ Fixed {args.struct} in {args.file}")
        else:
            print(f"✗ Could not fix {args.struct} in {args.file}: {msg}")
        return 0 if changed else 1
    
    # Fix all known unit structs
    fixed_count = 0
    for file_rel_path, struct_name in unit_structs:
        file_path = repo_root / file_rel_path
        if not file_path.exists():
            print(f"Warning: File not found: {file_rel_path}")
            continue
        
        changed, msg = fix_unit_struct(file_path, struct_name, args.dry_run)
        if changed:
            fixed_count += 1
            if not args.dry_run:
                print(f"✓ Fixed {struct_name} in {file_rel_path}")
    
    if args.dry_run:
        print(f"\nWould fix {fixed_count} unit structs")
    else:
        print(f"\n✓ Fixed {fixed_count} unit struct algorithmic patterns")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

