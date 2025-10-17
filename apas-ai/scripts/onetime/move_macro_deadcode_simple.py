#!/usr/bin/env python3
"""
Simple script to move specific dead code functions from src/Types.rs to tests/TestTypes.rs
"""
# Git commit: 584a672b6a34782766863c5f76a461d3297a741a
# Date: 2025-10-17 05:17:36 -0700


import sys
from pathlib import Path

def move_types_deadcode():
    """Move dead code functions from src/Types.rs to tests/TestTypes.rs"""
    
    # Read src/Types.rs
    src_path = Path("src/Types.rs")
    with open(src_path, 'r') as f:
        src_content = f.read()
    
    # Define the exact functions to move
    functions_to_move = [
        '''    #[allow(dead_code)]
    fn _ParaPair_type_checks() {
        let Pair(left, right) = ParaPair!(|| { 1usize }, || { 2usize });
        let _: usize = left;
        let _: usize = right;
    }''',
        
        '''    #[allow(dead_code)]
    fn _EdgeLit_type_checks() {
        let _ = EdgeLit!(1, 2); // non-empty infers (e.g., i32)
        let _: Edge<i32> = EdgeLit!(1, 2); // explicit type
    }''',
        
        '''    #[allow(dead_code)]
    fn _PairLit_type_checks() {
        let _ = PairLit!(1, 2); // non-empty infers (e.g., i32)
        let _: Pair<i32, i32> = PairLit!(1, 2); // explicit type
    }''',
        
        '''    #[allow(dead_code)]
    fn _EdgeList_type_checks() {
        let _ = EdgeList![(1, 2), (3, 4)]; // non-empty infers
        let _: Vec<Edge<i32>> = EdgeList![(1, 2), (3, 4)]; // explicit type
    }''',
        
        '''    #[allow(dead_code)]
    fn _PairList_type_checks() {
        let _ = PairList![(1, 2), (3, 4)]; // non-empty infers
        let _: Vec<Pair<i32, i32>> = PairList![(1, 2), (3, 4)]; // explicit type
    }'''
    ]
    
    # Convert to test functions
    test_functions = []
    for func in functions_to_move:
        test_func = func.replace('#[allow(dead_code)]', '#[test]')
        test_func = test_func.replace('fn _', 'fn test_')
        test_functions.append(test_func)
    
    # Remove from source file
    new_src_content = src_content
    for func in functions_to_move:
        new_src_content = new_src_content.replace(func + '\n\n', '')
        new_src_content = new_src_content.replace(func + '\n', '')
        new_src_content = new_src_content.replace(func, '')
    
    # Write updated source file
    with open(src_path, 'w') as f:
        f.write(new_src_content)
    
    # Read existing test file
    test_path = Path("tests/TestTypes.rs")
    with open(test_path, 'r') as f:
        test_content = f.read()
    
    # Add test functions to test file
    for test_func in test_functions:
        test_content += '\n' + test_func + '\n'
    
    # Write test file
    with open(test_path, 'w') as f:
        f.write(test_content)
    
    print(f"✅ Moved {len(functions_to_move)} dead code functions from src/Types.rs to tests/TestTypes.rs")
    return True

def main():
    print("Moving dead code macro tests from src/Types.rs...")
    print()
    
    if move_types_deadcode():
        print("🎉 Successfully moved dead code tests!")
        sys.exit(0)
    else:
        print("❌ Failed to move dead code tests")
        sys.exit(1)

if __name__ == "__main__":
    main()
