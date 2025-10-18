#!/usr/bin/env python3
"""Find all inherent impl blocks in the codebase (excluding Types.rs)
Git commit: 287bbc0a1f4e7c8b6e9d2f3a4c5b6d7e8f9a0b1c
Date: 2025-10-18

Inherent impls are: impl StructName { ... }
NOT trait impls: impl Trait for StructName { ... }
"""

import argparse
import re
import os
import sys
from pathlib import Path


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


def find_inherent_impls(src_dir="src"):
    """Find all inherent impl blocks"""
    
    with_generics = []
    without_generics = []
    
    for root, dirs, files in os.walk(src_dir):
        for file in files:
            if not file.endswith(".rs") or file == "Types.rs":
                continue
            
            filepath = os.path.join(root, file)
            try:
                with open(filepath, 'r', encoding='utf-8') as f:
                    for i, line in enumerate(f, 1):
                        # Match: impl<...> TypeName { or impl TypeName {
                        # But NOT: impl ... for ...
                        if re.match(r'^\s+impl', line) and ' for ' not in line:
                            stripped = line.strip()
                            # Check if it has generics
                            if '<' in stripped and '>' in stripped:
                                with_generics.append(f"{filepath}:{i}:{line.rstrip()}")
                            else:
                                without_generics.append(f"{filepath}:{i}:{line.rstrip()}")
            except Exception as e:
                print(f"Error reading {filepath}: {e}", file=sys.stderr)
    
    return with_generics, without_generics


def main():
    parser = argparse.ArgumentParser(description='Find all inherent impl blocks (excluding Types.rs)')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/find_inherent_impls.txt',
                       help='Path to log file (default: analyses/code_review/find_inherent_impls.txt)')
    args = parser.parse_args()
    
    project_root = Path(__file__).parent.parent.parent.parent
    log_path = project_root / args.log_file
    
    tee = TeeOutput(log_path)
    
    tee.print("INHERENT IMPL BLOCKS (excluding Types.rs)")
    tee.print("=" * 60)
    tee.print()
    
    src_dir = project_root / "src"
    with_generics, without_generics = find_inherent_impls(str(src_dir))
    
    tee.print("WITH GENERICS (custom trait bounds):")
    tee.print("-" * 40)
    for item in sorted(with_generics):
        # Make paths relative to project root
        rel_item = item.replace(str(project_root) + '/', '')
        tee.print(rel_item)
    
    tee.print()
    tee.print("WITHOUT GENERICS (likely utility types):")
    tee.print("-" * 40)
    for item in sorted(without_generics):
        # Make paths relative to project root
        rel_item = item.replace(str(project_root) + '/', '')
        tee.print(rel_item)
    
    tee.print()
    tee.print(f"TOTAL: {len(with_generics) + len(without_generics)} inherent impl blocks")
    
    # Also output just file list
    tee.print()
    tee.print("=" * 60)
    tee.print("FILES WITH INHERENT IMPLS:")
    tee.print("-" * 40)
    all_files = set()
    for item in with_generics + without_generics:
        filepath = item.split(':')[0]
        all_files.add(filepath)
    
    for filepath in sorted(all_files):
        # Make paths relative to project root
        rel_filepath = filepath.replace(str(project_root) + '/', '')
        tee.print(rel_filepath)
    
    tee.print()
    tee.print(f"TOTAL FILES: {len(all_files)}")
    tee.print()
    tee.print(f"Log written to: {log_path}")
    tee.close()

if __name__ == "__main__":
    main()
