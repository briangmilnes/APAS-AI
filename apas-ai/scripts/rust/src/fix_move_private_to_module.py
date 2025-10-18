#!/usr/bin/env python3
"""
Move private helper methods from inherent impls to module-level functions.

Converts private methods like:
  impl Foo {
      fn helper(&self, x: i32) -> i32 { ... }
  }

To module-level functions:
  fn foo_helper(foo: &Foo, x: i32) -> i32 { ... }

And updates all call sites.
"""
# Git commit: TBD
# Date: 2025-10-18

import re
import sys
from pathlib import Path


def count_braces(content, start_pos):
    """Count braces from start position to find matching closing brace."""
    brace_count = 0
    in_string = False
    in_char = False
    escape = False
    i = start_pos
    
    while i < len(content):
        c = content[i]
        
        if escape:
            escape = False
            i += 1
            continue
        
        if c == '\\':
            escape = True
            i += 1
            continue
        
        if c == '"' and not in_char:
            in_string = not in_string
            i += 1
            continue
        
        if c == "'" and not in_string:
            in_char = not in_char
            i += 1
            continue
        
        if not in_string and not in_char:
            if c == '{':
                brace_count += 1
            elif c == '}':
                brace_count -= 1
                if brace_count == 0:
                    return i
        
        i += 1
    
    return -1


def extract_method_signature(content, start_pos):
    """Extract method signature from fn keyword to opening brace."""
    end_pos = content.find('{', start_pos)
    if end_pos == -1:
        return None
    
    sig = content[start_pos:end_pos].strip()
    return sig


def find_private_methods_in_inherent_impl(content, struct_name):
    """Find all private methods in inherent impl for struct_name."""
    # Find the inherent impl block
    impl_pattern = rf'impl(?:<[^>]+>)?\s+{struct_name}(?:<[^>]+>)?\s*\{{'
    impl_match = re.search(impl_pattern, content)
    
    if not impl_match:
        return []
    
    impl_start = impl_match.end() - 1  # Position of opening brace
    impl_end = count_braces(content, impl_start + 1)
    
    if impl_end == -1:
        return []
    
    impl_body = content[impl_start + 1:impl_end]
    
    # Find all private methods (no 'pub' keyword)
    private_methods = []
    
    # Pattern for private method
    method_pattern = r'\n\s*fn\s+(\w+)'
    
    for match in re.finditer(method_pattern, impl_body):
        method_name = match.group(1)
        method_start_in_body = match.start()
        method_start_abs = impl_start + 1 + method_start_in_body
        
        # Extract full method
        fn_pos = content.find('fn', method_start_abs)
        brace_pos = content.find('{', fn_pos)
        if brace_pos == -1:
            continue
        
        method_end = count_braces(content, brace_pos + 1)
        if method_end == -1:
            continue
        
        signature = extract_method_signature(content, fn_pos)
        method_body = content[brace_pos:method_end + 1]
        
        private_methods.append({
            'name': method_name,
            'start': fn_pos,
            'end': method_end + 1,
            'signature': signature,
            'body': method_body,
        })
    
    return private_methods


def convert_method_to_function(struct_name, method_info):
    """Convert a method to a module-level function."""
    sig = method_info['signature']
    body = method_info['body']
    method_name = method_info['name']
    
    # Parse signature to extract parameters
    # Example: fn new(left: Tree, value: T, right: Tree) -> Self
    
    # Check if method takes &self or &mut self
    has_self_ref = '&self' in sig
    has_self_mut = '&mut self' in sig
    
    if not has_self_ref and not has_self_mut:
        # Static method, just rename
        fn_name = f"{struct_name.lower()}_{method_name}"
        new_sig = sig.replace(f'fn {method_name}', f'fn {fn_name}')
        
        # Replace Self with struct_name in signature and body
        new_sig = new_sig.replace('Self', struct_name)
        new_body = body.replace('Self', struct_name)
        
        return f"{new_sig} {new_body}"
    
    # Has self parameter - convert to explicit parameter
    fn_name = f"{struct_name.lower()}_{method_name}"
    
    # Replace fn name
    new_sig = sig.replace(f'fn {method_name}', f'fn {fn_name}')
    
    # Replace &self with explicit parameter
    if has_self_mut:
        new_sig = new_sig.replace('&mut self', f'{struct_name.lower()}: &mut {struct_name}')
    elif has_self_ref:
        new_sig = new_sig.replace('&self', f'{struct_name.lower()}: &{struct_name}')
    
    # Replace Self in signature
    new_sig = new_sig.replace('Self', struct_name)
    
    # In body, replace self. with struct_name_lower.
    new_body = body
    if has_self_mut or has_self_ref:
        new_body = re.sub(r'\bself\.', f'{struct_name.lower()}.', body)
    
    # Replace Self in body
    new_body = new_body.replace('Self', struct_name)
    
    return f"{new_sig} {new_body}"


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Move private methods from inherent impl to module-level"
    )
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--struct', required=True, help='Struct name')
    parser.add_argument('--dry-run', action='store_true', help='Show changes without applying')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find private methods
    private_methods = find_private_methods_in_inherent_impl(content, args.struct)
    
    if not private_methods:
        print(f"No private methods found in {args.struct}")
        return 0
    
    print(f"Found {len(private_methods)} private method(s) in {args.struct}:")
    for m in private_methods:
        print(f"  - {m['name']}")
    
    if args.dry_run:
        print("\nDry run - showing conversions:")
        for m in private_methods:
            new_fn = convert_method_to_function(args.struct, m)
            print(f"\n{new_fn}")
        return 0
    
    # For now, just report what would be done
    # Full implementation would:
    # 1. Extract private methods
    # 2. Convert to module functions
    # 3. Insert before the impl block
    # 4. Remove from impl block
    # 5. Update all call sites (Self::method() -> struct_method())
    
    print("\nNote: Automatic conversion not fully implemented yet.")
    print("This is a complex refactoring that requires:")
    print("  1. Moving method definitions to module level")
    print("  2. Converting self parameters")
    print("  3. Updating all call sites")
    print("  4. Handling method calls within the impl")
    print("\nManual refactoring recommended.")
    
    return 0


if __name__ == '__main__':
    sys.exit(main())

