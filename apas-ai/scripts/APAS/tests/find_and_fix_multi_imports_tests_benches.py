#!/usr/bin/env python3
"""
Script to find multi-import patterns in tests and benches that can be converted to wildcards.
Excludes Rust standard library imports to avoid conflicts.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path

def analyze_multi_imports_in_file(file_path: str) -> dict:
    """Analyze multi-import patterns in a file, excluding std library."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        results = {
            'multi_imports': [],
            'suggestions': []
        }
        
        # Pattern to match: use path::{Item1, Item2, Item3, ...}
        multi_import_pattern = r'use\s+((?:crate|apas_ai)::[^:]+(?:::[^:]+)*)::\{([^}]+)\};'
        matches = re.findall(multi_import_pattern, content)
        
        for module_path, items_str in matches:
            # Skip if any item has an alias (contains " as ")
            if ' as ' in items_str:
                continue
                
            # Skip if it's a standard library import
            if module_path.startswith('std::') or module_path.startswith('core::'):
                continue
                
            # Parse the items
            items = [item.strip() for item in items_str.split(',') if item.strip()]
            
            # Only suggest wildcard if there are multiple items (2+)
            if len(items) >= 2:
                results['multi_imports'].append({
                    'module_path': module_path,
                    'items': items,
                    'count': len(items),
                    'original': f"use {module_path}::{{{items_str}}};"
                })
                
                # Suggest wildcard replacement
                results['suggestions'].append({
                    'original': f"use {module_path}::{{{items_str}}};",
                    'suggested': f"use {module_path}::*;",
                    'module_path': module_path,
                    'items': items,
                    'count': len(items)
                })
        
        return results
        
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return {'multi_imports': [], 'suggestions': []}

def fix_multi_imports_in_file(file_path: str) -> list:
    """Fix multi-import patterns by converting to wildcards."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Pattern to match: use path::{Item1, Item2, Item3, ...}
        multi_import_pattern = r'use\s+((?:crate|apas_ai)::[^:]+(?:::[^:]+)*)::\{([^}]+)\};'
        
        def replace_multi_import(match):
            module_path = match.group(1)
            items_str = match.group(2)
            
            # Skip if any item has an alias
            if ' as ' in items_str:
                return match.group(0)  # Keep original
            
            # Skip if it's a standard library import
            if module_path.startswith('std::') or module_path.startswith('core::'):
                return match.group(0)  # Keep original
            
            # Parse items
            items = [item.strip() for item in items_str.split(',') if item.strip()]
            
            # Only replace if 2+ items
            if len(items) >= 2:
                changes_made.append(f"Converted {len(items)} items to wildcard: {module_path}")
                return f"use {module_path}::*;"
            else:
                return match.group(0)  # Keep original
        
        new_content = re.sub(multi_import_pattern, replace_multi_import, content)
        
        if new_content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(new_content)
        
        return changes_made
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def main():
    """Find and fix multi-import patterns in tests and benches only."""
    print("🔍 FINDING MULTI-IMPORT PATTERNS IN TESTS & BENCHES")
    print("=" * 55)
    print("Looking for use ...{X, Y, Z} that should be use ...*")
    print("(Excluding standard library imports)")
    
    directories = ['tests', 'benches']
    total_files_analyzed = 0
    total_multi_imports = 0
    all_suggestions = []
    
    # First pass: analyze
    for directory in directories:
        if not os.path.exists(directory):
            continue
            
        print(f"\n📁 Analyzing {directory}/...")
        dir_files = 0
        dir_imports = 0
        
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    total_files_analyzed += 1
                    dir_files += 1
                    
                    results = analyze_multi_imports_in_file(file_path)
                    
                    if results['multi_imports']:
                        dir_imports += len(results['multi_imports'])
                        total_multi_imports += len(results['multi_imports'])
                        
                        print(f"\n  📄 {file_path}")
                        for imp in results['multi_imports']:
                            print(f"     {imp['count']} items: {imp['module_path']}")
                            print(f"       Items: {', '.join(imp['items'])}")
                        
                        all_suggestions.extend([(file_path, s) for s in results['suggestions']])
        
        print(f"\n📊 {directory} summary: {dir_files} files analyzed, {dir_imports} multi-imports found")
    
    print(f"\n🎯 OVERALL ANALYSIS")
    print("=" * 30)
    print(f"Files analyzed: {total_files_analyzed}")
    print(f"Multi-imports found: {total_multi_imports}")
    
    if all_suggestions:
        print(f"\n💡 CONVERSION SUGGESTIONS ({len(all_suggestions)} total):")
        print("-" * 50)
        
        # Show examples
        for file_path, suggestion in all_suggestions[:10]:  # Show first 10
            rel_path = file_path.replace('tests/', '').replace('benches/', '')
            print(f"\n📁 {rel_path}")
            print(f"     - {suggestion['original']}")
            print(f"     + {suggestion['suggested']}")
        
        if len(all_suggestions) > 10:
            print(f"\n     ... and {len(all_suggestions) - 10} more conversions")
        
        print(f"\n🔧 Applying conversions automatically...")
        
        # Second pass: fix
        total_files_changed = 0
        total_changes = 0
        
        for directory in directories:
            if not os.path.exists(directory):
                continue
                
            print(f"\n🔄 Fixing {directory}/...")
            
            for root, dirs, files in os.walk(directory):
                for file in files:
                    if file.endswith('.rs'):
                        file_path = os.path.join(root, file)
                        
                        changes = fix_multi_imports_in_file(file_path)
                        if changes:
                            total_files_changed += 1
                            total_changes += len(changes)
                            print(f"  ✅ {file_path}")
                            for change in changes:
                                print(f"     - {change}")
        
        print(f"\n🎉 CONVERSION COMPLETE")
        print("=" * 30)
        print(f"Files changed: {total_files_changed}")
        print(f"Total conversions: {total_changes}")
        print(f"\n✅ Information duplication reduced in tests & benches!")
        print(f"🏗️  Run 'cargo test --no-run' and 'cargo bench --no-run' to verify.")
        
    else:
        print(f"\n✅ No multi-import patterns found in tests & benches!")
        print(f"All imports are already optimized.")

if __name__ == "__main__":
    main()
