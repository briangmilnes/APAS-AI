#!/usr/bin/env python3
"""
Script to fix UFCS delegation calls that should call imported aliases, not current trait.
This fixes the recursive call issue created by the previous script.
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path

def fix_recursive_trait_calls(file_path: str) -> list:
    """Fix recursive trait calls by restoring delegation to imported aliases."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        original_content = content
        changes_made = []
        
        # Check if this is a Chap19 file with the delegation pattern
        if 'Chap19' not in file_path:
            return []
        
        # Find the trait implementation and alias import
        if 'ArraySeqStPer' in file_path:
            # ArraySeqStPerTrait methods should delegate to ArraySeqStPerTraitChap18
            trait_pattern = r'impl<[^>]*>\s+ArraySeqStPerTrait<[^>]*>\s+for\s+ArraySeqStPerS<[^>]*>'
            if re.search(trait_pattern, content):
                # Within this impl block, calls to ArraySeqStPerTrait:: should be ArraySeqStPerTraitChap18::
                # But only for simple delegation methods like length, nth
                delegation_methods = ['length', 'nth', 'subseq_copy']
                
                for method in delegation_methods:
                    # Pattern: fn method(...) { ArraySeqStPerTrait::method(...) }
                    recursive_pattern = rf'fn\s+{method}\s*\([^{{]*\{{\s*ArraySeqStPerTrait::{method}\s*\([^}}]*\)\s*\}}'
                    
                    def replace_recursive_call(match):
                        method_def = match.group(0)
                        # Replace ArraySeqStPerTrait:: with ArraySeqStPerTraitChap18::
                        fixed_method = method_def.replace('ArraySeqStPerTrait::', 'ArraySeqStPerTraitChap18::')
                        return fixed_method
                    
                    new_content = re.sub(recursive_pattern, replace_recursive_call, content, flags=re.MULTILINE | re.DOTALL)
                    if new_content != content:
                        content = new_content
                        changes_made.append(f"Fixed recursive {method} call to delegate to Chap18")
        
        elif 'ArraySeqMtPer' in file_path:
            trait_pattern = r'impl<[^>]*>\s+ArraySeqMtPerTrait<[^>]*>\s+for\s+ArraySeqMtPerS<[^>]*>'
            if re.search(trait_pattern, content):
                delegation_methods = ['length', 'nth']
                
                for method in delegation_methods:
                    recursive_pattern = rf'fn\s+{method}\s*\([^{{]*\{{\s*ArraySeqMtPerTrait::{method}\s*\([^}}]*\)\s*\}}'
                    
                    def replace_recursive_call(match):
                        method_def = match.group(0)
                        fixed_method = method_def.replace('ArraySeqMtPerTrait::', 'ArraySeqMtPerTraitChap18::')
                        return fixed_method
                    
                    new_content = re.sub(recursive_pattern, replace_recursive_call, content, flags=re.MULTILINE | re.DOTALL)
                    if new_content != content:
                        content = new_content
                        changes_made.append(f"Fixed recursive {method} call to delegate to Chap18")
        
        elif 'ArraySeqMtEph' in file_path:
            trait_pattern = r'impl<[^>]*>\s+ArraySeqMtEphTrait<[^>]*>\s+for\s+ArraySeqMtEphS<[^>]*>'
            if re.search(trait_pattern, content):
                delegation_methods = ['length', 'nth_cloned', 'subseq_copy']
                
                for method in delegation_methods:
                    recursive_pattern = rf'fn\s+{method}\s*\([^{{]*\{{\s*ArraySeqMtEphTrait::{method}\s*\([^}}]*\)\s*\}}'
                    
                    def replace_recursive_call(match):
                        method_def = match.group(0)
                        fixed_method = method_def.replace('ArraySeqMtEphTrait::', 'ArraySeqMtEphTraitChap18::')
                        return fixed_method
                    
                    new_content = re.sub(recursive_pattern, replace_recursive_call, content, flags=re.MULTILINE | re.DOTALL)
                    if new_content != content:
                        content = new_content
                        changes_made.append(f"Fixed recursive {method} call to delegate to Chap18")
        
        elif 'ArraySeqStEph' in file_path:
            trait_pattern = r'impl<[^>]*>\s+ArraySeqStEphTrait<[^>]*>\s+for\s+ArraySeqStEphS<[^>]*>'
            if re.search(trait_pattern, content):
                delegation_methods = ['length', 'nth']
                
                for method in delegation_methods:
                    recursive_pattern = rf'fn\s+{method}\s*\([^{{]*\{{\s*ArraySeqStEphTrait::{method}\s*\([^}}]*\)\s*\}}'
                    
                    def replace_recursive_call(match):
                        method_def = match.group(0)
                        fixed_method = method_def.replace('ArraySeqStEphTrait::', 'ArraySeqStEphTraitChap18::')
                        return fixed_method
                    
                    new_content = re.sub(recursive_pattern, replace_recursive_call, content, flags=re.MULTILINE | re.DOTALL)
                    if new_content != content:
                        content = new_content
                        changes_made.append(f"Fixed recursive {method} call to delegate to Chap18")
        
        if content != original_content:
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            return changes_made
        else:
            return []
            
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return []

def main():
    """Fix recursive delegation calls in Chap19 files."""
    print("🔄 FIXING RECURSIVE TRAIT DELEGATION CALLS")
    print("=" * 50)
    
    chap19_files = [
        'src/Chap19/ArraySeqStPer.rs',
        'src/Chap19/ArraySeqMtPer.rs', 
        'src/Chap19/ArraySeqMtEph.rs',
        'src/Chap19/ArraySeqStEph.rs'
    ]
    
    total_changes = 0
    
    for file_path in chap19_files:
        if os.path.exists(file_path):
            changes = fix_recursive_trait_calls(file_path)
            if changes:
                total_changes += len(changes)
                print(f"✅ {file_path}")
                for change in changes:
                    print(f"   - {change}")
        else:
            print(f"⚠️  {file_path} not found")
    
    print(f"\n📊 SUMMARY:")
    print(f"   Total fixes applied: {total_changes}")
    
    if total_changes > 0:
        print(f"\n✅ Fixed recursive delegation calls!")
        print(f"🏗️  Run 'cargo build' to verify no more recursion warnings.")
    else:
        print(f"\n✅ No recursive delegation issues found.")

if __name__ == "__main__":
    main()
