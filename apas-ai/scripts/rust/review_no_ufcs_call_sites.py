#!/usr/bin/env python3
"""
Review: No UFCS at call sites.

RustRules.md Lines 309-320: "Replace <Type as Trait>::method(...) at call sites
with method-call syntax wherever possible. Keep UFCS inside impls/traits for
disambiguation; minimize UFCS in callers."

Checks src/, tests/, and benches/ for UFCS usage outside of impl/trait blocks.
"""

import re
import sys
from pathlib import Path


# Pattern to match UFCS: <Type as Trait>::method
UFCS_PATTERN = re.compile(r'<[^>]+\s+as\s+[^>]+>::\w+')


def main():
    repo_root = Path(__file__).parent.parent.parent
    
    search_dirs = [
        repo_root / "src",
        repo_root / "tests",
        repo_root / "benches",
    ]
    
    violations = []
    
    for search_dir in search_dirs:
        if not search_dir.exists():
            continue
        
        for rust_file in search_dir.rglob("*.rs"):
            with open(rust_file, 'r', encoding='utf-8') as f:
                in_impl = False
                in_trait = False
                brace_depth = 0
                
                for line_num, line in enumerate(f, start=1):
                    stripped = line.strip()
                    
                    # Skip comments
                    if stripped.startswith('//'):
                        continue
                    
                    # Track impl/trait blocks
                    if stripped.startswith('impl ') or ' impl ' in stripped:
                        in_impl = True
                        brace_depth = 0
                    elif stripped.startswith('trait ') or ' trait ' in stripped:
                        in_trait = True
                        brace_depth = 0
                    
                    # Track braces
                    brace_depth += line.count('{') - line.count('}')
                    
                    # Exit impl/trait when braces close
                    if (in_impl or in_trait) and brace_depth <= 0:
                        in_impl = False
                        in_trait = False
                    
                    # Check for UFCS outside impl/trait
                    if not in_impl and not in_trait:
                        ufcs_matches = UFCS_PATTERN.findall(line)
                        if ufcs_matches:
                            for match in ufcs_matches:
                                violations.append((rust_file, line_num, line.strip(), match))
    
    if violations:
        print("✗ Found UFCS at call sites (RustRules.md Lines 309-320):\n")
        for file_path, line_num, line_content, ufcs_expr in violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:{line_num}")
            print(f"    {line_content}")
            print(f"    UFCS: {ufcs_expr}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Use method-call syntax: value.method(...) instead of <Type as Trait>::method(value, ...)")
        return 1
    else:
        print("✓ No UFCS usage at call sites")
        return 0


if __name__ == "__main__":
    sys.exit(main())

