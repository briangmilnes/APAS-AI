#!/usr/bin/env python3
"""
Review: Integration test structure.

RustRules.md Lines 292-298: "Integration tests must have test functions at the 
root level of the file. NEVER use #[cfg(test)] modules in integration test files
- this prevents test discovery."

Checks all files in tests/ directory for #[cfg(test)] module usage.
"""

import sys
from pathlib import Path


def main():
    repo_root = Path(__file__).parent.parent.parent.parent
    tests_dir = repo_root / "tests"
    
    if not tests_dir.exists():
        print("✓ No tests/ directory found")
        return 0
    
    violations = []
    
    for test_file in tests_dir.rglob("*.rs"):
        with open(test_file, 'r', encoding='utf-8') as f:
            in_multiline_comment = False
            for line_num, line in enumerate(f, start=1):
                # Handle multi-line comments
                if '/*' in line:
                    in_multiline_comment = True
                if '*/' in line:
                    in_multiline_comment = False
                    continue
                if in_multiline_comment:
                    continue
                
                # Skip single-line comments
                stripped = line.strip()
                if stripped.startswith('//'):
                    continue
                
                # Check for #[cfg(test)]
                if '#[cfg(test)]' in line:
                    violations.append((test_file, line_num, line.strip()))
    
    if violations:
        print("✗ Found #[cfg(test)] in integration tests (RustRules.md Lines 292-298):\n")
        for file_path, line_num, line_content in violations:
            rel_path = file_path.relative_to(repo_root)
            print(f"  {rel_path}:{line_num}")
            print(f"    {line_content}")
            print()
        print(f"Total violations: {len(violations)}")
        print("\nFix: Remove #[cfg(test)] modules from integration tests.")
        print("Integration tests should have #[test] functions at root level.")
        return 1
    else:
        print("✓ No #[cfg(test)] modules in integration tests")
        return 0


if __name__ == "__main__":
    sys.exit(main())

