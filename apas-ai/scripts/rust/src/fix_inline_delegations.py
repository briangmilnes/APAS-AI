#!/usr/bin/env python3
"""
Fix delegation pattern by moving implementations from inherent impl to trait impl.

Replaces:
    impl Struct {
        pub fn foo() -> Self { /* actual code */ }
    }
    impl Trait for Struct {
        fn foo() -> Self { Struct::foo() }
    }

With:
    impl Trait for Struct {
        fn foo() -> Self { /* actual code */ }
    }
"""
# Git commit: e8e8f18
# Date: 2025-10-17

import re
import sys
from pathlib import Path


def extract_method_impl(content, method_name):
    """Extract a method implementation from content."""
    # Find method definition
    pattern = rf'(pub\s+)?fn\s+{method_name}\s*(<[^>]*>)?\s*(\([^)]*\))\s*(->\s*[^{{]+)?\s*\{{'
    match = re.search(pattern, content)
    
    if not match:
        return None
    
    # Extract the method body
    start = match.start()
    brace_start = match.end() - 1
    brace_count = 1
    i = brace_start + 1
    
    while i < len(content) and brace_count > 0:
        if content[i] == '{':
            brace_count += 1
        elif content[i] == '}':
            brace_count -= 1
        i += 1
    
    method_text = content[start:i]
    
    # Extract just the body (everything inside the outer braces)
    body_start = method_text.find('{') + 1
    body_end = method_text.rfind('}')
    body = method_text[body_start:body_end]
    
    # Extract signature (everything before the opening brace)
    sig_end = method_text.find('{')
    signature = method_text[:sig_end].strip()
    
    return {
        'full': method_text,
        'signature': signature,
        'body': body,
    }


def find_impl_block(lines, impl_type, struct_name, trait_name=None):
    """Find an impl block in the file."""
    if impl_type == 'inherent':
        pattern = rf'^\s*impl(?:<[^>]*>)?\s+{struct_name}(?:<[^>]*>)?\s*\{{'
    else:
        pattern = rf'^\s*impl(?:<[^>]*>)?\s+{trait_name}(?:<[^>]*>)?\s+for\s+{struct_name}(?:<[^>]*>)?\s*\{{'
    
    for i, line in enumerate(lines):
        if re.search(pattern, line):
            # Find end of this impl block
            brace_count = 0
            start = i
            
            for j in range(i, len(lines)):
                line_text = lines[j]
                brace_count += line_text.count('{') - line_text.count('}')
                if brace_count == 0 and j > i:
                    return (start, j)
            
            return (start, len(lines) - 1)
    
    return None


def fix_file(file_path, dry_run=False):
    """Fix delegation patterns in a file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
            lines = content.splitlines(keepends=True)
    except Exception as e:
        print(f"Error reading {file_path}: {e}", file=sys.stderr)
        return False
    
    # Find all impl blocks
    impl_pattern = r'impl(?:<[^>]*>)?\s+(?:(\w+)\s+for\s+)?(\w+)(?:<[^>]*>)?\s*\{'
    impls = []
    
    for match in re.finditer(impl_pattern, content):
        trait_name = match.group(1)
        struct_name = match.group(2)
        impl_type = 'trait' if trait_name else 'inherent'
        
        # Find the impl block content
        start_pos = match.start()
        brace_count = 0
        i = match.end() - 1
        
        while i < len(content):
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
                if brace_count == 0:
                    break
            i += 1
        
        impl_content = content[start_pos:i+1]
        
        impls.append({
            'type': impl_type,
            'struct': struct_name,
            'trait': trait_name,
            'content': impl_content,
            'start_pos': start_pos,
            'end_pos': i+1,
        })
    
    # Group by struct
    by_struct = {}
    for impl in impls:
        struct = impl['struct']
        if struct not in by_struct:
            by_struct[struct] = {'inherent': None, 'traits': []}
        
        if impl['type'] == 'inherent':
            by_struct[struct]['inherent'] = impl
        else:
            by_struct[struct]['traits'].append(impl)
    
    # Process each struct
    new_content = content
    offset = 0  # Track position changes
    changes_made = False
    
    for struct, impls_dict in by_struct.items():
        inherent = impls_dict['inherent']
        if not inherent:
            continue
        
        for trait_impl in impls_dict['traits']:
            # Find delegating methods
            delegation_pattern = rf'fn\s+(\w+)\s*[^{{]*\{{\s*{struct}::(\w+)\s*\('
            
            for match in re.finditer(delegation_pattern, trait_impl['content']):
                method_name = match.group(1)
                
                # Extract implementation from inherent impl
                inherent_method = extract_method_impl(inherent['content'], method_name)
                if not inherent_method:
                    continue
                
                # Find the delegating method in trait impl
                trait_method_pattern = rf'(fn\s+{method_name}\s*[^{{]*)\{{\s*{struct}::{method_name}\s*\([^)]*\)\s*\}}'
                trait_match = re.search(trait_method_pattern, trait_impl['content'])
                
                if trait_match:
                    # Replace delegation with actual implementation
                    old_method = trait_match.group(0)
                    new_method = trait_match.group(1) + ' {' + inherent_method['body'] + '}'
                    
                    # Apply replacement in new_content
                    trait_start = trait_impl['start_pos'] + offset
                    trait_content = new_content[trait_start:trait_start + len(trait_impl['content'])]
                    new_trait_content = trait_content.replace(old_method, new_method, 1)
                    
                    new_content = (new_content[:trait_start] + 
                                 new_trait_content + 
                                 new_content[trait_start + len(trait_impl['content']):])
                    
                    offset += len(new_trait_content) - len(trait_impl['content'])
                    changes_made = True
                    
                    if dry_run:
                        print(f"Would inline {struct}::{method_name} into {trait_impl['trait']} impl")
    
    if not changes_made:
        return False
    
    if dry_run:
        print(f"Would update {file_path}")
        return True
    
    # Now remove inherent impl blocks
    for struct, impls_dict in by_struct.items():
        inherent = impls_dict['inherent']
        if not inherent or not impls_dict['traits']:
            continue
        
        # Check if all inherent methods are now in trait impls
        # For now, just remove the inherent impl
        inherent_start = inherent['start_pos']
        inherent_end = inherent['end_pos']
        
        # Adjust for previous changes
        # This is simplified - in reality need to track position changes
        new_content = new_content[:inherent_start] + new_content[inherent_end:]
    
    try:
        with open(file_path, 'w') as f:
            f.write(new_content)
        print(f"Fixed: {file_path}")
        return True
    except Exception as e:
        print(f"Error writing {file_path}: {e}", file=sys.stderr)
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Inline delegating methods")
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    changed = fix_file(file_path, dry_run=args.dry_run)
    return 0 if changed else 1


if __name__ == '__main__':
    sys.exit(main())

