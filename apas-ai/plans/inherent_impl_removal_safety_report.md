# Inherent Impl Removal: Safety Analysis Report

**Date**: 2025-10-18  
**Analysis Tool**: `scripts/rust/src/analyze_safe_inherent_removals.py`

## Summary

| Category | Count | Action Required |
|----------|-------|-----------------|
| **SAFE** | 17 | Automated removal - all public methods in trait |
| **NEEDS_REVIEW** | 44 | Manual review - has private helper methods |
| **UNSAFE** | 9 | Extend trait first - has public methods not in trait |
| **TOTAL** | 70 | (includes multiple impls per file) |

## Phase 1: SAFE TO REMOVE (17 files)

These can be **automatically removed** with the fix script:

1. ✅ `src/Chap18/ArraySeqMtPer.rs` - ArraySeqMtPerS (10 methods)
2. ✅ `src/Chap18/ArraySeqStEph.rs` - ArraySeqStEphS (11 methods)
3. ✅ `src/Chap18/ArraySeqStPer.rs` - ArraySeqStPerS (8 methods)
4. ✅ `src/Chap23/BalBinTreeStEph.rs` - BalBinTree (7 methods)
5. ✅ `src/Chap23/PrimTreeSeqSt.rs` - PrimTreeSeqStS (8 methods)
6. ✅ `src/Chap37/AVLTreeSeq.rs` - AVLTreeS (12 methods)
7. ✅ `src/Chap37/AVLTreeSeqStEph.rs` - AVLTreeSeqStEphS (10 methods)
8. ✅ `src/Chap37/AVLTreeSeqStPer.rs` - AVLTreeSeqStPerS (2 methods)
9. ✅ `src/Chap44/DocumentIndex.rs` - DocumentIndex (3 methods)
10. ✅ `src/Chap45/BalancedTreePQ.rs` - BalancedTreePQ (7 methods) - has other unsafe impls
11. ✅ `src/Chap45/LeftistHeapPQ.rs` - LeftistHeapPQ (9 methods)
12. ✅ `src/Chap45/SortedListPQ.rs` - SortedListPQ (4 methods, appears twice)
13. ✅ `src/Chap45/UnsortedListPQ.rs` - UnsortedListPQ (2+3 methods, appears twice)
14. ✅ `src/Chap47clean/StructChainedHashTable.rs` - ChainList (1 method)
15. ✅ `src/Chap65/UnionFindStEph.rs` - UnionFindStEph (6 methods)

## Phase 2: NEEDS REVIEW (44 files)

These have **private helper methods** that need to be moved to trait impl or made module-level:

### Chapter 18: Sequences (3 files)
- `src/Chap18/ArraySeq.rs` - ArraySeqS: 12 private (new, set, length, nth, empty, singleton, isEmpty, isSingleton, subseq_copy, from_vec, iter, iter_mut)
- `src/Chap18/LinkedListStEph.rs` - LinkedListStEphS: 2 private (node_at, node_at_mut)
- `src/Chap18/LinkedListStPer.rs` - LinkedListStPerS: 1 private (node_at)

### Chapter 19: ArraySeq Slice (1 file)
- `src/Chap19/ArraySeqMtEphSlice.rs` - ArraySeqMtEphSliceS: 2 private (len, clamp_subrange)

### Chapter 37: BST Variants (15 files)
- `src/Chap37/AVLTreeSeq.rs` - AVLTreeSeqIter: 2 private (new, push_left)
- `src/Chap37/AVLTreeSeqStEph.rs` - AVLTreeSeqIterStEph: 2 private (new, push_left)
- `src/Chap37/AVLTreeSeqStPer.rs` - AVLTreeSeqStPerIter: 1 private (push_left)
- `src/Chap37/BSTAVLMtEph.rs` - BSTAVLMtEph: 12 private helpers
- `src/Chap37/BSTAVLStEph.rs` - BSTAVLStEph: 12 private helpers
- `src/Chap37/BSTBBAlphaMtEph.rs` - BSTBBAlphaMtEph: 12 private helpers
- `src/Chap37/BSTBBAlphaStEph.rs` - BSTBBAlphaStEph: 13 private helpers
- `src/Chap37/BSTRBMtEph.rs` - BSTRBMtEph: 13 private helpers
- `src/Chap37/BSTRBStEph.rs` - BSTRBStEph: 13 private helpers
- `src/Chap37/BSTSetAVLMtEph.rs` - BSTSetAVLMtEph: 3 private (values_vec, rebuild_from_vec, from_sorted_iter)
- `src/Chap37/BSTSetBBAlphaMtEph.rs` - BSTSetBBAlphaMtEph: 3 private
- `src/Chap37/BSTSetPlainMtEph.rs` - BSTSetPlainMtEph: 3 private
- `src/Chap37/BSTSetRBMtEph.rs` - BSTSetRBMtEph: 3 private
- `src/Chap37/BSTSetSplayMtEph.rs` - BSTSetSplayMtEph: 3 private
- `src/Chap37/BSTSplayMtEph.rs` - BSTSplayMtEph: 8 private helpers

### Chapter 38: Parametric BST (2 files)
- `src/Chap38/BSTParaMtEph.rs` - ParamBST: 14 private helpers
- `src/Chap38/BSTParaStEph.rs` - ParamBST: 8 private helpers

### Chapter 39: Treaps (2 files)
- `src/Chap39/BSTSetTreapMtEph.rs` - BSTSetTreapMtEph: 3 private
- `src/Chap39/BSTTreapMtEph.rs` - BSTTreapMtEph: 10 private helpers
- `src/Chap39/BSTTreapStEph.rs` - BSTTreapStEph: 10 private helpers

### Chapter 40: BST Augmentation (2 files)
- `src/Chap40/BSTKeyValueStEph.rs` - BSTKeyValueStEph: 8 private helpers
- `src/Chap40/BSTSizeStEph.rs` - BSTSizeStEph: 13 private helpers

### Chapter 45: Priority Queues (1 file)
- `src/Chap45/BinaryHeapPQ.rs` - BinaryHeapPQ: 8 private helpers (left_child, right_child, parent, is_heap, bubble_up, bubble_down, swap_elements, heapify)

### Chapter 49: Edit Distance / Subset Sum (8 files)
- `src/Chap49/MinEditDistMtEph.rs` - MinEditDistMtEphS: 1 private (min_edit_distance_rec)
- `src/Chap49/MinEditDistMtPer.rs` - MinEditDistMtPerS: 1 private (min_edit_distance_rec)
- `src/Chap49/MinEditDistStEph.rs` - MinEditDistStEphS: 1 private (min_edit_distance_rec)
- `src/Chap49/MinEditDistStPer.rs` - MinEditDistStPerS: 1 private (min_edit_distance_rec)
- `src/Chap49/SubsetSumMtEph.rs` - SubsetSumMtEphS: 1 private (subset_sum_rec)
- `src/Chap49/SubsetSumMtPer.rs` - SubsetSumMtPerS: 1 private (subset_sum_rec)
- `src/Chap49/SubsetSumStEph.rs` - SubsetSumStEphS: 1 private (subset_sum_rec)
- `src/Chap49/SubsetSumStPer.rs` - SubsetSumStPerS: 1 private (subset_sum_rec)

### Chapter 50: Dynamic Programming (8 files)
- `src/Chap50/MatrixChainMtEph.rs` - MatrixChainMtEphS: 3 private helpers
- `src/Chap50/MatrixChainMtPer.rs` - MatrixChainMtPerS: 3 private helpers
- `src/Chap50/MatrixChainStEph.rs` - MatrixChainStEphS: 2 private helpers
- `src/Chap50/MatrixChainStPer.rs` - MatrixChainStPerS: 2 private helpers
- `src/Chap50/OptBinSearchTreeMtEph.rs` - OBSTMtEphS: 2 private helpers
- `src/Chap50/OptBinSearchTreeMtPer.rs` - OBSTMtPerS: 2 private helpers
- `src/Chap50/OptBinSearchTreeStEph.rs` - OBSTStEphS: 1 private (obst_rec)
- `src/Chap50/OptBinSearchTreeStPer.rs` - OBSTStPerS: 1 private (obst_rec)

## Phase 3: UNSAFE TO REMOVE (9 files)

These have **public methods not in trait** - must extend trait first:

1. ⚠️ `src/Chap39/BSTParaTreapMtEph.rs` - ParamTreap: `expose_with_priority` (not in trait)
2. ⚠️ `src/Chap45/BalancedTreePQ.rs` - BalancedTreePQ: `is_sorted`, `height` (not in trait)
3. ⚠️ `src/Chap45/BalancedTreePQ.rs` - BalancedTreePQ: `split`, `join`, `filter`, `map` (not in trait)
4. ⚠️ `src/Chap45/BinaryHeapPQ.rs` - BinaryHeapPQ: `insert_all`, `extract_all_sorted`, `is_valid_heap`, `height`, `level_elements` (not in trait)
5. ⚠️ `src/Chap45/BinaryHeapPQ.rs` - BinaryHeapPQ: `from_vec`, `to_vec`, `to_sorted_vec` (not in trait)
6. ⚠️ `src/Chap47/HashFunctionTraits.rs` - PolynomialHashFunction: `new` (not in trait)
7. ⚠️ `src/Chap47/HashFunctionTraits.rs` - UniversalIntegerHashFunction: `new` (not in trait)
8. ⚠️ `src/Chap47/HashFunctionTraits.rs` - UniversalIntegerHashFamily: `new` (not in trait)
9. ⚠️ `src/Chap50/Probability.rs` - Probability: `new`, `value`, `infinity`, `zero` (not in trait)

### Actions Required for UNSAFE:
1. Add missing methods to traits
2. Update trait signatures
3. Then remove inherent impl

## Execution Strategy

### Immediate (Phase 1): 
Start with the **17 SAFE files** - pure automated removal

### Next (Phase 2a):
Handle **NEEDS_REVIEW with single private method** (8 files in Chapter 49):
- Simple recursive helpers that can be made module-level functions

### Later (Phase 2b):
Handle **NEEDS_REVIEW with multiple private methods**:
- BST implementations with complex private helpers
- May need to keep in trait impl as private methods

### Final (Phase 3):
Extend traits for **UNSAFE files**, then remove inherent impls

## Notes

- Some files appear multiple times (BalancedTreePQ.rs, SortedListPQ.rs, etc.) because they have multiple inherent impl blocks
- Priority: Phase 1 → Phase 2a → Phase 2b → Phase 3
- Can start with Phase 1 immediately (17 files, fully automated)

