#!/usr/bin/env python3
"""
Fix trait impl to be standalone by replacing StructName::method() calls with Self::method().

When trait impl calls inherent impl methods:
  impl Trait for StructName {
    fn method1() -> Self {
        StructName::from_vec(data)  // calls inherent impl
    }
  }

This script changes it to:
  impl Trait for StructName {
    fn method1() -> Self {
        Self::from_vec(data)  // calls trait impl
    }
  }
"""
# Git commit: 25ae22c50a0fcef6ba643cf969f9c755e1f73eab
# Date: 2025-10-18

import re
import sys
from pathlib import Path


def fix_trait_impl_calls(file_path, dry_run=False):
    """Replace StructName::method() with Self::method() in trait impls."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False
    
    original_content = content
    fixes = []
    
    # Find trait impls: impl Trait for StructName
    trait_impl_pattern = r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)(?:<[^>]+>)?'
    
    for trait_match in re.finditer(trait_impl_pattern, content):
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        
        # Find the opening brace
        trait_impl_start_pos = trait_match.end()
        brace_pos = content.find('{', trait_impl_start_pos)
        if brace_pos == -1:
            continue
        
        # Find matching closing brace
        brace_count = 1
        i = brace_pos + 1
        while i < len(content) and brace_count > 0:
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
            i += 1
        
        trait_impl_end = i
        trait_impl_body = content[brace_pos:trait_impl_end]
        
        # Find calls to StructName::method() in the trait impl
        # Pattern: StructName::identifier(
        call_pattern = rf'\b{struct_name}::(\w+)\('
        
        # Count matches
        matches = list(re.finditer(call_pattern, trait_impl_body))
        if not matches:
            continue
        
        # Filter out PascalCase (enum variants)
        method_calls = []
        for match in matches:
            method_name = match.group(1)
            # Only include if starts with lowercase or underscore (methods)
            if method_name[0].islower() or method_name[0] == '_':
                method_calls.append(method_name)
        
        if not method_calls:
            continue
        
        # Replace StructName:: with Self:: in the trait impl body
        new_body = re.sub(call_pattern, r'Self::\1(', trait_impl_body)
        
        fixes.append({
            'trait': trait_name,
            'struct': struct_name,
            'start': brace_pos,
            'end': trait_impl_end,
            'old_body': trait_impl_body,
            'new_body': new_body,
            'methods': list(set(method_calls))
        })
    
    if not fixes:
        return False
    
    if dry_run:
        print(f"\n{file_path}:")
        for fix in fixes:
            print(f"  Trait: {fix['trait']} for {fix['struct']}")
            print(f"  Would replace {fix['struct']}::method() with Self::method() for:")
            for method in sorted(fix['methods']):
                print(f"    - {method}")
        return True
    
    # Apply fixes in reverse order
    fixes.sort(key=lambda f: f['start'], reverse=True)
    
    for fix in fixes:
        content = content[:fix['start']] + fix['new_body'] + content[fix['end']:]
    
    # Write back
    try:
        with open(file_path, 'w') as f:
            f.write(content)
        total_methods = sum(len(fix['methods']) for fix in fixes)
        print(f"✓ Fixed {file_path}: replaced {total_methods} method call(s) in {len(fixes)} trait impl(s)")
        return True
    except Exception as e:
        with open(file_path, 'w') as f:
            f.write(original_content)
        print(f"Error writing {file_path}: {e}")
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Fix trait impl to use Self:: instead of StructName::"
    )
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    success = fix_trait_impl_calls(file_path, args.dry_run)
    return 0 if success else 1


if __name__ == '__main__':
    sys.exit(main())

