#!/usr/bin/env python3
"""
Fix trait impl forwarding by copying implementation from inherent to trait.

Handles both single-line and multi-line methods.
"""
# Git commit: TBD  
# Date: 2025-10-18

import re
import sys
from pathlib import Path


def find_method_body(content, start_pos, method_name):
    """Find method body starting from start_pos. Returns (signature_end, body_start, body_end)."""
    # Find the fn line
    fn_match = re.search(rf'\bfn\s+{method_name}\b', content[start_pos:])
    if not fn_match:
        return None
    
    fn_pos = start_pos + fn_match.start()
    
    # Find opening brace
    brace_pos = content.find('{', fn_pos)
    if brace_pos == -1:
        return None
    
    # Count braces to find closing brace
    brace_count = 1
    i = brace_pos + 1
    while i < len(content) and brace_count > 0:
        if content[i] == '{' and not in_string_or_comment(content, i):
            brace_count += 1
        elif content[i] == '}' and not in_string_or_comment(content, i):
            brace_count -= 1
        i += 1
    
    if brace_count != 0:
        return None
    
    close_brace_pos = i - 1
    
    return (fn_pos, brace_pos, close_brace_pos)


def in_string_or_comment(content, pos):
    """Quick check if position is in string or comment (simplified)."""
    # Check for // comment on same line
    line_start = content.rfind('\n', 0, pos) + 1
    line_content = content[line_start:pos]
    if '//' in line_content:
        return True
    return False


def extract_method_body_content(content, open_brace_pos, close_brace_pos):
    """Extract just the body content between braces."""
    body_with_braces = content[open_brace_pos:close_brace_pos+1]
    
    # Check if it's a single-line body: { expr }
    lines = body_with_braces.split('\n')
    if len(lines) == 1:
        # Single line: { expr }
        inner = body_with_braces[1:-1].strip()
        return inner
    else:
        # Multi-line
        inner = body_with_braces[1:-1]
        # Remove leading/trailing blank lines but preserve indentation
        lines = inner.split('\n')
        # Trim leading blank lines
        while lines and not lines[0].strip():
            lines.pop(0)
        # Trim trailing blank lines
        while lines and not lines[-1].strip():
            lines.pop()
        return '\n'.join(lines)


def fix_file(file_path, dry_run=False):
    """Fix trait impl forwarding in a file."""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False
    
    original_content = content
    
    # Find all impl blocks using regex
    # Pattern: impl<...> Trait for Struct
    trait_impl_pattern = r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)(?:<[^>]+>)?\s*\{'
    
    fixes_made = []
    
    for trait_match in re.finditer(trait_impl_pattern, content):
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        trait_impl_start = trait_match.end() - 1  # Position of opening brace
        
        # Find end of this trait impl
        brace_count = 1
        i = trait_impl_start + 1
        while i < len(content) and brace_count > 0:
            if content[i] == '{' and not in_string_or_comment(content, i):
                brace_count += 1
            elif content[i] == '}' and not in_string_or_comment(content, i):
                brace_count -= 1
            i += 1
        trait_impl_end = i - 1
        
        trait_impl_body = content[trait_impl_start+1:trait_impl_end]
        
        # Find methods in trait impl that forward to struct methods
        method_pattern = r'\bfn\s+(\w+)'
        for method_match in re.finditer(method_pattern, trait_impl_body):
            method_name = method_match.group(1)
            method_start_in_body = method_match.start()
            method_start_abs = trait_impl_start + 1 + method_start_in_body
            
            # Get trait method body
            trait_method_info = find_method_body(content, method_start_abs, method_name)
            if not trait_method_info:
                continue
            
            trait_fn_pos, trait_body_start, trait_body_end = trait_method_info
            trait_body_content = content[trait_body_start+1:trait_body_end].strip()
            
            # Check if it forwards to inherent impl
            forward_pattern = rf'{struct_name}::{method_name}\s*\('
            if not re.search(forward_pattern, trait_body_content):
                continue
            
            # Find inherent impl for this struct
            inherent_pattern = rf'impl(?:<[^>]+>)?\s+{struct_name}(?:<[^>]+>)?\s*\{{'
            inherent_match = re.search(inherent_pattern, content)
            if not inherent_match:
                continue
            
            inherent_start = inherent_match.end() - 1
            
            # Find end of inherent impl
            brace_count = 1
            i = inherent_start + 1
            while i < len(content) and brace_count > 0:
                if content[i] == '{' and not in_string_or_comment(content, i):
                    brace_count += 1
                elif content[i] == '}' and not in_string_or_comment(content, i):
                    brace_count -= 1
                i += 1
            inherent_end = i - 1
            
            # Find the same method in inherent impl
            inherent_method_info = find_method_body(content[inherent_start:inherent_end], 0, method_name)
            if not inherent_method_info:
                continue
            
            inh_fn_pos, inh_body_start, inh_body_end = inherent_method_info
            # Adjust positions relative to full content
            inh_body_start_abs = inherent_start + inh_body_start
            inh_body_end_abs = inherent_start + inh_body_end
            
            # Extract inherent method body content
            inherent_body = extract_method_body_content(content, inh_body_start_abs, inh_body_end_abs)
            
            # Get signature from trait method
            trait_sig = content[trait_fn_pos:trait_body_start+1]
            
            # Check if single line or multi-line in original inherent
            inherent_is_single_line = '\n' not in content[inh_body_start_abs:inh_body_end_abs+1]
            
            # Build replacement
            if inherent_is_single_line:
                # Keep as single line
                replacement = f"{trait_sig} {inherent_body} }}"
            else:
                # Multi-line: preserve indentation
                # Get indentation from trait method
                line_start = content.rfind('\n', 0, trait_fn_pos) + 1
                indent = len(content[line_start:trait_fn_pos]) - len(content[line_start:trait_fn_pos].lstrip())
                
                # Re-indent inherent body
                body_lines = inherent_body.split('\n')
                reindented = []
                for line in body_lines:
                    if line.strip():
                        reindented.append(' ' * (indent + 4) + line.lstrip())
                    else:
                        reindented.append('')
                
                replacement = trait_sig + '\n' + '\n'.join(reindented) + '\n' + ' ' * indent + '    }'
            
            fixes_made.append({
                'method': method_name,
                'trait': trait_name,
                'start': trait_fn_pos,
                'end': trait_body_end + 1,
                'replacement': replacement,
            })
    
    if not fixes_made:
        return False
    
    if dry_run:
        print(f"\n{file_path}:")
        print(f"  Would fix {len(fixes_made)} forwarding method(s):")
        for fix in fixes_made:
            print(f"    - {fix['method']} in {fix['trait']}")
        return True
    
    # Apply fixes in reverse order
    fixes_made.sort(key=lambda f: f['start'], reverse=True)
    
    for fix in fixes_made:
        content = content[:fix['start']] + fix['replacement'] + content[fix['end']:]
    
    # Write back
    try:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"✓ Fixed {file_path}: {len(fixes_made)} method(s) updated")
        return True
    except Exception as e:
        # Restore on error
        with open(file_path, 'w') as f:
            f.write(original_content)
        print(f"Error writing {file_path}: {e}")
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Fix trait impl forwarding (v2 - handles single-line methods)"
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

