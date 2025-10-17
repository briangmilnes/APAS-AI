#!/usr/bin/env python3
"""
Add Display implementations for structs that don't have them.

This script generates simple Display impls that show the struct name and fields.
For most data structures, this is sufficient for debugging.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


def extract_struct_info(lines, struct_line_idx):
    """Extract struct name, generic params with bounds, and visibility."""
    line = lines[struct_line_idx]
    
    # Match: pub struct Name
    match = re.match(r'\s*(pub\s+)?struct\s+(\w+)', line)
    if not match:
        return None
    
    visibility = match.group(1) or ''
    name = match.group(2)
    
    # Extract generics manually to handle nested angle brackets
    generics_raw = ''
    after_name = line[match.end():]
    if after_name.strip().startswith('<'):
        depth = 0
        start_idx = after_name.index('<')
        for i, char in enumerate(after_name[start_idx:], start=start_idx):
            if char == '<':
                depth += 1
            elif char == '>':
                depth -= 1
                if depth == 0:
                    generics_raw = after_name[start_idx:i+1]
                    break
    
    # Parse generic params WITH their bounds, handling nested generics
    generic_params = []
    generic_params_with_bounds = []
    if generics_raw:
        params_str = generics_raw[1:-1]  # Remove < >
        
        # Split by commas, but respect angle bracket nesting
        current_param = ''
        depth = 0
        for char in params_str:
            if char == '<':
                depth += 1
                current_param += char
            elif char == '>':
                depth -= 1
                current_param += char
            elif char == ',' and depth == 0:
                # This is a top-level comma, split here
                full_param = current_param.strip()
                param_name = full_param.split(':')[0].strip()
                if param_name and not param_name.startswith('\''):
                    generic_params.append(param_name)
                    generic_params_with_bounds.append(full_param)
                current_param = ''
            else:
                current_param += char
        
        # Don't forget the last parameter
        if current_param:
            full_param = current_param.strip()
            param_name = full_param.split(':')[0].strip()
            if param_name and not param_name.startswith('\''):
                generic_params.append(param_name)
                generic_params_with_bounds.append(full_param)
    
    # Create clean generics for type application (without bounds)
    clean_generics = f"<{', '.join(generic_params)}>" if generic_params else ''
    
    return {
        'name': name,
        'generics': clean_generics,
        'generic_params': generic_params,
        'generic_params_with_bounds': generic_params_with_bounds,
        'visibility': visibility.strip()
    }


def has_display_impl(lines, struct_name):
    """Check if struct already has Display impl."""
    pattern = rf'impl.*Display\s+for\s+{struct_name}'
    for line in lines:
        if re.search(pattern, line):
            return True
    return False


def generate_display_impl(struct_info, indent="    "):
    """Generate a simple Display implementation."""
    name = struct_info['name']
    generics = struct_info['generics']
    generic_params = struct_info.get('generic_params', [])
    generic_params_with_bounds = struct_info.get('generic_params_with_bounds', [])
    
    # Build impl signature
    if generic_params:
        impl_generics = f"<{', '.join(generic_params)}>"
        
        # Only generate where clause if params have bounds
        # Filter out params without bounds (just "T" vs "T: Bound")
        params_with_bounds_only = [p for p in generic_params_with_bounds if ':' in p]
        
        if params_with_bounds_only:
            where_clause = f"\n{indent}where\n"
            for param_with_bounds in params_with_bounds_only:
                where_clause += f"{indent}    {param_with_bounds},\n"
            where_clause = where_clause.rstrip(',\n') + "\n"
        else:
            where_clause = ""
    else:
        impl_generics = ""
        where_clause = ""
    
    type_with_generics = f"{name}{generics}"
    
    impl = f"\n{indent}impl{impl_generics} std::fmt::Display for {type_with_generics}"
    if where_clause:
        impl += where_clause
    impl += f"{indent}{{\n"
    impl += f"{indent}    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{\n"
    impl += f'{indent}        write!(f, "{name}{{}}", "")\n'
    impl += f"{indent}    }}\n"
    impl += f"{indent}}}\n"
    
    return impl


def find_insert_location(lines):
    """Find where to insert new impl blocks (after all existing impls, before module closing brace)."""
    # Find the last impl block's closing brace
    last_impl_end = 0
    in_impl = False
    brace_count = 0
    
    for i, line in enumerate(lines):
        stripped = line.strip()
        
        # Start of impl block
        if re.match(r'impl\s', stripped):
            in_impl = True
            brace_count = 0
        
        # Count braces inside impl
        if in_impl:
            brace_count += stripped.count('{')
            brace_count -= stripped.count('}')
            
            # Found the end of this impl block
            if brace_count == 0 and '}' in stripped:
                last_impl_end = i
                in_impl = False
    
    # Insert after the last impl block
    if last_impl_end > 0:
        return last_impl_end + 1
    
    # No impls found, insert before the final closing brace
    for i in range(len(lines) - 1, -1, -1):
        if lines[i].strip() == '}':
            return i
    
    return len(lines)


def fix_file(filepath, context):
    """Add Display implementations for structs that don't have them."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return 0
    
    # Find all structs that need Display
    structs_needing_display = []
    
    for i, line in enumerate(lines):
        match = re.match(r'\s*(?:pub\s+)?struct\s+(\w+)', line)
        if match:
            struct_name = match.group(1)
            
            if not has_display_impl(lines, struct_name):
                struct_info = extract_struct_info(lines, i)
                if struct_info:
                    structs_needing_display.append(struct_info)
    
    if not structs_needing_display:
        print(f"  No structs need Display implementation")
        return 0
    
    # Find where to insert impls
    insert_idx = find_insert_location(lines)
    
    # Generate and insert Display impls
    new_impls = []
    for struct_info in structs_needing_display:
        impl_code = generate_display_impl(struct_info)
        new_impls.append(impl_code)
        print(f"  {struct_info['name']}: Added Display impl")
    
    # Insert all impls at the end
    lines.insert(insert_idx, '\n'.join(new_impls))
    
    # Write back
    try:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.writelines(lines)
        return len(structs_needing_display)
    except Exception as e:
        print(f"Error writing {filepath}: {e}")
        return 0


def main():
    parser = create_review_parser(
        description="Add Display implementations for structs"
    )
    parser.add_argument(
        'files',
        nargs='*',
        help='Specific files to fix'
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    if args.files:
        files_to_fix = [Path(f) if Path(f).is_absolute() else context.repo_root / f for f in args.files]
    else:
        print("Usage: fix_add_display_impls.py file1.rs file2.rs ...")
        return 1

    total_fixes = 0
    for filepath in files_to_fix:
        if not filepath.exists():
            print(f"✗ File not found: {filepath}")
            continue
        
        print(f"\nFixing {context.relative_path(filepath)}...")
        fixes = fix_file(filepath, context)
        total_fixes += fixes

    print(f"\n{'='*70}")
    print(f"Total: Added Display impl to {total_fixes} struct(s)")
    return 0


if __name__ == '__main__':
    sys.exit(main())

