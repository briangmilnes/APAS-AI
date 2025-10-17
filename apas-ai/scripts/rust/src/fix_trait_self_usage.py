#!/usr/bin/env python3
"""
Fix script to replace concrete types with Self in trait return types.

This script automatically changes trait method return types from concrete types
(e.g., SetStEph<T>, MappingStEph<X,Y>) to Self when appropriate.

Example fix:
    pub trait SetTrait<T> {
        fn empty() -> Set<T>;  // BEFORE
        fn empty() -> Self;    // AFTER
    }
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


def extract_trait_name_from_signature(trait_line):
    """Extract trait name from trait definition line."""
    match = re.search(r'\btrait\s+(\w+)', trait_line)
    if match:
        return match.group(1)
    return None


def find_impl_struct_name(lines, trait_name):
    """Find the struct name that implements this trait in the same file."""
    impl_pattern = re.compile(rf'\bimpl.*\b{re.escape(trait_name)}\b.*\bfor\s+(\w+)')
    
    for line in lines:
        match = impl_pattern.search(line)
        if match:
            return match.group(1)
    
    return None


def extract_trait_generic_params(trait_line):
    """Extract generic parameters from trait definition."""
    # Extract <T>, <T, U>, etc. from trait definition
    match = re.search(r'trait\s+\w+<([^>]+)>', trait_line)
    if match:
        params = match.group(1)
        # Split by comma and clean up
        return [p.split(':')[0].strip() for p in params.split(',')]
    return []


def should_use_self(return_type_name, generics, struct_name, trait_name, trait_generics):
    """Determine if a return type should use Self instead of concrete type."""
    if not struct_name:
        return False
    
    # If the return type doesn't match the struct name, it's not Self
    if return_type_name != struct_name:
        # Also check if it's the trait name without "Trait" suffix
        if trait_name.endswith('Trait'):
            base_name = trait_name[:-5]
            if return_type_name != base_name:
                return False
        else:
            return False
    
    # Now check if the generics match
    # Self means the same type with the same generic parameters as the trait
    # e.g., if trait is SetTrait<T>, Self means Set<T>, not Set<Pair<T, U>>
    
    if not generics or generics == '<>':
        # No generics in return type, but struct has generics? Probably wrong
        if trait_generics:
            return False
        return True
    
    # Extract generic params from return type
    # e.g., "<T>" -> ["T"], "<Pair<T, U>>" -> ["Pair<T, U>"]
    gen_content = generics[1:-1]  # Remove < and >
    
    # Simple check: if generics exactly match trait generics, it's Self
    # e.g., <T> matches trait SetTrait<T>
    # but <Pair<T, U>> does not match trait SetTrait<T>
    
    if len(trait_generics) == 1:
        # Single generic parameter
        if gen_content.strip() == trait_generics[0]:
            return True
        # Check for simple cases like T, U, X, Y
        if gen_content.strip() in trait_generics:
            return True
    
    # For multi-param generics, check if they match exactly
    gen_params = [p.strip() for p in gen_content.split(',')]
    if gen_params == trait_generics:
        return True
    
    return False


def fix_file(filepath, context):
    """Fix a single Rust file by replacing concrete types with Self in traits."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return 0

    original_lines = lines[:]
    in_trait = False
    trait_name = None
    trait_line = None
    trait_generics = []
    trait_start_line = 0
    brace_depth = 0
    struct_name = None
    fixes_made = 0
    
    for line_num, line in enumerate(lines):
        stripped = line.strip()
        
        # Skip comments
        if stripped.startswith('//'):
            continue
        
        # Detect trait start
        if not in_trait and 'trait ' in line and '{' in line:
            trait_name = extract_trait_name_from_signature(line)
            if trait_name:
                in_trait = True
                trait_line = line
                trait_generics = extract_trait_generic_params(line)
                trait_start_line = line_num
                brace_depth = line.count('{') - line.count('}')
                # Find the implementing struct name
                struct_name = find_impl_struct_name(lines, trait_name)
                continue
        
        if in_trait:
            # Track brace depth
            brace_depth += line.count('{') - line.count('}')
            
            # Check if we're still in the trait
            if brace_depth <= 0:
                in_trait = False
                trait_name = None
                trait_generics = []
                struct_name = None
                continue
            
            # Look for method signatures with return types
            if 'fn ' in line and '->' in line and not stripped.startswith('//'):
                # Extract return type - handle various patterns
                # Pattern: -> Type<...> or -> &Type<...> or -> &mut Type<...>
                match = re.search(r'->\s*(&\s*mut\s+|&\s*)?(\w+)(<[^>]*>)?', line)
                
                if match:
                    ref_mut = (match.group(1) or '').strip()
                    return_type_name = match.group(2)
                    generics = match.group(3) or ''
                    
                    if return_type_name != 'Self' and should_use_self(return_type_name, generics, struct_name, trait_name, trait_generics):
                        # Build the replacement
                        old_return = f"{ref_mut} {return_type_name}{generics}" if ref_mut else f"{return_type_name}{generics}"
                        new_return = f"{ref_mut} Self" if ref_mut else "Self"
                        
                        # Replace in the line, preserving spacing
                        new_line = line.replace(f"-> {old_return.strip()}", f"-> {new_return.strip()}")
                        
                        if new_line != line:
                            lines[line_num] = new_line
                            fixes_made += 1
                            
                            method_match = re.search(r'fn\s+(\w+)', line)
                            method_name = method_match.group(1) if method_match else 'unknown'
                            print(f"  Line {line_num + 1}: {method_name}() -> {old_return.strip()} => -> {new_return.strip()}")
    
    # Write back if changes were made
    if fixes_made > 0:
        try:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.writelines(lines)
            print(f"✓ Fixed {fixes_made} return type(s) in {context.relative_path(filepath)}")
        except Exception as e:
            print(f"Error writing {filepath}: {e}")
            return 0
    
    return fixes_made


def main():
    parser = create_review_parser(
        description="Fix trait methods to use Self instead of concrete types in return types"
    )
    parser.add_argument(
        'files',
        nargs='*',
        help='Specific files to fix (if not provided, will prompt)'
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    if args.files:
        # Fix specific files provided as arguments
        files_to_fix = [Path(f) for f in args.files]
    else:
        print("No files specified. Use: fix_trait_self_usage.py file1.rs file2.rs ...")
        return 1

    total_fixes = 0
    for filepath in files_to_fix:
        if not filepath.exists():
            # Try relative to repo root
            filepath = context.repo_root / filepath
        
        if not filepath.exists():
            print(f"✗ File not found: {filepath}")
            continue
        
        print(f"\nFixing {context.relative_path(filepath)}...")
        fixes = fix_file(filepath, context)
        total_fixes += fixes

    print(f"\n{'='*70}")
    print(f"Total: Fixed {total_fixes} return type(s) in {len(files_to_fix)} file(s)")
    return 0


if __name__ == '__main__':
    sys.exit(main())

