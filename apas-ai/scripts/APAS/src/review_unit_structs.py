#!/usr/bin/env python3
"""
Review: Unit structs with algorithms should be free functions.

APASRules.md Lines 183-188: "Unit structs with algorithmic impl blocks
should be converted to free functions with documentary traits."
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    src_dir = repo_root / "src"
    
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    candidates = []
    
    unit_struct_pattern = re.compile(r'pub struct (\w+);')
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            content = f.read()
            lines = content.split('\n')
        
        # Find unit structs
        unit_structs = unit_struct_pattern.findall(content)
        
        for struct_name in unit_structs:
            # Check if there's an impl block for this struct
            impl_pattern = re.compile(rf'impl\s+{struct_name}\s*\{{')
            if impl_pattern.search(content):
                # Check if it's algorithmic (has methods but no state)
                # Unit structs by definition have no fields, so any impl is algorithmic
                rel_path = src_file.relative_to(repo_root)
                
                # Find the impl block and count methods
                impl_match = impl_pattern.search(content)
                if impl_match:
                    # Count pub fn in this impl
                    after_impl = content[impl_match.start():]
                    # Simple heuristic: count pub fn until we hit a closing brace at column 0
                    impl_end = after_impl.find('\n}')
                    if impl_end > 0:
                        impl_body = after_impl[:impl_end]
                        method_count = len(re.findall(r'pub fn \w+', impl_body))
                        if method_count > 0:
                            candidates.append((rel_path, struct_name, method_count))
    
    if candidates:
        print("⚠ Unit structs with algorithmic impl blocks (APASRules.md Lines 183-188):\n")
        print("Note: Consider converting to free functions with documentary traits.\n")
        for file_path, struct_name, method_count in candidates:
            print(f"  {file_path}")
            print(f"    struct {struct_name}; - {method_count} method(s)")
            print()
        print(f"Total candidates: {len(candidates)}")
        print(f"Total violations: {len(candidates)}")
        print("\nConsider: Convert to module with documentary trait + free functions.")
        print("Keep unit structs for: type markers, strategies, data containers.")
        return 1
    
    print("✓ No algorithmic unit structs found")
    return 0


if __name__ == "__main__":
    sys.exit(main())

