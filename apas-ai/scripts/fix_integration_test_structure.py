#!/usr/bin/env python3
"""
Script to fix integration test structure by removing #[cfg(test)] modules
and moving imports and test functions to file root level.
"""

import os
import re
import sys
from pathlib import Path

def fix_test_file(file_path):
    """Fix a single test file by restructuring it correctly."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Skip files that don't have #[cfg(test)] modules
        if '#[cfg(test)]' not in content:
            print(f"✓ {file_path} - Already correct structure")
            return True
            
        # Extract copyright header and file-level comments
        lines = content.split('\n')
        header_lines = []
        content_start = 0
        
        for i, line in enumerate(lines):
            if line.startswith('//!') or line.strip() == '':
                header_lines.append(line)
                content_start = i + 1
            else:
                break
        
        # Find the #[cfg(test)] module
        cfg_test_pattern = r'#\[cfg\(test\)\]\s*\n\s*mod\s+\w+\s*\{'
        match = re.search(cfg_test_pattern, content)
        
        if not match:
            print(f"✓ {file_path} - No #[cfg(test)] module found")
            return True
        
        # Extract content inside the #[cfg(test)] module
        start_pos = match.end() - 1  # Position of opening brace
        brace_count = 1
        pos = start_pos + 1
        
        while pos < len(content) and brace_count > 0:
            if content[pos] == '{':
                brace_count += 1
            elif content[pos] == '}':
                brace_count -= 1
            pos += 1
        
        if brace_count != 0:
            print(f"✗ {file_path} - Unmatched braces in #[cfg(test)] module")
            return False
        
        # Extract the module content (between braces)
        module_content = content[start_pos + 1:pos - 1]
        
        # Parse the module content to extract imports and test functions
        module_lines = module_content.strip().split('\n')
        imports = []
        test_functions = []
        current_function = []
        in_function = False
        brace_depth = 0
        
        for line in module_lines:
            stripped = line.strip()
            
            if stripped.startswith('use '):
                # Remove leading whitespace from imports
                imports.append(stripped)
            elif stripped.startswith('#[test]'):
                if current_function and in_function:
                    test_functions.append('\n'.join(current_function))
                current_function = [line]
                in_function = True
                brace_depth = 0
            elif in_function:
                current_function.append(line)
                # Count braces to know when function ends
                brace_depth += line.count('{') - line.count('}')
                if brace_depth == 0 and stripped.endswith('}'):
                    test_functions.append('\n'.join(current_function))
                    current_function = []
                    in_function = False
            elif stripped and not stripped.startswith('//'):
                # Non-import, non-test content
                if current_function and in_function:
                    current_function.append(line)
        
        # Add any remaining function
        if current_function and in_function:
            test_functions.append('\n'.join(current_function))
        
        # Reconstruct the file
        new_content = []
        
        # Add header
        new_content.extend(header_lines)
        if header_lines and header_lines[-1].strip():
            new_content.append('')
        
        # Add imports
        if imports:
            new_content.extend(imports)
            new_content.append('')
        
        # Add test functions
        for i, func in enumerate(test_functions):
            if i > 0:
                new_content.append('')
            new_content.append(func)
        
        # Write the fixed file
        new_file_content = '\n'.join(new_content)
        
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(new_file_content)
        
        print(f"✓ {file_path} - Fixed structure (moved {len(imports)} imports, {len(test_functions)} test functions)")
        return True
        
    except Exception as e:
        print(f"✗ {file_path} - Error: {e}")
        return False

def main():
    """Fix all test files with incorrect structure."""
    if len(sys.argv) > 1:
        # Fix specific files
        files_to_fix = sys.argv[1:]
    else:
        # Fix all test files
        test_dir = Path('tests')
        files_to_fix = list(test_dir.rglob('*.rs'))
    
    fixed_count = 0
    error_count = 0
    
    for file_path in files_to_fix:
        if fix_test_file(file_path):
            fixed_count += 1
        else:
            error_count += 1
    
    print(f"\n📊 Summary: {fixed_count} files processed successfully, {error_count} errors")
    return error_count == 0

if __name__ == '__main__':
    success = main()
    sys.exit(0 if success else 1)
