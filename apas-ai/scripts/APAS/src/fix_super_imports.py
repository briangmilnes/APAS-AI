#!/usr/bin/env python3
"""
Script to fix 'use super::*;' imports in test files by replacing them with proper crate imports.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
import sys
from pathlib import Path

def fix_super_imports(file_path):
    """Fix super imports in a test file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        if 'use super::*;' not in content:
            return True
        
        # Extract chapter and module name from file path
        path_parts = Path(file_path).parts
        
        # Handle different file patterns
        if 'Chap' in str(file_path):
            # Extract chapter number
            chap_match = re.search(r'Chap(\d+)', str(file_path))
            if chap_match:
                chap_num = chap_match.group(1)
                
                # Extract module name from filename
                filename = Path(file_path).stem
                if filename.startswith('Test'):
                    module_name = filename[4:]  # Remove 'Test' prefix
                    
                    # Generate proper imports based on common patterns
                    imports = []
                    imports.append(f"use apas_ai::Chap{chap_num}::{module_name}::{module_name}::*;")
                    
                    # Add common additional imports based on module type
                    if 'Graph' in module_name:
                        imports.append("use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;")
                    elif 'TreePQ' in module_name:
                        imports.append("use apas_ai::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;")
                    elif 'Array' in module_name:
                        imports.append("use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;")
                    
                    imports.append("use apas_ai::Types::Types::*;")
                    
                    # Replace the super import
                    new_content = content.replace('use super::*;', '\n'.join(imports))
                    
                    with open(file_path, 'w', encoding='utf-8') as f:
                        f.write(new_content)
                    
                    print(f"✓ {file_path} - Fixed super import")
                    return True
        
        # Handle special cases
        if 'TestOptBinSearchTree.rs' in str(file_path):
            imports = [
                "use apas_ai::Chap49::OptBinSearchTree::OptBinSearchTree::*;",
                "use apas_ai::Types::Types::*;"
            ]
            new_content = content.replace('use super::*;', '\n'.join(imports))
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
            print(f"✓ {file_path} - Fixed super import (special case)")
            return True
        
        print(f"? {file_path} - Could not determine proper imports")
        return True
        
    except Exception as e:
        print(f"✗ {file_path} - Error: {e}")
        return False

def main():
    """Fix super imports in all test files."""
    # Find all test files with super imports
    test_files = []
    for root, dirs, files in os.walk('tests'):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                try:
                    with open(file_path, 'r', encoding='utf-8') as f:
                        if 'use super::*;' in f.read():
                            test_files.append(file_path)
                except:
                    pass
    
    fixed_count = 0
    error_count = 0
    
    for file_path in test_files:
        if fix_super_imports(file_path):
            fixed_count += 1
        else:
            error_count += 1
    
    print(f"\n📊 Summary: {fixed_count} files processed successfully, {error_count} errors")
    return error_count == 0

if __name__ == '__main__':
    success = main()
    sys.exit(0 if success else 1)
