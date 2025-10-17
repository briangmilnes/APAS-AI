#!/usr/bin/env python3
"""
Review script: Default Trait Implementations Pattern

Checks that trait default implementations follow the pattern:
- One-line defaults (≤120 chars): body in trait
- Multi-line defaults: signature only in trait, body in impl

RustRules.md: "Default Trait Implementations (Pattern)"
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path
sys.path.insert(0, str(Path(__file__).parent.parent.parent / "lib"))
from review_utils import run_review, get_repo_root


def check_file(file_path: Path, context) -> list[str]:
    """Check a single Rust file for trait default implementation pattern."""
    try:
        content = file_path.read_text(encoding='utf-8')
    except Exception as e:
        return [f"ERROR: Could not read {file_path}: {e}"]
    
    errors = []
    lines = content.splitlines()
    
    in_trait = False
    trait_name = None
    trait_start = 0
    brace_depth = 0
    
    for i, line in enumerate(lines, 1):
        # Track trait blocks
        if re.match(r'\s*pub\s+trait\s+\w+', line):
            in_trait = True
            trait_name = re.search(r'trait\s+(\w+)', line).group(1)
            trait_start = i
            brace_depth = 0
        
        if in_trait:
            brace_depth += line.count('{') - line.count('}')
            if brace_depth == 0 and '{' in line:
                in_trait = False
                continue
            
            # Look for default implementations in trait that span multiple lines
            fn_match = re.match(r'\s*fn\s+(\w+)\s*[<(].*\{', line)
            if fn_match:
                method_name = fn_match.group(1)
                
                # Check if this is a multi-line default (body extends past this line)
                # Simple heuristic: if the line ends with '{' and doesn't have '}', it's multi-line
                if line.rstrip().endswith('{') and '}' not in line:
                    # Find where this method ends
                    method_brace_depth = 1
                    j = i
                    while j < len(lines) and method_brace_depth > 0:
                        j += 1
                        if j <= len(lines):
                            method_brace_depth += lines[j-1].count('{') - lines[j-1].count('}')
                    
                    method_end = j
                    line_count = method_end - i + 1
                    
                    if line_count > 1:
                        # This is a multi-line default in trait - potential violation
                        rel_path = context.relative_path(file_path)
                        errors.append(
                            f"{rel_path}:{i}: Multi-line default implementation for '{method_name}' "
                            f"in trait '{trait_name}' ({line_count} lines). Consider moving body to impl block "
                            f"and leaving only signature in trait."
                        )
    
    return errors


def main():
    repo_root = get_repo_root()
    return run_review(
        description="Check trait default implementation pattern (one-line in trait, multi-line in impl)",
        rule_name="Default Trait Implementations Pattern",
        rule_reference="RustRules.md: Default Trait Implementations (Pattern)",
        directories=[repo_root / "src"],
        check_function=check_file,
        fix_suggestion="Move multi-line default implementations to impl block, leave only signature in trait."
    )


if __name__ == "__main__":
    sys.exit(main())

