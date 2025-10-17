#!/usr/bin/env python3
"""
Script to rename ArraySeqS! macro calls to ArraySeqSLit! for consistency.
Usage: python3 fix_arrayseq_macro.py <file1> <file2> ...
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
import re
from pathlib import Path

def fix_arrayseq_macro_in_file(file_path):
    """Replace ArraySeqS! with ArraySeqSLit! in the given file."""
    path = Path(file_path)
    
    if not path.exists():
        print(f"Error: File {file_path} does not exist")
        return False
    
    try:
        # Read the file
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Count occurrences before replacement
        original_count = content.count('ArraySeqS!')
        
        if original_count == 0:
            print(f"No ArraySeqS! occurrences found in {file_path}")
            return True
        
        # Replace ArraySeqS! with ArraySeqSLit!
        new_content = content.replace('ArraySeqS!', 'ArraySeqSLit!')
        
        # Verify the replacement worked
        new_count = new_content.count('ArraySeqSLit!')
        remaining_old = new_content.count('ArraySeqS!')
        
        if remaining_old > 0:
            print(f"Warning: {remaining_old} ArraySeqS! occurrences still remain in {file_path}")
        
        # Write the file back
        with open(path, 'w', encoding='utf-8') as f:
            f.write(new_content)
        
        print(f"✅ {file_path}: Replaced {original_count} occurrences of ArraySeqS! with ArraySeqSLit!")
        return True
        
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
        return False

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 fix_arrayseq_macro.py <file1> <file2> ...")
        print("Example: python3 fix_arrayseq_macro.py tests/Chap18/TestArraySeqMacro.rs src/Chap18/ArraySeq.rs")
        sys.exit(1)
    
    files_to_process = sys.argv[1:]
    success_count = 0
    
    print(f"Processing {len(files_to_process)} files...")
    print()
    
    for file_path in files_to_process:
        if fix_arrayseq_macro_in_file(file_path):
            success_count += 1
        print()
    
    print(f"Summary: {success_count}/{len(files_to_process)} files processed successfully")
    
    if success_count == len(files_to_process):
        print("🎉 All files processed successfully!")
        sys.exit(0)
    else:
        print("❌ Some files failed to process")
        sys.exit(1)

if __name__ == "__main__":
    main()
