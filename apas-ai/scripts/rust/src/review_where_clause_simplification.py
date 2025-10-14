#!/usr/bin/env python3
"""
Review: Where clause simplification.

RustRules.md Lines 322-329: "Replace fn method<F>(...) where F: Fn(...); with
fn method<F: Fn(...)>(...); for simple bounds. Minimize where clauses across
codebase by inlining bounds."

Checks src/ for overly simple where clauses that could be inlined.
"""

import re
import sys
from pathlib import Path


# Pattern to detect simple where clause (single bound on single type)
SIMPLE_WHERE_PATTERN = re.compile(
    r'fn\s+\w+<(\w+)>\s*\([^)]*\)\s*(?:->\s*[^{]+)?\s*where\s+\1:\s*(\w+(?:::\w+)*)'
)


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    src_dir = repo_root / "src"
    
    if not src_dir.exists():
        print("✓ No src/ directory found")
        return 0
    
    violations = []
    
    for src_file in src_dir.rglob("*.rs"):
        with open(src_file, 'r', encoding='utf-8') as f:
            content = f.read()
            lines = content.split('\n')
            
            for line_num, line in enumerate(lines, start=1):
                # Skip comments
                stripped = line.strip()
                if stripped.startswith('//'):
                    continue
                
                # Check for simple where clause pattern
                # This is a basic check - full analysis would need AST parsing
                if ' where ' in line and 'fn ' in line:
                    # Look for pattern: fn name<T>(...) where T: SomeTrait
                    match = SIMPLE_WHERE_PATTERN.search(line)
                    if match:
                        type_param = match.group(1)
                        bound = match.group(2)
                        violations.append((src_file, line_num, line.strip(), type_param, bound))
    
    if violations:
        print("✗ Found simplifiable where clauses (RustRules.md Lines 322-329):\n")
        for file_path, line_num, line_content, type_param, bound in violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:{line_num}")
            print(f"    {line_content[:100]}")
            print(f"    Suggestion: fn name<{type_param}: {bound}>(...)  (inline the bound)")
            print()
        print(f"Total potential simplifications: {len(violations)}")
        print("\nNote: This is a basic check. Review each case manually.")
        return 1
    else:
        print("✓ No obvious where clause simplifications found")
        return 0


if __name__ == "__main__":
    sys.exit(main())

