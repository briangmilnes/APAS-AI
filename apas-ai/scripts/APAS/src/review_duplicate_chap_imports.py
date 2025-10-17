#!/usr/bin/env python3
"""
Script to find remaining files that import from both Chap18 and Chap19 (duplicate imports).
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path

def check_file_for_duplicate_imports(file_path):
    """Check if a file imports from both Chap18 and Chap19."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        has_chap18_import = bool(re.search(r'use crate::Chap18::', content))
        has_chap19_import = bool(re.search(r'use crate::Chap19::', content))
        
        if has_chap18_import and has_chap19_import:
            # Extract the actual imports for analysis
            chap18_imports = re.findall(r'use crate::Chap18::[^;]+;', content)
            chap19_imports = re.findall(r'use crate::Chap19::[^;]+;', content)
            
            return {
                'has_duplicates': True,
                'chap18_imports': chap18_imports,
                'chap19_imports': chap19_imports
            }
        else:
            return {'has_duplicates': False}
            
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return {'has_duplicates': False}

def main():
    """Find all files with duplicate chapter imports."""
    src_directories = ['src', 'tests', 'benches']
    duplicate_files = []
    total_files_checked = 0
    
    for src_dir in src_directories:
        if not os.path.exists(src_dir):
            continue
            
        for root, dirs, files in os.walk(src_dir):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    total_files_checked += 1
                    
                    result = check_file_for_duplicate_imports(file_path)
                    if result['has_duplicates']:
                        duplicate_files.append({
                            'file': file_path,
                            'chap18_imports': result['chap18_imports'],
                            'chap19_imports': result['chap19_imports']
                        })
    
    print(f"🔍 CHECKING FOR REMAINING DUPLICATE CHAPTER IMPORTS")
    print(f"=" * 60)
    print(f"Files checked: {total_files_checked}")
    print(f"Files with Chap18 + Chap19 imports: {len(duplicate_files)}")
    print()
    
    if duplicate_files:
        print(f"❌ REMAINING DUPLICATE IMPORTS FOUND:")
        print(f"-" * 40)
        for item in duplicate_files:
            print(f"\n📁 {item['file']}")
            print(f"   Chap18 imports: {len(item['chap18_imports'])}")
            for imp in item['chap18_imports']:
                print(f"     - {imp}")
            print(f"   Chap19 imports: {len(item['chap19_imports'])}")
            for imp in item['chap19_imports']:
                print(f"     - {imp}")
        
        print(f"\n⚠️  These files need attention to resolve duplicate imports!")
    else:
        print(f"✅ NO DUPLICATE CHAPTER IMPORTS FOUND!")
        print(f"All files import from either Chap18 OR Chap19, not both.")

if __name__ == "__main__":
    main()
