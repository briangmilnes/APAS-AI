#!/usr/bin/env python3
"""
Script to revert Pattern 1 changes that broke compilation.
Reverts Chap19 imports back to Chap18 where they were incorrectly changed.
"""

import os
import re
from pathlib import Path

def revert_pattern1_changes(file_path):
    """Revert Pattern 1 changes in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Revert Pattern 1: ArraySeqStEph - revert Chap19 back to Chap18 where needed
        if 'Chap19::ArraySeqStEph' in content:
            # Revert Chap19::ArraySeqStEph back to Chap18::ArraySeqStEph
            old_pattern = r'use crate::Chap19::ArraySeqStEph::ArraySeqStEph::'
            new_pattern = 'use crate::Chap18::ArraySeqStEph::ArraySeqStEph::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqStEph: Chap19 → Chap18 (revert)")
        
        # Revert Pattern 2: ArraySeqStPer - revert Chap19 back to Chap18 where needed
        if 'Chap19::ArraySeqStPer' in content:
            # Revert Chap19::ArraySeqStPer back to Chap18::ArraySeqStPer
            old_pattern = r'use crate::Chap19::ArraySeqStPer::ArraySeqStPer::'
            new_pattern = 'use crate::Chap18::ArraySeqStPer::ArraySeqStPer::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqStPer: Chap19 → Chap18 (revert)")
        
        # Revert Pattern 3: ArraySeqMtEph - revert Chap19 back to Chap18 where needed
        if 'Chap19::ArraySeqMtEph' in content:
            # Revert Chap19::ArraySeqMtEph back to Chap18::ArraySeqMtEph
            old_pattern = r'use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::'
            new_pattern = 'use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqMtEph: Chap19 → Chap18 (revert)")
        
        # Revert Pattern 4: ArraySeqMtPer - revert Chap19 back to Chap18 where needed
        if 'Chap19::ArraySeqMtPer' in content:
            # Revert Chap19::ArraySeqMtPer back to Chap18::ArraySeqMtPer
            old_pattern = r'use crate::Chap19::ArraySeqMtPer::ArraySeqMtPer::'
            new_pattern = 'use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::'
            if re.search(old_pattern, content):
                content = re.sub(old_pattern, new_pattern, content)
                changes_made.append("ArraySeqMtPer: Chap19 → Chap18 (revert)")
        
        # Write back if changes were made
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return changes_made
        
        return []
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def revert_all_pattern1_changes():
    """Revert Pattern 1 changes across all Rust files."""
    
    directories = ['src', 'tests', 'benches']
    total_files_processed = 0
    total_files_changed = 0
    all_changes = {}
    
    print("🔧 Reverting Pattern 1: Duplicate Chapter Usage")
    print("=" * 60)
    print("Reverting problematic Chap19 imports back to Chap18...")
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
                    
                    changes = revert_pattern1_changes(file_path)
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
    
    return total_files_changed > 0

if __name__ == "__main__":
    success = revert_all_pattern1_changes()
    if success:
        print(f"\n✅ Pattern 1 reverts applied successfully!")
        print("🏗️  Run 'cargo build' to verify changes compile correctly.")
    else:
        print(f"\n✅ No Pattern 1 reverts needed.")
