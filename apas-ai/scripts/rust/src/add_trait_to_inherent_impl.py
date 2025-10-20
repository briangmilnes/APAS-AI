#!/usr/bin/env python3
"""
Automatically convert inherent impl blocks to trait impls.
Extracts method signatures to create a trait definition.
Preserves APAS algorithmic analysis comments in the trait.
"""
import re
import sys
from pathlib import Path

def extract_struct_name(content):
    """Find the main struct name (matches filename pattern)."""
    # Look for pub struct StructNameS<...> or pub struct StructName {
    match = re.search(r'pub struct (\w+S?)<', content)
    if match:
        return match.group(1)
    match = re.search(r'pub struct (\w+)\s*\{', content)
    if match:
        return match.group(1)
    return None

def find_inherent_impl_block(content, struct_name):
    """Find the inherent impl block for the given struct."""
    # Look for impl<...> StructName {
    pattern = rf'(    impl<[^>]+>\s+{re.escape(struct_name)}\s*\{{)'
    match = re.search(pattern, content, re.MULTILINE)
    if not match:
        # Try without generics
        pattern = rf'(    impl\s+{re.escape(struct_name)}\s*\{{)'
        match = re.search(pattern, content, re.MULTILINE)
    
    if not match:
        return None, None, None
    
    start = match.start()
    impl_header = match.group(1)
    
    # Find matching closing brace
    brace_count = 0
    in_impl = False
    end = start
    
    for i in range(start, len(content)):
        if content[i] == '{':
            brace_count += 1
            in_impl = True
        elif content[i] == '}':
            brace_count -= 1
            if in_impl and brace_count == 0:
                end = i + 1
                break
    
    if end > start:
        impl_block = content[start:end]
        return start, end, impl_block
    
    return None, None, None

def extract_method_signatures(impl_block):
    """Extract method signatures from impl block, preserving APAS comments."""
    methods = []
    lines = impl_block.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Look for APAS comment
        apas_comment = None
        if '/// APAS:' in line or '// APAS:' in line:
            apas_comment = line.strip()
            i += 1
            if i >= len(lines):
                break
            line = lines[i]
        
        # Look for pub fn or fn at the right indentation
        if re.match(r'\s{8}(pub\s+)?fn\s+', line):
            # Collect the full function signature (might span multiple lines)
            sig_lines = [line]
            brace_depth = 0
            paren_depth = line.count('(') - line.count(')')
            
            # Keep collecting until we hit the opening brace
            while not ('{' in line and paren_depth == 0):
                i += 1
                if i >= len(lines):
                    break
                line = lines[i]
                sig_lines.append(line)
                paren_depth += line.count('(') - line.count(')')
            
            # Extract just the signature (without the body)
            full_sig = '\n'.join(sig_lines)
            # Remove the opening brace and everything after
            sig = re.sub(r'\s*\{.*', '', full_sig, flags=re.DOTALL)
            
            methods.append({
                'apas_comment': apas_comment,
                'signature': sig.strip()
            })
        
        i += 1
    
    return methods

def generate_trait_definition(struct_name, methods, impl_header):
    """Generate trait definition from method signatures."""
    # Extract generic parameters from impl header
    generic_match = re.search(r'impl<([^>]+)>', impl_header)
    generics = f"<{generic_match.group(1)}>" if generic_match else ""
    
    trait_name = f"{struct_name}Trait"
    
    trait_lines = [
        f"    pub trait {trait_name}{generics} {{",
    ]
    
    for method in methods:
        if method['apas_comment']:
            trait_lines.append(f"        {method['apas_comment']}")
        
        # Convert to trait method signature (add semicolon, remove body)
        sig = method['signature']
        # Re-indent to trait level (8 spaces)
        sig = sig.replace('        pub fn ', '        fn ')
        sig = sig.replace('        fn ', '        fn ')
        trait_lines.append(f"        {sig};")
        trait_lines.append("")  # blank line between methods
    
    trait_lines.append("    }")
    
    return '\n'.join(trait_lines)

def convert_inherent_to_trait_impl(impl_block, struct_name):
    """Convert inherent impl to trait impl."""
    trait_name = f"{struct_name}Trait"
    
    # Replace impl<...> StructName { with impl<...> StructNameTrait for StructName {
    pattern = rf'(impl<[^>]+>)\s+{re.escape(struct_name)}\s*\{{'
    replacement = rf'\1 {trait_name} for {struct_name} {{'
    
    result = re.sub(pattern, replacement, impl_block)
    
    # If no generics, try simpler pattern
    if result == impl_block:
        pattern = rf'(impl)\s+{re.escape(struct_name)}\s*\{{'
        replacement = rf'\1 {trait_name} for {struct_name} {{'
        result = re.sub(pattern, replacement, impl_block)
    
    return result

def transform_file(filepath):
    """Transform a file to add trait and convert inherent impl."""
    path = Path(filepath)
    
    if not path.exists():
        print(f"ERROR: File not found: {filepath}")
        return False
    
    content = path.read_text()
    
    # Extract struct name
    struct_name = extract_struct_name(content)
    if not struct_name:
        print(f"ERROR: Could not find struct definition in {filepath}")
        return False
    
    print(f"Processing {path.name}: struct {struct_name}")
    
    # Find inherent impl block
    start, end, impl_block = find_inherent_impl_block(content, struct_name)
    if not impl_block:
        print(f"  ERROR: Could not find inherent impl for {struct_name}")
        return False
    
    # Extract method signatures
    methods = extract_method_signatures(impl_block)
    if not methods:
        print(f"  ERROR: No methods found in impl block")
        return False
    
    print(f"  Found {len(methods)} methods")
    
    # Generate trait definition
    impl_header = impl_block.split('\n')[0]
    trait_def = generate_trait_definition(struct_name, methods, impl_header)
    
    # Convert inherent impl to trait impl
    trait_impl = convert_inherent_to_trait_impl(impl_block, struct_name)
    
    # Insert trait definition before the impl block
    new_content = (
        content[:start] +
        trait_def + '\n\n' +
        trait_impl +
        content[end:]
    )
    
    # Write back
    path.write_text(new_content)
    print(f"  ✓ Added trait {struct_name}Trait and converted impl")
    
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: add_trait_to_inherent_impl.py <file1.rs> [file2.rs ...]")
        sys.exit(1)
    
    success_count = 0
    fail_count = 0
    
    for filepath in sys.argv[1:]:
        print()
        if transform_file(filepath):
            success_count += 1
        else:
            fail_count += 1
    
    print()
    print("=" * 60)
    print(f"SUCCESS: {success_count} files")
    print(f"FAILED:  {fail_count} files")

if __name__ == '__main__':
    main()

