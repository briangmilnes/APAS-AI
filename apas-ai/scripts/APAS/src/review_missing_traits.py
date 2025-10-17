#!/usr/bin/env python3
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import os
import re
from pathlib import Path

def has_data_structure_and_impl(file_path):
    """Check if file has a data structure (struct/enum) with an impl block"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
            
            # Look for pub struct or pub enum
            has_data_structure = bool(re.search(r'pub\s+(struct|enum)\s+\w+', content))
            
            # Look for impl block for that data structure
            has_impl = bool(re.search(r'impl.*\{', content))
            
            return has_data_structure and has_impl
    except:
        return False

def has_trait_for_data_structure(file_path):
    """Check if file has a trait (not just documentary trait)"""
    try:
        with open(file_path, 'r') as f:
            content = f.read()
            # Look for any pub trait pattern
            return bool(re.search(r'pub trait\s+\w+', content))
    except:
        return False

def is_example_or_exercise_file(file_path):
    """Check if this is an Example or Exercise file"""
    filename = os.path.basename(file_path)
    return ('Example' in filename or 'Exercise' in filename or 'Problem' in filename)

def analyze_src_directory():
    """Find all .rs files in src/ that have data structures with impls but no traits"""
    src_dir = Path('src')
    missing_traits = []
    
    for rs_file in src_dir.rglob('*.rs'):
        if rs_file.name == 'lib.rs' or rs_file.name == 'Types.rs':
            continue
            
        file_path = str(rs_file)
        
        # Check if it has a data structure with impl
        if has_data_structure_and_impl(file_path):
            # Check if it has a trait
            if not has_trait_for_data_structure(file_path):
                # Check if it's an example/exercise file
                is_example = is_example_or_exercise_file(file_path)
                missing_traits.append((file_path, is_example))
    
    return missing_traits

def main():
    missing_traits = analyze_src_directory()
    
    # Separate into categories
    examples_exercises = [f for f, is_ex in missing_traits if is_ex]
    algorithms_data_structures = [f for f, is_ex in missing_traits if not is_ex]
    
    print("=== DATA STRUCTURES WITH IMPLS BUT NO TRAITS ===\n")
    
    print(f"EXAMPLES/EXERCISES/PROBLEMS ({len(examples_exercises)} files):")
    for file_path in sorted(examples_exercises):
        print(f"  {file_path}")
    
    print(f"\nDATA STRUCTURES ({len(algorithms_data_structures)} files):")
    for file_path in sorted(algorithms_data_structures):
        print(f"  {file_path}")
    
    print(f"\nTOTAL: {len(missing_traits)} files with data structures missing traits")
    
    # Show the specific example mentioned
    example_file = "src/Chap65/UnionFindStEph.rs"
    if example_file in [f for f, _ in missing_traits]:
        print(f"\n✓ Confirmed: {example_file} has data structure with impl but no trait")

if __name__ == "__main__":
    main()
