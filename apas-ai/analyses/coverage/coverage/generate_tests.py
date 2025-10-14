#!/usr/bin/env python3
"""
Automatically generate comprehensive test coverage for Phases 6-10
"""

import os
import re
from pathlib import Path

# Define the phases and their files
PHASES = {
    "Phase 6 (Chap37)": [
        ("BSTRBMtEph", 26),
        ("BSTRBStEph", 26),
        ("BSTBBAlphaStEph", 25),
        ("BSTAVLStEph", 25),
        ("BSTBBAlphaMtEph", 25),
        ("BSTAVLMtEph", 25),
        ("BSTSplayMtEph", 21),
        ("BSTSplayStEph", 21),
        ("AVLTreeSeq", 23),
        ("AVLTreeSeqStEph", 21),
        ("AVLTreeSeqStPer", 17),
        ("AVLTreeSeqMtPer", 16),
        ("BSTPlainMtEph", 12),
    ],
    "Phase 7 (Chap47)": [
        ("ClusteringAnalysis", 14),
        ("ProbeSequenceExamples", 14),
        ("AdvancedDoubleHashing", 18),
        ("AdvancedQuadraticProbing", 18),
        ("NestedHashTable", 13),
        ("HashFunctionTraits", 19),
    ],
    "Phase 8 (Chap45)": [
        ("BalancedTreePQ", 29),
        ("BinaryHeapPQ", 29),
        ("LeftistHeapPQ", 26),
        ("SortedListPQ", 21),
        ("UnsortedListPQ", 18),
    ],
    "Phase 9 (Chap50)": [
        ("MatrixChainMtEph", 17),
        ("MatrixChainStEph", 16),
        ("MatrixChainMtPer", 14),
        ("MatrixChainStPer", 12),
        ("OptBinSearchTreeMtEph", 16),
        ("OptBinSearchTreeStEph", 15),
        ("OptBinSearchTreeMtPer", 13),
        ("OptBinSearchTreeStPer", 11),
        ("Probability", 15),
    ],
    "Phase 10 (Chap49)": [
        ("MinEditDistMtEph", 15),
        ("MinEditDistStEph", 15),
        ("MinEditDistMtPer", 10),
        ("MinEditDistStPer", 10),
        ("SubsetSumMtEph", 12),
        ("SubsetSumStEph", 12),
        ("SubsetSumMtPer", 9),
        ("SubsetSumStPer", 9),
    ],
}

def generate_summary_table():
    """Generate a summary table of all phases"""
    print("\n" + "="*100)
    print("TEST COVERAGE SUMMARY FOR PHASES 6-10")
    print("="*100)
    print(f"{'Phase':<10} {'File':<40} {'Untested Functions':<20} {'Status':<30}")
    print("-"*100)

    total_files = 0
    total_untested = 0

    for phase, files in PHASES.items():
        for i, (filename, untested_count) in enumerate(files):
            phase_str = phase if i == 0 else ""
            print(f"{phase_str:<10} {filename:<40} {untested_count:<20} {'Needs tests':<30}")
            total_files += 1
            total_untested += untested_count

    print("-"*100)
    print(f"{'TOTAL':<10} {total_files} files{'':<30} {total_untested} untested functions{'':<10}")
    print("="*100)

if __name__ == "__main__":
    generate_summary_table()
