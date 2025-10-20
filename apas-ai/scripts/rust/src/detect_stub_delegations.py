#!/usr/bin/env python3
"""
Detect stub delegation anti-pattern in inherent impl blocks.

Anti-pattern from RustRules.md (lines 134-168):
"A 'helper' function that is called once from a trait or impl is a stub and should be in the impl, if not public."

Finds inherent impl methods that:
1. Are simple delegations (single-line body calling another method)
2. Are only called once (or only in tests)
3. Should be moved to trait impl or inlined

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


def is_simple_delegation(method_body):
    """Check if method body is a simple delegation (one statement calling another method)."""
    # Remove comments and whitespace
    body = re.sub(r'//.*$', '', method_body, flags=re.MULTILINE)
    body = re.sub(r'/\*.*?\*/', '', body, flags=re.DOTALL)
    body = body.strip()
    
    # Single return statement that calls another method
    patterns = [
        r'^\{\s*self\.(\w+)\([^}]*\)\s*\}$',  # { self.method(...) }
        r'^\{\s*Self::(\w+)\([^}]*\)\s*\}$',  # { Self::method(...) }
        r'^\{\s*return\s+self\.(\w+)\([^}]*\);\s*\}$',  # { return self.method(...); }
        r'^\{\s*return\s+Self::(\w+)\([^}]*\);\s*\}$',  # { return Self::method(...); }
    ]
    
    for pattern in patterns:
        if re.search(pattern, body, re.DOTALL):
            return True
    
    return False


def find_inherent_methods(filepath):
    """Find all inherent impl methods and analyze them."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            content = f.read()
            lines = content.split('\n')
    except Exception as e:
        print(f"Error reading {filepath}: {e}", file=sys.stderr)
        return []
    
    results = []
    
    # Find inherent impl blocks: impl<...> TypeName<...> {
    # Check next few lines to exclude trait impls
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
        
        # Find the impl block content
        impl_start = i
        brace_count = 0
        impl_end = i
        
        for j in range(i, len(lines)):
            for char in lines[j]:
                if char == '{':
                    brace_count += 1
                elif char == '}':
                    brace_count -= 1
                    if brace_count == 0:
                        impl_end = j
                        break
            if brace_count == 0:
                break
        
        # Extract struct name from impl line
        impl_line = lines[i]
        struct_match = re.search(r'impl(?:<[^>]*>)?\s+(\w+)(?:<[^>]*>)?\s*\{', impl_line)
        if not struct_match:
            continue
        
        struct_name = struct_match.group(1)
        
        # Find all public methods in this impl block
        impl_content = '\n'.join(lines[impl_start:impl_end+1])
        
        # Find method definitions
        method_pattern = r'pub\s+fn\s+(\w+)\s*(?:<[^>]*>)?\s*\([^)]*\)(?:\s*->\s*[^{]+)?\s*\{'
        
        for method_match in re.finditer(method_pattern, impl_content):
            method_name = method_match.group(1)
            
            # Find method body
            method_start_pos = method_match.end() - 1  # At opening brace
            brace_count = 0
            method_end_pos = method_start_pos
            
            for pos in range(method_start_pos, len(impl_content)):
                if impl_content[pos] == '{':
                    brace_count += 1
                elif impl_content[pos] == '}':
                    brace_count -= 1
                    if brace_count == 0:
                        method_end_pos = pos
                        break
            
            method_body = impl_content[method_start_pos:method_end_pos+1]
            
            # Check if it's a simple delegation
            if is_simple_delegation(method_body):
                # Calculate line number
                lines_before = impl_content[:method_match.start()].count('\n')
                method_line = impl_start + lines_before + 1
                
                results.append({
                    'file': filepath,
                    'line': method_line,
                    'struct': struct_name,
                    'method': method_name,
                    'body': method_body.strip()
                })
    
    return results


def count_method_usages(method_name, struct_name, src_dir):
    """Count how many times a method is called in src/ and tests/."""
    src_count = 0
    test_count = 0
    
    # Pattern to find method calls: struct.method( or Type::method(
    patterns = [
        rf'\b{struct_name}::{method_name}\s*\(',
        rf'\.{method_name}\s*\(',
    ]
    
    # Search in src/
    for src_file in (src_dir / 'src').rglob('*.rs'):
        try:
            content = src_file.read_text(encoding='utf-8')
            for pattern in patterns:
                src_count += len(re.findall(pattern, content))
        except:
            pass
    
    # Search in tests/
    for test_file in (src_dir / 'tests').rglob('*.rs'):
        try:
            content = test_file.read_text(encoding='utf-8')
            for pattern in patterns:
                test_count += len(re.findall(pattern, content))
        except:
            pass
    
    return src_count, test_count


def main():
    project_root = Path(__file__).parent.parent.parent.parent
    src_dir = project_root / 'src'
    log_path = project_root / 'analyses' / 'code_review' / 'stub_delegations.txt'
    
    tee = TeeOutput(log_path)
    
    tee.print("STUB DELEGATION ANTI-PATTERNS")
    tee.print("=" * 70)
    tee.print()
    tee.print("Inherent impl methods that are simple delegations and may violate")
    tee.print("the 'No Stub Delegation' rule (RustRules.md lines 134-168)")
    tee.print()
    
    all_delegations = []
    
    # Scan all source files
    for rs_file in src_dir.rglob('*.rs'):
        if rs_file.name == 'Types.rs':
            continue
        
        methods = find_inherent_methods(rs_file)
        all_delegations.extend(methods)
    
    tee.print(f"Found {len(all_delegations)} potential stub delegations")
    tee.print()
    
    # Analyze usage patterns
    tee.print("USAGE ANALYSIS:")
    tee.print("-" * 70)
    tee.print()
    
    only_in_tests = []
    single_use = []
    multiple_use = []
    
    for delegation in all_delegations:
        src_count, test_count = count_method_usages(
            delegation['method'],
            delegation['struct'],
            project_root
        )
        
        delegation['src_count'] = src_count
        delegation['test_count'] = test_count
        total = src_count + test_count
        
        if total == 0:
            # Not used at all - might be dead code
            only_in_tests.append(delegation)
        elif src_count == 0 and test_count > 0:
            # Only used in tests
            only_in_tests.append(delegation)
        elif total == 1:
            # Used exactly once
            single_use.append(delegation)
        else:
            # Used multiple times - might be legitimate
            multiple_use.append(delegation)
    
    # Report results
    tee.print("ONLY USED IN TESTS (or not used at all):")
    tee.print("-" * 70)
    for d in sorted(only_in_tests, key=lambda x: (x['file'], x['line'])):
        rel_path = str(d['file']).replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{d['line']}")
        tee.print(f"  {d['struct']}::{d['method']}()")
        tee.print(f"  Usage: {d['test_count']} in tests, {d['src_count']} in src")
        tee.print(f"  Body: {d['body'][:100]}...")
        tee.print()
    
    tee.print()
    tee.print("SINGLE USE (called exactly once):")
    tee.print("-" * 70)
    for d in sorted(single_use, key=lambda x: (x['file'], x['line'])):
        rel_path = str(d['file']).replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{d['line']}")
        tee.print(f"  {d['struct']}::{d['method']}()")
        tee.print(f"  Usage: {d['test_count']} in tests, {d['src_count']} in src")
        tee.print()
    
    tee.print()
    tee.print("MULTIPLE USE (might be legitimate):")
    tee.print("-" * 70)
    for d in sorted(multiple_use, key=lambda x: (x['file'], x['line'])):
        rel_path = str(d['file']).replace(str(project_root) + '/', '')
        tee.print(f"{rel_path}:{d['line']}")
        tee.print(f"  {d['struct']}::{d['method']}()")
        tee.print(f"  Usage: {d['test_count']} in tests, {d['src_count']} in src")
        tee.print()
    
    tee.print()
    tee.print("=" * 70)
    tee.print("SUMMARY:")
    tee.print(f"  Only in tests/unused: {len(only_in_tests)}")
    tee.print(f"  Single use: {len(single_use)}")
    tee.print(f"  Multiple use: {len(multiple_use)}")
    tee.print(f"  TOTAL: {len(all_delegations)}")
    tee.print()
    tee.print(f"Log written to: {log_path}")
    
    tee.close()


if __name__ == "__main__":
    main()

