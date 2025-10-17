#!/usr/bin/env python3
"""
Script to move #[allow(dead_code)] fn _*_type_checks() functions from src files to test files.
Usage: python3 move_macro_deadcode_tests.py <src_file>
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
import re
from pathlib import Path

def find_deadcode_functions(content):
    """Find all #[allow(dead_code)] fn _*_type_checks() functions."""
    functions = []
    lines = content.split('\n')
    
    i = 0
    while i < len(lines):
        line = lines[i]
        # Look for #[allow(dead_code)]
        if '#[allow(dead_code)]' in line:
            # Check if next line has a function that ends with _type_checks
            if i + 1 < len(lines) and 'fn _' in lines[i + 1] and '_type_checks()' in lines[i + 1]:
                # Found a dead code function, now find its complete body
                start_line = i
                brace_count = 0
                func_started = False
                end_line = i + 1
                
                # Find the opening brace and track nested braces
                for j in range(i + 1, len(lines)):
                    if '{' in lines[j]:
                        func_started = True
                        brace_count += lines[j].count('{')
                    if '}' in lines[j]:
                        brace_count -= lines[j].count('}')
                    
                    if func_started and brace_count == 0:
                        end_line = j
                        break
                
                # Extract the complete function
                func_lines = lines[start_line:end_line + 1]
                func_text = '\n'.join(func_lines)
                
                # Calculate character positions
                start_pos = sum(len(lines[k]) + 1 for k in range(start_line))
                end_pos = start_pos + len(func_text)
                
                functions.append({
                    'text': func_text,
                    'start': start_pos,
                    'end': end_pos,
                    'start_line': start_line,
                    'end_line': end_line
                })
                
                i = end_line + 1
            else:
                i += 1
        else:
            i += 1
    
    return functions

def convert_to_test_function(func_text):
    """Convert dead code function to test function."""
    # Replace #[allow(dead_code)] with #[test]
    test_func = func_text.replace('#[allow(dead_code)]', '#[test]')
    
    # Change function name from _*_type_checks to test_*_type_checks
    test_func = re.sub(r'fn (_[^(]+_type_checks)', r'fn test\1', test_func)
    
    return test_func

def determine_test_file(src_file_path):
    """Determine the appropriate test file for a src file."""
    src_path = Path(src_file_path)
    
    if src_path.name == "Types.rs":
        return "tests/TestTypes.rs"
    
    # For other files, create test file based on src structure
    # e.g., src/Chap18/ArraySeq.rs -> tests/Chap18/TestArraySeq.rs
    relative_path = src_path.relative_to("src")
    test_name = f"Test{relative_path.stem}.rs"
    test_dir = Path("tests") / relative_path.parent
    return test_dir / test_name

def move_deadcode_tests(src_file_path):
    """Move dead code tests from src file to test file."""
    src_path = Path(src_file_path)
    
    if not src_path.exists():
        print(f"Error: Source file {src_file_path} does not exist")
        return False
    
    # Read source file
    try:
        with open(src_path, 'r', encoding='utf-8') as f:
            src_content = f.read()
    except Exception as e:
        print(f"Error reading {src_file_path}: {e}")
        return False
    
    # Find dead code functions
    deadcode_functions = find_deadcode_functions(src_content)
    
    if not deadcode_functions:
        print(f"No dead code type check functions found in {src_file_path}")
        return True
    
    print(f"Found {len(deadcode_functions)} dead code functions in {src_file_path}")
    
    # Determine test file
    test_file_path = determine_test_file(src_file_path)
    test_path = Path(test_file_path)
    
    # Create test directory if it doesn't exist
    test_path.parent.mkdir(parents=True, exist_ok=True)
    
    # Convert functions to test functions
    test_functions = []
    for func in deadcode_functions:
        test_func = convert_to_test_function(func['text'])
        test_functions.append(test_func)
    
    # Remove functions from source file (in reverse order to maintain indices)
    new_src_content = src_content
    for func in reversed(deadcode_functions):
        new_src_content = new_src_content[:func['start']] + new_src_content[func['end']:]
    
    # Clean up any double newlines left behind
    new_src_content = re.sub(r'\n\n\n+', '\n\n', new_src_content)
    
    # Write updated source file
    try:
        with open(src_path, 'w', encoding='utf-8') as f:
            f.write(new_src_content)
        print(f"✅ Removed {len(deadcode_functions)} functions from {src_file_path}")
    except Exception as e:
        print(f"Error writing {src_file_path}: {e}")
        return False
    
    # Prepare test file content
    if test_path.exists():
        # Read existing test file
        try:
            with open(test_path, 'r', encoding='utf-8') as f:
                test_content = f.read()
        except Exception as e:
            print(f"Error reading existing test file {test_file_path}: {e}")
            return False
    else:
        # Create basic test file structure
        module_path = src_path.relative_to("src").with_suffix("").as_posix().replace("/", "::")
        test_content = f"""//! Tests for {module_path}

use apas_ai::{module_path}::*;
use apas_ai::Types::Types::*;

"""
    
    # Add test functions to test file
    for test_func in test_functions:
        test_content += test_func + "\n\n"
    
    # Write test file
    try:
        with open(test_path, 'w', encoding='utf-8') as f:
            f.write(test_content)
        print(f"✅ Added {len(test_functions)} test functions to {test_file_path}")
    except Exception as e:
        print(f"Error writing test file {test_file_path}: {e}")
        return False
    
    return True

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 move_macro_deadcode_tests.py <src_file>")
        print("Example: python3 move_macro_deadcode_tests.py src/Types.rs")
        sys.exit(1)
    
    src_file = sys.argv[1]
    
    print(f"Moving dead code macro tests from {src_file}...")
    print()
    
    if move_deadcode_tests(src_file):
        print("🎉 Successfully moved dead code tests!")
        sys.exit(0)
    else:
        print("❌ Failed to move dead code tests")
        sys.exit(1)

if __name__ == "__main__":
    main()
