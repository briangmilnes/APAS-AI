#!/usr/bin/env python3
"""
Convert inherent impl blocks with only private helpers to module-level functions.

Transformations:
- fn foo() -> T (function) → stays as fn foo() -> T at module level
- fn foo(&self) -> T (method) → fn foo(this: &StructName) -> T at module level
- fn foo(&mut self) -> T (method) → fn foo(this: &mut StructName) -> T at module level

Also updates method bodies:
- self.field → this.field
- self.method() → method(this) if it's a helper in same impl
"""
import re
import sys
from pathlib import Path

def find_impl_block(lines, start_line):
    """Find the complete impl block starting at start_line (0-indexed)."""
    brace_count = 0
    impl_start = start_line
    impl_end = start_line
    
    for i in range(start_line, len(lines)):
        line = lines[i]
        brace_count += line.count('{') - line.count('}')
        if brace_count == 0 and '{' in line:
            impl_end = i
            break
    
    return impl_start, impl_end

def extract_struct_name(impl_line):
    """Extract struct name from impl line."""
    # impl<T: ...> StructName {
    match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)', impl_line)
    return match.group(1) if match else None

def extract_generic_params(impl_line):
    """Extract generic parameters from impl line."""
    # impl<T: StT + Ord> StructName {
    match = re.search(r'impl<([^>]+)>', impl_line)
    return match.group(1) if match else ""

def parse_function(lines, start_idx):
    """Parse a function starting at start_idx, return (end_idx, function_info)."""
    # Collect all comment lines before the function
    comments = []
    i = start_idx
    while i > 0 and (lines[i-1].strip().startswith('///') or lines[i-1].strip().startswith('//')):
        i -= 1
    
    if i < start_idx:
        comments = lines[i:start_idx]
    
    # Find the function signature (might span multiple lines)
    sig_start = start_idx
    sig_lines = []
    brace_count = 0
    paren_depth = 0
    
    i = start_idx
    while i < len(lines):
        line = lines[i]
        sig_lines.append(line)
        paren_depth += line.count('(') - line.count(')')
        
        if '{' in line and paren_depth == 0:
            # Found opening brace of function body
            break
        i += 1
    
    # Now find the closing brace
    brace_count = 0
    body_start = i
    for j in range(i, len(lines)):
        brace_count += lines[j].count('{') - lines[j].count('}')
        if brace_count == 0 and '{' in lines[j]:
            body_end = j
            break
    
    signature = ''.join(sig_lines)
    
    # Check if it's a method (has &self or &mut self)
    is_method = '&self' in signature or '&mut self' in signature
    is_mut_method = '&mut self' in signature
    
    # Extract function name
    fn_match = re.search(r'fn\s+(\w+)', signature)
    fn_name = fn_match.group(1) if fn_match else "unknown"
    
    # Get full function (comments + signature + body)
    full_function = lines[sig_start - len(comments):body_end + 1]
    
    return body_end, {
        'name': fn_name,
        'is_method': is_method,
        'is_mut': is_mut_method,
        'signature': signature.strip(),
        'full_lines': full_function,
        'body_lines': lines[body_start:body_end + 1],
        'comment_lines': comments,
        'start': sig_start - len(comments),
        'end': body_end + 1
    }

def convert_method_to_function(func_info, struct_name, generic_params):
    """Convert a method to a module-level function."""
    lines = func_info['full_lines']
    new_lines = []
    
    # Keep comments as-is
    for line in func_info['comment_lines']:
        new_lines.append(line)
    
    # Transform the signature
    sig = func_info['signature']
    
    if func_info['is_method']:
        # Replace &self or &mut self with explicit parameter
        if func_info['is_mut']:
            # fn foo(&mut self, ...) -> fn foo(this: &mut StructName<...>, ...)
            this_param = f"this: &mut {struct_name}"
            if generic_params:
                this_param = f"this: &mut {struct_name}<{generic_params}>"
            sig = sig.replace('&mut self', this_param)
        else:
            # fn foo(&self, ...) -> fn foo(this: &StructName<...>, ...)
            this_param = f"this: &{struct_name}"
            if generic_params:
                this_param = f"this: &{struct_name}<{generic_params}>"
            sig = sig.replace('&self', this_param)
    
    # Remove 'pub' if present (these are all private)
    sig = sig.replace('pub fn', 'fn')
    
    # Adjust indentation: impl methods are at 8 spaces, module functions at 4
    sig = sig.replace('        fn ', '    fn ')
    
    new_lines.append(sig)
    
    # Transform the body
    body_start_idx = len(func_info['comment_lines']) + len([l for l in func_info['full_lines'] if l in sig])
    for line in func_info['body_lines'][1:]:  # Skip the line with opening brace (already in sig)
        new_line = line
        
        # Replace self. with this.
        if 'self.' in line and func_info['is_method']:
            new_line = new_line.replace('self.', 'this.')
        
        # Adjust indentation: 8 spaces -> 4 spaces for function body
        # But preserve relative indentation beyond that
        if new_line.startswith('        '):
            # Function body content: remove 4 spaces
            new_line = new_line[4:]
        elif new_line.startswith('    }'):
            # Keep closing brace at 4 spaces
            pass
        
        new_lines.append(new_line)
    
    return new_lines

def process_file(filepath, impl_line_num, struct_name):
    """Process a single file to convert one inherent impl to module functions."""
    path = Path(filepath)
    
    if not path.exists():
        print(f"ERROR: File not found: {filepath}")
        return False
    
    content = path.read_text()
    lines = content.split('\n')
    
    # Find the impl block (line numbers are 1-indexed from the report)
    impl_start, impl_end = find_impl_block(lines, impl_line_num - 1)
    
    impl_header = lines[impl_start]
    struct_from_impl = extract_struct_name(impl_header)
    generic_params = extract_generic_params(impl_header)
    
    if struct_from_impl != struct_name:
        print(f"WARNING: Expected struct {struct_name}, found {struct_from_impl}")
        struct_name = struct_from_impl
    
    print(f"  Struct: {struct_name}, Generics: <{generic_params}>")
    
    # Parse all functions in the impl block
    functions = []
    i = impl_start + 1  # Start after 'impl ... {'
    while i <= impl_end:
        line = lines[i]
        
        # Look for function definitions
        if re.match(r'\s{8}(pub\s+)?fn\s+', line):
            end_idx, func_info = parse_function(lines, i)
            functions.append(func_info)
            print(f"    - {func_info['name']}() [{'method' if func_info['is_method'] else 'function'}]")
            i = end_idx + 1
        else:
            i += 1
    
    # Convert each function
    new_module_functions = []
    for func in functions:
        converted = convert_method_to_function(func, struct_name, generic_params)
        new_module_functions.extend(converted)
        new_module_functions.append('')  # Blank line between functions
    
    # Build new file content
    # Keep everything before the impl block
    new_content_lines = lines[:impl_start]
    
    # Add converted functions at the same location (module level)
    new_content_lines.extend(new_module_functions)
    
    # Keep everything after the impl block
    new_content_lines.extend(lines[impl_end + 1:])
    
    # Write back
    new_content = '\n'.join(new_content_lines)
    path.write_text(new_content)
    
    print(f"  ✓ Converted {len(functions)} helpers to module-level functions")
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: convert_helper_impls_to_functions.py <analysis_file.txt>")
        print()
        print("Reads analysis file and converts all 'only private helpers' impl blocks.")
        sys.exit(1)
    
    analysis_file = Path(sys.argv[1])
    
    if not analysis_file.exists():
        print(f"ERROR: Analysis file not found: {analysis_file}")
        sys.exit(1)
    
    # Parse the analysis file to extract files and line numbers
    content = analysis_file.read_text()
    lines = content.split('\n')
    
    to_convert = []
    in_only_private = False
    
    for line in lines:
        if 'ONLY PRIVATE HELPERS' in line:
            in_only_private = True
            continue
        if 'MIXED' in line:
            in_only_private = False
            break
        
        if in_only_private and line.strip().startswith('src/'):
            # Parse: src/Chap49/SubsetSumStPer.rs:42
            match = re.match(r'(src/[^:]+):(\d+)', line.strip())
            if match:
                filepath = match.group(1)
                line_num = int(match.group(2))
                
                # Next line should have "impl StructName {"
                # We'll parse it when processing
                to_convert.append({
                    'file': filepath,
                    'line': line_num,
                })
    
    print(f"Found {len(to_convert)} inherent impl blocks to convert")
    print("=" * 80)
    print()
    
    success_count = 0
    fail_count = 0
    
    for item in to_convert:
        filepath = item['file']
        line_num = item['line']
        
        print(f"Processing {filepath}:{line_num}")
        
        # Read the file to extract struct name from the impl line
        path = Path(filepath)
        if not path.exists():
            print(f"  ERROR: File not found")
            fail_count += 1
            continue
        
        lines_in_file = path.read_text().split('\n')
        if line_num > len(lines_in_file):
            print(f"  ERROR: Line {line_num} out of range")
            fail_count += 1
            continue
        
        impl_line = lines_in_file[line_num - 1]
        struct_name = extract_struct_name(impl_line)
        
        if not struct_name:
            print(f"  ERROR: Could not extract struct name from: {impl_line}")
            fail_count += 1
            continue
        
        try:
            if process_file(filepath, line_num, struct_name):
                success_count += 1
            else:
                fail_count += 1
        except Exception as e:
            print(f"  ERROR: {e}")
            fail_count += 1
        
        print()
    
    print("=" * 80)
    print(f"SUCCESS: {success_count} impl blocks converted")
    print(f"FAILED:  {fail_count} impl blocks")

if __name__ == '__main__':
    main()

