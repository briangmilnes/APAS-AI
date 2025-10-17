#!/usr/bin/env python3
"""
Fix structs to satisfy StT requirements by adding missing derives.

StT = Eq + Clone + Display + Debug + Sized

This script:
1. Adds Clone, Debug, Eq to #[derive(...)] if missing (auto-derivable)
2. Reports Display as needing manual implementation

Note: Display cannot be auto-derived and requires manual impl.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import sys
from pathlib import Path

# Add lib directory to path for imports
sys.path.insert(0, str(Path(__file__).parent.parent.parent / 'lib'))
from review_utils import ReviewContext, create_review_parser


def extract_derives_line(lines, struct_line_idx):
    """Find the #[derive(...)] line before a struct."""
    for i in range(struct_line_idx - 1, max(0, struct_line_idx - 5), -1):
        line = lines[i]
        if '#[derive(' in line:
            return i, line
        elif not line.strip().startswith('#'):
            break
    return None, None


def parse_derives(derive_line):
    """Parse derives from #[derive(...)] line."""
    match = re.search(r'#\[derive\((.*?)\)\]', derive_line)
    if match:
        traits_str = match.group(1)
        return [t.strip() for t in traits_str.split(',')]
    return []


def has_manual_impl(lines, struct_name, trait_name):
    """Check if struct has a manual impl for trait."""
    pattern = rf'impl.*{trait_name}\s+for\s+{struct_name}'
    for line in lines:
        if re.search(pattern, line):
            return True
    return False


def fix_file(filepath, context):
    """Add missing StT derives to structs."""
    try:
        with open(filepath, 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return 0
    
    original_lines = lines[:]
    fixes_made = 0
    
    # Find all struct definitions (pub and private)
    i = 0
    while i < len(lines):
        line = lines[i]
        match = re.match(r'\s*(?:pub\s+)?struct\s+(\w+)', line)
        
        if match:
            struct_name = match.group(1)
            
            # Find derive line
            derive_idx, derive_line = extract_derives_line(lines, i)
            
            if derive_line:
                # Parse existing derives
                existing = parse_derives(derive_line)
                
                # Check what's missing (only auto-derivable ones)
                missing = []
                if 'Clone' not in existing and not has_manual_impl(lines, struct_name, 'Clone'):
                    missing.append('Clone')
                if 'Debug' not in existing and not has_manual_impl(lines, struct_name, 'Debug'):
                    missing.append('Debug')
                if 'Eq' not in existing and not has_manual_impl(lines, struct_name, 'Eq'):
                    missing.append('Eq')
                # Note: We check for PartialEq which is needed for Eq
                if 'Eq' in missing and 'PartialEq' not in existing:
                    missing.insert(missing.index('Eq'), 'PartialEq')
                
                if missing:
                    # Add missing traits to derive
                    new_derives = existing + missing
                    indent = derive_line[:len(derive_line) - len(derive_line.lstrip())]
                    new_derive_line = f"{indent}#[derive({', '.join(new_derives)})]\n"
                    lines[derive_idx] = new_derive_line
                    
                    fixes_made += 1
                    print(f"  {struct_name}: Added {', '.join(missing)}")
            else:
                # No derive line exists - need to add one
                # Check what's missing
                missing = []
                if not has_manual_impl(lines, struct_name, 'Clone'):
                    missing.append('Clone')
                if not has_manual_impl(lines, struct_name, 'Debug'):
                    missing.append('Debug')
                if not has_manual_impl(lines, struct_name, 'Eq'):
                    missing.extend(['PartialEq', 'Eq'])
                
                if missing:
                    # Insert new derive line before struct
                    indent = line[:len(line) - len(line.lstrip())]
                    new_derive_line = f"{indent}#[derive({', '.join(missing)})]\n"
                    lines.insert(i, new_derive_line)
                    
                    fixes_made += 1
                    print(f"  {struct_name}: Added new #[derive({', '.join(missing)})]")
                    i += 1  # Skip the line we just inserted
        
        i += 1
    
    # Write back if changes were made
    if fixes_made > 0:
        try:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.writelines(lines)
            return fixes_made
        except Exception as e:
            print(f"Error writing {filepath}: {e}")
            return 0
    
    return 0


def main():
    parser = create_review_parser(
        description="Fix structs to satisfy StT by adding missing derives"
    )
    parser.add_argument(
        'files',
        nargs='*',
        help='Specific files to fix'
    )
    args = parser.parse_args()
    context = ReviewContext(args)

    if args.files:
        files_to_fix = [Path(f) if Path(f).is_absolute() else context.repo_root / f for f in args.files]
    else:
        print("Usage: fix_stt_compliance.py file1.rs file2.rs ...")
        return 1

    total_fixes = 0
    for filepath in files_to_fix:
        if not filepath.exists():
            print(f"✗ File not found: {filepath}")
            continue
        
        print(f"\nFixing {context.relative_path(filepath)}...")
        fixes = fix_file(filepath, context)
        total_fixes += fixes

    print(f"\n{'='*70}")
    print(f"Total: Fixed {total_fixes} struct(s)")
    print(f"\nNote: Display cannot be auto-derived and needs manual implementation.")
    return 0


if __name__ == '__main__':
    sys.exit(main())

