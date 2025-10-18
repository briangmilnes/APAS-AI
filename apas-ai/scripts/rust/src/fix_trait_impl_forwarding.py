#!/usr/bin/env python3
"""
Fix trait impl forwarding by copying implementation from inherent to trait.

When a trait impl method just forwards to inherent impl:
  impl Trait for Type {
    fn method() { Type::method() }  // forwarding
  }

Replace with actual implementation from inherent impl:
  impl Trait for Type {
    fn method() { /* actual code from inherent impl */ }
  }
"""
# Git commit: TBD
# Date: 2025-10-18

import re
import sys
from pathlib import Path


def find_impl_blocks(lines):
    """Find all impl blocks with their start/end positions."""
    impl_blocks = []
    i = 0
    
    while i < len(lines):
        line = lines[i].strip()
        
        if not line.startswith('impl'):
            i += 1
            continue
        
        # Parse impl line
        impl_info = parse_impl_line(line)
        if not impl_info:
            i += 1
            continue
        
        # Find opening brace
        brace_line = i
        while brace_line < len(lines) and '{' not in lines[brace_line]:
            brace_line += 1
        
        if brace_line >= len(lines):
            i += 1
            continue
        
        # Count braces to find end
        brace_count = 0
        j = brace_line
        while j < len(lines):
            brace_count += lines[j].count('{') - lines[j].count('}')
            if brace_count == 0:
                break
            j += 1
        
        impl_blocks.append({
            'type': impl_info[0],
            'struct_name': impl_info[1],
            'trait_name': impl_info[2],
            'start': i,
            'end': j,
            'brace_start': brace_line,
        })
        
        i = j + 1
    
    return impl_blocks


def parse_impl_line(line):
    """Parse impl line to extract type (inherent/trait), struct name, trait name."""
    line = re.sub(r'//.*$', '', line).strip()
    
    # Check for trait impl: impl Trait for Type
    trait_match = re.search(r'impl(?:<[^>]+>)?\s+(?:[\w:]+::)?(\w+)(?:<[^>]+>)?\s+for\s+(\w+)', line)
    if trait_match:
        trait_name = trait_match.group(1)
        struct_name = trait_match.group(2)
        return ('trait', struct_name, trait_name)
    
    # Check for inherent impl: impl Type
    inherent_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?', line)
    if inherent_match:
        struct_name = inherent_match.group(1)
        return ('inherent', struct_name, None)
    
    return None


def extract_method(lines, start, end, method_name):
    """Extract a complete method from impl block."""
    i = start
    while i < end:
        line = lines[i]
        
        # Check if this is the method we want
        if re.search(rf'\bfn\s+{method_name}\b', line):
            method_start = i
            
            # Find opening brace
            brace_line = i
            while brace_line < end and '{' not in lines[brace_line]:
                brace_line += 1
            
            if brace_line >= end:
                return None
            
            # Count braces to find method end
            brace_count = 0
            j = brace_line
            while j < end:
                brace_count += lines[j].count('{') - lines[j].count('}')
                if brace_count == 0:
                    break
                j += 1
            
            # Extract signature and body
            signature_lines = lines[method_start:brace_line+1]
            body_lines = lines[brace_line+1:j]
            
            return {
                'start': method_start,
                'end': j,
                'signature': ''.join(signature_lines),
                'body': ''.join(body_lines),
                'full': ''.join(lines[method_start:j+1]),
            }
        
        i += 1
    
    return None


def is_forwarding_method(method_text, struct_name, method_name):
    """Check if a method just forwards to inherent impl."""
    # Remove comments and whitespace
    text = re.sub(r'//.*$', '', method_text, flags=re.MULTILINE)
    text = re.sub(r'/\*.*?\*/', '', text, flags=re.DOTALL)
    text = text.strip()
    
    # Look for pattern: StructName::method_name(
    pattern = rf'{struct_name}::{method_name}\s*\('
    
    return bool(re.search(pattern, text))


def fix_file(file_path, dry_run=False):
    """Fix trait impl forwarding in a file."""
    try:
        with open(file_path, 'r') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False
    
    original_lines = lines.copy()
    
    # Find all impl blocks
    impl_blocks = find_impl_blocks(lines)
    
    # Group by struct
    struct_impls = {}
    for block in impl_blocks:
        struct_name = block['struct_name']
        if struct_name not in struct_impls:
            struct_impls[struct_name] = {'inherent': None, 'traits': []}
        
        if block['type'] == 'inherent':
            struct_impls[struct_name]['inherent'] = block
        else:
            struct_impls[struct_name]['traits'].append(block)
    
    # Process each struct's trait impls
    fixes = []
    
    for struct_name, impls in struct_impls.items():
        if not impls['inherent']:
            continue
        
        inherent = impls['inherent']
        
        for trait_block in impls['traits']:
            # Find forwarding methods in trait impl
            trait_start = trait_block['brace_start'] + 1
            trait_end = trait_block['end']
            
            # Scan for methods
            i = trait_start
            while i < trait_end:
                line = lines[i]
                
                # Check for method
                method_match = re.search(r'\bfn\s+(\w+)', line)
                if method_match:
                    method_name = method_match.group(1)
                    
                    # Extract trait method
                    trait_method = extract_method(lines, i, trait_end, method_name)
                    if not trait_method:
                        i += 1
                        continue
                    
                    # Check if it's forwarding
                    if is_forwarding_method(trait_method['body'], struct_name, method_name):
                        # Find same method in inherent impl
                        inherent_method = extract_method(
                            lines,
                            inherent['brace_start'] + 1,
                            inherent['end'],
                            method_name
                        )
                        
                        if inherent_method:
                            fixes.append({
                                'trait_method': trait_method,
                                'inherent_method': inherent_method,
                                'method_name': method_name,
                                'struct_name': struct_name,
                                'trait_name': trait_block['trait_name'],
                            })
                    
                    i = trait_method['end'] + 1
                else:
                    i += 1
    
    if not fixes:
        return False
    
    # Apply fixes in reverse order (so line numbers stay valid)
    fixes.sort(key=lambda f: f['trait_method']['start'], reverse=True)
    
    for fix in fixes:
        trait_method = fix['trait_method']
        inherent_method = fix['inherent_method']
        
        # Get indentation from trait method
        trait_indent = len(lines[trait_method['start']]) - len(lines[trait_method['start']].lstrip())
        
        # Get inherent method body lines
        inherent_body_lines = inherent_method['body'].rstrip().split('\n')
        
        # Re-indent inherent body to match trait indentation
        reindented_body = []
        for body_line in inherent_body_lines:
            if body_line.strip():  # Non-empty line
                # Preserve relative indentation
                reindented_body.append(' ' * trait_indent + '    ' + body_line.lstrip())
            else:
                reindented_body.append(body_line)
        
        # Build new method (signature from trait + body from inherent)
        new_method_lines = []
        
        # Add signature lines
        sig_text = trait_method['signature']
        new_method_lines.extend(sig_text.split('\n'))
        
        # Add body from inherent
        new_method_lines.extend(reindented_body)
        
        # Add closing brace
        new_method_lines.append(' ' * trait_indent + '    }')
        
        # Replace in lines
        lines[trait_method['start']:trait_method['end']+1] = [
            line + '\n' if not line.endswith('\n') else line
            for line in new_method_lines
        ]
    
    if dry_run:
        print(f"\n{file_path}:")
        print(f"  Would fix {len(fixes)} forwarding method(s):")
        for fix in reversed(fixes):
            print(f"    - {fix['method_name']} in {fix['trait_name']}")
        return True
    
    # Write back
    try:
        with open(file_path, 'w') as f:
            f.writelines(lines)
        print(f"✓ Fixed {file_path}: {len(fixes)} method(s) updated")
        return True
    except Exception as e:
        # Restore original on error
        with open(file_path, 'w') as f:
            f.writelines(original_lines)
        print(f"Error writing {file_path}: {e}")
        return False


def main():
    import argparse
    
    parser = argparse.ArgumentParser(
        description="Fix trait impl forwarding by copying implementation from inherent"
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

