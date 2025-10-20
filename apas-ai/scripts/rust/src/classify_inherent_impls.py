#!/usr/bin/env python3
"""Classify inherent impls: do they have a corresponding trait?"""
import re
from pathlib import Path

project_root = Path("/home/milnes/APASVERUS/APAS-AI/apas-ai")

# Read the inherent impls from the log file
log_file = project_root / "analyses/code_review/find_inherent_impls.txt"

with open(log_file, 'r') as f:
    lines = f.readlines()

# Parse the WITH GENERICS section
in_with_generics = False
with_generics_impls = []

for line in lines:
    if "WITH GENERICS" in line:
        in_with_generics = True
        continue
    if "WITHOUT GENERICS" in line:
        break
    if in_with_generics and line.strip() and not line.startswith('-'):
        # Parse: src/Chap06/LabUnDirGraphMtEph.rs:189:    impl<...> StructName {
        match = re.match(r'(src/[^:]+):(\d+):\s+impl.*\s+(\w+)', line)
        if match:
            filepath, line_num, struct_name = match.groups()
            with_generics_impls.append((filepath, line_num, struct_name, line.strip()))

print("CLASSIFICATION OF INHERENT IMPLS (WITH GENERICS)")
print("=" * 80)
print()

# Classify each impl
has_trait = []
no_trait = []
helper_structs = []  # Node, Inner, Iterator, etc.

helper_patterns = ['Node', 'Inner', 'Iter', 'Validator', 'Analyzer', 'Manager', 'Stats', 'Utils', 'Tester', 'Examples']

for filepath, line_num, struct_name, impl_line in with_generics_impls:
    full_path = project_root / filepath
    
    # Check if it's a helper struct
    is_helper = any(pattern in struct_name for pattern in helper_patterns)
    
    # Read the file to check for trait
    try:
        with open(full_path, 'r') as f:
            content = f.read()
        
        # Look for "pub trait StructNameTrait" or similar
        # Be more careful: struct might be "FooS" with trait "FooTrait" or "FooSTrait"
        base_name = struct_name.rstrip('S') if struct_name.endswith('S') else struct_name
        trait_patterns = [
            f"trait {struct_name}Trait",           # FooSTrait for FooS
            f"trait {base_name}Trait",             # FooTrait for FooS
            f"pub trait {struct_name}Trait",       # pub trait FooSTrait
            f"pub trait {base_name}Trait",         # pub trait FooTrait
        ]
        
        has_corresponding_trait = any(re.search(re.escape(pattern), content) for pattern in trait_patterns)
        
        if is_helper:
            helper_structs.append((filepath, struct_name, impl_line))
        elif has_corresponding_trait:
            has_trait.append((filepath, struct_name, impl_line))
        else:
            no_trait.append((filepath, struct_name, impl_line))
    except Exception as e:
        print(f"Error reading {filepath}: {e}")

print(f"HELPER STRUCTS (Node, Inner, Iterator, etc.) - {len(helper_structs)} items")
print("-" * 80)
for filepath, struct_name, impl_line in helper_structs[:10]:
    print(f"{filepath}")
    print(f"  {impl_line}")
print(f"... and {len(helper_structs) - 10} more" if len(helper_structs) > 10 else "")
print()

print(f"HAS CORRESPONDING TRAIT - {len(has_trait)} items")
print("-" * 80)
for filepath, struct_name, impl_line in has_trait[:10]:
    print(f"{filepath}")
    print(f"  {impl_line}")
print(f"... and {len(has_trait) - 10} more" if len(has_trait) > 10 else "")
print()

print(f"NO TRAIT (needs trait) - {len(no_trait)} items")
print("-" * 80)
for filepath, struct_name, impl_line in no_trait:
    print(f"{filepath}")
    print(f"  {impl_line}")
print()

print("=" * 80)
print(f"SUMMARY:")
print(f"  Helper structs (leave as-is): {len(helper_structs)}")
print(f"  Has trait already: {len(has_trait)}")
print(f"  Needs trait: {len(no_trait)}")
