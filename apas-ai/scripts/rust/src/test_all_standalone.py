#!/usr/bin/env python3
"""
Test which files have standalone trait impls by commenting out inherent impls.
Git commit: 25ae22c50a0fcef6ba643cf969f9c755e1f73eab
Date: 2025-10-18
"""

import subprocess
import sys
from pathlib import Path

# Files to test
FILES = [
    "src/Chap37/AVLTreeSeq.rs",
    "src/Chap37/AVLTreeSeqStEph.rs",
    "src/Chap37/BSTAVLMtEph.rs",
    "src/Chap37/BSTAVLStEph.rs",
    "src/Chap37/BSTBBAlphaMtEph.rs",
    "src/Chap37/BSTBBAlphaStEph.rs",
    "src/Chap37/BSTRBMtEph.rs",
    "src/Chap37/BSTRBStEph.rs",
    "src/Chap37/BSTSetAVLMtEph.rs",
    "src/Chap37/BSTSetBBAlphaMtEph.rs",
    "src/Chap37/BSTSetPlainMtEph.rs",
    "src/Chap37/BSTSetRBMtEph.rs",
    "src/Chap37/BSTSetSplayMtEph.rs",
    "src/Chap37/BSTSplayMtEph.rs",
    "src/Chap37/BSTSplayStEph.rs",
    "src/Chap38/BSTParaMtEph.rs",
    "src/Chap38/BSTParaStEph.rs",
    "src/Chap39/BSTParaTreapMtEph.rs",
    "src/Chap39/BSTSetTreapMtEph.rs",
    "src/Chap39/BSTTreapMtEph.rs",
    "src/Chap39/BSTTreapStEph.rs",
    "src/Chap40/BSTKeyValueStEph.rs",
    "src/Chap40/BSTSizeStEph.rs",
    "src/Chap44/DocumentIndex.rs",
    "src/Chap45/BalancedTreePQ.rs",
    "src/Chap45/BinaryHeapPQ.rs",
    "src/Chap45/LeftistHeapPQ.rs",
    "src/Chap45/SortedListPQ.rs",
    "src/Chap45/UnsortedListPQ.rs",
    "src/Chap47/HashFunctionTraits.rs",
    "src/Chap49/MinEditDistMtEph.rs",
    "src/Chap49/MinEditDistMtPer.rs",
    "src/Chap49/MinEditDistStEph.rs",
    "src/Chap49/MinEditDistStPer.rs",
    "src/Chap49/SubsetSumMtEph.rs",
    "src/Chap49/SubsetSumMtPer.rs",
    "src/Chap49/SubsetSumStEph.rs",
    "src/Chap49/SubsetSumStPer.rs",
    "src/Chap50/MatrixChainMtEph.rs",
    "src/Chap50/MatrixChainMtPer.rs",
    "src/Chap50/MatrixChainStEph.rs",
    "src/Chap50/MatrixChainStPer.rs",
    "src/Chap50/OptBinSearchTreeMtEph.rs",
    "src/Chap50/OptBinSearchTreeMtPer.rs",
    "src/Chap50/OptBinSearchTreeStEph.rs",
    "src/Chap50/OptBinSearchTreeStPer.rs",
]

def main():
    print("Testing which trait impls are standalone...")
    print("=" * 80)
    
    standalone = []
    needs_fix = []
    
    for file_path in FILES:
        print(f"\nTesting: {file_path}")
        
        # Read file
        try:
            with open(file_path, 'r') as f:
                content = f.read()
        except Exception as e:
            print(f"  ✗ Error reading: {e}")
            continue
        
        # Find inherent impl
        import re
        # Look for impl<...> StructName { or impl StructName {
        inherent_pattern = r'impl(?:<[^>]+>)?\s+(\w+)(?:<[^>]+>)?\s*\{'
        
        found_inherent = False
        for match in re.finditer(inherent_pattern, content):
            # Check if this is NOT a trait impl (no "for" keyword before it)
            before = content[max(0, match.start() - 100):match.start()]
            if ' for ' not in before[-50:]:
                found_inherent = True
                break
        
        if not found_inherent:
            print(f"  ℹ No inherent impl found")
            standalone.append((file_path, "No inherent impl"))
            continue
        
        # Comment out inherent impl
        modified = content
        for match in re.finditer(inherent_pattern, content):
            before = content[max(0, match.start() - 100):match.start()]
            if ' for ' not in before[-50:]:
                # This is an inherent impl
                start = match.start()
                
                # Find matching closing brace
                brace_count = 1
                i = match.end()
                while i < len(content) and brace_count > 0:
                    if content[i] == '{':
                        brace_count += 1
                    elif content[i] == '}':
                        brace_count -= 1
                    i += 1
                
                impl_block = content[start:i]
                commented = '// TESTING\n// ' + '\n// '.join(impl_block.split('\n'))
                modified = content[:start] + commented + content[i:]
                break
        
        # Write temporary file
        try:
            with open(file_path, 'w') as f:
                f.write(modified)
        except Exception as e:
            print(f"  ✗ Error writing: {e}")
            continue
        
        # Test compilation
        result = subprocess.run(
            ['cargo', 'check', '--lib', '--message-format', 'short'],
            capture_output=True,
            text=True
        )
        
        # Restore original
        try:
            with open(file_path, 'w') as f:
                f.write(content)
        except Exception as e:
            print(f"  ✗ Error restoring: {e}")
        
        if result.returncode == 0:
            print(f"  ✓ STANDALONE")
            standalone.append((file_path, "Standalone"))
        else:
            # Extract first error
            errors = [line for line in result.stderr.split('\n') if 'error[' in line]
            error_msg = errors[0] if errors else "Unknown error"
            print(f"  ✗ NEEDS FIX: {error_msg[:80]}")
            needs_fix.append((file_path, error_msg))
    
    print("\n" + "=" * 80)
    print(f"\nRESULTS:")
    print(f"  Standalone: {len(standalone)}")
    print(f"  Needs fix: {len(needs_fix)}")
    
    if standalone:
        print(f"\n✓ STANDALONE FILES ({len(standalone)}):")
        for file, note in standalone:
            print(f"  {file}")
    
    if needs_fix:
        print(f"\n✗ NEEDS FIX ({len(needs_fix)}):")
        for file, error in needs_fix:
            print(f"  {file}")
            print(f"    {error[:100]}")

if __name__ == '__main__':
    main()

