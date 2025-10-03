#!/usr/bin/env python3
"""
Script to fix Pattern 2: Convert specific item imports to wildcard imports
Converts imports like use crate::Module::{Item1, Item2} to use crate::Module::*
"""

import os
import re
from pathlib import Path

def fix_specific_imports(file_path):
    """Fix specific imports in a single file by converting to wildcard imports."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Pattern: use crate::Module::SubModule::{Item1, Item2, ...}; -> use crate::Module::SubModule::*;
        # But preserve Types::Types imports as they are special
        
        # Find all specific imports (with braces)
        specific_import_pattern = r'use crate::([^:]+(?:::[^:]+)*)::\{([^}]+)\};'
        matches = re.findall(specific_import_pattern, content)
        
        for match in matches:
            module_path = match[0].strip()
            items = match[1].strip()
            
            # Skip Types::Types imports - they should stay specific
            if 'Types::Types' in module_path:
                continue
                
            # Skip if it's just importing a single trait and struct pair (common pattern)
            item_list = [item.strip() for item in items.split(',')]
            if len(item_list) == 2:
                # Check if it's a Struct + StructTrait pattern
                if (len(item_list) == 2 and 
                    any(item.endswith('Trait') for item in item_list) and
                    any(not item.endswith('Trait') and item[0].isupper() for item in item_list)):
                    # This is likely a struct + trait pair, keep it specific
                    continue
            
            # Convert to wildcard import
            old_import = f"use crate::{module_path}::{{{items}}};"
            new_import = f"use crate::{module_path}::*;"
            
            if old_import in content:
                content = content.replace(old_import, new_import)
                changes_made.append(f"{module_path}: specific -> wildcard")
        
        # Write back if changes were made
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return changes_made
        
        return []
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def fix_all_specific_imports():
    """Fix specific imports across all Rust files."""
    
    directories = ['src', 'tests', 'benches']
    total_files_processed = 0
    total_files_changed = 0
    all_changes = {}
    
    print("🔧 Fixing Pattern 2: Specific Item Imports")
    print("=" * 60)
    print("Converting specific imports to wildcard imports (preserving Types::Types)...")
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
                    
                    changes = fix_specific_imports(file_path)
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
            for file_path in sorted(files)[:5]:  # Show first 5
                print(f"   - {file_path}")
            if len(files) > 5:
                print(f"   ... and {len(files) - 5} more files")
    
    return total_files_changed > 0

if __name__ == "__main__":
    success = fix_all_specific_imports()
    if success:
        print(f"\n✅ Pattern 2 fixes applied successfully!")
        print("🏗️  Run 'cargo build' to verify changes compile correctly.")
    else:
        print(f"\n✅ No Pattern 2 issues found to fix.")
