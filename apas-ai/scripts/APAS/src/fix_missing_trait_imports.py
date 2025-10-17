#!/usr/bin/env python3
"""
Script to fix missing trait imports for files that import Chap18 structs but need Chap19 traits.
Adds the necessary trait imports to make trait methods like 'update' available.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path

def fix_missing_trait_imports(file_path):
    """Fix missing trait imports in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Check if file imports ArraySeqStPerS from Chap18
        has_chap18_struct = 'use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS' in content
        
        # Check if file already has the Chap19 trait import
        has_chap19_trait = 'use crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait' in content
        
        # Check if file uses update method (indicating it needs the trait)
        uses_update_method = 'ArraySeqStPerS::update(' in content
        
        if has_chap18_struct and uses_update_method and not has_chap19_trait:
            # Find the last use statement to add the trait import after it
            use_statements = []
            for line_num, line in enumerate(content.split('\n')):
                if line.strip().startswith('use crate::'):
                    use_statements.append(line_num)
            
            if use_statements:
                # Add the trait import after the last use statement
                lines = content.split('\n')
                last_use_line = use_statements[-1]
                
                # Insert the trait import
                trait_import = "    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait;"
                lines.insert(last_use_line + 1, trait_import)
                
                content = '\n'.join(lines)
                changes_made.append(f"Added ArraySeqStPerTrait import")
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            return changes_made
        else:
            return []
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def main():
    """Process all Rust files to fix missing trait imports."""
    src_directories = ['src', 'tests', 'benches']
    total_files_processed = 0
    total_changes = 0
    
    for src_dir in src_directories:
        if not os.path.exists(src_dir):
            continue
            
        print(f"\n🔍 Processing {src_dir}/ directory...")
        
        for root, dirs, files in os.walk(src_dir):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    total_files_processed += 1
                    
                    changes = fix_missing_trait_imports(file_path)
                    if changes:
                        total_changes += 1
                        print(f"📝 {file_path}")
                        for change in changes:
                            print(f"   ✓ {change}")
    
    print(f"\n📊 SUMMARY:")
    print(f"   Files processed: {total_files_processed}")
    print(f"   Files changed: {total_changes}")
    
    if total_changes > 0:
        print(f"\n✅ Fixed missing trait imports in {total_changes} files")
    else:
        print(f"\n✅ No missing trait imports found")

if __name__ == "__main__":
    main()
