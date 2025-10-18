#!/usr/bin/env python3
"""
Fix trait impl forwarding by copying implementation from inherent to trait.
Handles parameter renaming and destructuring differences.

When trait impl forwards to inherent impl:
  impl Trait for Type {
    fn method(param: Pair<A, B>) -> Ret {
        Type::method(self, param)  // forwarding
    }
  }

And inherent impl destructures:
  impl Type {
    fn method(Pair(a, b): Pair<A, B>) -> Ret {
        // actual logic using a, b
    }
  }

This script:
1. Copies the logic from inherent to trait
2. Adds destructuring in trait impl body if needed
"""
# Git commit: 25ae22c50a0fcef6ba643cf969f9c755e1f73eab
# Date: 2025-10-18

import re
import sys
from pathlib import Path


def parse_signature(sig_text):
    """Parse function signature to extract parameter info."""
    # Remove comments
    sig_text = re.sub(r'//.*$', '', sig_text, flags=re.MULTILINE)
    
    # Extract parameters from signature
    # Pattern: fn name<generics>(params) -> return
    param_match = re.search(r'\((.*?)\)\s*(?:->|{)', sig_text, re.DOTALL)
    if not param_match:
        return []
    
    params_text = param_match.group(1)
    
    # Split by comma (simplified - doesn't handle nested generics perfectly)
    params = []
    depth = 0
    current = []
    
    for char in params_text:
        if char in '<([':
            depth += 1
        elif char in '>)]':
            depth -= 1
        elif char == ',' and depth == 0:
            param_str = ''.join(current).strip()
            if param_str:
                params.append(parse_parameter(param_str))
            current = []
            continue
        current.append(char)
    
    # Don't forget last parameter
    param_str = ''.join(current).strip()
    if param_str:
        params.append(parse_parameter(param_str))
    
    return params


def parse_parameter(param_str):
    """Parse a single parameter."""
    # Handle patterns:
    # self, &self, &mut self
    # name: Type
    # Pattern(a, b): Type  (destructuring)
    
    if param_str in ['self', '&self', '&mut self']:
        return {'kind': 'self', 'text': param_str}
    
    # Check for destructuring pattern
    # Pattern(a, b): Type or Pattern{a, b}: Type
    destruct_match = re.match(r'(\w+)\s*[\(\{]([^\)\}]+)[\)\}]\s*:\s*(.+)', param_str)
    if destruct_match:
        pattern_name = destruct_match.group(1)
        inner_vars = [v.strip() for v in destruct_match.group(2).split(',')]
        type_name = destruct_match.group(3).strip()
        return {
            'kind': 'destructure',
            'pattern': pattern_name,
            'vars': inner_vars,
            'type': type_name,
            'text': param_str
        }
    
    # Regular parameter: name: Type
    parts = param_str.split(':', 1)
    if len(parts) == 2:
        name = parts[0].strip()
        type_name = parts[1].strip()
        return {
            'kind': 'regular',
            'name': name,
            'type': type_name,
            'text': param_str
        }
    
    return {'kind': 'unknown', 'text': param_str}


def find_method_in_content(content, start_pos, method_name):
    """Find method and return (fn_pos, sig_end, body_start, body_end)."""
    # Find fn keyword
    fn_pattern = rf'\bfn\s+{method_name}\b'
    fn_match = re.search(fn_pattern, content[start_pos:])
    if not fn_match:
        return None
    
    fn_pos = start_pos + fn_match.start()
    
    # Find opening brace
    brace_pos = content.find('{', fn_pos)
    if brace_pos == -1:
        return None
    
    # Count braces to find closing
    brace_count = 1
    i = brace_pos + 1
    while i < len(content) and brace_count > 0:
        char = content[i]
        if char == '{':
            brace_count += 1
        elif char == '}':
            brace_count -= 1
        i += 1
    
    close_brace_pos = i - 1
    
    return (fn_pos, brace_pos, close_brace_pos)


def build_fixed_method(trait_sig_text, inherent_body_text, trait_params, inherent_params):
    """Build the fixed trait method with proper parameter handling."""
    # Check if we need to add destructuring
    needs_destructure = False
    destructure_stmt = None
    
    # Compare parameters (skip 'self')
    trait_non_self = [p for p in trait_params if p['kind'] != 'self']
    inherent_non_self = [p for p in inherent_params if p['kind'] != 'self']
    
    if len(trait_non_self) == len(inherent_non_self):
        for t_param, i_param in zip(trait_non_self, inherent_non_self):
            # If inherent destructures but trait doesn't
            if i_param['kind'] == 'destructure' and t_param['kind'] == 'regular':
                # Check if types match
                if i_param['type'] == t_param['type']:
                    needs_destructure = True
                    # Build destructure statement
                    # let Pattern(var1, var2, ...) = trait_param_name;
                    vars_str = ', '.join(i_param['vars'])
                    destructure_stmt = f"let {i_param['pattern']}({vars_str}) = {t_param['name']};"
    
    # Build the method
    # If single-line, keep single line
    is_single_line = '\n' not in inherent_body_text
    
    if is_single_line:
        if needs_destructure:
            # Convert to multi-line
            body_parts = [
                f"        {destructure_stmt}",
                f"        {inherent_body_text.strip()}"
            ]
            return trait_sig_text + "\n" + "\n".join(body_parts) + "\n    }"
        else:
            return trait_sig_text + " " + inherent_body_text.strip() + " }"
    else:
        # Multi-line
        if needs_destructure:
            # Insert destructure at beginning of body
            body_lines = inherent_body_text.strip().split('\n')
            body_with_destructure = [f"        {destructure_stmt}"] + body_lines
            return trait_sig_text + "\n" + "\n".join(body_with_destructure) + "\n    }"
        else:
            # Keep as-is
            return trait_sig_text + "\n" + inherent_body_text.rstrip() + "\n    }"


def fix_file(file_path, dry_run=False):
    """Fix trait impl forwarding in a file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False
    
    original_content = content
    fixes = []
    
    # Find trait impls
    trait_impl_pattern = r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)(?:<[^>]+>)?\s*\{'
    
    for trait_match in re.finditer(trait_impl_pattern, content):
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        trait_impl_start = trait_match.end() - 1
        
        # Find trait impl end
        brace_count = 1
        i = trait_impl_start + 1
        while i < len(content) and brace_count > 0:
            if content[i] == '{':
                brace_count += 1
            elif content[i] == '}':
                brace_count -= 1
            i += 1
        trait_impl_end = i - 1
        
        # Find methods in trait impl
        method_pattern = r'\bfn\s+(\w+)'
        for method_match in re.finditer(method_pattern, content[trait_impl_start:trait_impl_end]):
            method_name = method_match.group(1)
            method_start = trait_impl_start + method_match.start()
            
            # Get trait method info
            trait_method = find_method_in_content(content, method_start, method_name)
            if not trait_method:
                continue
            
            trait_fn_pos, trait_body_start, trait_body_end = trait_method
            trait_body = content[trait_body_start+1:trait_body_end].strip()
            
            # Check if it forwards
            forward_pattern = rf'{struct_name}::{method_name}\s*\('
            if not re.search(forward_pattern, trait_body):
                continue
            
            # Find inherent impl
            inherent_pattern = rf'impl(?:<[^>]+>)?\s+{struct_name}(?:<[^>]+>)?\s*\{{'
            inherent_match = re.search(inherent_pattern, content)
            if not inherent_match:
                continue
            
            inherent_start = inherent_match.end() - 1
            brace_count = 1
            i = inherent_start + 1
            while i < len(content) and brace_count > 0:
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                i += 1
            inherent_end = i - 1
            
            # Find same method in inherent
            inherent_method = find_method_in_content(content, inherent_start, method_name)
            if not inherent_method:
                continue
            
            inh_fn_pos, inh_body_start, inh_body_end = inherent_method
            
            # Extract signatures and bodies
            trait_sig = content[trait_fn_pos:trait_body_start+1].rstrip()
            inherent_sig = content[inh_fn_pos:inh_body_start+1].rstrip()
            inherent_body = content[inh_body_start+1:inh_body_end]
            
            # Parse parameters
            trait_params = parse_signature(trait_sig)
            inherent_params = parse_signature(inherent_sig)
            
            # Build fixed method
            fixed_method = build_fixed_method(trait_sig, inherent_body, trait_params, inherent_params)
            
            fixes.append({
                'method': method_name,
                'trait': trait_name,
                'start': trait_fn_pos,
                'end': trait_body_end + 1,
                'replacement': fixed_method,
            })
    
    if not fixes:
        return False
    
    if dry_run:
        print(f"\n{file_path}:")
        print(f"  Would fix {len(fixes)} forwarding method(s):")
        for fix in fixes:
            print(f"    - {fix['method']} in {fix['trait']}")
        return True
    
    # Apply fixes in reverse order
    fixes.sort(key=lambda f: f['start'], reverse=True)
    
    for fix in fixes:
        content = content[:fix['start']] + fix['replacement'] + content[fix['end']:]
    
    # Write back
    try:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"✓ Fixed {file_path}: {len(fixes)} method(s) updated")
        return True
    except Exception as e:
        with open(file_path, 'w') as f:
            f.write(original_content)
        print(f"Error writing {file_path}: {e}")
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Fix trait impl forwarding v3 - handles parameter destructuring"
    )
    parser.add_argument('--file', required=True, help='File to fix')
    parser.add_argument('--dry-run', action='store_true', help='Show what would be done')
    args = parser.parse_args()
    
    file_path = Path(args.file)
    if not file_path.exists():
        print(f"Error: {file_path} not found", file=sys.stderr)
        return 1
    
    success = fix_file(file_path, args.dry_run)
    return 0 if success else 1


if __name__ == '__main__':
    sys.exit(main())

