#!/usr/bin/env python3
"""Find inherent impl blocks for sub-structs (structs not matching the filename)

Pattern to detect:
- File: ArraySeqMtEphSlice.rs
- Has: impl<T: StTInMtT> Inner<T> { ... }
- Inner != ArraySeqMtEphSlice, so it's a sub-struct

These often have simple constructors like new() and accessors like len()/length()
that could be trait methods or might be acceptable as inherent.

Git commit: [new]
Date: 2025-10-19
"""

import re
import os
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


def extract_methods_from_impl(content, impl_start_line, lines):
    """Extract all method names from an impl block."""
    methods = []
    
    # Find the impl block boundaries
    brace_count = 0
    impl_end = impl_start_line
    
    for i in range(impl_start_line, len(lines)):
        for char in lines[i]:
            if char == '{':
                brace_count += 1
            elif char == '}':
                brace_count -= 1
                if brace_count == 0:
                    impl_end = i
                    break
        if brace_count == 0:
            break
    
    impl_content = '\n'.join(lines[impl_start_line:impl_end+1])
    
    # Find all method definitions (public and private)
    method_pattern = r'\b(?:pub\s+)?fn\s+(\w+)\s*(?:<[^>]*>)?\s*\('
    for match in re.finditer(method_pattern, impl_content):
        method_name = match.group(1)
        methods.append(method_name)
    
    return methods


def find_substruct_inherent_impls(src_dir="src"):
    """Find inherent impl blocks for sub-structs (not matching filename)."""
    
    results = []
    
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if not file.endswith(".rs") or file == "Types.rs":
                continue
            
            filepath = os.path.join(root, file)
            # Expected main struct name from filename
            expected_struct = file[:-3]  # Remove .rs
            
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    lines = f.readlines()
                
                for i, line in enumerate(lines):
                    if not re.match(r'^\s+impl', line):
                        continue
                    
                    # Skip if has 'for' on same line
                    if ' for ' in line:
                        continue
                    
                    # Check next few lines for 'for' (multiline trait impl)
                    is_trait_impl = False
                    if not line.rstrip().endswith('{'):
                        for j in range(i + 1, min(i + 5, len(lines))):
                            next_line = lines[j].strip()
                            if next_line.startswith('for '):
                                is_trait_impl = True
                                break
                            if next_line.endswith('{'):
                                break
                    
                    if is_trait_impl:
                        continue
                    
                    # Extract struct name from impl line
                    struct_match = re.search(r'impl(?:<[^>]*>)?\s+(\w+)(?:<[^>]*>)?\s*\{?', line)
                    if not struct_match:
                        continue
                    
                    struct_name = struct_match.group(1)
                    
                    # Check if struct name matches expected filename
                    if struct_name == expected_struct:
                        continue  # This is the main struct, not a sub-struct
                    
                    # This is a sub-struct!
                    methods = extract_methods_from_impl(filepath, i, lines)
                    
                    results.append({
                        'file': filepath,
                        'line': i + 1,
                        'expected_struct': expected_struct,
                        'actual_struct': struct_name,
                        'impl_line': line.strip(),
                        'methods': methods,
                        'method_count': len(methods)
                    })
                    
            except Exception as e:
                print(f"Error reading {filepath}: {e}", file=sys.stderr)
    
    return results


def main():
    project_root = Path(__file__).parent.parent.parent.parent
    log_path = project_root / 'analyses' / 'code_review' / 'substruct_inherent_impls.txt'
    
    tee = TeeOutput(log_path)
    
    tee.print("SUB-STRUCT INHERENT IMPL BLOCKS")
    tee.print("=" * 70)
    tee.print()
    tee.print("Inherent impl blocks for structs that DON'T match the filename.")
    tee.print("These are helper/sub-structs that might need trait conversion.")
    tee.print()
    
    src_dir = project_root / 'src'
    results = find_substruct_inherent_impls(str(src_dir))
    
    tee.print(f"Found {len(results)} sub-struct inherent impl blocks")
    tee.print()
    
    # Group by file
    by_file = defaultdict(list)
    for r in results:
        by_file[r['file']].append(r)
    
    # Report by file
    tee.print("DETAILED LISTING:")
    tee.print("-" * 70)
    tee.print()
    
    for filepath in sorted(by_file.keys()):
        rel_path = filepath.replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}")
        
        for impl_info in sorted(by_file[filepath], key=lambda x: x['line']):
            tee.print(f"  Line {impl_info['line']}: impl {impl_info['actual_struct']}")
            tee.print(f"    Expected: {impl_info['expected_struct']}")
            tee.print(f"    Methods ({impl_info['method_count']}): {', '.join(impl_info['methods'])}")
            tee.print()
        tee.print()
    
    # Categorize by method count
    tee.print()
    tee.print("=" * 70)
    tee.print("BY METHOD COUNT:")
    tee.print("-" * 70)
    tee.print()
    
    simple_impls = [r for r in results if r['method_count'] <= 3]
    medium_impls = [r for r in results if 4 <= r['method_count'] <= 10]
    complex_impls = [r for r in results if r['method_count'] > 10]
    
    tee.print(f"SIMPLE (1-3 methods): {len(simple_impls)}")
    tee.print("-" * 40)
    for r in sorted(simple_impls, key=lambda x: (x['file'], x['line'])):
        rel_path = r['file'].replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{r['line']}")
        tee.print(f"  {r['actual_struct']}: {', '.join(r['methods'])}")
        tee.print()
    
    tee.print()
    tee.print(f"MEDIUM (4-10 methods): {len(medium_impls)}")
    tee.print("-" * 40)
    for r in sorted(medium_impls, key=lambda x: (x['file'], x['line'])):
        rel_path = r['file'].replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{r['line']}")
        tee.print(f"  {r['actual_struct']}: {len(r['methods'])} methods")
        tee.print()
    
    tee.print()
    tee.print(f"COMPLEX (>10 methods): {len(complex_impls)}")
    tee.print("-" * 40)
    for r in sorted(complex_impls, key=lambda x: (x['file'], x['line'])):
        rel_path = r['file'].replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{r['line']}")
        tee.print(f"  {r['actual_struct']}: {len(r['methods'])} methods")
        tee.print()
    
    # Look for common patterns
    tee.print()
    tee.print("=" * 70)
    tee.print("COMMON PATTERNS:")
    tee.print("-" * 70)
    tee.print()
    
    has_new = [r for r in results if 'new' in r['methods']]
    has_len = [r for r in results if 'len' in r['methods'] or 'length' in r['methods']]
    node_impls = [r for r in results if 'Node' in r['actual_struct']]
    
    tee.print(f"Has 'new' method: {len(has_new)}")
    tee.print(f"Has 'len' or 'length': {len(has_len)}")
    tee.print(f"Struct name contains 'Node': {len(node_impls)}")
    tee.print()
    
    tee.print("=" * 70)
    tee.print("SUMMARY:")
    tee.print(f"  Total sub-struct impls: {len(results)}")
    tee.print(f"  Simple (1-3 methods): {len(simple_impls)}")
    tee.print(f"  Medium (4-10 methods): {len(medium_impls)}")
    tee.print(f"  Complex (>10 methods): {len(complex_impls)}")
    tee.print()
    tee.print(f"Log written to: {log_path}")
    
    tee.close()


if __name__ == "__main__":
    main()

