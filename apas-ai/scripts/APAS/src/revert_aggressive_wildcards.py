#!/usr/bin/env python3
"""
Script to identify and revert aggressive wildcard conversions.
Standard library imports and external crates should usually be specific, not wildcards.
"""

import os
import re
from pathlib import Path

def analyze_wildcard_imports(file_path: str) -> dict:
    """Analyze wildcard imports to identify potentially problematic ones."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        results = {
            'risky_wildcards': [],
            'safe_wildcards': [],
            'suggestions': []
        }
        
        # Find all wildcard imports
        wildcard_pattern = r'use\s+([^;]+)::\*;'
        matches = re.findall(wildcard_pattern, content)
        
        for import_path in matches:
            import_path = import_path.strip()
            
            # Categorize imports
            if import_path.startswith('std::') or import_path.startswith('core::'):
                # Standard library wildcards are risky
                results['risky_wildcards'].append({
                    'path': import_path,
                    'type': 'std_library',
                    'full_import': f"use {import_path}::*;",
                    'reason': 'Standard library wildcards can cause name conflicts'
                })
            elif any(ext in import_path for ext in ['rand::', 'ordered_float::', 'serde::']):
                # External crate wildcards are risky
                results['risky_wildcards'].append({
                    'path': import_path,
                    'type': 'external_crate',
                    'full_import': f"use {import_path}::*;",
                    'reason': 'External crate wildcards can cause name conflicts'
                })
            elif import_path.startswith('crate::'):
                # Internal crate wildcards are generally safer
                results['safe_wildcards'].append({
                    'path': import_path,
                    'type': 'internal_crate',
                    'full_import': f"use {import_path}::*;"
                })
            else:
                # Other wildcards need inspection
                results['risky_wildcards'].append({
                    'path': import_path,
                    'type': 'other',
                    'full_import': f"use {import_path}::*;",
                    'reason': 'Unknown import type - needs inspection'
                })
        
        return results
        
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return {'risky_wildcards': [], 'safe_wildcards': [], 'suggestions': []}

def check_recent_wildcard_conversions(file_path: str) -> list:
    """Check for recent wildcard conversions that might be from our script."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Look for patterns that suggest recent conversion from multi-import
        risky_patterns = []
        
        # Pattern 1: rand::* imports (likely converted from {Rng, SeedableRng})
        if 'use rand::*;' in content:
            risky_patterns.append({
                'pattern': 'use rand::*;',
                'suggestion': 'Consider reverting to specific imports like {Rng, SeedableRng}',
                'reason': 'rand::* imports many items that could conflict'
            })
        
        # Pattern 2: std library wildcards
        std_wildcards = re.findall(r'use (std::[^:]+)::\*;', content)
        for std_path in std_wildcards:
            risky_patterns.append({
                'pattern': f'use {std_path}::*;',
                'suggestion': f'Consider specific imports from {std_path}',
                'reason': 'Standard library wildcards can cause name conflicts'
            })
        
        return risky_patterns
        
    except Exception as e:
        print(f"Error checking {file_path}: {e}")
        return []

def main():
    """Analyze wildcard imports for safety."""
    print("🔍 ANALYZING WILDCARD IMPORT SAFETY")
    print("=" * 45)
    
    directories = ['src', 'tests', 'benches']
    total_files = 0
    risky_files = 0
    all_risky_imports = []
    
    for directory in directories:
        if not os.path.exists(directory):
            continue
            
        print(f"\n📁 Analyzing {directory}/...")
        
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    total_files += 1
                    
                    # Check wildcard safety
                    wildcard_analysis = analyze_wildcard_imports(file_path)
                    recent_conversions = check_recent_wildcard_conversions(file_path)
                    
                    if wildcard_analysis['risky_wildcards'] or recent_conversions:
                        risky_files += 1
                        all_risky_imports.append((file_path, wildcard_analysis, recent_conversions))
                        
                        print(f"\n⚠️  {file_path}")
                        
                        if wildcard_analysis['risky_wildcards']:
                            print(f"   Risky wildcards:")
                            for risky in wildcard_analysis['risky_wildcards']:
                                print(f"     - {risky['full_import']} ({risky['type']})")
                                print(f"       {risky['reason']}")
                        
                        if recent_conversions:
                            print(f"   Recent conversions to review:")
                            for conversion in recent_conversions:
                                print(f"     - {conversion['pattern']}")
                                print(f"       {conversion['suggestion']}")
    
    print(f"\n📊 SAFETY ANALYSIS SUMMARY")
    print("=" * 30)
    print(f"Files analyzed: {total_files}")
    print(f"Files with risky wildcards: {risky_files}")
    
    if all_risky_imports:
        print(f"\n🎯 RECOMMENDATIONS:")
        print("-" * 20)
        
        # Count by type
        std_lib_count = 0
        external_count = 0
        rand_count = 0
        
        for file_path, analysis, conversions in all_risky_imports:
            for risky in analysis['risky_wildcards']:
                if risky['type'] == 'std_library':
                    std_lib_count += 1
                elif risky['type'] == 'external_crate':
                    external_count += 1
            
            for conversion in conversions:
                if 'rand::*' in conversion['pattern']:
                    rand_count += 1
        
        if rand_count > 0:
            print(f"\n🔧 HIGH PRIORITY: {rand_count} rand::* imports")
            print("   These should likely be reverted to specific imports:")
            print("   use rand::{Rng, SeedableRng}; // instead of use rand::*;")
        
        if std_lib_count > 0:
            print(f"\n⚠️  MEDIUM PRIORITY: {std_lib_count} std library wildcards")
            print("   Consider reverting to specific imports if name conflicts occur")
        
        if external_count > 0:
            print(f"\n⚠️  REVIEW: {external_count} other external crate wildcards")
            print("   May be safe, but review for potential conflicts")
        
        print(f"\n💡 GENERAL RULE:")
        print("   ✅ Internal crate wildcards (use crate::Module::*) - Usually safe")
        print("   ⚠️  External crate wildcards (use rand::*) - Often risky") 
        print("   ❌ Standard library wildcards (use std::*) - Usually risky")
        
    else:
        print(f"\n✅ All wildcard imports appear safe!")
        print("   No risky standard library or external crate wildcards found.")

if __name__ == "__main__":
    main()
