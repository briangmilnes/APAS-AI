#!/usr/bin/env python3
"""
Lift pure functions (no self parameter) from inherent impls to module level.
Conservative approach: minimal transformations.
"""
import re
import sys
from pathlib import Path

def extract_type_and_generics_from_impl(impl_line):
    """Extract type name and generic parameters from impl line."""
    # Match patterns like:
    # impl<T: StTInMtT + Ord> Node<T> {
    # impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {
    type_name = None
    generics = ""
    
    gen_match = re.search(r'impl<([^>]+)>', impl_line)
    if gen_match:
        generics = f"<{gen_match.group(1)}>"
    
    # Extract type name (after generics, before {)
    type_match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)', impl_line)
    if type_match:
        type_name = type_match.group(1)
    
    return type_name, generics

def find_inherent_impls(lines):
    """Find all inherent impl blocks (not trait impls)."""
    impls = []
    i = 0
    while i < len(lines):
        line = lines[i]
        # Look for impl blocks at module level (4 spaces indentation)
        if re.match(r'^    impl\b', line) and ' for ' not in line:
            # Check next few lines to make sure it's not a trait impl
            is_trait = False
            for j in range(i, min(i + 5, len(lines))):
                if ' for ' in lines[j]:
                    is_trait = True
                    break
                if lines[j].rstrip().endswith('{'):
                    break
            
            if not is_trait:
                # Find the end of this impl block by counting braces
                brace_count = 0
                start = i
                for j in range(i, len(lines)):
                    for char in lines[j]:
                        if char == '{':
                            brace_count += 1
                        elif char == '}':
                            brace_count -= 1
                            if brace_count == 0:
                                impls.append((start, j))
                                i = j - 1
                                break
                    if brace_count == 0:
                        break
        i += 1
    return impls

def parse_function(lines, start_idx):
    """
    Parse a function starting at start_idx.
    Returns (end_idx, is_function, name, full_lines).
    is_function = True if no self parameter (pure function).
    """
    # Parse signature (might span multiple lines until opening brace)
    sig_lines = []
    i = start_idx
    while i < len(lines):
        sig_lines.append(lines[i])
        if '{' in lines[i]:
            break
        i += 1
    
    signature = ''.join(sig_lines)
    
    # Extract function name
    fn_match = re.search(r'fn\s+(\w+)', signature)
    fn_name = fn_match.group(1) if fn_match else "unknown"
    
    # Check if it's a pure function (no self)
    has_self = '&self' in signature or '&mut self' in signature or 'mut self' in signature or 'self:' in signature
    is_function = not has_self
    
    # Find closing brace of function body
    brace_count = 0
    for j in range(i, len(lines)):
        for char in lines[j]:
            if char == '{':
                brace_count += 1
            elif char == '}':
                brace_count -= 1
                if brace_count == 0:
                    # Return full lines including the function
                    return j, is_function, fn_name, lines[start_idx:j + 1]
    
    # Shouldn't reach here
    return len(lines) - 1, is_function, fn_name, lines[start_idx:]

def rewrite_function_for_module_level(func_lines, type_name, generics):
    """
    Rewrite a function for module level:
    1. Adjust indentation (8 spaces -> 4 spaces)
    2. Add generics to fn signature if not already there
    3. Replace Self:: with direct calls
    4. For Node impl blocks: rename fn new to fn new_node
    """
    result = []
    fn_name_in_sig = None
    
    for i, line in enumerate(func_lines):
        # Adjust indentation
        if line.startswith('        '):  # 8 spaces
            line = line[4:]  # Remove 4, leaving 4
        
        # Handle function signature
        if i == 0 and line.strip().startswith('fn '):
            # Extract function name
            fn_match = re.search(r'fn\s+(\w+)', line)
            if fn_match:
                fn_name_in_sig = fn_match.group(1)
            
            # Add generics if not present
            if generics and '<' not in line.split('(')[0]:
                line = re.sub(r'(fn\s+\w+)', r'\1' + generics, line)
            
            # For Node impl with fn new, rename to new_node
            if type_name == 'Node' and fn_name_in_sig == 'new':
                line = re.sub(r'fn\s+new\b', 'fn new_node', line)
                fn_name_in_sig = 'new_node'
        
        # Replace Self:: with nothing (direct function calls)
        line = re.sub(r'\bSelf::', '', line)
        
        result.append(line)
    
    return result, fn_name_in_sig

def find_module_insertion_point(lines):
    """Find where to insert lifted functions in the module (after type aliases, before first struct/impl)."""
    in_module = False
    after_types = 0
    
    for i, line in enumerate(lines):
        # Detect module start
        if re.match(r'^pub mod \w+ \{', line):
            in_module = True
            continue
        
        if in_module:
            # Track type aliases and type definitions
            if re.match(r'^    (pub\s+)?type \w+', line):
                after_types = i + 1
            # Stop at first struct, impl, trait, or fn
            elif re.match(r'^    (pub\s+)?(struct|impl|trait|fn)\b', line):
                return after_types if after_types > 0 else i
    
    return 0

def process_file(filepath):
    """Process a single file."""
    path = Path(filepath)
    
    if not path.exists():
        print(f"ERROR: File not found: {filepath}")
        return False
    
    content = path.read_text()
    lines = content.split('\n')
    
    # Find all inherent impl blocks
    impl_blocks = find_inherent_impls(lines)
    
    if not impl_blocks:
        print(f"No inherent impl blocks found")
        return True
    
    print(f"Found {len(impl_blocks)} inherent impl block(s)")
    
    # Process each impl block and collect lifted functions
    # Work from end to start so line numbers don't shift
    all_lifted = []
    replacements_needed = []  # (old_name, new_name) for Node::new -> new_node
    
    for impl_start, impl_end in reversed(impl_blocks):
        print(f"\n  Impl at lines {impl_start+1}-{impl_end+1}: {lines[impl_start].strip()}")
        
        type_name, generics = extract_type_and_generics_from_impl(lines[impl_start])
        
        functions_to_lift = []
        impl_has_methods = False
        
        i = impl_start + 1
        while i < impl_end:
            line = lines[i]
            
            # Skip comments and blank lines
            if re.match(r'^\s*(//|$)', line):
                i += 1
                continue
            
            # Check if this is a function definition
            if re.match(r'^        (pub\s+)?fn\s+', line):
                end_idx, is_function, fn_name, func_lines = parse_function(lines, i)
                
                if is_function:
                    print(f"    ✓ Lifting: {fn_name}()")
                    rewritten, actual_name = rewrite_function_for_module_level(func_lines, type_name, generics)
                    functions_to_lift.append((fn_name, actual_name, rewritten))
                    # Track Node::new -> new_node replacement
                    if type_name == 'Node' and fn_name == 'new':
                        replacements_needed.append(('Node::new(', 'new_node('))
                else:
                    print(f"    - Keeping method: {fn_name}()")
                    impl_has_methods = True
                
                i = end_idx + 1
            elif line.strip() == '}':
                # Closing brace of impl
                break
            else:
                # Other content
                i += 1
        
        if functions_to_lift:
            all_lifted.extend([(impl_start, impl_end, fn_name, new_name, fn_lines) 
                              for fn_name, new_name, fn_lines in reversed(functions_to_lift)])
            
            if not impl_has_methods:
                print(f"  → Impl now empty, will remove")
                # Remove the entire impl block
                lines = lines[:impl_start] + lines[impl_end + 1:]
            else:
                print(f"  → Impl has methods, keeping impl block")
    
    if not all_lifted:
        print("\n✓ No functions to lift (only methods with self)")
        return True
    
    # Apply Node::new -> new_node replacements throughout the file
    for old_pattern, new_pattern in replacements_needed:
        print(f"\n  Replacing {old_pattern} with {new_pattern}")
        for i in range(len(lines)):
            if old_pattern in lines[i]:
                lines[i] = lines[i].replace(old_pattern, new_pattern)
    
    # Find insertion point in module
    insert_at = find_module_insertion_point(lines)
    
    # Build lifted functions text
    lifted_lines = []
    seen_funcs = set()
    for _, _, old_name, new_name, func_lines in reversed(all_lifted):
        fn_id = new_name or old_name
        if fn_id not in seen_funcs:
            seen_funcs.add(fn_id)
            if lifted_lines:  # Add blank line between functions
                lifted_lines.append('')
            lifted_lines.extend(func_lines)
    
    # Insert lifted functions
    if lifted_lines:
        lines = lines[:insert_at] + [''] + lifted_lines + lines[insert_at:]
    
    # Write back
    path.write_text('\n'.join(lines))
    
    print(f"\n✓ Lifted {len(seen_funcs)} function(s) to module level")
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: lift_functions_from_impls.py <source_file.rs>")
        print()
        print("Lifts pure functions (no self) from inherent impls to module level.")
        print("Rewrites Self:: calls. Removes empty impl blocks.")
        sys.exit(1)
    
    source_file = sys.argv[1]
    
    print(f"Processing {source_file}")
    print("=" * 80)
    
    try:
        success = process_file(source_file)
        sys.exit(0 if success else 1)
    except Exception as e:
        print(f"\nERROR: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)

if __name__ == '__main__':
    main()
