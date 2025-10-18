#!/usr/bin/env python3
"""
Convert an inherent impl to a trait impl following Single Implementation Pattern.

Steps:
1. Extract all public methods from inherent impl
2. Create trait definition with those methods
3. Create trait impl that implements the trait
4. Keep private methods in inherent impl (these should be converted to module functions manually)

Git commit: 725dae7fef3f6f5b33f3f8e0c3e8f0e6e5d5e5d5
"""

import re
import sys
from pathlib import Path
import argparse

class TeeOutput:
    """Print to both stdout and file."""
    def __init__(self, filepath):
        self.file = open(filepath, 'w')
        self.stdout = sys.stdout
    
    def print(self, *args, **kwargs):
        print(*args, **kwargs)
        print(*args, **kwargs, file=self.file)
    
    def close(self):
        self.file.close()

def extract_impl_block(lines, start_line):
    """Extract the full impl block starting at start_line (0-indexed)."""
    brace_count = 0
    in_impl = False
    impl_lines = []
    
    for i in range(start_line, len(lines)):
        line = lines[i]
        impl_lines.append(line)
        
        for char in line:
            if char == '{':
                brace_count += 1
                in_impl = True
            elif char == '}':
                brace_count -= 1
                if in_impl and brace_count == 0:
                    return impl_lines, i
    
    return impl_lines, len(lines) - 1

def parse_methods(impl_lines):
    """Parse methods from impl block, separating public and private."""
    public_methods = []
    private_methods = []
    
    in_method = False
    current_method = []
    brace_count = 0
    is_public = False
    
    for line in impl_lines[1:]:  # Skip first line (impl declaration)
        stripped = line.strip()
        
        # Check if this is a method declaration
        if stripped.startswith('pub fn ') or stripped.startswith('fn '):
            if current_method:  # Save previous method
                if is_public:
                    public_methods.append(current_method)
                else:
                    private_methods.append(current_method)
            
            current_method = [line]
            is_public = stripped.startswith('pub fn ')
            in_method = True
            brace_count = line.count('{') - line.count('}')
        elif in_method:
            current_method.append(line)
            brace_count += line.count('{') - line.count('}')
            
            if brace_count == 0 and '{' in ''.join(current_method):
                # Method is complete
                if is_public:
                    public_methods.append(current_method)
                else:
                    private_methods.append(current_method)
                current_method = []
                in_method = False
    
    return public_methods, private_methods

def create_trait_definition(impl_header, public_methods):
    """Create trait definition from impl header and public methods."""
    # Extract struct name and generics
    match = re.search(r'impl<([^>]+)>\s+(\w+)', impl_header)
    if not match:
        return None
    
    generics = match.group(1)
    struct_name = match.group(2)
    trait_name = f"{struct_name}Trait"
    
    trait_lines = [f"pub trait {trait_name}<{generics}> {{"]
    
    # Add method signatures
    for method in public_methods:
        # Convert method implementation to signature
        for line in method:
            stripped = line.strip()
            if stripped.startswith('pub fn '):
                # Extract signature (everything before the first '{')
                sig = stripped.split('{')[0].strip()
                # Remove 'pub' since trait methods don't use it
                sig = sig.replace('pub fn ', 'fn ')
                trait_lines.append(f"    {sig};")
                break
    
    trait_lines.append("}")
    return trait_lines, trait_name

def main():
    parser = argparse.ArgumentParser(description='Convert inherent impl to trait impl')
    parser.add_argument('file', help='Source file to fix')
    parser.add_argument('--line', type=int, help='Line number of impl block')
    parser.add_argument('--dry-run', action='store_true', help='Show changes without applying')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/fix_convert_inherent_to_trait.txt',
                       help='Output log file path')
    args = parser.parse_args()
    
    filepath = Path(args.file)
    log_path = Path(args.log_file)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    
    tee = TeeOutput(log_path)
    
    tee.print(f"Converting inherent impl to trait impl: {filepath}")
    tee.print("="*80)
    
    if not filepath.exists():
        tee.print(f"ERROR: File not found: {filepath}")
        tee.close()
        return 1
    
    lines = filepath.read_text().split('\n')
    
    # Find impl block
    impl_start = None
    if args.line:
        impl_start = args.line - 1  # Convert to 0-indexed
    else:
        # Find first inherent impl with generics
        for i, line in enumerate(lines):
            if re.match(r'^\s*impl<[^>]+>\s+\w+\s*\{', line) and ' for ' not in line:
                impl_start = i
                break
    
    if impl_start is None:
        tee.print("ERROR: No inherent impl found")
        tee.close()
        return 1
    
    tee.print(f"Found impl at line {impl_start + 1}")
    
    # Extract impl block
    impl_lines, impl_end = extract_impl_block(lines, impl_start)
    impl_header = impl_lines[0].strip()
    
    tee.print(f"Impl block: lines {impl_start + 1}-{impl_end + 1}")
    tee.print(f"Header: {impl_header}")
    
    # Parse methods
    public_methods, private_methods = parse_methods(impl_lines)
    
    tee.print(f"\nFound {len(public_methods)} public methods, {len(private_methods)} private methods")
    
    if not public_methods:
        tee.print("WARNING: No public methods found. Nothing to convert.")
        tee.close()
        return 0
    
    # Create trait definition
    trait_result = create_trait_definition(impl_header, public_methods)
    if not trait_result:
        tee.print("ERROR: Could not parse impl header")
        tee.close()
        return 1
    
    trait_lines, trait_name = trait_result
    
    tee.print(f"\nCreated trait: {trait_name}")
    tee.print("\nTrait definition:")
    for line in trait_lines:
        tee.print(f"  {line}")
    
    # Create trait impl
    trait_impl_header = impl_header.replace(f"impl<", f"impl<").replace("> ", f"> {trait_name} for ")
    # Need to extract struct name
    match = re.search(r'impl<([^>]+)>\s+(\w+)', impl_header)
    struct_name = match.group(2)
    trait_impl_header = f"impl<{match.group(1)}> {trait_name}<{match.group(1)}> for {struct_name}<{match.group(1)}> {{"
    
    tee.print(f"\nTrait impl header: {trait_impl_header}")
    
    if args.dry_run:
        tee.print("\n[DRY RUN] Changes not applied")
        tee.print("\nNOTE: This script handles simple cases. Complex impls may need manual conversion:")
        tee.print("  - Associated types or constants")
        tee.print("  - Complex generic bounds")
        tee.print("  - Private helper methods (should become module functions)")
        tee.close()
        return 0
    
    tee.print("\nWARNING: Automatic conversion not yet implemented")
    tee.print("This would require careful handling of:")
    tee.print("  - Trait definition insertion location")
    tee.print("  - Trait impl creation")
    tee.print("  - Private method handling")
    tee.print("\nPlease convert manually using the information above.")
    
    tee.print(f"\nLog written to: {log_path}")
    tee.close()
    return 0

if __name__ == "__main__":
    sys.exit(main())

