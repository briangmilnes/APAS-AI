#!/usr/bin/env python3
"""
Review: Find structs with both inherent impl and custom trait impl.

This is generally a malpattern - most structs should have either:
1. Just an inherent impl (no trait), OR
2. Just a trait impl (no inherent impl beyond standard traits)

Having both usually indicates accidental duplication or confusion about
whether functionality should be in the trait or the inherent impl.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


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
    'Add', 'Sub', 'Mul', 'Div', 'Rem', 'Neg',
    'BitAnd', 'BitOr', 'BitXor', 'Shl', 'Shr',
    'Not',
    'Send', 'Sync',
    'Fn', 'FnMut', 'FnOnce',
    'Error',
}


def extract_impl_info(line):
    """
    Extract information from an impl line.
    Returns (impl_type, struct_name, trait_name)
    - impl_type: 'inherent' or 'trait'
    - struct_name: name of the struct
    - trait_name: name of the trait (if trait impl), or None
    """
    line = re.sub(r'//.*$', '', line).strip()
    
    # Check for trait impl: impl ... TraitName ... for StructName
    trait_match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)', line)
    if trait_match:
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        return ('trait', struct_name, trait_name)
    
    # Check for inherent impl: impl ... StructName
    inherent_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{', line)
    if inherent_match:
        struct_name = inherent_match.group(1)
        return ('inherent', struct_name, None)
    
    return None


def review_file(file_path):
    """Review a single file for structs with both inherent and trait impls."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return []
    
    # Track impls per struct
    struct_impls = {}  # struct_name -> {'inherent': bool, 'traits': [trait_names]}
    
    for i, line in enumerate(lines, start=1):
        stripped = line.strip()
        
        if not stripped.startswith('impl'):
            continue
        
        impl_info = extract_impl_info(stripped)
        if not impl_info:
            continue
        
        impl_type, struct_name, trait_name = impl_info
        
        if struct_name not in struct_impls:
            struct_impls[struct_name] = {'inherent': False, 'traits': [], 'lines': []}
        
        if impl_type == 'inherent':
            struct_impls[struct_name]['inherent'] = True
            struct_impls[struct_name]['lines'].append((i, 'inherent', None))
        elif impl_type == 'trait':
            # Only track custom traits, not standard library traits
            if trait_name not in STANDARD_TRAITS:
                struct_impls[struct_name]['traits'].append(trait_name)
                struct_impls[struct_name]['lines'].append((i, 'trait', trait_name))
    
    # Collect violations
    violations = []
    for struct_name, info in sorted(struct_impls.items()):
        if info['inherent'] and info['traits']:
            violations.append({
                'struct': struct_name,
                'traits': info['traits'],
                'lines': info['lines']
            })
    
    return violations


def main():
    # Find workspace root (contains Cargo.toml)
    script_path = Path(__file__).resolve()
    workspace_root = script_path
    while workspace_root.parent != workspace_root:
        if (workspace_root / 'Cargo.toml').exists():
            break
        workspace_root = workspace_root.parent
    
    # Find all Rust files in src/
    src_dir = workspace_root / 'src'
    if not src_dir.exists():
        print(f"Error: {src_dir} not found", file=sys.stderr)
        return 1
    
    rust_files = sorted(src_dir.rglob('*.rs'))
    
    total_violations = 0
    
    for file_path in rust_files:
        violations = review_file(file_path)
        
        if violations:
            rel_path = file_path.relative_to(workspace_root)
            
            for v in violations:
                total_violations += 1
                print(f"\n  {rel_path}")
                print(f"    Struct: {v['struct']}")
                print(f"    Has both inherent impl AND custom trait impl(s): {', '.join(v['traits'])}")
                
                for line_num, impl_type, trait_name in v['lines']:
                    if impl_type == 'inherent':
                        print(f"      Line {line_num}: inherent impl")
                    else:
                        print(f"      Line {line_num}: {trait_name} trait impl")
    
    print(f"\n{'='*70}")
    print(f"Total: {total_violations} struct(s) with both inherent and trait impls")
    print(f"{'='*70}")
    
    return 1 if total_violations > 0 else 0


if __name__ == '__main__':
    sys.exit(main())

