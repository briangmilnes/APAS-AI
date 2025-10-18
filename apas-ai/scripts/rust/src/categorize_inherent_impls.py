#!/usr/bin/env python3
"""
Categorize remaining inherent impls by fixability.

Categories:
1. Public methods, no existing trait -> Script can convert
2. Public methods, has existing trait -> Manual work (forwarding wrappers)
3. Private methods only -> No problem, no conversion needed
"""

import subprocess
import sys

def main():
    # Get list of files with inherent impls
    result = subprocess.run(['python3', 'scripts/rust/src/detect_inherent_needs_trait.py'], 
                           capture_output=True, text=True)

    files = []
    for line in result.stdout.split('\n'):
        if line.startswith('src/'):
            files.append(line.strip())

    print(f"Total files with inherent impls: {len(files)}\n")

    has_pub_fn = []
    has_trait = []
    private_only = []

    for file in files:
        # Check for pub fn
        pub_fn_result = subprocess.run(['grep', '-c', 'pub fn ', file], 
                                       capture_output=True, text=True)
        pub_fn_count = int(pub_fn_result.stdout.strip() or '0')
        
        # Check for existing trait
        has_trait_def = subprocess.run(['grep', '-q', 'pub trait.*Trait', file], 
                                       capture_output=True).returncode == 0
        
        if pub_fn_count > 0:
            if has_trait_def:
                has_trait.append((file, pub_fn_count))
            else:
                has_pub_fn.append((file, pub_fn_count))
        else:
            private_only.append(file)

    print(f"Category 1: Public methods, NO existing trait (script can convert):")
    print(f"  Count: {len(has_pub_fn)}")
    for f, count in has_pub_fn:
        print(f"    {f} ({count} pub fn)")

    print(f"\nCategory 2: Public methods, HAS existing trait (manual work needed):")
    print(f"  Count: {len(has_trait)}")
    for f, count in has_trait:
        print(f"    {f} ({count} pub fn)")

    print(f"\nCategory 3: Private methods only (no problem, no conversion needed):")
    print(f"  Count: {len(private_only)}")
    print(f"    (Not listing {len(private_only)} files - all BST Node/Inner types)")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())

