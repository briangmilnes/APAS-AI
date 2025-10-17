#!/usr/bin/env python3
"""
Review script: No Trait Method Duplication

Detects cases where trait methods are duplicated as inherent methods on the same type.

RustRules.md: "No Trait Method Duplication (MANDATORY)"
- Never duplicate trait method implementations as inherent methods
- Trait methods are the single source of truth
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import run_review, get_repo_root


def extract_impl_blocks(content: str) -> list[dict]:
    """
    Extract all impl blocks from Rust source.
    Returns list of dicts with:
    - type: 'inherent' or 'trait'
    - type_name: the type being implemented
    - trait_name: trait name (if trait impl)
    - methods: list of method names
    - line: starting line number
    """
    impl_blocks = []
    lines = content.split('\n')
    
    # Match: impl<...> TypeName<...> { ... }  (inherent)
    # Match: impl<...> TraitName<...> for TypeName<...> { ... }  (trait)
    impl_pattern = re.compile(
        r'^\s*impl(?:<[^>]+>)?\s+(?:([A-Z]\w+)(?:<[^>]+>)?\s+for\s+)?([A-Z]\w+)(?:<[^>]+>)?\s*(?:where[^{]*)?\s*\{'
    )
    
    # Match function definitions
    fn_pattern = re.compile(r'^\s*(?:pub\s+)?fn\s+(\w+)\s*[(<]')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        impl_match = impl_pattern.match(line)
        
        if impl_match:
            trait_name = impl_match.group(1)  # None for inherent impls
            type_name = impl_match.group(2)
            
            # Find methods in this impl block
            methods = []
            brace_depth = 0
            start_line = i + 1
            
            # Count opening brace
            brace_depth += line.count('{') - line.count('}')
            i += 1
            
            while i < len(lines) and brace_depth > 0:
                line = lines[i]
                brace_depth += line.count('{') - line.count('}')
                
                fn_match = fn_pattern.match(line)
                if fn_match and brace_depth == 1:  # Only top-level methods
                    method_name = fn_match.group(1)
                    methods.append((method_name, i + 1))
                
                i += 1
            
            impl_blocks.append({
                'type': 'inherent' if trait_name is None else 'trait',
                'type_name': type_name,
                'trait_name': trait_name,
                'methods': methods,
                'line': start_line
            })
        else:
            i += 1
    
    return impl_blocks


def find_duplicate_methods(impl_blocks: list[dict]) -> list[dict]:
    """
    Find methods that appear in both trait impl and inherent impl for same type.
    Returns list of violations with type_name, method_name, inherent_line, trait_line.
    """
    violations = []
    
    # Group impl blocks by type
    by_type = {}
    for block in impl_blocks:
        type_name = block['type_name']
        if type_name not in by_type:
            by_type[type_name] = {'inherent': [], 'trait': []}
        by_type[type_name][block['type']].append(block)
    
    # Check each type for duplicates
    for type_name, blocks in by_type.items():
        inherent_blocks = blocks['inherent']
        trait_blocks = blocks['trait']
        
        if not inherent_blocks or not trait_blocks:
            continue
        
        # Build set of all trait methods
        trait_methods = {}
        for trait_block in trait_blocks:
            for method_name, line_num in trait_block['methods']:
                key = method_name
                if key not in trait_methods:
                    trait_methods[key] = (trait_block['trait_name'], line_num)
        
        # Check inherent methods against trait methods
        for inherent_block in inherent_blocks:
            for method_name, line_num in inherent_block['methods']:
                if method_name in trait_methods:
                    trait_name, trait_line = trait_methods[method_name]
                    violations.append({
                        'type_name': type_name,
                        'method_name': method_name,
                        'inherent_line': line_num,
                        'trait_name': trait_name,
                        'trait_line': trait_line
                    })
    
    return violations


def check_file(file_path: Path, context) -> list[str]:
    """Check a single Rust file for trait method duplication."""
    try:
        content = file_path.read_text(encoding='utf-8')
    except Exception as e:
        return [f"ERROR: Could not read {file_path}: {e}"]
    
    impl_blocks = extract_impl_blocks(content)
    violations = find_duplicate_methods(impl_blocks)
    
    errors = []
    for v in violations:
        rel_path = context.relative_path(file_path)
        errors.append(
            f"{rel_path}:{v['inherent_line']}: "
            f"Duplicate method '{v['method_name']}' in inherent impl for {v['type_name']} "
            f"(also in {v['trait_name']} trait impl at line {v['trait_line']})"
        )
    
    return errors


def main():
    repo_root = get_repo_root()
    return run_review(
        description="Detect trait methods duplicated as inherent methods",
        rule_name="No Trait Method Duplication",
        rule_reference="RustRules.md: No Trait Method Duplication (MANDATORY)",
        directories=[repo_root / "src", repo_root / "tests", repo_root / "benches"],
        check_function=check_file,
        fix_suggestion="Delete the inherent method and keep only the trait method implementation."
    )


if __name__ == "__main__":
    sys.exit(main())

