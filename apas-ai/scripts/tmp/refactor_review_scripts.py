#!/usr/bin/env python3
"""
One-time script to refactor review scripts to use review_utils.

Adds standardized --file and --dry-run support to all review scripts.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
from pathlib import Path


def add_imports_and_context(content: str) -> str:
    """Add review_utils imports and convert to use ReviewContext."""
    
    # Already refactored?
    if 'from review_utils import' in content:
        return None
    
    # Find import section
    lines = content.split('\n')
    new_lines = []
    
    i = 0
    # Copy docstring and shebang
    while i < len(lines) and (lines[i].startswith('#') or lines[i].startswith('"""') or '"""' in lines[i] or not lines[i].strip()):
        new_lines.append(lines[i])
        if '"""' in lines[i] and i > 0:  # End of docstring
            i += 1
            break
        i += 1
    
    # Add blank line
    if new_lines and new_lines[-1].strip():
        new_lines.append('')
    
    # Add imports
    new_lines.append('import sys')
    new_lines.append('from pathlib import Path')
    new_lines.append('')
    new_lines.append('sys.path.insert(0, str(Path(__file__).parent.parent / "lib"))')
    new_lines.append('from review_utils import ReviewContext, create_review_parser')
    
    # Skip old imports
    while i < len(lines) and (lines[i].strip().startswith('import ') or lines[i].strip().startswith('from ') or not lines[i].strip()):
        i += 1
    
    # Add rest of file
    new_lines.extend(lines[i:])
    
    return '\n'.join(new_lines)


def main():
    repo_root = Path(__file__).parent.parent.parent
    scripts_dir = repo_root / "scripts"
    
    # Find all review_*.py scripts
    review_scripts = list(scripts_dir.rglob("review_*.py"))
    
    # Exclude orchestrator scripts (they call other scripts, different pattern)
    orchestrators = [
        'review_rust.py',
        'review_rust_src.py',
        'review_rust_tests.py',
        'review_rust_benches.py',
        'review_APAS.py',
        'review_APAS_src.py',
        'review_APAS_tests.py',
        'review_APAS_benches.py',
    ]
    
    scripts_to_refactor = [s for s in review_scripts if s.name not in orchestrators]
    
    print(f"Found {len(scripts_to_refactor)} review scripts to refactor")
    print(f"Skipping {len(orchestrators)} orchestrator scripts\n")
    
    refactored_count = 0
    skipped_count = 0
    
    for script_path in scripts_to_refactor:
        with open(script_path, 'r') as f:
            content = f.read()
        
        new_content = add_imports_and_context(content)
        
        if new_content is None:
            print(f"⏭️  {script_path.relative_to(repo_root)} (already refactored)")
            skipped_count += 1
            continue
        
        # This is just adding imports - the main() function refactoring needs manual work
        # So let's just mark which ones need work
        print(f"🔧 {script_path.relative_to(repo_root)} (needs manual refactoring)")
        refactored_count += 1
    
    print(f"\n✅ {skipped_count} already refactored")
    print(f"🔧 {refactored_count} need manual refactoring")


if __name__ == "__main__":
    main()

