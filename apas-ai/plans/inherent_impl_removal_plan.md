# Plan: Remove Inherent Impls (Single Implementation Pattern)

**Date**: 2025-10-18  
**Goal**: Remove all inherent impl blocks for structs that have custom trait impls, implementing the "Single Implementation Pattern" from RustRules.md

## Context

From RustRules.md:
> **Single Implementation Pattern (MANDATORY)**: Data structures with custom traits MUST have ONLY trait implementations. NO inherent impl blocks alongside trait impl blocks for the same type.

## Scope

**Total Violations**: 60 structs with both inherent and trait impls

**Exclusions**: 
- `src/Types.rs` - Do NOT touch this file (contains core type definitions like Pair, Triple, etc.)
- Standard library trait impls (Debug, Display, Clone, etc.) - These remain unchanged

## Strategy

### Phase 1: Automated Removal (Safe Cases)
Use `scripts/rust/src/fix_remove_inherent_impls.py` to automatically remove inherent impls.

**Process**:
1. Review each file to ensure trait impl covers all inherent methods
2. Run fix script on file
3. Run `cargo check` to verify compilation
4. Run tests for affected module

### Phase 2: Manual Review (Complex Cases)
Some files have multiple inherent impl blocks or complex patterns that need manual review:
- `src/Chap45/BalancedTreePQ.rs` - 3 inherent impls + 1 trait impl
- `src/Chap45/BinaryHeapPQ.rs` - 3 inherent impls + 1 trait impl  
- `src/Chap45/LeftistHeapPQ.rs` - 2 impls
- `src/Chap45/SortedListPQ.rs` - 3 impls
- `src/Chap45/UnsortedListPQ.rs` - 3 impls

### Phase 3: Verification
1. Run full test suite: `cargo test`
2. Run clippy: `cargo clippy --all-targets`
3. Verify no trait method duplication remains
4. Check test coverage hasn't decreased

## File List (60 files)

### Chapter 18: Sequences (6 files)
- [ ] src/Chap18/ArraySeq.rs
- [ ] src/Chap18/ArraySeqMtPer.rs
- [ ] src/Chap18/ArraySeqStEph.rs
- [ ] src/Chap18/ArraySeqStPer.rs
- [ ] src/Chap18/LinkedListStEph.rs
- [ ] src/Chap18/LinkedListStPer.rs

### Chapter 19: Advanced Sequences (1 file)
- [ ] src/Chap19/ArraySeqMtEphSlice.rs

### Chapter 23: Trees (2 files)
- [ ] src/Chap23/BalBinTreeStEph.rs
- [ ] src/Chap23/PrimTreeSeqSt.rs

### Chapter 37: BST Variants (15 files)
- [ ] src/Chap37/AVLTreeSeq.rs
- [ ] src/Chap37/AVLTreeSeqStEph.rs
- [ ] src/Chap37/AVLTreeSeqStPer.rs
- [ ] src/Chap37/BSTAVLMtEph.rs
- [ ] src/Chap37/BSTAVLStEph.rs
- [ ] src/Chap37/BSTBBAlphaMtEph.rs
- [ ] src/Chap37/BSTBBAlphaStEph.rs
- [ ] src/Chap37/BSTRBMtEph.rs
- [ ] src/Chap37/BSTRBStEph.rs
- [ ] src/Chap37/BSTSetAVLMtEph.rs
- [ ] src/Chap37/BSTSetBBAlphaMtEph.rs
- [ ] src/Chap37/BSTSetPlainMtEph.rs
- [ ] src/Chap37/BSTSetRBMtEph.rs
- [ ] src/Chap37/BSTSetSplayMtEph.rs
- [ ] src/Chap37/BSTSplayMtEph.rs
- [ ] src/Chap37/BSTSplayStEph.rs

### Chapter 38: Parametric BSTs (2 files)
- [ ] src/Chap38/BSTParaMtEph.rs
- [ ] src/Chap38/BSTParaStEph.rs

### Chapter 39: Treaps (4 files)
- [ ] src/Chap39/BSTParaTreapMtEph.rs
- [ ] src/Chap39/BSTSetTreapMtEph.rs
- [ ] src/Chap39/BSTTreapMtEph.rs
- [ ] src/Chap39/BSTTreapStEph.rs

### Chapter 40: BST Augmentation (2 files)
- [ ] src/Chap40/BSTKeyValueStEph.rs
- [ ] src/Chap40/BSTSizeStEph.rs

### Chapter 44: Document Index (1 file)
- [ ] src/Chap44/DocumentIndex.rs

### Chapter 45: Priority Queues (5 files) ⚠️ COMPLEX
- [ ] src/Chap45/BalancedTreePQ.rs (3 inherent impls)
- [ ] src/Chap45/BinaryHeapPQ.rs (3 inherent impls)
- [ ] src/Chap45/LeftistHeapPQ.rs
- [ ] src/Chap45/SortedListPQ.rs (3 inherent impls)
- [ ] src/Chap45/UnsortedListPQ.rs (3 inherent impls)

### Chapter 47: Hash Tables (4 files)
- [ ] src/Chap47/HashFunctionTraits.rs (3 structs: PolynomialHashFunction, UniversalIntegerHashFamily, UniversalIntegerHashFunction)
- [ ] src/Chap47clean/StructChainedHashTable.rs

### Chapter 49: Edit Distance / Subset Sum (8 files)
- [ ] src/Chap49/MinEditDistMtEph.rs
- [ ] src/Chap49/MinEditDistMtPer.rs
- [ ] src/Chap49/MinEditDistStEph.rs
- [ ] src/Chap49/MinEditDistStPer.rs
- [ ] src/Chap49/SubsetSumMtEph.rs
- [ ] src/Chap49/SubsetSumMtPer.rs
- [ ] src/Chap49/SubsetSumStEph.rs
- [ ] src/Chap49/SubsetSumStPer.rs

### Chapter 50: Dynamic Programming (8 files)
- [ ] src/Chap50/MatrixChainMtEph.rs
- [ ] src/Chap50/MatrixChainMtPer.rs
- [ ] src/Chap50/MatrixChainStEph.rs
- [ ] src/Chap50/MatrixChainStPer.rs
- [ ] src/Chap50/OptBinSearchTreeMtEph.rs
- [ ] src/Chap50/OptBinSearchTreeMtPer.rs
- [ ] src/Chap50/OptBinSearchTreeStEph.rs
- [ ] src/Chap50/OptBinSearchTreeStPer.rs

### Chapter 65: Union-Find (1 file)
- [ ] src/Chap65/UnionFindStEph.rs

## Implementation Steps

### For Each File:

1. **Pre-check**
   ```bash
   # Verify file compiles
   cargo check --message-format=short 2>&1 | grep "filename_here"
   ```

2. **Remove inherent impl**
   ```bash
   python3 scripts/rust/src/fix_remove_inherent_impls.py --file src/ChapXX/File.rs
   ```

3. **Verify compilation**
   ```bash
   cargo check
   ```

4. **Run affected tests**
   ```bash
   cargo test --test TestChapXX
   ```

5. **Mark complete** in this plan

### Batch Processing Strategy

**Batch 1: Simple Cases (40 files)**
- Chapters 18, 19, 23, 37, 38, 39, 40, 49, 50, 65
- Single inherent impl per struct
- Straightforward trait impl

**Batch 2: Complex Cases (5 files)**  
- Chapter 45 Priority Queues
- Multiple inherent impls
- May need manual consolidation

**Batch 3: Hash Functions (4 files)**
- Chapter 47
- Multiple structs per file
- Trait-based design patterns

## Success Criteria

✅ All inherent impl blocks removed (except for Types.rs)  
✅ All tests pass: `cargo test`  
✅ No clippy warnings: `cargo clippy --all-targets`  
✅ Review script shows 0 violations: `python3 scripts/rust/src/review_inherent_and_trait_impl.py`  
✅ Code coverage maintained or improved

## Rollback Plan

If issues arise:
```bash
git reset --hard HEAD  # If not committed
git revert <commit>     # If committed
```

## Notes

- **Types.rs exclusion**: Core type definitions (Pair, Triple, etc.) should NOT be modified
- **Standard traits**: Keep all Debug, Display, Clone, etc. implementations
- **Test coverage**: Each file has corresponding tests in `tests/ChapXX/`
- **Parallel work**: Can process multiple chapters simultaneously (independent)

## Automation Script

For batch processing:
```bash
#!/bin/bash
# Remove inherent impls in batch

for file in \
  src/Chap18/ArraySeq.rs \
  src/Chap18/ArraySeqMtPer.rs \
  # ... add all 60 files
do
  echo "Processing $file..."
  python3 scripts/rust/src/fix_remove_inherent_impls.py --file "$file"
  
  if ! cargo check --quiet; then
    echo "ERROR: $file failed to compile!"
    git checkout "$file"
  fi
done

cargo test
```

## Timeline Estimate

- **Batch 1** (Simple): 2-3 hours (automated + verification)
- **Batch 2** (Complex): 1-2 hours (manual review + testing)
- **Batch 3** (Hash): 30 min - 1 hour
- **Final verification**: 30 min

**Total**: 4-6 hours

