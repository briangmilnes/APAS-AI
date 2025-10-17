#!/usr/bin/env python3
"""
Detect potential trait method conflicts when methods move from inherent impls to trait defaults.

This script identifies test/benchmark files that import multiple APAS modules via wildcards,
where those modules have traits with overlapping method names. These are call sites that
would break if methods move from inherent impls to trait default implementations.

Example problem:
    use apas_ai::SetStEph::*;     // SetStEphTrait has .size()
    use apas_ai::Graph::*;         // GraphTrait has .size()
    let s = SetStEph::empty();
    s.size();  // ERROR: ambiguous after refactor!
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path
from collections import defaultdict

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


def extract_wildcard_imports(lines):
    """Extract all wildcard imports from a file."""
    imports = []
    for line in lines:
        # Match: use apas_ai::ModuleName::*;
        match = re.search(r'use\s+apas_ai::([^:]+(?:::[^:]+)*)::(\*|{[^}]*\*[^}]*});', line)
        if match:
            module_path = match.group(1)
            imports.append(module_path)
    return imports


def extract_trait_methods(filepath):
    """Extract all method names from all traits in a file."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception:
        return set()
    
    methods = set()
    in_trait = False
    brace_depth = 0
    
    for line in lines:
        stripped = line.strip()
        
        # Skip comments
        if stripped.startswith('//'):
            continue
        
        # Detect trait start
        if not in_trait and 'pub trait ' in line and '{' in line:
            in_trait = True
            brace_depth = line.count('{') - line.count('}')
            continue
        
        if in_trait:
            # Track brace depth
            brace_depth += line.count('{') - line.count('}')
            
            # Check if we're still in the trait
            if brace_depth <= 0:
                in_trait = False
                continue
            
            # Extract method name from method signature
            # Match: fn method_name(...) or fn method_name<...>(...)
            match = re.search(r'\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)', stripped)
            if match and not stripped.startswith('//'):
                method_name = match.group(1)
                methods.add(method_name)
    
    return methods


def find_module_file(module_path, repo_root):
    """Find the source file for a given module path."""
    # Convert Chap05::SetStEph to src/Chap05/SetStEph.rs
    parts = module_path.split('::')
    file_path = repo_root / 'src' / '/'.join(parts[:-1]) / f"{parts[-1]}.rs"
    
    if file_path.exists():
        return file_path
    
    # Try without the last component (might be a module re-export)
    if len(parts) > 1:
        file_path = repo_root / 'src' / '/'.join(parts) / 'mod.rs'
        if file_path.exists():
            return file_path
    
    return None


def check_file_for_conflicts(filepath, context):
    """Check a test/benchmark file for potential trait method conflicts."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        return None
    
    # Extract wildcard imports
    wildcard_imports = extract_wildcard_imports(lines)
    
    if len(wildcard_imports) < 2:
        # No conflicts possible with 0 or 1 imports
        return None
    
    # Build map of module -> trait methods
    module_methods = {}
    for module_path in wildcard_imports:
        module_file = find_module_file(module_path, context.repo_root)
        if module_file:
            methods = extract_trait_methods(module_file)
            if methods:
                module_methods[module_path] = methods
    
    if len(module_methods) < 2:
        # Need at least 2 modules with traits to have conflicts
        return None
    
    # Find overlapping method names
    conflicts = defaultdict(list)
    modules = list(module_methods.keys())
    
    for i in range(len(modules)):
        for j in range(i + 1, len(modules)):
            mod1 = modules[i]
            mod2 = modules[j]
            overlap = module_methods[mod1] & module_methods[mod2]
            
            if overlap:
                for method in overlap:
                    conflicts[method].append((mod1, mod2))
    
    if not conflicts:
        return None
    
    return {
        'file': filepath,
        'imports': wildcard_imports,
        'conflicts': dict(conflicts),
        'module_methods': module_methods
    }


def main():
    parser = create_review_parser(
        description="Detect potential trait method conflicts from wildcard imports"
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    # Check test and benchmark files
    dirs_to_check = []
    for dir_name in ['tests', 'benches']:
        dir_path = context.repo_root / dir_name
        if dir_path.exists():
            dirs_to_check.append(dir_path)
    
    if not dirs_to_check:
        print("✓ No tests/ or benches/ directories found")
        return 0
    
    if context.dry_run:
        files = context.find_files(dirs_to_check)
        print(f"Would check {len(files)} file(s) for trait method conflicts")
        return 0
    
    files = context.find_files(dirs_to_check)
    print(f"Analyzing {len(files)} test/benchmark files for trait method conflicts...")
    print("=" * 80)
    
    all_conflicts = []
    
    for filepath in files:
        result = check_file_for_conflicts(filepath, context)
        if result:
            all_conflicts.append(result)
    
    if not all_conflicts:
        print("\n✓ No trait method conflicts detected!")
        print("All test/benchmark files are safe for trait default implementation refactor.")
        return 0
    
    # Report conflicts
    print(f"\n✗ Found {len(all_conflicts)} file(s) with potential trait method conflicts:\n")
    
    total_conflicting_methods = 0
    
    for result in sorted(all_conflicts, key=lambda x: len(x['conflicts']), reverse=True):
        rel_path = context.relative_path(result['file'])
        conflicts = result['conflicts']
        total_conflicting_methods += len(conflicts)
        
        print(f"\n{rel_path}:")
        print(f"  Imports {len(result['imports'])} modules with wildcards:")
        for imp in result['imports']:
            method_count = len(result['module_methods'].get(imp, set()))
            print(f"    - {imp} ({method_count} trait methods)")
        
        print(f"\n  {len(conflicts)} conflicting method(s):")
        for method, module_pairs in sorted(conflicts.items()):
            print(f"    • {method}()")
            for mod1, mod2 in module_pairs:
                print(f"        ↳ {mod1} vs {mod2}")
    
    print("\n" + "=" * 80)
    print(f"Summary:")
    print(f"  Files with conflicts: {len(all_conflicts)}")
    print(f"  Total conflicting methods: {total_conflicting_methods}")
    print(f"\nThese files will need fixes before moving methods to trait defaults:")
    print(f"  1. Remove wildcard imports and use specific imports")
    print(f"  2. Use fully-qualified syntax: Trait::method(&obj)")
    print(f"  3. Use type ascription or turbofish to disambiguate")
    
    return 1


if __name__ == '__main__':
    sys.exit(main())

