#!/usr/bin/env python3
"""
Script to find duplicate datastructure imports across different chapters.
Looks for cases where the same datastructure is imported from multiple chapters.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from collections import defaultdict
from pathlib import Path

def extract_imports_from_file(file_path):
    """Extract all use statements from a Rust file."""
    imports = []
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find all use statements
        use_pattern = r'use\s+crate::([^;]+);'
        matches = re.findall(use_pattern, content)
        
        for match in matches:
            imports.append(match.strip())
            
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        
    return imports

def parse_import_path(import_path):
    """Parse an import path to extract chapter, module, and imported items."""
    # Pattern: Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::{AVLTreeSeqStEphS, AVLTreeSeqStEphTrait}
    chapter_pattern = r'Chap(\d+)::'
    chapter_match = re.search(chapter_pattern, import_path)
    chapter = int(chapter_match.group(1)) if chapter_match else None
    
    # Extract the datastructure name (the part after the second ::)
    parts = import_path.split('::')
    if len(parts) >= 3:
        datastructure = parts[1]  # e.g., AVLTreeSeqStEph
        
        # Extract imported items if they exist
        if '{' in import_path and '}' in import_path:
            items_match = re.search(r'\{([^}]+)\}', import_path)
            if items_match:
                items = [item.strip() for item in items_match.group(1).split(',')]
            else:
                items = []
        else:
            # Single import or wildcard
            if import_path.endswith('::*'):
                items = ['*']
            else:
                items = [parts[-1]]
                
        return {
            'chapter': chapter,
            'datastructure': datastructure,
            'items': items,
            'full_path': import_path
        }
    
    return None

def find_duplicate_imports():
    """Find files that import the same datastructure from multiple chapters."""
    
    # Directories to scan
    directories = ['src', 'tests', 'benches']
    
    file_imports = {}  # file_path -> list of parsed imports
    datastructure_usage = defaultdict(lambda: defaultdict(list))  # datastructure -> chapter -> [files]
    
    print("🔍 Scanning for duplicate datastructure imports...\n")
    
    for directory in directories:
        if not os.path.exists(directory):
            continue
            
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    imports = extract_imports_from_file(file_path)
                    
                    parsed_imports = []
                    for import_path in imports:
                        parsed = parse_import_path(import_path)
                        if parsed and parsed['chapter']:
                            parsed_imports.append(parsed)
                            
                            # Track datastructure usage by chapter
                            ds = parsed['datastructure']
                            chapter = parsed['chapter']
                            datastructure_usage[ds][chapter].append(file_path)
                    
                    if parsed_imports:
                        file_imports[file_path] = parsed_imports
    
    # Find duplicates
    duplicates_found = False
    
    print("📋 DUPLICATE DATASTRUCTURE IMPORTS ANALYSIS")
    print("=" * 60)
    
    # Check each file for multiple chapters of same datastructure
    for file_path, imports in file_imports.items():
        datastructures_in_file = defaultdict(list)
        
        for imp in imports:
            ds = imp['datastructure']
            datastructures_in_file[ds].append(imp)
        
        # Check for duplicates within this file
        for ds, ds_imports in datastructures_in_file.items():
            if len(ds_imports) > 1:
                chapters = [imp['chapter'] for imp in ds_imports]
                if len(set(chapters)) > 1:  # Multiple different chapters
                    duplicates_found = True
                    print(f"\n🚨 DUPLICATE FOUND in {file_path}")
                    print(f"   Datastructure: {ds}")
                    print(f"   Imported from chapters: {sorted(set(chapters))}")
                    
                    for imp in ds_imports:
                        print(f"     Chap{imp['chapter']}: {imp['full_path']}")
                        print(f"       Items: {', '.join(imp['items'])}")
    
    # Summary by datastructure
    print(f"\n📊 DATASTRUCTURE USAGE SUMMARY")
    print("=" * 40)
    
    multi_chapter_ds = {}
    for ds, chapters in datastructure_usage.items():
        if len(chapters) > 1:
            multi_chapter_ds[ds] = chapters
            print(f"\n📦 {ds}:")
            for chapter in sorted(chapters.keys()):
                files = chapters[chapter]
                print(f"   Chap{chapter}: {len(files)} files")
                for file in sorted(set(files)):
                    print(f"     - {file}")
    
    if not duplicates_found:
        print("\n✅ No duplicate datastructure imports found within individual files.")
    
    if not multi_chapter_ds:
        print("\n✅ No datastructures found imported from multiple chapters.")
    else:
        print(f"\n⚠️  Found {len(multi_chapter_ds)} datastructures imported from multiple chapters.")
    
    return file_imports, multi_chapter_ds

if __name__ == "__main__":
    find_duplicate_imports()
