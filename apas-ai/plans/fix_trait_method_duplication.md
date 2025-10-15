# Plan: Fix Trait Method Duplication (121 violations)

**Rule:** RustRules.md - No Trait Method Duplication (MANDATORY)

**Strategy:** Delete inherent methods that duplicate trait methods. Keep only trait implementations.

**Verification after EACH file:**
1. `cargo check --lib`
2. `cargo check --tests`
3. `cargo nextest run --no-fail-fast`

---

## Phase 1: Core Data Structures (8 files, 32 violations)

### File 1: `src/Chap17/MathSeq.rs` (2 violations)
**Delete lines 123, 126:**
- Line 123: `pub fn empty() -> Self`
- Line 126: `pub fn singleton(item: T) -> Self`

**Keep:** Trait methods at lines 177, 179

**Impact:** Low - straightforward sequence type

---

### File 2: `src/Chap05/SetStEph.rs` (7 violations)
**Delete lines 103, 111, 113, 115, 126, 155, 173:**
- Line 103: `pub fn empty() -> Set<T>`
- Line 111: `pub fn size(&self) -> N`
- Line 113: `pub fn mem(&self, key: &T) -> B`
- Line 115: `pub fn union(&self, other: &Set<T>) -> Set<T>`
- Line 126: `pub fn intersection(&self, other: &Set<T>) -> Set<T>`
- Line 155: `pub fn CartesianProduct<U: StT + Hash>(...)`
- Line 173: `pub fn iter(&self) -> ...`

**Keep:** Trait methods at lines 185, 193, 195, 197, 208, 237, 255

**Impact:** Medium - heavily used throughout codebase

---

### File 3: `src/Chap18/ArraySeq.rs` (6 violations)
**Delete lines 125, 127, 129, 131, 133, 135:**
- Line 125: `pub fn length(&self) -> N`
- Line 127: `pub fn nth(&self, index: N) -> &T`
- Line 129: `pub fn empty() -> Self`
- Line 131: `pub fn singleton(item: T) -> Self`
- Line 133: `pub fn isEmpty(&self) -> B`
- Line 135: `pub fn isSingleton(&self) -> B`

**Keep:** Trait methods at lines 191, 193, 195, 197, 275, 277

**Impact:** Medium - base sequence type

---

### File 4: `src/Chap18/ArraySeqStEph.rs` (5 violations)
**Delete lines 25, 27, 29, 31, 33:**
- Line 25: `pub fn new(length: N, init_value: T) -> Self`
- Line 27: `pub fn empty() -> Self`
- Line 29: `pub fn singleton(item: T) -> Self`
- Line 31: `pub fn length(&self) -> N`
- Line 33: `pub fn nth(&self, index: N) -> &T`

**Keep:** Trait methods at lines 178, 188, 190, 184, 186

**Impact:** Medium - ephemeral variant

---

### File 5: `src/Chap18/ArraySeqStPer.rs` (5 violations)
**Delete lines 24, 25, 26, 27, 28:**
- Line 24: `pub fn new(length: N, init_value: T) -> Self`
- Line 25: `pub fn empty() -> Self`
- Line 26: `pub fn singleton(item: T) -> Self`
- Line 27: `pub fn length(&self) -> N`
- Line 28: `pub fn nth(&self, index: N) -> &T`

**Keep:** Trait methods at lines 141, 144, 145, 142, 143

**Impact:** Medium - persistent variant

---

### File 6: `src/Chap18/ArraySeqMtEph.rs` (2 violations)
**Delete lines 27, 34:**
- Line 27: `pub fn new(length: N, init_value: T) -> Self`
- Line 34: `pub fn singleton(item: T) -> Self`

**Keep:** Trait methods at lines 181, 193

**Impact:** Low - multi-threaded ephemeral variant

---

### File 7: `src/Chap18/ArraySeqMtPer.rs` (3 violations)
**Delete lines 35, 43, 45:**
- Line 35: `pub fn singleton(item: T) -> Self`
- Line 43: `pub fn length(&self) -> N`
- Line 45: `pub fn nth(&self, index: N) -> &T`

**Keep:** Trait methods at lines 190, 191, 192

**Impact:** Low - multi-threaded persistent variant

---

### File 8: `src/Chap18/LinkedListStEph.rs` (4 violations)
**Delete lines 23, 25, 32, 43:**
- Line 23: `pub fn empty() -> Self`
- Line 25: `pub fn new(n: N, init_value: T) -> Self`
- Line 32: `pub fn singleton(item: T) -> Self`
- Line 43: `pub fn length(&self) -> N`

**Keep:** Trait methods at lines 267, 252, 269, 263

**Impact:** Medium - linked list implementation

---

## Phase 2: Linked Lists & Trees (3 files, 9 violations)

### File 9: `src/Chap18/LinkedListStPer.rs` (4 violations)
**Delete lines 95, 97, 104, 116:**
- Line 95: `pub fn empty() -> Self`
- Line 97: `pub fn new(n: N, init_value: T) -> Self`
- Line 104: `pub fn singleton(item: T) -> Self`
- Line 116: `pub fn length(&self) -> N`

**Keep:** Trait methods at lines 235, 228, 236, 237

**Impact:** Medium - persistent linked list

---

### File 10: `src/Chap23/PrimTreeSeqSt.rs` (6 violations)
**Delete lines 66, 71, 76, 81, 86, 91:**
- Line 66: `pub fn empty() -> Self`
- Line 71: `pub fn singleton(item: T) -> Self`
- Line 76: `pub fn from_vec(vec: Vec<T>) -> Self`
- Line 81: `pub fn into_vec(self) -> Vec<T>`
- Line 86: `pub fn as_slice(&self) -> &[T]`
- Line 91: `pub fn length(&self) -> N`

**Keep:** Trait methods at lines 126, 128, 130, 132, 134, 136

**Impact:** Low - primitive tree sequence

---

### File 11: `src/Chap23/BalBinTreeStEph.rs` (1 violation)
**Delete line 51:**
- Line 51: `pub fn leaf(x: T) -> Self`

**Keep:** Trait method at line 110

**Impact:** Low - balanced binary tree

---

## Phase 3: AVL & Plain BST (3 files, 7 violations)

### File 12: `src/Chap37/AVLTreeSeq.rs` (1 violation)
**Delete line 99:**
- Line 99: `pub fn new() -> Self`

**Keep:** Trait method at line 206

**Impact:** Low - AVL tree sequence

---

### File 13: `src/Chap37/AVLTreeSeqStEph.rs` (1 violation)
**Delete line 78:**
- Line 78: `pub fn new() -> Self`

**Keep:** Trait method at line 151

**Impact:** Low - ephemeral AVL tree sequence

---

### File 14: `src/Chap37/BSTBBAlphaStEph.rs` (5 violations)
**Delete lines 72, 92, 94, 96, 98:**
- Line 72: `pub fn is_empty(&self) -> B`
- Line 92: `pub fn find(&self, key: &T) -> Option<&T>`
- Line 94: `pub fn contains(&self, key: &T) -> B`
- Line 96: `pub fn minimum(&self) -> Option<&T>`
- Line 98: `pub fn maximum(&self) -> Option<&T>`

**Keep:** Trait methods at lines 232, 246, 248, 250, 252

**Impact:** Medium - weight-balanced BST

---

## Phase 4: BST Set Implementations (6 files, 60 violations)

**Pattern:** Each file has identical 10 duplicates (lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201)

### File 15: `src/Chap37/BSTSetAVLMtEph.rs` (10 violations)
**Delete lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201:**
- `size`, `is_empty`, `find`, `contains`, `minimum`, `maximum`, `insert`
- `filter`, `reduce`, `iter_in_order`, `as_tree`

**Keep:** Trait methods at lines 230-264

**Impact:** Medium - AVL-based BST set

---

### File 16: `src/Chap37/BSTSetBBAlphaMtEph.rs` (10 violations)
**Delete lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201:**
- Same pattern as File 15

**Keep:** Trait methods at lines 230-264

**Impact:** Medium - BB-Alpha-based BST set

---

### File 17: `src/Chap37/BSTSetPlainMtEph.rs` (10 violations)
**Delete lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201:**
- Same pattern, but trait methods at lines 331, 338, 345, 347 for last 4

**Keep:** Trait methods at lines 230-347

**Impact:** Medium - Plain BST set

---

### File 18: `src/Chap37/BSTSetRBMtEph.rs` (10 violations)
**Delete lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201:**
- Same pattern as File 15

**Keep:** Trait methods at lines 230-264

**Impact:** Medium - Red-Black BST set

---

### File 19: `src/Chap37/BSTSetSplayMtEph.rs` (10 violations)
**Delete lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201:**
- Same pattern as File 15

**Keep:** Trait methods at lines 230-264

**Impact:** Medium - Splay BST set

---

### File 20: `src/Chap39/BSTSetTreapMtEph.rs` (10 violations)
**Delete lines 75, 77, 79, 81, 83, 85, 87, 176, 189, 199, 201:**
- Same pattern as File 15

**Keep:** Trait methods at lines 230-264

**Impact:** Medium - Treap BST set

---

## Phase 5: Miscellaneous (3 files, 3 violations)

### File 21: `src/Chap12/Exercise12_1.rs` (1 violation)
**Delete line 52:**
- Line 52: `pub fn unlock(&mut self)`

**Keep:** Trait method at line 71

**Impact:** Low - exercise file

---

### File 22: `src/Chap47clean/StructChainedHashTable.rs` (1 violation)
**Delete line 32:**
- Line 32: `pub fn new() -> Self`

**Keep:** Trait method at line 36

**Impact:** Low - hash table chain list

---

### File 23: `src/Chap65/UnionFindStEph.rs` (1 violation)
**Delete line 145:**
- Line 145: `pub fn equals(&mut self, x: N, y: N) -> B`

**Keep:** Trait method at line 213

**Impact:** Low - union-find

---

## Summary

**Total:** 23 files, 121 violations

**Estimated time:** 30-45 minutes (2 min/file average)

**Risk level by phase:**
- Phase 1: Medium (core data structures, heavily used)
- Phase 2: Medium (linked lists & trees)
- Phase 3: Low-Medium (AVL & BST variants)
- Phase 4: Medium (6 similar BST set implementations)
- Phase 5: Low (miscellaneous exercises)

**Verification strategy:**
After each file:
1. `cargo check --lib` (compile library)
2. `cargo check --tests` (compile tests)
3. `cargo nextest run --no-fail-fast` (run all tests)

If any step fails, stop and report error before continuing.

---

## Expected Outcome

- Zero compiler warnings
- Zero test failures
- All 121 violations resolved
- Coverage improvement for affected modules (especially MathSeq: 89.2% → ~95%+)
- Cleaner, DRY codebase with single source of truth for behavior

---

**Ready for execution? Awaiting approval.**

