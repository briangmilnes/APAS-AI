#!/usr/bin/env python3
"""
Script to find and fix `use ...{X as Y}` patterns where UFCS calls to Y should be changed to X.
Finds alias imports and replaces UFCS calls from alias (Y) back to original name (X).
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path
from typing import Dict, List, Tuple

def extract_aliases_from_file(file_path: str) -> Dict[str, str]:
    """Extract alias mappings from a file: {alias_name: original_name}"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        aliases = {}
        
        # Pattern to match: use path::{OriginalName as AliasName}
        # Also handles: use path::{Name1, OriginalName as AliasName, Name3}
        alias_pattern = r'use\s+[^;]*\{[^}]*\b(\w+)\s+as\s+(\w+)[^}]*\}[^;]*;'
        matches = re.findall(alias_pattern, content)
        
        for original, alias in matches:
            aliases[alias] = original
            
        return aliases
        
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return {}

def fix_ufcs_calls_in_file(file_path: str, aliases: Dict[str, str]) -> List[str]:
    """Fix UFCS calls in a file, replacing alias calls with original names."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        for alias, original in aliases.items():
            # Pattern for UFCS calls: AliasName::method_name
            ufcs_pattern = rf'\b{re.escape(alias)}::'
            
            if re.search(ufcs_pattern, content):
                # Replace alias:: with original::
                new_content = re.sub(ufcs_pattern, f'{original}::', content)
                if new_content != content:
                    content = new_content
                    changes_made.append(f"UFCS: {alias}:: -> {original}::")
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            return changes_made
        else:
            return []
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def analyze_directory(directory: str) -> Tuple[Dict[str, Dict[str, str]], int]:
    """Analyze all Rust files in a directory for alias imports."""
    all_aliases = {}
    total_files = 0
    
    if not os.path.exists(directory):
        return all_aliases, total_files
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                total_files += 1
                
                aliases = extract_aliases_from_file(file_path)
                if aliases:
                    all_aliases[file_path] = aliases
    
    return all_aliases, total_files

def fix_directory(directory: str) -> Tuple[int, int, List[str]]:
    """Fix UFCS alias calls in all files in a directory."""
    print(f"\n🔧 FIXING UFCS ALIASES IN {directory.upper()}/")
    print("=" * 60)
    
    # First pass: collect all aliases
    all_aliases, total_files = analyze_directory(directory)
    
    if not all_aliases:
        print(f"✅ No alias imports found in {total_files} files")
        return 0, total_files, []
    
    print(f"📋 Found alias imports in {len(all_aliases)} files:")
    for file_path, aliases in all_aliases.items():
        print(f"   📁 {file_path}")
        for alias, original in aliases.items():
            print(f"      {original} as {alias}")
    
    # Second pass: fix UFCS calls in all files
    print(f"\n🔄 Fixing UFCS calls...")
    files_changed = 0
    all_changes = []
    
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.endswith('.rs'):
                file_path = os.path.join(root, file)
                
                # Collect all aliases that might be used in this file
                file_aliases = {}
                for aliases_file, aliases in all_aliases.items():
                    file_aliases.update(aliases)
                
                if file_aliases:
                    changes = fix_ufcs_calls_in_file(file_path, file_aliases)
                    if changes:
                        files_changed += 1
                        print(f"   ✅ {file_path}")
                        for change in changes:
                            print(f"      - {change}")
                            all_changes.append(f"{file_path}: {change}")
    
    return files_changed, total_files, all_changes

def main():
    """Main function to process src, tests, and benches directories."""
    print("🎯 UFCS ALIAS FIXER")
    print("=" * 40)
    print("Finding use ...{X as Y} patterns and fixing UFCS Y:: calls to X::")
    
    directories = ['src', 'tests', 'benches']
    total_changes = []
    
    for directory in directories:
        if os.path.exists(directory):
            files_changed, total_files, changes = fix_directory(directory)
            total_changes.extend(changes)
            
            print(f"\n📊 {directory.upper()} SUMMARY:")
            print(f"   Files processed: {total_files}")
            print(f"   Files changed: {files_changed}")
        else:
            print(f"\n⚠️  Directory {directory}/ not found")
    
    print(f"\n🎉 OVERALL SUMMARY")
    print("=" * 30)
    print(f"Total changes made: {len(total_changes)}")
    
    if total_changes:
        print(f"\n📋 ALL CHANGES:")
        for change in total_changes:
            print(f"   • {change}")
        
        print(f"\n✅ UFCS alias fixes completed!")
        print(f"🏗️  Run 'cargo build' to verify changes compile correctly.")
    else:
        print(f"\n✅ No UFCS alias issues found - codebase is clean!")

if __name__ == "__main__":
    main()
