#!/usr/bin/env python3
"""
Script to fix Pattern 1: Duplicate chapter usage
Consolidates ArraySeq imports to use the highest chapter number (Chap19 over Chap18)
"""

import os
import re
from pathlib import Path

def fix_duplicate_chapter_imports(file_path):
    """Fix duplicate chapter imports in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Pattern 1: ArraySeqStEph - prefer Chap19 over Chap18
        if 'Chap18::ArraySeqStEph' in content:
            # Replace Chap18::ArraySeqStEph with Chap19::ArraySeqStEph
            old_pattern = r'use crate::Chap18::ArraySeqStEph::ArraySeqStEph::'
            new_pattern = 'use crate::Chap19::ArraySeqStEph::ArraySeqStEph::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqStEph: Chap18 → Chap19")
        
        # Pattern 2: ArraySeqStPer - prefer Chap19 over Chap18  
        if 'Chap18::ArraySeqStPer' in content:
            # Replace Chap18::ArraySeqStPer with Chap19::ArraySeqStPer
            old_pattern = r'use crate::Chap18::ArraySeqStPer::ArraySeqStPer::'
            new_pattern = 'use crate::Chap19::ArraySeqStPer::ArraySeqStPer::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqStPer: Chap18 → Chap19")
        
        # Pattern 3: ArraySeqMtEph - prefer Chap19 over Chap18
        if 'Chap18::ArraySeqMtEph' in content:
            # Replace Chap18::ArraySeqMtEph with Chap19::ArraySeqMtEph
            old_pattern = r'use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::'
            new_pattern = 'use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqMtEph: Chap18 → Chap19")
        
        # Pattern 4: ArraySeqMtPer - prefer Chap19 over Chap18
        if 'Chap18::ArraySeqMtPer' in content:
            # Replace Chap18::ArraySeqMtPer with Chap19::ArraySeqMtPer
            old_pattern = r'use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::'
            new_pattern = 'use crate::Chap19::ArraySeqMtPer::ArraySeqMtPer::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqMtPer: Chap18 → Chap19")
        
        # Write back if changes were made
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return changes_made
        
        return []
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def fix_all_duplicate_chapter_imports():
    """Fix duplicate chapter imports across all Rust files."""
    
    directories = ['src', 'tests', 'benches']
    total_files_processed = 0
    total_files_changed = 0
    all_changes = {}
    
    print("🔧 Fixing Pattern 1: Duplicate Chapter Usage")
    print("=" * 60)
    print("Consolidating ArraySeq imports to use Chap19 over Chap18...")
    print()
    
    for directory in directories:
        if not os.path.exists(directory):
            continue
            
        print(f"📁 Processing {directory}/...")
        
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    total_files_processed += 1
                    
                    changes = fix_duplicate_chapter_imports(file_path)
                    if changes:
                        total_files_changed += 1
                        all_changes[file_path] = changes
                        print(f"  ✅ {file_path}")
                        for change in changes:
                            print(f"     - {change}")
    
    print(f"\n📊 SUMMARY")
    print("=" * 30)
    print(f"Files processed: {total_files_processed}")
    print(f"Files changed: {total_files_changed}")
    
    if all_changes:
        print(f"\n📋 DETAILED CHANGES")
        print("-" * 40)
        
        change_summary = {}
        for file_path, changes in all_changes.items():
            for change in changes:
                if change not in change_summary:
                    change_summary[change] = []
                change_summary[change].append(file_path)
        
        for change_type, files in change_summary.items():
            print(f"\n🔄 {change_type}: {len(files)} files")
            for file_path in sorted(files):
                print(f"   - {file_path}")
    
    return total_files_changed > 0

if __name__ == "__main__":
    success = fix_all_duplicate_chapter_imports()
    if success:
        print(f"\n✅ Pattern 1 fixes applied successfully!")
        print("🏗️  Run 'cargo build' to verify changes compile correctly.")
    else:
        print(f"\n✅ No Pattern 1 issues found to fix.")
