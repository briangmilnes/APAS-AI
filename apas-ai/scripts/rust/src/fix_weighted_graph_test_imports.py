#!/usr/bin/env python3
"""
Fix test and benchmark file imports for converted weighted graph types.
Changes WeightedXXX::WeightedXXX to WeightedXXX::* to import traits.
"""

import sys
import re

test_files = [
    "tests/Chap06/TestWeightedDirGraphMtEphFloat.rs",
    "tests/Chap06/TestWeightedDirGraphMtEphInt.rs",
    "tests/Chap06/TestWeightedDirGraphStEphFloat.rs",
    "tests/Chap06/TestWeightedDirGraphStEphInt.rs",
    "tests/Chap06/TestWeightedUnDirGraphMtEphFloat.rs",
    "tests/Chap06/TestWeightedUnDirGraphMtEphInt.rs",
    "tests/Chap06/TestWeightedUnDirGraphStEphFloat.rs",
    "tests/Chap06/TestWeightedUnDirGraphStEphInt.rs",
    "tests/Chap57/TestDijkstraStEphFloat.rs",
    "tests/Chap57/TestDijkstraStEphInt.rs",
    "tests/Chap58/TestBellmanFordStEphFloat.rs",
    "tests/Chap58/TestBellmanFordStEphInt.rs",
    "tests/Chap59/TestJohnsonMtEphFloat.rs",
    "tests/Chap59/TestJohnsonMtEphInt.rs",
    "tests/Chap59/TestJohnsonStEphFloat.rs",
    "tests/Chap59/TestJohnsonStEphInt.rs",
    "benches/Chap06/BenchWeightedDirGraphMtEphFloat.rs",
    "benches/Chap06/BenchWeightedDirGraphMtEphInt.rs",
    "benches/Chap06/BenchWeightedDirGraphStEphFloat.rs",
    "benches/Chap06/BenchWeightedDirGraphStEphInt.rs",
    "benches/Chap06/BenchWeightedUnDirGraphMtEphFloat.rs",
    "benches/Chap06/BenchWeightedUnDirGraphMtEphInt.rs",
    "benches/Chap06/BenchWeightedUnDirGraphStEphFloat.rs",
    "benches/Chap06/BenchWeightedUnDirGraphStEphInt.rs",
    "benches/Chap57/BenchDijkstraStEphFloat.rs",
    "benches/Chap57/BenchDijkstraStEphInt.rs",
    "benches/Chap58/BenchBellmanFordStEphFloat.rs",
    "benches/Chap58/BenchBellmanFordStEphInt.rs",
    "benches/Chap59/BenchJohnsonMtEphFloat.rs",
    "benches/Chap59/BenchJohnsonMtEphInt.rs",
    "benches/Chap59/BenchJohnsonStEphFloat.rs",
    "benches/Chap59/BenchJohnsonStEphInt.rs",
]

weighted_types = [
    "WeightedDirGraphMtEphFloat",
    "WeightedDirGraphMtEphInt",
    "WeightedDirGraphStEphFloat",
    "WeightedDirGraphStEphInt",
    "WeightedUnDirGraphMtEphFloat",
    "WeightedUnDirGraphMtEphInt",
    "WeightedUnDirGraphStEphFloat",
    "WeightedUnDirGraphStEphInt",
]

def fix_file(filepath):
    with open(filepath, 'r') as f:
        content = f.read()
    
    original = content
    
    for typ in weighted_types:
        # Pattern: use apas_ai::Chap06::TypeName::TypeName::TypeName;
        # Replace with: use apas_ai::Chap06::TypeName::TypeName::*;
        pattern = f"use apas_ai::Chap06::{typ}::{typ}::{typ};"
        replacement = f"use apas_ai::Chap06::{typ}::{typ}::*;"
        content = content.replace(pattern, replacement)
    
    if content != original:
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"✓ Fixed: {filepath}")
        return True
    else:
        print(f"  Skipped (no changes): {filepath}")
        return False

def main():
    fixed_count = 0
    for filepath in test_files:
        if fix_file(filepath):
            fixed_count += 1
    
    print(f"\n✓ Fixed {fixed_count}/{len(test_files)} test/benchmark files")

if __name__ == "__main__":
    main()

