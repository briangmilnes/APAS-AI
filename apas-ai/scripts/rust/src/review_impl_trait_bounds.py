#!/usr/bin/env python3
"""Review inherent impl blocks and show their corresponding trait bounds.
Git commit: 287bbc0a1f4e7c8b6e9d2f3a4c5b6d7e8f9a0b1c
Date: 2025-10-18

For each inherent impl with generics, show:
- The impl declaration with its bounds
- The corresponding trait declaration with its bounds
- Allows comparison to identify mismatches, contradictions, etc.
"""

import argparse
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


def extract_trait_name(impl_line):
    """Extract trait name from impl line."""
    # Pattern: impl<...> TraitName<...> or impl<...> TypeName {
    # We want the name after the generics
    match = re.search(r'impl<[^>]+>\s+(\w+)', impl_line)
    if match:
        return match.group(1)
    return None


def find_trait_definition(src_dir, trait_name, start_file):
    """Find the trait definition with its bounds."""
    # First try the same file
    try:
        with open(start_file, 'r', encoding='utf-8') as f:
            for i, line in enumerate(f, 1):
                if re.search(rf'pub\s+trait\s+{trait_name}\b', line):
                    return f"{start_file}:{i}:{line.rstrip()}"
    except:
        pass
    
    # Then try other files in the same directory
    start_dir = os.path.dirname(start_file)
    for file in os.listdir(start_dir):
        if file.endswith('.rs'):
            filepath = os.path.join(start_dir, file)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    for i, line in enumerate(f, 1):
                        if re.search(rf'pub\s+trait\s+{trait_name}\b', line):
                            return f"{filepath}:{i}:{line.rstrip()}"
            except:
                pass
    
    return None


def find_inherent_impls_with_traits(src_dir="src"):
    """Find all inherent impl blocks with generics and their trait definitions."""
    
    results = []
    
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if not file.endswith(".rs") or file == "Types.rs":
                continue
            
            filepath = os.path.join(root, file)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    for i, line in enumerate(f, 1):
                        # Match: impl<...> TypeName { or impl<...> TraitName
                        # But NOT: impl ... for ...
                        if re.match(r'^\s+impl', line) and ' for ' not in line:
                            stripped = line.strip()
                            # Only care about ones with generics
                            if '<' in stripped and '>' in stripped:
                                trait_name = extract_trait_name(stripped)
                                trait_def = None
                                if trait_name:
                                    trait_def = find_trait_definition(src_dir, trait_name, filepath)
                                
                                results.append({
                                    'file': filepath,
                                    'line': i,
                                    'impl_line': line.rstrip(),
                                    'trait_name': trait_name,
                                    'trait_def': trait_def
                                })
            except Exception as e:
                print(f"Error reading {filepath}: {e}", file=sys.stderr)
    
    return results


def main():
    parser = argparse.ArgumentParser(
        description='Review inherent impls with their corresponding trait bounds'
    )
    parser.add_argument('--log_file', 
                       default='analyses/code_review/review_impl_trait_bounds.txt',
                       help='Path to log file (default: analyses/code_review/review_impl_trait_bounds.txt)')
    args = parser.parse_args()
    
    project_root = Path(__file__).parent.parent.parent.parent
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    tee.print("INHERENT IMPL BLOCKS WITH TRAIT BOUNDS COMPARISON")
    tee.print("=" * 80)
    tee.print()
    
    src_dir = project_root / "src"
    results = find_inherent_impls_with_traits(str(src_dir))
    
    # Group by file for better readability
    by_file = defaultdict(list)
    for result in results:
        rel_file = result['file'].replace(str(project_root) + '/', '')
        by_file[rel_file].append(result)
    
    total_impls = 0
    total_with_traits = 0
    total_missing_traits = 0
    
    def extract_bounds(line):
        """Extract just the generic bounds from a line."""
        # Find the part between < and >
        match = re.search(r'<([^>]+)>', line)
        if match:
            return match.group(1)
        return None
    
    for filepath in sorted(by_file.keys()):
        for result in by_file[filepath]:
            total_impls += 1
            rel_file = result['file'].replace(str(project_root) + '/', '')
            
            impl_line = result['impl_line'].strip()
            impl_bounds = extract_bounds(impl_line)
            
            tee.print(f"{rel_file}:{result['line']}")
            
            if result['trait_def']:
                total_with_traits += 1
                # Parse trait def
                trait_file, trait_line, trait_decl = result['trait_def'].split(':', 2)
                trait_bounds = extract_bounds(trait_decl.strip())
                
                # Determine if this is a trait impl or inherent impl
                # Trait impl: impl<...> TraitName<...> (no 'for', matches trait name)
                # Inherent impl: impl<...> TypeName { (has '{')
                if ' for ' in impl_line or impl_line.endswith('{'):
                    # Inherent impl with trait found (private inherent method)
                    tee.print(f"  trait:         <{trait_bounds}>")
                    tee.print(f"  trait impl:    NONE")
                    tee.print(f"  inherent impl: <{impl_bounds}>")
                else:
                    # Trait impl
                    tee.print(f"  trait:         <{trait_bounds}>")
                    tee.print(f"  trait impl:    <{impl_bounds}>")
                    tee.print(f"  inherent impl: NONE")
            else:
                total_missing_traits += 1
                tee.print(f"  trait:         NONE")
                tee.print(f"  trait impl:    NONE")
                tee.print(f"  inherent impl: <{impl_bounds}>")
    
    tee.print(f"\n{'='*80}")
    tee.print("SUMMARY")
    tee.print('='*80)
    tee.print(f"Total impl blocks with generics: {total_impls}")
    tee.print(f"  With trait definitions: {total_with_traits}")
    tee.print(f"  Without trait definitions: {total_missing_traits}")
    tee.print()
    tee.print(f"Log written to: {log_path}")
    tee.close()

if __name__ == "__main__":
    main()

