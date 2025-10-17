#!/usr/bin/env python3
"""
Fix: Integration test structure.

RustRules.md Lines 292-298: Remove #[cfg(test)] wrappers from integration tests.

Transforms:
  #[cfg(test)]
  mod tests {
      use ...;
      #[test]
      fn test_foo() { ... }
  }

Into:
  use ...;
  #[test]
  fn test_foo() { ... }
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import argparse
import sys
from pathlib import Path


def fix_file(file_path, dry_run=False):
    """Remove #[cfg(test)] wrapper and un-indent contents."""
    with open(file_path, 'r', encoding='utf-8') as f:
        lines = f.readlines()
    
    # Find #[cfg(test)] line
    cfg_test_line = None
    mod_line = None
    
    for i, line in enumerate(lines):
        if line.strip() == '#[cfg(test)]':
            cfg_test_line = i
            # Next non-empty line should be mod declaration
            for j in range(i + 1, min(i + 5, len(lines))):
                if lines[j].strip().startswith('mod ') and '{' in lines[j]:
                    mod_line = j
                    break
            break
    
    if cfg_test_line is None:
        return False  # No #[cfg(test)] found
    
    if mod_line is None:
        print(f"Warning: Found #[cfg(test)] but no mod declaration in {file_path}")
        return False
    
    # Find closing brace (last non-empty line should be just "}")
    closing_brace_line = None
    for i in range(len(lines) - 1, -1, -1):
        stripped = lines[i].strip()
        if stripped == '}':
            closing_brace_line = i
            break
        elif stripped and not stripped.startswith('//'):
            # Found non-empty, non-comment line that's not }
            break
    
    if closing_brace_line is None:
        print(f"Warning: Could not find closing brace in {file_path}")
        return False
    
    # Determine indentation level (usually 4 spaces)
    indent = '    '  # Default to 4 spaces
    for i in range(mod_line + 1, min(mod_line + 10, len(lines))):
        if lines[i].startswith('    ') and lines[i].strip():
            # Found indented line, measure it
            indent = ''
            for char in lines[i]:
                if char == ' ':
                    indent += ' '
                else:
                    break
            break
    
    # Build new file contents
    new_lines = []
    
    # Copy lines before #[cfg(test)]
    new_lines.extend(lines[:cfg_test_line])
    
    # Skip #[cfg(test)] and mod lines
    # Un-indent and copy content between mod and closing brace
    for i in range(mod_line + 1, closing_brace_line):
        line = lines[i]
        # Remove one level of indentation if present
        if line.startswith(indent):
            new_lines.append(line[len(indent):])
        elif line.strip() == '':
            new_lines.append(line)  # Keep blank lines as-is
        else:
            new_lines.append(line)  # Line not indented (shouldn't happen)
    
    # Skip closing brace, copy any remaining lines (shouldn't be any)
    if closing_brace_line + 1 < len(lines):
        new_lines.extend(lines[closing_brace_line + 1:])
    
    if dry_run:
        print(f"Would fix: {file_path}")
        return True
    
    # Write back
    with open(file_path, 'w', encoding='utf-8') as f:
        f.writelines(new_lines)
    
    return True


def main():
    parser = argparse.ArgumentParser(description="Fix integration test structure.")
    parser.add_argument('--file', type=str, help="Fix single file")
    parser.add_argument('--dry-run', action='store_true', help="Show what would be fixed")
    args = parser.parse_args()
    
    repo_root = Path(__file__).parent.parent.parent.parent
    tests_dir = repo_root / "tests"
    
    if not tests_dir.exists():
        print("✓ No tests/ directory found")
        return 0
    
    if args.file:
        file_path = Path(args.file)
        if not file_path.is_absolute():
            file_path = repo_root / file_path
        
        if not file_path.exists():
            print(f"Error: File not found: {file_path}")
            return 1
        
        if fix_file(file_path, args.dry_run):
            print(f"✓ Fixed: {file_path.relative_to(repo_root)}")
            return 0
        else:
            print(f"✗ No changes needed: {file_path.relative_to(repo_root)}")
            return 1
    
    # Find all files with #[cfg(test)]
    files_to_fix = []
    for test_file in tests_dir.rglob("*.rs"):
        with open(test_file, 'r', encoding='utf-8') as f:
            if '#[cfg(test)]' in f.read():
                files_to_fix.append(test_file)
    
    if not files_to_fix:
        print("✓ No integration test files need fixing")
        return 0
    
    print(f"Found {len(files_to_fix)} files to fix\n")
    
    fixed_count = 0
    for test_file in sorted(files_to_fix):
        if fix_file(test_file, args.dry_run):
            rel_path = test_file.relative_to(repo_root)
            if args.dry_run:
                print(f"Would fix: {rel_path}")
            else:
                print(f"✓ Fixed: {rel_path}")
            fixed_count += 1
    
    if args.dry_run:
        print(f"\nWould fix {fixed_count} files")
    else:
        print(f"\n✓ Fixed {fixed_count} integration test files")
        print("\nRemoved #[cfg(test)] wrappers and un-indented contents.")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())

