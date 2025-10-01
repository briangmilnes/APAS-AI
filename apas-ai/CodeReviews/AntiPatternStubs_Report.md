# Anti-Pattern Stub Functions Report

## Summary

Identified **8 source files** with stub anti-patterns across **2 chapters** (Chap43, Chap45).

### Anti-Pattern Definition

From `rules/RustRules.md` line 254:
> "Do not add stub functions inside the same module that simply call another function/method in that module. Call sites should invoke the original API directly rather than indirection stubs."

---

## Affected Files by Chapter

### Chap45: Priority Queues (6 files)

All 5 priority queue implementations + HeapsortExample have this anti-pattern.

#### 1. `src/Chap45/BinaryHeapPQ.rs`

**Stub Methods (lines 288-308):**
- `pub fn new() -> Self` → calls `Self::empty()`
- `pub fn len(&self) -> N` → calls `self.size()`
- `pub fn is_empty(&self) -> bool` → calls `BinaryHeapPQTrait::is_empty(self)`
- `pub fn peek(&self) -> Option<&T>` → calls `self.find_min()`

**Stub Ops Struct (lines 441-468):**
- `pub struct BinaryHeapPQOps` with 5 methods:
  - `empty()` → `BinaryHeapPQ::empty()`
  - `insert()` → `pq.insert(element)`
  - `delete_min()` → `pq.delete_min()`
  - `meld()` → `pq1.meld(pq2)`
  - `from_seq()` → `BinaryHeapPQ::from_seq(seq)`

**Total stubs: 9 functions**

#### 2. `src/Chap45/UnsortedListPQ.rs`

**Stub Methods (lines 158-177):**
- `pub fn new() -> Self` → calls `Self::empty()`
- `pub fn len(&self) -> N` → calls `self.size()`
- `pub fn is_empty(&self) -> bool` → calls `UnsortedListPQTrait::is_empty(self)`
- `pub fn peek(&self) -> Option<&T>` → calls `self.find_min()`

**Stub Ops Struct (lines 279-306):**
- `pub struct UnsortedListPQOps` with 5 methods (same pattern as BinaryHeapPQOps)

**Total stubs: 9 functions**

#### 3. `src/Chap45/SortedListPQ.rs`

**Stub Methods (lines 207-226):**
- `pub fn new() -> Self` → calls `Self::empty()`
- `pub fn len(&self) -> N` → calls `self.size()`
- `pub fn is_empty(&self) -> bool` → calls `SortedListPQTrait::is_empty(self)`
- `pub fn peek(&self) -> Option<&T>` → calls `self.find_min()`

**Stub Ops Struct (lines 354-381):**
- `pub struct SortedListPQOps` with 5 methods (same pattern)

**Total stubs: 9 functions**

#### 4. `src/Chap45/BalancedTreePQ.rs`

**Stub Methods (lines 187-205):**
- `pub fn new() -> Self` → calls `Self::empty()`
- `pub fn len(&self) -> N` → calls `self.size()`
- `pub fn is_empty(&self) -> bool` → calls `BalancedTreePQTrait::is_empty(self)`
- `pub fn peek(&self) -> Option<&T>` → calls `self.find_min()`

**Stub Ops Struct (lines 378-405):**
- `pub struct BalancedTreePQOps` with 5 methods (same pattern)

**Total stubs: 9 functions**

#### 5. `src/Chap45/LeftistHeapPQ.rs`

**Stub Methods (lines 305-324):**
- `pub fn new() -> Self` → calls `Self::empty()`
- `pub fn len(&self) -> N` → calls `self.size()`
- `pub fn is_empty(&self) -> bool` → calls `LeftistHeapPQTrait::is_empty(self)`
- `pub fn peek(&self) -> Option<&T>` → calls `self.find_min()`

**Stub Ops Struct (lines 373-400):**
- `pub struct LeftistHeapPQOps` with 5 methods (same pattern)

**Total stubs: 9 functions**

#### 6. `src/Chap45/HeapsortExample.rs`

**Five Stub HeapsortOps Structs (lines 141-244):**
Each implements `HeapsortOps` trait by just calling underlying PQ methods:

- `UnsortedListPQHeapsortOps` (lines 142-160)
  - `empty()` → `UnsortedListPQ::empty()`
  - `insert()` → `pq.insert(element)`
  - `delete_min()` → `pq.delete_min()`
  - `is_empty()` → `pq.is_empty()`

- `SortedListPQHeapsortOps` (lines 163-181)
- `BalancedTreePQHeapsortOps` (lines 184-202)
- `BinaryHeapPQHeapsortOps` (lines 205-223)
- `LeftistHeapPQHeapsortOps` (lines 226-244)

**Total stubs: 20 functions (5 structs × 4 methods each)**

---

### Chap43: Ordered Tables (2 files)

#### 7. `src/Chap43/OrderedTableStEph.rs`

**Stub Method (lines 61-65):**
- `pub fn new() -> Self` → creates struct with `TableStEph::empty()`

**Total stubs: 1 function**

**Note:** This is a minor case - `new()` is slightly more than a pure delegate since it wraps the result in the struct, but it's still redundant since `empty()` already exists on the trait.

#### 8. `src/Chap43/OrderedTableMtEph.rs`

**Stub Method (lines 63-67):**
- `pub fn new() -> Self` → creates struct with `TableMtEph::empty()`

**Total stubs: 1 function**

---

## Total Stub Count

| File | Stub Methods | Stub Ops Structs | Total Stubs |
|------|-------------|------------------|-------------|
| BinaryHeapPQ.rs | 4 | 1 struct (5 methods) | 9 |
| UnsortedListPQ.rs | 4 | 1 struct (5 methods) | 9 |
| SortedListPQ.rs | 4 | 1 struct (5 methods) | 9 |
| BalancedTreePQ.rs | 4 | 1 struct (5 methods) | 9 |
| LeftistHeapPQ.rs | 4 | 1 struct (5 methods) | 9 |
| HeapsortExample.rs | 0 | 5 structs (4 methods each) | 20 |
| OrderedTableStEph.rs | 1 | 0 | 1 |
| OrderedTableMtEph.rs | 1 | 0 | 1 |
| **TOTAL** | **22** | **11 structs (45 methods)** | **67** |

---

## Recommended Fixes

### For Priority Queue Files (Chap45)

1. **Remove stub methods:**
   - Delete `new()`, `len()`, `is_empty()`, `peek()` from each impl block
   - Call sites should use trait methods directly: `PQ::empty()`, `pq.size()`, `PQTrait::is_empty(&pq)`, `pq.find_min()`

2. **Remove *Ops structs:**
   - Delete `BinaryHeapPQOps`, `UnsortedListPQOps`, `SortedListPQOps`, `BalancedTreePQOps`, `LeftistHeapPQOps`
   - These structs provide zero value - they're pure indirection
   - Call sites should use trait methods directly

3. **Remove HeapsortOps structs:**
   - Delete all 5 `*HeapsortOps` structs in `HeapsortExample.rs`
   - The `HeapsortOps` trait itself is questionable - it just adds indirection
   - Consider inlining the heapsort logic or using the trait methods directly

### For Ordered Table Files (Chap43)

1. **Remove `new()` methods:**
   - Delete `new()` from `OrderedTableStEph` and `OrderedTableMtEph`
   - Call sites should use `OrderedTableStEph::empty()` directly

---

## Impact Analysis

### Files to Modify (8 source files)
- `src/Chap45/BinaryHeapPQ.rs`
- `src/Chap45/UnsortedListPQ.rs`
- `src/Chap45/SortedListPQ.rs`
- `src/Chap45/BalancedTreePQ.rs`
- `src/Chap45/LeftistHeapPQ.rs`
- `src/Chap45/HeapsortExample.rs`
- `src/Chap43/OrderedTableStEph.rs`
- `src/Chap43/OrderedTableMtEph.rs`

### Test Files to Update

Must find and update all call sites. Likely affected:
- `tests/Chap45/*.rs` (all 6 test files)
- `tests/Chap43/*.rs` (2 test files)
- Any benchmarks using these APIs

### Benchmark Files to Update

- `benches/Chap45/*.rs` (6 benchmark files)

---

## Estimated Effort

- **Removing stubs from source files:** ~30 minutes (8 files)
- **Finding and updating all call sites:** ~90 minutes (scanning + fixing tests + benchmarks)
- **Verification (build + test):** ~20 minutes
- **Rule compliance checks:** ~20 minutes

**Total estimated time: ~2.5 hours (160 minutes)**

---

## Next Steps

1. Create detailed plan following `PrePlanChecklist.md`
2. Execute relentlessly file-by-file
3. Verify compilation after each file
4. Run full test suite at end
5. Run full benchmark compilation at end


