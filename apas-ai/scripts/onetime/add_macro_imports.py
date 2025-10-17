#!/usr/bin/env python3
"""
Script to add macro imports to test files for modules that have macros.
Usage: python3 add_macro_imports.py <src_file>
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
import re
from pathlib import Path

def find_macros_in_file(src_file_path):
    """Find all macro_rules! definitions in a source file."""
    try:
        with open(src_file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {src_file_path}: {e}")
        return []
    
    # Find all macro_rules! definitions
    macro_pattern = r'macro_rules!\s+(\w+)'
    macros = re.findall(macro_pattern, content)
    
    return macros

def determine_test_file(src_file_path):
    """Determine the test file path for a source file."""
    src_path = Path(src_file_path)
    
    if src_path.name == "Types.rs":
        return Path("tests/TestTypes.rs")
    
    # For ChapXX/Module.rs -> tests/ChapXX/TestModule.rs
    if src_path.parent.name.startswith("Chap"):
        chapter = src_path.parent.name
        module = src_path.stem
        test_file = f"Test{module}.rs"
        return Path("tests") / chapter / test_file
    
    return None

def add_macro_imports_to_test_file(test_file_path, macros):
    """Add macro imports to a test file."""
    if not test_file_path.exists():
        print(f"Test file {test_file_path} does not exist")
        return False
    
    try:
        with open(test_file_path, 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {test_file_path}: {e}")
        return False
    
    # Check if imports already exist
    existing_imports = []
    missing_imports = []
    
    for macro in macros:
        if f"use apas_ai::{macro};" in content:
            existing_imports.append(macro)
        else:
            missing_imports.append(macro)
    
    if not missing_imports:
        print(f"All macro imports already exist in {test_file_path}")
        return True
    
    # Find the last use statement to add after it (handle indented use statements too)
    use_pattern = r'^\s*use [^;]+;'
    use_matches = list(re.finditer(use_pattern, content, re.MULTILINE))
    
    if not use_matches:
        print(f"No existing use statements found in {test_file_path}")
        return False
    
    # Insert after the last use statement
    last_use = use_matches[-1]
    insert_pos = last_use.end()
    
    # Create import statements (match indentation of existing imports)
    import_lines = []
    # Get indentation from the last use statement
    last_use_line = content[content.rfind('\n', 0, last_use.start()) + 1:last_use.end()]
    indent = re.match(r'^(\s*)', last_use_line).group(1)
    
    for macro in missing_imports:
        import_lines.append(f"{indent}use apas_ai::{macro};")
    
    # Insert the imports
    new_content = (
        content[:insert_pos] + 
        '\n' + 
        '\n'.join(import_lines) + 
        content[insert_pos:]
    )
    
    # Write back to file
    try:
        with open(test_file_path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        print(f"✅ Added {len(missing_imports)} macro imports to {test_file_path}")
        for macro in missing_imports:
            print(f"   • use apas_ai::{macro};")
        return True
    except Exception as e:
        print(f"Error writing {test_file_path}: {e}")
        return False

def process_src_file(src_file_path):
    """Process a source file and add macro imports to its test file."""
    src_path = Path(src_file_path)
    
    if not src_path.exists():
        print(f"Error: Source file {src_file_path} does not exist")
        return False
    
    # Find macros in source file
    macros = find_macros_in_file(src_file_path)
    
    if not macros:
        print(f"No macros found in {src_file_path}")
        return True
    
    print(f"Found {len(macros)} macros in {src_file_path}:")
    for macro in macros:
        print(f"   • {macro}")
    
    # Determine test file
    test_file_path = determine_test_file(src_file_path)
    
    if not test_file_path:
        print(f"Could not determine test file for {src_file_path}")
        return False
    
    print(f"Target test file: {test_file_path}")
    
    # Add imports to test file
    return add_macro_imports_to_test_file(test_file_path, macros)

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 add_macro_imports.py <src_file>")
        print("Example: python3 add_macro_imports.py src/Chap41/AVLTreeSetMtPer.rs")
        sys.exit(1)
    
    src_file = sys.argv[1]
    
    print(f"Processing {src_file}...")
    print()
    
    if process_src_file(src_file):
        print("🎉 Successfully processed file!")
        sys.exit(0)
    else:
        print("❌ Failed to process file")
        sys.exit(1)

if __name__ == "__main__":
    main()
