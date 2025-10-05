# Test Coverage Plan: Next 20 Low-Coverage Modules

**Created:** 2025-10-04  
**Status:** READY FOR EXECUTION  
**Completed from Previous Plan:** 4 modules, 101 tests (AugOrderedTableMtEph, AugOrderedTableStEph, BSTReducedStEph, AugOrderedTableStPer)

---

## EXECUTIVE SUMMARY

**Target:** Add comprehensive tests for 20 modules with lowest coverage  
**Estimated Tests:** 550-650 new tests across all modules  
**Strategy:** Systematic 3-step process per module: Write → Compile → Run  
**Priority:** High-value modules with existing partial test infrastructure

---

## MODULE BREAKDOWN (Ranked by Untested Methods)

### TIER 1: CRITICAL DATA STRUCTURES (30+ untested methods)

#### Module 5: **ArraySeqMtEphSlice** (31 untested methods) ⚠️ DEFERRED
- **Source:** `src/Chap19/ArraySeqMtEphSlice.rs` (439 lines)
- **Tests:** `tests/Chap19/TestArraySeqMtEphSlice.rs` (EXISTS, 16 tests)
- **Status:** API signature complexity requires investigation
- **Estimated Tests:** +15 tests
- **Key Methods:** append, append_select, flatten, reduce, scan, iterate, inject
- **Notes:** tabulate parameter order differs from other variants

#### Module 6: **BSTParaTreapMtEph** (30 untested methods)
- **Source:** `src/Chap39/BSTParaTreapMtEph.rs` (457 lines)
- **Tests:** `tests/Chap39/TestBSTParaTreapMtEph.rs` (EXISTS, 1 test)
- **Estimated Tests:** +25 tests
- **Key Methods:** Parameterized treap with all BST operations
- **Dependencies:** ParamTreap, BSTParaTreapTrait
- **Test Pattern:** Similar to BSTReducedStEph (17 tests completed)

#### Module 7: **OrderedTableMtEph** (30 untested methods)
- **Source:** `src/Chap43/OrderedTableMtEph.rs` (420 lines)
- **Tests:** `tests/Chap43/TestOrderedTableMtEph.rs` (EXISTS, minimal)
- **Estimated Tests:** +25 tests
- **Key Methods:** Ordered table without augmentation (base for AugOrderedTable)
- **Test Pattern:** Similar to AugOrderedTable but without reduce operations

#### Module 8: **OrderedTableStEph** (30 untested methods)
- **Source:** `src/Chap43/OrderedTableStEph.rs` (391 lines)
- **Tests:** `tests/Chap43/TestOrderedTableStEph.rs` (EXISTS, minimal)
- **Estimated Tests:** +25 tests
- **Key Methods:** Single-threaded ephemeral ordered table
- **Test Pattern:** Mirror of OrderedTableMtEph

---

### TIER 2: ADVANCED BST VARIANTS (28-29 untested methods)

#### Module 9: **BSTParaMtEph** (29 untested methods)
- **Source:** `src/Chap38/BSTParaMtEph.rs` (384 lines)
- **Tests:** `tests/Chap38/TestBSTParaMtEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +20 tests
- **Key Methods:** Parallel BST operations
- **Dependencies:** BSTParaTrait

#### Module 10: **BalancedTreePQ** (29 untested methods)
- **Source:** `src/Chap45/BalancedTreePQ.rs` (424 lines)
- **Tests:** `tests/Chap45/TestBalancedTreePQ.rs` (EXISTS, partial)
- **Estimated Tests:** +20 tests
- **Key Methods:** Priority queue using balanced tree
- **Test Pattern:** Similar to other PQ variants (UnsortedListPQ, SortedListPQ)

#### Module 11: **BinaryHeapPQ** (29 untested methods)
- **Source:** `src/Chap45/BinaryHeapPQ.rs` (400 lines)
- **Tests:** `tests/Chap45/TestBinaryHeapPQ.rs` (EXISTS, needs tests)
- **Estimated Tests:** +20 tests
- **Key Methods:** Binary heap priority queue operations
- **Critical:** heap_up, heap_down, build_heap, heapsort variants

#### Module 12: **BSTSizeStEph** (28 untested methods)
- **Source:** `src/Chap40/BSTSizeStEph.rs` (310 lines)
- **Tests:** `tests/Chap40/TestBSTSizeStEph.rs` (EXISTS, needs tests)
- **Estimated Tests:** +20 tests
- **Key Methods:** Size-augmented BST
- **Test Pattern:** Similar to BSTReducedStEph (17 tests completed)

#### Module 13: **ArraySeqMtPer** (28 untested methods) ⚠️ NO TEST FILE
- **Source:** `src/Chap18/ArraySeqMtPer.rs` (392 lines)
- **Tests:** NEEDS CREATION
- **Estimated Tests:** +25 tests (create file + tests)
- **Key Methods:** Multi-threaded persistent array sequence
- **Test Pattern:** Similar to ArraySeqStPer

---

### TIER 3: SEQUENCE VARIANTS (27 untested methods)

#### Module 14: **ArraySeqMtEph** (27 untested methods) ⚠️ NO TEST FILE
- **Source:** `src/Chap18/ArraySeqMtEph.rs` (390 lines)
- **Tests:** NEEDS CREATION
- **Estimated Tests:** +25 tests (create file + tests)
- **Key Methods:** Multi-threaded ephemeral array sequence

#### Module 15: **ArraySeq** (27 untested methods) ⚠️ NO TEST FILE
- **Source:** `src/Chap18/ArraySeq.rs` (390 lines)
- **Tests:** NEEDS CREATION
- **Estimated Tests:** +25 tests (create file + tests)
- **Key Methods:** Base array sequence trait

#### Module 16: **OrderedTableStPer** (27 untested methods)
- **Source:** `src/Chap43/OrderedTableStPer.rs` (375 lines)
- **Tests:** `tests/Chap43/TestOrderedTableStPer.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +25 tests
- **Key Methods:** Single-threaded persistent ordered table

---

### TIER 4: RED-BLACK & BST VARIANTS (25-26 untested methods)

#### Module 17: **BSTRBMtEph** (26 untested methods)
- **Source:** `src/Chap37/BSTRBMtEph.rs`
- **Tests:** `tests/Chap37/TestBSTRBMtEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +20 tests
- **Key Methods:** Red-black tree multi-threaded ephemeral

#### Module 18: **BSTRBStEph** (26 untested methods)
- **Source:** `src/Chap37/BSTRBStEph.rs`
- **Tests:** `tests/Chap37/TestBSTRBStEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +20 tests
- **Key Methods:** Red-black tree single-threaded ephemeral

#### Module 19: **LeftistHeapPQ** (26 untested methods)
- **Source:** `src/Chap45/LeftistHeapPQ.rs`
- **Tests:** `tests/Chap45/TestLeftistHeapPQ.rs` (EXISTS, needs tests)
- **Estimated Tests:** +20 tests
- **Key Methods:** Leftist heap priority queue
- **Critical:** merge, s_value, rank operations

#### Module 20: **BSTBBAlphaStEph** (25 untested methods)
- **Source:** `src/Chap37/BSTBBAlphaStEph.rs`
- **Tests:** `tests/Chap37/TestBSTBBAlphaStEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +15 tests
- **Key Methods:** BB[α] balanced tree

#### Module 21: **BSTAVLStEph** (25 untested methods)
- **Source:** `src/Chap37/BSTAVLStEph.rs`
- **Tests:** `tests/Chap37/TestBSTAVLStEph.rs` (EXISTS, has tests)
- **Estimated Tests:** +15 tests
- **Key Methods:** AVL tree operations
- **Note:** Likely has good coverage already

#### Module 22: **BSTBBAlphaMtEph** (25 untested methods)
- **Source:** `src/Chap37/BSTBBAlphaMtEph.rs`
- **Tests:** `tests/Chap37/TestBSTBBAlphaMtEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +15 tests
- **Key Methods:** BB[α] balanced tree multi-threaded

#### Module 23: **ArraySeqStEph** (25 untested methods)
- **Source:** `src/Chap18/ArraySeqStEph.rs`
- **Tests:** `tests/Chap18/TestArraySeqStEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +20 tests
- **Key Methods:** Single-threaded ephemeral array sequence

#### Module 24: **LinkedListStEph** (25 untested methods)
- **Source:** `src/Chap18/LinkedListStEph.rs`
- **Tests:** `tests/Chap18/TestLinkedListStEph.rs` (EXISTS, needs expansion)
- **Estimated Tests:** +20 tests
- **Key Methods:** Linked list operations

---

## EXECUTION STRATEGY

### Phase 1: High-Impact Ordered Tables (Modules 7-8, 16)
**Estimated Time:** 3-4 hours  
**Tests Added:** ~75 tests  
**Rationale:** Builds on completed AugOrderedTable work
- OrderedTableMtEph (30 methods)
- OrderedTableStEph (30 methods)
- OrderedTableStPer (27 methods)

### Phase 2: Priority Queue Suite (Modules 10-11, 19)
**Estimated Time:** 3-4 hours  
**Tests Added:** ~60 tests  
**Rationale:** Complete PQ coverage (already have BalancedTreePQ partial)
- BalancedTreePQ (29 methods)
- BinaryHeapPQ (29 methods)
- LeftistHeapPQ (26 methods)

### Phase 3: BST Advanced Variants (Modules 6, 9, 12)
**Estimated Time:** 3-4 hours  
**Tests Added:** ~65 tests  
**Rationale:** Builds on BSTReducedStEph work
- BSTParaTreapMtEph (30 methods)
- BSTParaMtEph (29 methods)
- BSTSizeStEph (28 methods)

### Phase 4: Array Sequence Family (Modules 13-15, 23)
**Estimated Time:** 4-5 hours  
**Tests Added:** ~95 tests  
**Rationale:** Complete array sequence coverage
- ArraySeqMtPer (28 methods) - NEW FILE
- ArraySeqMtEph (27 methods) - NEW FILE
- ArraySeq (27 methods) - NEW FILE
- ArraySeqStEph (25 methods)

### Phase 5: Red-Black & BB Trees (Modules 17-18, 20, 22)
**Estimated Time:** 3-4 hours  
**Tests Added:** ~70 tests  
**Rationale:** Complete balanced tree variants
- BSTRBMtEph (26 methods)
- BSTRBStEph (26 methods)
- BSTBBAlphaStEph (25 methods)
- BSTBBAlphaMtEph (25 methods)

### Phase 6: Miscellaneous (Modules 21, 24)
**Estimated Time:** 2-3 hours  
**Tests Added:** ~35 tests  
**Rationale:** Complete remaining low-coverage items
- BSTAVLStEph (25 methods)
- LinkedListStEph (25 methods)

### Phase 7: ArraySeqMtEphSlice Revisit (Module 5)
**Estimated Time:** 2-3 hours  
**Tests Added:** ~15 tests  
**Rationale:** Investigate API differences and complete deferred module
- ArraySeqMtEphSlice (31 methods)

---

## TEST PATTERNS BY MODULE TYPE

### Ordered Table Tests
```
1. Empty table operations
2. Singleton operations
3. Insert/delete basic
4. Find/lookup operations
5. Domain/collect operations
6. Difference/union/intersection
7. Restrict/subtract operations
8. Tabulate from set
9. Key navigation (first, last, next, previous)
10. Rank/select operations
11. Range operations (get_key_range)
12. Split/join operations (split_key, split_rank_key)
13. Map/filter operations
14. Display/Debug formatting
15. Large dataset operations
```

### Priority Queue Tests
```
1. Empty PQ operations
2. Insert single/multiple
3. Find_min/delete_min
4. Meld/merge operations
5. Is_empty checks
6. Size tracking
7. Priority ordering verification
8. Heap property validation
9. Large dataset operations
10. Stress tests
```

### BST Tests
```
1. Empty tree
2. Singleton tree
3. Insert/find/contains operations
4. Delete operations
5. Min/max operations
6. Height/size tracking
7. Keys/values collections
8. Rotation validation (for balanced variants)
9. Balance property checks
10. Large dataset operations
11. Range queries
12. Traversal operations
```

### Array Sequence Tests
```
1. Empty sequence
2. Singleton
3. New with size
4. Tabulate operations
5. Map/filter/reduce
6. Append operations
7. Slice/subseq operations
8. Update operations
9. Scan operations
10. Flatten operations
11. Inject operations
12. Concurrent operations (for Mt variants)
```

---

## TESTING CHECKLIST (Per Module)

### Pre-Test Phase
- [ ] Read source file to understand API
- [ ] Check existing test file (if any)
- [ ] List all trait methods
- [ ] Identify method signatures and parameters
- [ ] Review any special semantics (persistent vs ephemeral, etc.)

### Test Writing Phase
- [ ] Write test for empty/singleton cases
- [ ] Write tests for core operations (insert, delete, find)
- [ ] Write tests for advanced operations
- [ ] Write tests for edge cases (large datasets, boundary conditions)
- [ ] Write tests for error conditions (where applicable)
- [ ] Add macro tests (if Lit! macro exists)

### Compilation Phase
- [ ] Run `cargo test --test TestModuleName`
- [ ] Fix type errors
- [ ] Fix method signature mismatches
- [ ] Fix import issues
- [ ] Verify zero warnings

### Execution Phase
- [ ] Run tests and verify all pass
- [ ] Fix any test logic errors
- [ ] Fix any assertion errors
- [ ] Verify test coverage is comprehensive

---

## RISK MITIGATION

### Known Issues
1. **ArraySeqMtEphSlice API differences** - tabulate parameter order, inject as assoc function
2. **Missing test files** - ArraySeqMtPer, ArraySeqMtEph, ArraySeq (need creation)
3. **Unrelated build errors** - TestMatrixChainMtPer macro issues (not our responsibility)

### Contingency Plans
- **API mismatches**: Read source trait carefully, use grep to find correct signatures
- **Missing files**: Create from templates using existing test files as patterns
- **Build failures**: Isolate our tests, verify they compile independently

---

## SUCCESS METRICS

### Quantitative Goals
- **Tests Added:** 550-650 comprehensive tests
- **Coverage Improvement:** Expect 15-25% increase on these 20 modules
- **Compilation:** Zero warnings across all new tests
- **Pass Rate:** 100% of tests passing

### Qualitative Goals
- Comprehensive edge case coverage
- Clear, readable test names
- Proper use of APAS idioms (no Vec, proper trait usage)
- Good documentation in test comments

---

## ESTIMATED TOTAL EFFORT

**Writing Tests:** 20-25 hours  
**Debugging/Fixing:** 5-8 hours  
**Verification:** 2-3 hours  
**Total:** 27-36 hours (3.5 to 4.5 working days)

---

## NEXT ACTIONS

1. **IMMEDIATE:** Start with Phase 1 (OrderedTableMtEph, OrderedTableStEph, OrderedTableStPer)
2. **SHORT-TERM:** Complete Phases 2-3 (PQ Suite + BST Variants)
3. **MEDIUM-TERM:** Tackle Phase 4 (Array Sequence Family with new file creation)
4. **LONG-TERM:** Clean up remaining BST variants and revisit ArraySeqMtEphSlice

---

**READY FOR EXECUTION - AWAITING GO SIGNAL**

