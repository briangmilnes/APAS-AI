#!/usr/bin/env python3
"""
Script to check for any remaining UFCS alias usage that needs fixing.
Specifically looks for patterns where imported aliases are used in UFCS calls.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path

def check_alias_usage_in_file(file_path: str) -> dict:
    """Check if aliases are being used in UFCS calls in a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        results = {
            'aliases': {},
            'ufcs_calls': [],
            'potential_issues': []
        }
        
        # Extract aliases: {alias_name: original_name}
        alias_pattern = r'use\s+[^;]*\{[^}]*\b(\w+)\s+as\s+(\w+)[^}]*\}[^;]*;'
        alias_matches = re.findall(alias_pattern, content)
        
        for original, alias in alias_matches:
            results['aliases'][alias] = original
        
        if not results['aliases']:
            return results
        
        # Look for UFCS calls using any of the aliases
        for alias, original in results['aliases'].items():
            # Pattern: AliasName::method_name
            ufcs_pattern = rf'\b{re.escape(alias)}::'
            ufcs_matches = re.findall(ufcs_pattern, content)
            
            if ufcs_matches:
                results['ufcs_calls'].extend([(alias, original, match) for match in ufcs_matches])
                
                # Find the specific lines with these calls
                lines = content.split('\n')
                for line_num, line in enumerate(lines, 1):
                    if re.search(ufcs_pattern, line):
                        results['potential_issues'].append({
                            'line': line_num,
                            'content': line.strip(),
                            'alias': alias,
                            'original': original
                        })
        
        return results
        
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return {'aliases': {}, 'ufcs_calls': [], 'potential_issues': []}

def main():
    """Check all Rust files for remaining alias UFCS usage."""
    print("🔍 CHECKING FOR REMAINING ALIAS UFCS USAGE")
    print("=" * 50)
    
    directories = ['src', 'tests', 'benches']
    total_files_with_issues = 0
    total_issues = 0
    
    for directory in directories:
        if not os.path.exists(directory):
            continue
            
        print(f"\n📁 Checking {directory}/...")
        dir_files_with_issues = 0
        dir_issues = 0
        
        for root, dirs, files in os.walk(directory):
            for file in files:
                if file.endswith('.rs'):
                    file_path = os.path.join(root, file)
                    
                    results = check_alias_usage_in_file(file_path)
                    
                    if results['potential_issues']:
                        dir_files_with_issues += 1
                        total_files_with_issues += 1
                        dir_issues += len(results['potential_issues'])
                        total_issues += len(results['potential_issues'])
                        
                        print(f"\n⚠️  {file_path}")
                        print(f"   Aliases found: {results['aliases']}")
                        
                        for issue in results['potential_issues']:
                            print(f"   Line {issue['line']}: {issue['content']}")
                            print(f"      → Using alias '{issue['alias']}' (original: '{issue['original']}')")
        
        print(f"\n📊 {directory} summary: {dir_files_with_issues} files, {dir_issues} potential issues")
    
    print(f"\n🎯 OVERALL SUMMARY")
    print("=" * 30)
    print(f"Files with potential issues: {total_files_with_issues}")
    print(f"Total potential issues: {total_issues}")
    
    if total_issues == 0:
        print(f"\n✅ No remaining alias UFCS usage found!")
        print(f"All alias patterns are properly handled.")
    else:
        print(f"\n⚠️  Found {total_issues} potential alias UFCS usages to review.")
        print(f"These may need manual inspection to determine if they should be changed.")

if __name__ == "__main__":
    main()
