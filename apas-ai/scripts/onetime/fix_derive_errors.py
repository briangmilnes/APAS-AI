#!/usr/bin/env python3
"""
Fix derive errors by removing derives that cause compilation errors.

Reads cargo output and removes problematic derives like:
- Eq from structs with f64 fields
- Clone/Eq from structs with Atomic* fields
- Clone/Eq from structs with Mutex/RwLock fields
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import re
import subprocess
import sys
from pathlib import Path
from collections import defaultdict

def get_cargo_errors():
    """Run cargo check and parse derive-related errors."""
    result = subprocess.run(
        ['cargo', 'check', '--lib'],
        capture_output=True,
        text=True,
        cwd='/home/milnes/APASVERUS/APAS-AI/apas-ai'
    )
    
    errors = []
    lines = result.stderr.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        
        # Look for trait bound errors
        if re.search(r'error\[E0277\]|error\[E0369\]', line):
            # Extract file and line number
            file_match = None
            trait_issue = None
            
            # Look ahead for file location and trait info
            for j in range(i, min(i + 20, len(lines))):
                if '-->' in lines[j] and '/src/' in lines[j]:
                    file_match = re.search(r'(src/[^:]+):(\d+):', lines[j])
                
                if 'in this derive macro expansion' in lines[j]:
                    # Found a derive-related error
                    if file_match:
                        # Determine which trait is problematic
                        if 'Clone' in line or 'Clone' in lines[j-1]:
                            trait_issue = 'Clone'
                        elif 'Eq' in line:
                            trait_issue = 'Eq'
                        elif 'PartialEq' in line:
                            trait_issue = 'PartialEq'
                        
                        if trait_issue:
                            errors.append({
                                'file': file_match.group(1),
                                'line': int(file_match.group(2)),
                                'trait': trait_issue
                            })
                    break
        
        i += 1
    
    return errors

def fix_derive_line(line, traits_to_remove):
    """Remove specific traits from a #[derive(...)] line."""
    match = re.match(r'(\s*#\[derive\()([^)]+)(\)\])', line)
    if not match:
        return line
    
    prefix, derives_str, suffix = match.groups()
    derives = [d.strip() for d in derives_str.split(',')]
    
    # Remove problematic traits
    derives = [d for d in derives if d not in traits_to_remove]
    
    if not derives:
        # No derives left, remove the whole line
        return None
    
    return f"{prefix}{', '.join(derives)}{suffix}\n"

def fix_file(filepath, line_num, trait):
    """Fix a specific derive error in a file."""
    try:
        with open(filepath, 'r') as f:
            lines = f.readlines()
    except Exception as e:
        print(f"Error reading {filepath}: {e}")
        return False
    
    # Look backwards from the error line to find the #[derive(...)] line
    for i in range(line_num - 1, max(0, line_num - 10), -1):
        if '#[derive(' in lines[i]:
            # Determine which traits to remove
            traits_to_remove = {trait}
            
            # If removing Eq, also remove PartialEq
            if trait == 'Eq':
                traits_to_remove.add('PartialEq')
                traits_to_remove.add('Eq')
            
            # If removing PartialEq, also remove Eq
            if trait == 'PartialEq':
                traits_to_remove.add('PartialEq')
                traits_to_remove.add('Eq')
            
            original = lines[i]
            fixed = fix_derive_line(lines[i], traits_to_remove)
            
            if fixed is None:
                # Remove the entire line
                del lines[i]
                print(f"  Removed entire #[derive] from line {i+1}")
            elif fixed != original:
                lines[i] = fixed
                print(f"  Removed {', '.join(traits_to_remove)} from line {i+1}")
            else:
                print(f"  No change needed at line {i+1}")
            
            # Write back
            try:
                with open(filepath, 'w') as f:
                    f.writelines(lines)
                return True
            except Exception as e:
                print(f"Error writing {filepath}: {e}")
                return False
    
    print(f"  Could not find #[derive] line before line {line_num}")
    return False

def main():
    print("Analyzing cargo errors...")
    errors = get_cargo_errors()
    
    if not errors:
        print("No derive-related errors found!")
        return 0
    
    # Group by file
    by_file = defaultdict(list)
    for error in errors:
        by_file[error['file']].append(error)
    
    print(f"Found {len(errors)} derive errors in {len(by_file)} files\n")
    
    fixed_count = 0
    for filepath, file_errors in sorted(by_file.items()):
        # Skip Chap47 and Exercise files
        if 'Chap47' in filepath or 'Exercise' in filepath:
            print(f"Skipping {filepath} (AI slop or exercise)")
            continue
        
        print(f"Fixing {filepath}...")
        for error in file_errors:
            if fix_file(filepath, error['line'], error['trait']):
                fixed_count += 1
    
    print(f"\n{'='*70}")
    print(f"Fixed {fixed_count} derive errors")
    return 0

if __name__ == '__main__':
    sys.exit(main())


