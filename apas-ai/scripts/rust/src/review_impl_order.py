#!/usr/bin/env python3
"""
Review: Implementation order - standard traits should be at the bottom.

Malpattern:
1. Data structure (struct/enum)
2. Trait definition for that data structure
3. Standard trait impls (Eq, PartialEq, Debug, Display, etc.) <- WRONG POSITION
4. Inherent impl (impl Type { ... })
5. Custom trait impls

Correct order:
1. Data structure (struct/enum)
2. Trait definition for that data structure
3. Inherent impl (impl Type { ... })
4. Custom trait impls
5. Standard trait impls (Eq, PartialEq, Debug, Display, etc.) <- AT THE BOTTOM

Standard traits include: Eq, PartialEq, Ord, PartialOrd, Debug, Display, Clone, 
Copy, Hash, Default, From, Into, TryFrom, TryInto, Deref, DerefMut, Drop, etc.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Standard library traits that should come after custom impls
STANDARD_TRAITS = {
    'Eq', 'PartialEq', 'Ord', 'PartialOrd',
    'Debug', 'Display', 
    'Clone', 'Copy',
    'Hash', 
    'Default',
    'From', 'Into', 'TryFrom', 'TryInto',
    'AsRef', 'AsMut',
    'Deref', 'DerefMut',
    'Drop',
    'Iterator', 'IntoIterator',
    'Index', 'IndexMut',
    'Add', 'Sub', 'Mul', 'Div', 'Rem', 'Neg',  # Arithmetic ops
    'BitAnd', 'BitOr', 'BitXor', 'Shl', 'Shr',  # Bitwise ops
    'Not',
    'Send', 'Sync',
    'Fn', 'FnMut', 'FnOnce',
    'Error',
}


def extract_trait_name(impl_line):
    """
    Extract trait name from an impl line.
    Returns (trait_name, is_trait_impl)
    
    Examples:
    impl Eq for Foo -> ('Eq', True)
    impl<T> Display for Foo<T> -> ('Display', True)
    impl<T> std::fmt::Debug for Foo<T> -> ('Debug', True)
    impl<T: StT> MyTrait<T> for Foo<T> -> ('MyTrait', True)
    impl Foo -> (None, False)  # inherent impl
    impl<T> Foo<T> -> (None, False)  # inherent impl
    """
    # Remove comments
    impl_line = re.sub(r'//.*$', '', impl_line).strip()
    
    # Pattern: impl [generic_params] TraitName[<trait_generics>] for Type
    # or: impl [generic_params] Type  (inherent impl)
    
    # Check for trait impl: "impl ... Trait for Type"
    # Handle:
    # - qualified paths like std::fmt::Debug
    # - trait generics like MyTrait<T>
    match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+', impl_line)
    if match:
        trait_name = match.group(1)
        return (trait_name, True)
    
    # Inherent impl
    return (None, False)


def is_struct_or_enum_line(line):
    """Check if line defines a struct or enum."""
    stripped = line.strip()
    return bool(re.match(r'(pub\s+)?(?:struct|enum)\s+\w+', stripped))


def is_trait_definition(line):
    """Check if line defines a trait."""
    stripped = line.strip()
    return bool(re.match(r'(pub\s+)?trait\s+\w+', stripped))


def is_impl_line(line):
    """Check if line starts an impl block."""
    stripped = line.strip()
    return bool(re.match(r'impl(?:<[^>]+>)?\s+', stripped))


def check_impl_order(file_path):
    """
    Check for standard trait impls appearing before custom trait impls.
    Standard traits should be at the bottom of the file.
    
    Returns list of violations: (file_path, line_num, trait_name, context)
    """
    violations = []
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return violations
    
    # Track state for each data structure
    current_struct = None
    struct_line = None
    seen_custom_impl = False
    seen_standard_impl = False
    first_standard_impl = None
    
    for i, line in enumerate(lines, 1):
        stripped = line.strip()
        
        # Skip empty lines and comments
        if not stripped or stripped.startswith('//'):
            continue
        
        # New struct/enum resets state
        if is_struct_or_enum_line(line):
            # Reset for new struct
            match = re.search(r'(?:struct|enum)\s+(\w+)', stripped)
            if match:
                current_struct = match.group(1)
                struct_line = i
                seen_custom_impl = False
                seen_standard_impl = False
                first_standard_impl = None
            continue
        
        # Check impl blocks
        if is_impl_line(line):
            trait_name, is_trait_impl = extract_trait_name(line)
            
            if not is_trait_impl:
                # Inherent impl - ignore
                continue
            
            if not trait_name:
                continue
            
            # Determine if it's a standard or custom trait
            if trait_name in STANDARD_TRAITS:
                # Standard trait impl
                if not seen_standard_impl:
                    seen_standard_impl = True
                    first_standard_impl = (i, trait_name, stripped)
            else:
                # Custom trait impl
                if seen_standard_impl:
                    # Violation: standard impl came before this custom impl
                    violations.append({
                        'file': file_path,
                        'struct': current_struct,
                        'struct_line': struct_line,
                        'standard_impl_line': first_standard_impl[0],
                        'standard_trait': first_standard_impl[1],
                        'custom_impl_line': i,
                        'custom_trait': trait_name,
                    })
                
                seen_custom_impl = True
    
    return violations


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
    ]
    
    all_violations = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rs_file in search_dir.rglob("*.rs"):
            violations = check_impl_order(rs_file)
            all_violations.extend(violations)
    
    if all_violations:
        print("✗ Implementation Order Violations:\n")
        print("Standard trait impls (Eq, Debug, Display, etc.) should be AT THE BOTTOM (after custom trait impls).\n")
        
        for v in all_violations:
            rel_path = v['file'].relative_to(repo_root)
            print(f"  {rel_path}:{v['struct_line']}")
            if v['struct']:
                print(f"    Struct: {v['struct']}")
            print(f"    Line {v['standard_impl_line']}: {v['standard_trait']} impl (standard trait)")
            print(f"    Line {v['custom_impl_line']}: {v['custom_trait']} impl (custom trait)")
            print(f"    → Standard trait impls should move to the bottom (after all custom impls)")
            print()
        
        print(f"Total violations: {len(all_violations)}")
        print("\nCorrect order:")
        print("  1. Data structure (struct/enum)")
        print("  2. Trait definition")
        print("  3. Inherent impl (impl Type { ... })")
        print("  4. Custom trait implementations")
        print("  5. Standard trait implementations (Eq, Debug, etc.) <- AT THE BOTTOM")
        return 1
    
    print("✓ All implementations are in correct order")
    return 0


if __name__ == "__main__":
    sys.exit(main())

