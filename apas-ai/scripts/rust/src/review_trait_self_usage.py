#!/usr/bin/env python3
"""
Review script to detect trait methods using concrete types instead of Self in return types.

This script finds traits where methods return Type<...> or &Type<...> or &mut Type<...>
instead of Self, &Self, or &mut Self, which is more idiomatic and flexible.

Example violation:
    pub trait SetTrait<T> {
        fn empty() -> Set<T>;  // Should be: fn empty() -> Self;
        fn insert(&mut self, x: T) -> &mut Set<T>;  // Should be: -> &mut Self;
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
    # Handle: pub trait MyTrait<T> or trait MyTrait or trait MyTrait: Sized
    match = re.search(r'\btrait\s+(\w+)', trait_line)
    if match:
        return match.group(1)
    return None


def find_impl_struct_name(lines, trait_name):
    """Find the struct name that implements this trait in the same file."""
    # Look for: impl<...> TraitName<...> for StructName<...>
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


def extract_return_type(method_sig):
    """Extract the return type from a method signature."""
    # Handle: fn foo() -> Type or fn foo() -> &Type or fn foo() -> &mut Type
    match = re.search(r'->\s*(&\s*mut\s+|&\s*)?(\w+)(<[^>]*>)?', method_sig)
    if match:
        ref_mut = (match.group(1) or '').strip()
        type_name = match.group(2)
        generics = match.group(3) or ''
        return ref_mut, type_name, generics
    return None, None, None


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


def main():
    parser = create_review_parser(
        description="Detect trait methods using concrete types instead of Self in return types"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Collect all Rust files
    dirs_to_check = []
    for dir_name in ['src', 'tests', 'benches']:
        dir_path = context.repo_root / dir_name
        if dir_path.exists():
            dirs_to_check.append(dir_path)
    
    if not dirs_to_check:
        print("✓ No src/, tests/, or benches/ directories found")
        return 0
    
    if context.dry_run:
        files = context.find_files(dirs_to_check)
        print(f"Would check {len(files)} file(s) for trait Self usage in {len(dirs_to_check)} directories")
        return 0
    
    files = context.find_files(dirs_to_check)
    print(f"Reviewing {len(files)} Rust files for trait Self usage...")
    
    all_violations = []
    files_with_violations = {}
    
    for filepath in files:
        violations = review_file_with_count(filepath)
        if violations:
            all_violations.extend(violations)
            files_with_violations[filepath] = violations
    
    if all_violations:
        print(f"\n✗ Found {len(all_violations)} violation(s) in {len(files_with_violations)} file(s):\n")
        
        # Group by file and print details
        for filepath, violations in sorted(files_with_violations.items()):
            rel_path = context.relative_path(filepath)
            print(f"\n{rel_path}: {len(violations)} violation(s)")
            for v in violations:
                print(f"  Line {v['line']}: {v['trait']}::{v['method']}() -> {v['current']}")
                print(f"    Should be: -> {v['should_be']}")
                if v['struct']:
                    print(f"    Implemented for: {v['struct']}")
        
        return 1
    else:
        print("✓ Trait Self Usage: No violations found")
        return 0


def review_file_with_count(filepath):
    """Review a file and return violations list."""
    violations = []
    
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception:
        return violations

    in_trait = False
    trait_name = None
    trait_start_line = 0
    trait_line = None
    trait_generics = []
    brace_depth = 0
    struct_name = None
    
    for line_num, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Skip comments
        if stripped.startswith('//'):
            continue
        
        # Detect trait start
        if not in_trait and 'trait ' in line and '{' in line:
            trait_name = extract_trait_name_from_signature(line)
            if trait_name:
                in_trait = True
                trait_start_line = line_num
                trait_line = line
                trait_generics = extract_trait_generic_params(line)
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
                # Extract method signature (might span multiple lines, but we'll handle simple cases)
                ref_mut, return_type_name, generics = extract_return_type(line)
                
                if return_type_name and return_type_name != 'Self':
                    # Check if this should be Self
                    if should_use_self(return_type_name, generics, struct_name, trait_name, trait_generics):
                        method_match = re.search(r'fn\s+(\w+)', line)
                        method_name = method_match.group(1) if method_match else 'unknown'
                        
                        # Construct what it should be
                        should_be = f"{ref_mut} Self" if ref_mut else "Self"
                        current = f"{ref_mut} {return_type_name}{generics}" if ref_mut else f"{return_type_name}{generics}"
                        
                        violations.append({
                            'line': line_num,
                            'trait': trait_name,
                            'method': method_name,
                            'current': current.strip(),
                            'should_be': should_be.strip(),
                            'struct': struct_name,
                            'filepath': filepath
                        })
    
    return violations


if __name__ == '__main__':
    sys.exit(main())

