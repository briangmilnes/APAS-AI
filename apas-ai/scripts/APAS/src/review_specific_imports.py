#!/usr/bin/env python3
"""
Script to find specific import patterns like Type::B, N, etc.
Looks for imports that bring in specific items from modules.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from collections import defaultdict
from pathlib import Path

def extract_specific_imports_from_file(file_path):
    """Extract specific import patterns from a Rust file."""
    specific_imports = []
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find all use statements with specific patterns
        use_patterns = [
            # Pattern 1: use crate::Module::{Item1, Item2, ...}
            r'use\s+crate::([^:]+)::\{([^}]+)\};',
            # Pattern 2: use crate::Module::SubModule::{Item1, Item2, ...}
            r'use\s+crate::([^:]+::[^:]+)::\{([^}]+)\};',
            # Pattern 3: use crate::Module::SubModule::SubSubModule::{Item1, Item2, ...}
            r'use\s+crate::([^:]+::[^:]+::[^:]+)::\{([^}]+)\};',
            # Pattern 4: use crate::Module::SubModule::SubSubModule::SubSubSubModule::{Item1, Item2, ...}
            r'use\s+crate::([^:]+::[^:]+::[^:]+::[^:]+)::\{([^}]+)\};',
        ]
        
        for pattern in use_patterns:
            matches = re.findall(pattern, content)
            for match in matches:
                module_path = match[0].strip()
                items = [item.strip() for item in match[1].split(',')]
                specific_imports.append({
                    'module_path': module_path,
                    'items': items,
                    'full_import': f"use crate::{module_path}::{{{', '.join(items)}}};",
                    'file': file_path
                })
        
        # Also look for single specific imports
        single_import_pattern = r'use\s+crate::([^:]+(?:::[^:]+)*)::([\w]+);'
        single_matches = re.findall(single_import_pattern, content)
        for match in single_matches:
            module_path = match[0].strip()
            item = match[1].strip()
            # Skip if it's a module import (ends with same name as module)
            if not module_path.endswith(f"::{item}"):
                specific_imports.append({
                    'module_path': module_path,
                    'items': [item],
                    'full_import': f"use crate::{module_path}::{item};",
                    'file': file_path
                })
            
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        
    return specific_imports

def categorize_imports(imports):
    """Categorize imports by type."""
    categories = {
        'Types_imports': [],
        'single_letter_imports': [],
        'trait_imports': [],
        'struct_imports': [],
        'function_imports': [],
        'other_imports': []
    }
    
    for imp in imports:
        module_path = imp['module_path']
        items = imp['items']
        
        # Check if importing from Types module
        if 'Types::Types' in module_path:
            categories['Types_imports'].append(imp)
            continue
            
        # Check for single letter imports (like B, N, etc.)
        single_letters = [item for item in items if len(item) == 1 and item.isupper()]
        if single_letters:
            categories['single_letter_imports'].append(imp)
            continue
            
        # Check for trait imports (end with Trait)
        trait_items = [item for item in items if item.endswith('Trait')]
        if trait_items:
            categories['trait_imports'].append(imp)
            continue
            
        # Check for struct imports (start with uppercase, not ending in Trait)
        struct_items = [item for item in items if len(item) > 0 and item[0].isupper() and not item.endswith('Trait')]
        if struct_items:
            categories['struct_imports'].append(imp)
            continue
            
        # Check for function imports (start with lowercase)
        function_items = [item for item in items if len(item) > 0 and item[0].islower()]
        if function_items:
            categories['function_imports'].append(imp)
            continue
            
        categories['other_imports'].append(imp)
    
    return categories

def find_specific_imports():
    """Find files with specific import patterns."""
    
    # Directories to scan
    directories = ['src', 'tests', 'benches']
    
    all_imports = []
    
    print("🔍 Scanning for specific import patterns...\n")
    
    for directory in directories:
        if not os.path.exists(directory):
            continue
            
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    imports = extract_specific_imports_from_file(file_path)
                    all_imports.extend(imports)
    
    if not all_imports:
        print("✅ No specific import patterns found.")
        return
    
    # Categorize imports
    categories = categorize_imports(all_imports)
    
    print("📋 SPECIFIC IMPORT PATTERNS ANALYSIS")
    print("=" * 60)
    
    # Types imports
    if categories['Types_imports']:
        print(f"\n🎯 TYPES MODULE IMPORTS ({len(categories['Types_imports'])} found)")
        print("-" * 40)
        for imp in categories['Types_imports']:
            print(f"📁 {imp['file']}")
            print(f"   {imp['full_import']}")
            print(f"   Items: {', '.join(imp['items'])}")
            print()
    
    # Single letter imports
    if categories['single_letter_imports']:
        print(f"\n🔤 SINGLE LETTER IMPORTS ({len(categories['single_letter_imports'])} found)")
        print("-" * 40)
        single_letter_summary = defaultdict(list)
        for imp in categories['single_letter_imports']:
            single_letters = [item for item in imp['items'] if len(item) == 1 and item.isupper()]
            for letter in single_letters:
                single_letter_summary[letter].append(imp['file'])
            print(f"📁 {imp['file']}")
            print(f"   {imp['full_import']}")
            print(f"   Single letters: {', '.join(single_letters)}")
            print()
        
        print("📊 Single Letter Summary:")
        for letter in sorted(single_letter_summary.keys()):
            files = single_letter_summary[letter]
            print(f"   {letter}: {len(files)} files")
    
    # Trait imports
    if categories['trait_imports']:
        print(f"\n🏷️  TRAIT IMPORTS ({len(categories['trait_imports'])} found)")
        print("-" * 40)
        for imp in categories['trait_imports']:
            traits = [item for item in imp['items'] if item.endswith('Trait')]
            print(f"📁 {imp['file']}")
            print(f"   {imp['full_import']}")
            print(f"   Traits: {', '.join(traits)}")
            print()
    
    # Struct imports
    if categories['struct_imports']:
        print(f"\n🏗️  STRUCT IMPORTS ({len(categories['struct_imports'])} found)")
        print("-" * 40)
        for imp in categories['struct_imports'][:10]:  # Show first 10
            structs = [item for item in imp['items'] if len(item) > 0 and item[0].isupper() and not item.endswith('Trait')]
            print(f"📁 {imp['file']}")
            print(f"   {imp['full_import']}")
            print(f"   Structs: {', '.join(structs)}")
            print()
        if len(categories['struct_imports']) > 10:
            print(f"   ... and {len(categories['struct_imports']) - 10} more struct imports")
    
    # Function imports
    if categories['function_imports']:
        print(f"\n⚙️  FUNCTION IMPORTS ({len(categories['function_imports'])} found)")
        print("-" * 40)
        for imp in categories['function_imports'][:10]:  # Show first 10
            functions = [item for item in imp['items'] if len(item) > 0 and item[0].islower()]
            print(f"📁 {imp['file']}")
            print(f"   {imp['full_import']}")
            print(f"   Functions: {', '.join(functions)}")
            print()
        if len(categories['function_imports']) > 10:
            print(f"   ... and {len(categories['function_imports']) - 10} more function imports")
    
    # Summary
    print(f"\n📊 SUMMARY")
    print("=" * 30)
    print(f"Types module imports: {len(categories['Types_imports'])}")
    print(f"Single letter imports: {len(categories['single_letter_imports'])}")
    print(f"Trait imports: {len(categories['trait_imports'])}")
    print(f"Struct imports: {len(categories['struct_imports'])}")
    print(f"Function imports: {len(categories['function_imports'])}")
    print(f"Other imports: {len(categories['other_imports'])}")
    print(f"Total specific imports: {len(all_imports)}")

if __name__ == "__main__":
    find_specific_imports()
