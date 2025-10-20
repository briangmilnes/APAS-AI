#!/usr/bin/env python3
"""
Detect Default trait implementations and analyze if they can be simplified.

Checks:
1. Manual Default impls that just call Default::default() on fields
   -> Could use #[derive(Default)] instead
2. Manual Default impls with custom logic
   -> Need to stay manual
3. Structs with #[derive(Default)] already
   -> Already optimized

Git commit: [new]
Date: 2025-10-19
"""

import re
import sys
from pathlib import Path
from collections import defaultdict


class TeeOutput:
    """Write to both stdout and a log file."""
    def __init__(self, log_path):
        self.log_path = Path(log_path)
        self.log_path.parent.mkdir(parents=True, exist_ok=True)
        self.log_file = open(self.log_path, 'w', encoding='utf-8')
    
    def write(self, text):
        print(text, end='', flush=True)
        self.log_file.write(text)
        self.log_file.flush()
    
    def print(self, text=''):
        self.write(text + '\n')
    
    def close(self):
        self.log_file.close()


def find_struct_definition(content, struct_name):
    """Find struct definition and check if it has #[derive(Default)]."""
    # Look for struct definition
    struct_pattern = rf'(?:pub\s+)?struct\s+{struct_name}\s*(?:<[^>]*>)?\s*\{{'
    
    for match in re.finditer(struct_pattern, content):
        # Look backwards from struct to find derives
        start = max(0, match.start() - 500)
        before_struct = content[start:match.start()]
        
        # Check for #[derive(...)] containing Default
        derive_pattern = r'#\[derive\([^\]]*Default[^\]]*\)\]'
        if re.search(derive_pattern, before_struct):
            return 'derived'
        
        return 'no_derive'
    
    return None


def analyze_default_impl(content, impl_start, impl_end):
    """Analyze a Default impl to see if it's trivial."""
    impl_content = content[impl_start:impl_end]
    
    # Find the default() method
    method_pattern = r'fn\s+default\s*\(\s*\)\s*->\s*Self\s*\{([^}]+)\}'
    method_match = re.search(method_pattern, impl_content, re.DOTALL)
    
    if not method_match:
        # Multi-line or complex implementation
        return 'complex', None
    
    method_body = method_match.group(1).strip()
    
    # Check if it's a simple struct initialization
    # Pattern: StructName { field1: Type::default(), field2: Type::default(), ... }
    # or: Self { field1: Type::default(), ... }
    
    # Remove comments
    method_body = re.sub(r'//.*$', '', method_body, flags=re.MULTILINE)
    method_body = re.sub(r'/\*.*?\*/', '', method_body, flags=re.DOTALL)
    method_body = method_body.strip()
    
    # Check if it's struct initialization syntax
    struct_init_pattern = r'^(?:Self|\w+)\s*\{([^}]+)\}$'
    struct_init_match = re.match(struct_init_pattern, method_body, re.DOTALL)
    
    if not struct_init_match:
        return 'complex', method_body
    
    fields_str = struct_init_match.group(1)
    
    # Parse field initializations
    # Look for: field_name: expression
    field_pattern = r'(\w+)\s*:\s*([^,}]+)'
    fields = re.findall(field_pattern, fields_str)
    
    if not fields:
        return 'empty', method_body
    
    # Check if all fields use Default::default() or ::default()
    all_default = True
    custom_fields = []
    
    for field_name, field_expr in fields:
        field_expr = field_expr.strip()
        
        # Check if it's a default call
        default_patterns = [
            r'^Default::default\(\)$',
            r'^\w+::default\(\)$',
            r'^<[^>]+>::default\(\)$',
        ]
        
        is_default = any(re.match(pattern, field_expr) for pattern in default_patterns)
        
        if not is_default:
            all_default = False
            custom_fields.append((field_name, field_expr))
    
    if all_default:
        return 'trivial', method_body
    else:
        return 'partial', method_body
    
    return 'complex', method_body


def find_default_impls(src_dir="src"):
    """Find all Default trait implementations."""
    
    results = []
    
    for filepath in Path(src_dir).rglob('*.rs'):
        if filepath.name == 'Types.rs':
            continue
        
        try:
            content = filepath.read_text(encoding='utf-8')
        except Exception as e:
            print(f"Error reading {filepath}: {e}", file=sys.stderr)
            continue
        
        # Find all impl Default blocks
        # Pattern: impl<...> Default for TypeName<...> {
        impl_pattern = r'impl(?:<[^>]*>)?\s+Default\s+for\s+(\w+)(?:<[^>]*>)?\s*\{'
        
        for match in re.finditer(impl_pattern, content):
            struct_name = match.group(1)
            impl_start = match.start()
            
            # Find the end of the impl block
            brace_count = 0
            i = match.end() - 1  # Start at opening brace
            
            while i < len(content):
                if content[i] == '{':
                    brace_count += 1
                elif content[i] == '}':
                    brace_count -= 1
                    if brace_count == 0:
                        break
                i += 1
            
            impl_end = i + 1
            
            # Calculate line number
            line_num = content[:impl_start].count('\n') + 1
            
            # Check if struct already has #[derive(Default)]
            struct_status = find_struct_definition(content, struct_name)
            
            # Analyze the implementation
            impl_type, impl_body = analyze_default_impl(content, impl_start, impl_end)
            
            results.append({
                'file': str(filepath),
                'line': line_num,
                'struct': struct_name,
                'impl_type': impl_type,
                'struct_status': struct_status,
                'body': impl_body[:200] if impl_body else None
            })
    
    return results


def main():
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / 'src'
    log_path = project_root / 'analyses' / 'code_review' / 'default_impls.txt'
    
    tee = TeeOutput(log_path)
    
    tee.print("DEFAULT TRAIT IMPLEMENTATIONS ANALYSIS")
    tee.print("=" * 70)
    tee.print()
    tee.print("Analyzing Default trait implementations to find simplification opportunities.")
    tee.print()
    
    results = find_default_impls(str(src_dir))
    
    tee.print(f"Found {len(results)} Default trait implementations")
    tee.print()
    
    # Categorize results
    trivial = [r for r in results if r['impl_type'] == 'trivial']
    partial = [r for r in results if r['impl_type'] == 'partial']
    complex_impls = [r for r in results if r['impl_type'] == 'complex']
    empty = [r for r in results if r['impl_type'] == 'empty']
    
    # Report trivial implementations (can use #[derive(Default)])
    tee.print("TRIVIAL IMPLEMENTATIONS (can use #[derive(Default)]):")
    tee.print("-" * 70)
    tee.print()
    tee.print("These manually implement Default by calling ::default() on all fields.")
    tee.print("They can be replaced with #[derive(Default)] on the struct.")
    tee.print()
    
    for r in sorted(trivial, key=lambda x: (x['file'], x['line'])):
        rel_path = r['file'].replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{r['line']}")
        tee.print(f"  struct: {r['struct']}")
        if r['body']:
            tee.print(f"  body: {r['body'][:100]}...")
        tee.print()
    
    # Report partial implementations (some custom values)
    tee.print()
    tee.print("PARTIAL CUSTOM (some fields have custom defaults):")
    tee.print("-" * 70)
    tee.print()
    tee.print("These set some fields to custom values, not just ::default().")
    tee.print("Must remain manual implementations.")
    tee.print()
    
    for r in sorted(partial, key=lambda x: (x['file'], x['line'])):
        rel_path = r['file'].replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{r['line']}")
        tee.print(f"  struct: {r['struct']}")
        if r['body']:
            tee.print(f"  body: {r['body'][:150]}...")
        tee.print()
    
    # Report complex implementations
    tee.print()
    tee.print("COMPLEX IMPLEMENTATIONS (must remain manual):")
    tee.print("-" * 70)
    tee.print()
    tee.print("These have complex logic and cannot use #[derive(Default)].")
    tee.print()
    
    for r in sorted(complex_impls, key=lambda x: (x['file'], x['line'])):
        rel_path = r['file'].replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{r['line']}")
        tee.print(f"  struct: {r['struct']}")
        if r['body']:
            # Show first line of body
            first_line = r['body'].split('\n')[0] if r['body'] else ''
            tee.print(f"  body: {first_line[:100]}...")
        tee.print()
    
    # Report empty implementations
    if empty:
        tee.print()
        tee.print("EMPTY IMPLEMENTATIONS:")
        tee.print("-" * 70)
        tee.print()
        for r in sorted(empty, key=lambda x: (x['file'], x['line'])):
            rel_path = r['file'].replace(str(project_root) + '/', '')
            tee.print(f"{rel_path}:{r['line']}: {r['struct']}")
    
    # Summary
    tee.print()
    tee.print("=" * 70)
    tee.print("SUMMARY:")
    tee.print("-" * 70)
    tee.print(f"  Trivial (can use #[derive(Default)]): {len(trivial)}")
    tee.print(f"  Partial custom (some custom values): {len(partial)}")
    tee.print(f"  Complex (need manual impl): {len(complex_impls)}")
    tee.print(f"  Empty: {len(empty)}")
    tee.print(f"  TOTAL: {len(results)}")
    tee.print()
    
    # Show potential savings
    if trivial:
        tee.print(f"RECOMMENDATION: {len(trivial)} Default implementations can be")
        tee.print(f"replaced with #[derive(Default)] to reduce boilerplate.")
    
    tee.print()
    tee.print(f"Log written to: {log_path}")
    
    tee.close()


if __name__ == "__main__":
    main()

