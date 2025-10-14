#!/usr/bin/env python3
"""
Script to fix Pattern 3: Consolidate Types module imports
Ensures all files use 'use crate::Types::Types::*;' consistently
"""

import os
import re
from pathlib import Path

def fix_types_imports(file_path):
    """Fix Types module imports in a single file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Check if file already has the standard Types import
        has_standard_types_import = 'use crate::Types::Types::*;' in content
        
        # Find all Types-related imports
        types_import_patterns = [
            r'use crate::Types::Types::\{[^}]+\};',  # Specific Types imports
            r'use crate::Types::Types::[^;]+;',      # Single Types imports
        ]
        
        found_types_imports = []
        for pattern in types_import_patterns:
            matches = re.findall(pattern, content)
            found_types_imports.extend(matches)
        
        if found_types_imports and not has_standard_types_import:
            # Replace all specific Types imports with wildcard
            for types_import in found_types_imports:
                if types_import in content:
                    content = content.replace(types_import, 'use crate::Types::Types::*;')
                    changes_made.append(f"Replaced specific Types import with wildcard")
        
        elif found_types_imports and has_standard_types_import:
            # Remove duplicate specific Types imports if wildcard already exists
            for types_import in found_types_imports:
                if types_import != 'use crate::Types::Types::*;' and types_import in content:
                    content = content.replace(types_import, '')
                    # Clean up any resulting double newlines
                    content = re.sub(r'\n\n\n+', '\n\n', content)
                    changes_made.append(f"Removed duplicate Types import")
        
        # Also consolidate external crate imports that are Types-related
        # Look for patterns like: use crate::Types::Types::OrderedF64;
        single_types_pattern = r'use crate::Types::Types::([^;]+);'
        single_matches = re.findall(single_types_pattern, content)
        
        if single_matches and not has_standard_types_import:
            # If we have single Types imports but no wildcard, add wildcard and remove singles
            # Find a good place to insert the wildcard import (after other use statements)
            use_statements = re.findall(r'use [^;]+;', content)
            if use_statements:
                # Insert after the last use statement
                last_use = use_statements[-1]
                if 'use crate::Types::Types::*;' not in content:
                    content = content.replace(last_use, last_use + '\nuse crate::Types::Types::*;')
                    changes_made.append("Added wildcard Types import")
                
                # Remove the single imports
                for match in single_matches:
                    single_import = f'use crate::Types::Types::{match};'
                    if single_import in content:
                        content = content.replace(single_import, '')
                        changes_made.append(f"Removed single Types import: {match}")
        
        # Clean up any resulting formatting issues
        content = re.sub(r'\n\n\n+', '\n\n', content)
        
        # Write back if changes were made
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return changes_made
        
        return []
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def fix_all_types_imports():
    """Fix Types imports across all Rust files."""
    
    directories = ['src', 'tests', 'benches']
    total_files_processed = 0
    total_files_changed = 0
    all_changes = {}
    
    print("🔧 Fixing Pattern 3: Types Module Imports")
    print("=" * 60)
    print("Consolidating Types module imports to use wildcard...")
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
                    
                    changes = fix_types_imports(file_path)
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
    success = fix_all_types_imports()
    if success:
        print(f"\n✅ Pattern 3 fixes applied successfully!")
        print("🏗️  Run 'cargo build' to verify changes compile correctly.")
    else:
        print(f"\n✅ No Pattern 3 issues found to fix.")
