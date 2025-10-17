#!/usr/bin/env python3
"""
Script to fix import formatting across all Rust source files.
Applies consistent formatting rules:
1. Blank line after pub mod declaration
2. Standard library imports grouped together
3. Blank line after standard library imports
4. Types imports first among crate imports
5. Other crate imports following
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import subprocess
import sys
from pathlib import Path

def fix_imports_in_file(file_path):
    """Fix import formatting in a single file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match pub mod declaration and imports
    pub_mod_pattern = r'^(pub mod \w+ \{)\s*\n'
    
    # Find pub mod declaration
    pub_mod_match = re.search(pub_mod_pattern, content, re.MULTILINE)
    if not pub_mod_match:
        return False, "No pub mod found"
    
    # Split content at pub mod line
    lines = content.split('\n')
    pub_mod_line_idx = None
    
    for i, line in enumerate(lines):
        if re.match(r'pub mod \w+ \{', line.strip()):
            pub_mod_line_idx = i
            break
    
    if pub_mod_line_idx is None:
        return False, "Could not find pub mod line"
    
    # Find the import section (starts after pub mod, ends at first non-import/non-blank line)
    import_start_idx = pub_mod_line_idx + 1
    import_end_idx = import_start_idx
    
    # Skip to first non-blank line after pub mod
    while import_start_idx < len(lines) and lines[import_start_idx].strip() == '':
        import_start_idx += 1
    
    # Find end of imports
    for i in range(import_start_idx, len(lines)):
        line = lines[i].strip()
        if line == '' or line.startswith('use ') or line.startswith('pub use ') or line.startswith('//'):
            import_end_idx = i + 1
        else:
            break
    
    # Extract imports
    import_lines = []
    for i in range(import_start_idx, import_end_idx):
        line = lines[i].strip()
        if line.startswith('use ') or line.startswith('pub use '):
            # Normalize pub use to use
            if line.startswith('pub use '):
                line = line.replace('pub use ', 'use ', 1)
            import_lines.append(line)
    
    if not import_lines:
        # No imports to fix, just ensure blank line after pub mod
        if import_start_idx < len(lines) and lines[import_start_idx].strip() != '':
            lines.insert(pub_mod_line_idx + 1, '')
            new_content = '\n'.join(lines)
            if new_content != original_content:
                with open(file_path, 'w') as f:
                    f.write(new_content)
                return True, "Added blank line after pub mod"
        return False, "No changes needed"
    
    # Categorize imports
    std_imports = []
    types_imports = []
    crate_imports = []
    
    for imp in import_lines:
        if imp.startswith('use std::'):
            std_imports.append(imp)
        elif 'crate::Types::Types::' in imp:
            types_imports.append(imp)
        elif imp.startswith('use crate::'):
            crate_imports.append(imp)
        else:
            # External crates or other imports
            crate_imports.append(imp)
    
    # Sort within each category
    std_imports.sort()
    types_imports.sort()
    crate_imports.sort()
    
    # Build new import section
    new_import_lines = []
    
    # Add blank line after pub mod
    new_import_lines.append('')
    
    # Add std imports
    if std_imports:
        new_import_lines.extend(std_imports)
        new_import_lines.append('')  # Blank line after std imports
    
    # Add Types imports first
    if types_imports:
        new_import_lines.extend(types_imports)
    
    # Add other crate imports
    if crate_imports:
        new_import_lines.extend(crate_imports)
    
    # Add blank line after all imports (only if there are imports)
    if import_lines:
        new_import_lines.append('')
    
    # Reconstruct file
    new_lines = (
        lines[:pub_mod_line_idx + 1] +  # Everything up to and including pub mod
        new_import_lines +              # New import section
        lines[import_end_idx:]          # Everything after imports
    )
    
    new_content = '\n'.join(new_lines)
    
    if new_content != original_content:
        with open(file_path, 'w') as f:
            f.write(new_content)
        return True, "Fixed imports"
    
    return False, "No changes needed"

def find_rust_files():
    """Find all Rust source files with pub mod declarations."""
    rust_files = []
    # Handle both running from project root and from scripts directory
    search_paths = []
    if os.path.exists('src'):
        search_paths.extend(['src', 'tests', 'benches'])
    else:
        search_paths.extend(['../src', '../tests', '../benches'])
    
    for search_path in search_paths:
        if not os.path.exists(search_path):
            continue
        for root, dirs, files in os.walk(search_path):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    try:
                        with open(file_path, 'r') as f:
                            content = f.read()
                            if re.search(r'^pub mod \w+ \{', content, re.MULTILINE):
                                rust_files.append(file_path)
                    except Exception as e:
                        print(f"Error reading {file_path}: {e}")
    return rust_files

def main():
    """Main function to process all files."""
    print("Finding Rust files with pub mod declarations...")
    rust_files = find_rust_files()
    print(f"Found {len(rust_files)} files to process")
    
    processed = 0
    errors = 0
    batch_size = 20
    
    for i, file_path in enumerate(rust_files):
        try:
            changed, message = fix_imports_in_file(file_path)
            if changed:
                print(f"✓ {file_path}: {message}")
                processed += 1
            else:
                print(f"- {file_path}: {message}")
            
            # Compile after every batch_size files
            if (i + 1) % batch_size == 0 or i == len(rust_files) - 1:
                if processed > 0:
                    print(f"\nCompiling after processing {min(i + 1, len(rust_files))} files...")
                    result = subprocess.run(['cargo', 'build', '--quiet'], 
                                          capture_output=True, text=True)
                    if result.returncode != 0:
                        print(f"✗ Compilation failed after batch ending at {file_path}")
                        print(result.stderr)
                        return 1
                    else:
                        print(f"✓ Compilation successful after batch {(i // batch_size) + 1}")
        except Exception as e:
            print(f"✗ Error processing {file_path}: {e}")
            errors += 1
    
    print(f"\nSummary:")
    print(f"Files processed: {processed}")
    print(f"Errors: {errors}")
    print(f"Total files checked: {len(rust_files)}")
    
    if processed > 0:
        print("\nRunning final compilation check...")
        result = subprocess.run(['cargo', 'build', '--quiet'])
        if result.returncode == 0:
            print("✓ Final compilation successful")
        else:
            print("✗ Final compilation failed")
            return 1
    
    return 0

if __name__ == '__main__':
    sys.exit(main())
