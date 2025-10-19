#!/usr/bin/env python3
"""
Detect inherent impl blocks that need to be converted to trait impls.

These are inherent impls that:
1. Have generic bounds
2. Do NOT have a corresponding trait definition
3. Violate the Single Implementation Pattern

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

def find_trait_name(filepath, impl_line):
    """Try to find the trait name by looking for 'impl TraitName for'."""
    # Extract struct name from impl line
    # impl<...> StructName { or impl StructName {
    match = re.search(r'impl(?:<[^>]+>)?\s+(\w+)', impl_line)
    if not match:
        return None
    
    struct_name = match.group(1)
    trait_name = f"{struct_name}Trait"
    
    # Check if this trait exists in the file
    content = filepath.read_text()
    if f'trait {trait_name}' in content or f'pub trait {trait_name}' in content:
        return trait_name
    
    return None

def main():
    parser = argparse.ArgumentParser(description='Detect inherent impls needing trait conversion')
    parser.add_argument('--log_file', 
                       default='analyses/code_review/detect_inherent_needs_trait.txt',
                       help='Output log file path')
    args = parser.parse_args()
    
    project_root = Path.cwd()
    src_dir = project_root / "src"
    log_path = project_root / args.log_file
    log_path.parent.mkdir(parents=True, exist_ok=True)
    
    tee = TeeOutput(log_path)
    
    tee.print("INHERENT IMPLS NEEDING TRAIT CONVERSION")
    tee.print("="*80)
    tee.print()
    
    # Pattern to match impl blocks with generics
    impl_pattern = re.compile(r'^\s*impl<[^>]+>\s+\w+')
    
    results = []
    
    for rs_file in sorted(src_dir.rglob("*.rs")):
        if 'Types.rs' in str(rs_file):
            continue
        
        rel_path = rs_file.relative_to(project_root)
        lines = rs_file.read_text().split('\n')
        
        for i, line in enumerate(lines, 1):
            if impl_pattern.match(line) and '{' in line:
                # Check if it's a trait impl (has 'for' keyword)
                if ' for ' in line:
                    continue
                
                # Check if a trait exists for this impl
                trait_name = find_trait_name(rs_file, line)
                if trait_name:
                    continue
                
                # This is an inherent impl without a trait
                results.append({
                    'file': str(rs_file.absolute()),
                    'line': i,
                    'impl': line.strip()
                })
    
    # Group by file
    by_file = {}
    for r in results:
        if r['file'] not in by_file:
            by_file[r['file']] = []
        by_file[r['file']].append(r)
    
    # Print results
    for filepath in sorted(by_file.keys()):
        tee.print(f"{filepath}")
        for r in by_file[filepath]:
            tee.print(f"  Line {r['line']}: {r['impl']}")
        tee.print()
    
    tee.print("="*80)
    tee.print(f"Total files with inherent impls needing traits: {len(by_file)}")
    tee.print(f"Total inherent impl blocks needing conversion: {len(results)}")
    tee.print()
    tee.print(f"Log written to: {log_path}")
    tee.close()

if __name__ == "__main__":
    main()

