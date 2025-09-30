# Chapter 27: Contraction - Implementation Plan

## PrePlanChecklist Review

### 1. ✅ Does the plan include full algorithmic analysis?
Yes - Work, Span, and Parallelism analysis for all operations in all modules.

### 2. ✅ Does the plan avoid Vec usage?
Yes - uses ArraySeqStEph and ArraySeqMtEph throughout, no Vec.

### 3. ✅ Does the plan include copyright headers?
Yes - all files will have copyright headers on line 1.

### 4. ✅ Does the plan avoid thresholding in Mt files?
Yes - Mt implementations use unconditional parallelism with ParaPair!.

### 5. ✅ Does the plan schedule creating each *Per file before the *Eph files?
N/A - Only implementing Eph versions as requested (no Per versions).

### 6. ✅ Does the plan include test files?
Yes - 4 test files, one for each source module.

### 7. ✅ Does the plan include benchmark files?
Yes - 4 benchmark files with APAS timing rules.

### 8. ✅ Does the plan include Cargo.toml updates?
Yes - registration of all tests and benchmarks.

### 9. ✅ Does the plan include src/lib.rs updates?
Yes - module declarations for Chap27.

### 10. ✅ Does the plan avoid rayon?
Yes - using ParaPair! and thread::scope only.

---

## Modules Overview

### Source Files (4)

#### 1. `src/Chap27/ReduceContractStEph.rs`
**Purpose:** Sequential reduce using contraction technique (Algorithm 27.2)

**Data Structure:** Works with `ArraySeqStEph<T>`

**Methods:**
- `reduce_contract(f: &F, id: T, a: &ArraySeqStEph<T>) -> T`
  - APAS: Work Θ(n), Span Θ(n)
  - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
  - Base case: `|a| ≤ 1`
  - Contract: Create sequence `b[i] = f(a[2i], a[2i+1])` for pairs
  - Solve: Recursively solve smaller instance
  - Expand: Return result directly (no expansion needed)

**Example:**
```rust
// Input: [2, 1, 3, 2, 2, 5, 4, 1] with + operation
// Contract to: [3, 5, 7, 5]
// Recurse: [8, 12]
// Recurse: [20]
// Return: 20
```

**Traits:**
```rust
pub trait ReduceContractStEphTrait<T: StT> {
    fn reduce_contract<F: Fn(&T, &T) -> T>(
        a: &ArraySeqStEphS<T>,
        f: &F,
        id: T,
    ) -> T;
}
```

---

#### 2. `src/Chap27/ScanContractStEph.rs`
**Purpose:** Sequential scan using contraction technique (Algorithm 27.3)

**Data Structure:** Works with `ArraySeqStEph<T>`

**Methods:**
- `scan_contract(f: &F, id: T, a: &ArraySeqStEph<T>) -> (ArraySeqStEph<T>, T)`
  - APAS: Work Θ(n), Span Θ(n)
  - claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
  - Base case: `|a| = 0` returns `(empty, id)`, `|a| = 1` returns `([id], a[0])`
  - Contract: Create `a'[i] = f(a[2i], a[2i+1])`
  - Solve: Recursively compute `(r, t) = scan(f, id, a')`
  - Expand: Build result where even positions use `r[i/2]`, odd use `f(r[i/2], a[i-1])`

**Example:**
```rust
// Input: [2, 1, 3, 2, 2, 5, 4, 1] with + operation
// Contract to: [3, 5, 7, 5]
// Scan: ([0, 3, 8, 15], 20)
// Expand even positions: [0, ?, 3, ?, 8, ?, 15, ?]
// Expand odd positions: [0, 2, 3, 6, 8, 10, 15, 19]
// Return: ([0, 2, 3, 6, 8, 10, 15, 19], 20)
```

**Traits:**
```rust
pub trait ScanContractStEphTrait<T: StT + Clone> {
    fn scan_contract<F: Fn(&T, &T) -> T>(
        a: &ArraySeqStEphS<T>,
        f: &F,
        id: T,
    ) -> (ArraySeqStEphS<T>, T);
}
```

---

#### 3. `src/Chap27/ReduceContractMtEph.rs`
**Purpose:** Parallel reduce using contraction technique

**Data Structure:** Works with `ArraySeqMtEph<T>`

**Parallel Operations:**
- Contraction step uses ParaPair! to process pairs in parallel
- No thresholding - unconditionally parallel

**Methods:**
- `reduce_contract_parallel(f: &F, id: T, a: &ArraySeqMtEph<T>) -> T`
  - APAS: Work Θ(n), Span Θ(log n)
  - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
  - Base case: `|a| ≤ 1`
  - Contract: Parallel tabulate pairs using map
  - Solve: Recursively solve in parallel
  - Expand: Return result

**Parallelism Strategy:**
- Uses `ArraySeqMtEph::map()` for contraction (already parallel)
- Recursive calls use ParaPair! for maximal parallelism

**Traits:**
```rust
pub trait ReduceContractMtEphTrait<T: StTInMtT + Send + Sync + 'static> {
    fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
        a: &ArraySeqMtEphS<T>,
        f: &F,
        id: T,
    ) -> T;
}
```

---

#### 4. `src/Chap27/ScanContractMtEph.rs`
**Purpose:** Parallel scan using contraction technique

**Data Structure:** Works with `ArraySeqMtEph<T>`

**Parallel Operations:**
- Contraction: Parallel tabulate pairs
- Expansion: Parallel tabulate to fill missing positions
- Uses ParaPair! throughout

**Methods:**
- `scan_contract_parallel(f: &F, id: T, a: &ArraySeqMtEph<T>) -> (ArraySeqMtEph<T>, T)`
  - APAS: Work Θ(n), Span Θ(log n)
  - claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
  - Base case: `|a| ≤ 1`
  - Contract: Parallel map pairs
  - Solve: Recursive parallel call
  - Expand: Parallel tabulate for even/odd positions

**Parallelism Strategy:**
- Contraction uses `ArraySeqMtEph::map()`
- Expansion uses `ArraySeqMtEph::tabulate()` (parallel)
- No sequential fallbacks - unconditionally parallel

**Traits:**
```rust
pub trait ScanContractMtEphTrait<T: StTInMtT + Send + Sync + Clone + 'static> {
    fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
        a: &ArraySeqMtEphS<T>,
        f: &F,
        id: T,
    ) -> (ArraySeqMtEphS<T>, T);
}
```

---

## Test Files (4)

### 1. `tests/Chap27/TestReduceContractStEph.rs`
**Test Cases:**
- Empty sequence
- Single element
- Powers of 2 sizes (2, 4, 8, 16)
- Non-power-of-2 sizes
- Different operations (sum, product, max, min)
- Identity element behavior

### 2. `tests/Chap27/TestScanContractStEph.rs`
**Test Cases:**
- Empty sequence
- Single element  
- Powers of 2 sizes
- Non-power-of-2 sizes
- Verify prefix sums correct
- Different operations
- Check returned total matches reduce

### 3. `tests/Chap27/TestReduceContractMtEph.rs`
**Test Cases:**
- Same as StEph but with larger sizes
- Verify parallelism correctness
- Test with 100+ elements

### 4. `tests/Chap27/TestScanContractMtEph.rs`
**Test Cases:**
- Same as StEph but with larger sizes
- Verify parallel correctness
- Test with 100+ elements

---

## Benchmark Files (4)

All benchmarks follow APAS timing rules:
- Warm-up: ≤ 1s
- Measurement: ≈ 6s
- Sample size: ≈ 30

### 1. `benches/Chap27/BenchReduceContractStEph.rs`
- reduce_contract_100: sequence of 100 elements
- reduce_contract_1000: sequence of 1000 elements

### 2. `benches/Chap27/BenchScanContractStEph.rs`
- scan_contract_100: sequence of 100 elements
- scan_contract_1000: sequence of 1000 elements

### 3. `benches/Chap27/BenchReduceContractMtEph.rs`
- reduce_contract_parallel_1000: sequence of 1000 elements
- reduce_contract_parallel_10000: sequence of 10000 elements

### 4. `benches/Chap27/BenchScanContractMtEph.rs`
- scan_contract_parallel_1000: sequence of 1000 elements
- scan_contract_parallel_10000: sequence of 10000 elements

---

## Dependencies

**From Chap18:**
- `ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait}`
- For Mt implementations

**From Chap19:**
- `ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait}`
- For St implementations

**From Types:**
- `StT`, `StTInMtT`, `N`, `B`

**Macros:**
- `ParaPair!` for parallel implementations

---

## File Updates

### `src/lib.rs`
```rust
pub mod Chap27 {
    pub mod ReduceContractStEph;
    pub mod ScanContractStEph;
    pub mod ReduceContractMtEph;
    pub mod ScanContractMtEph;
}
```

### `Cargo.toml`
Add 4 test registrations:
```toml
[[test]]
name = "TestReduceContractStEph"
path = "tests/Chap27/TestReduceContractStEph.rs"

[[test]]
name = "TestScanContractStEph"
path = "tests/Chap27/TestScanContractStEph.rs"

[[test]]
name = "TestReduceContractMtEph"
path = "tests/Chap27/TestReduceContractMtEph.rs"

[[test]]
name = "TestScanContractMtEph"
path = "tests/Chap27/TestScanContractMtEph.rs"
```

Add 4 benchmark registrations:
```toml
[[bench]]
name = "BenchReduceContractStEph"
path = "benches/Chap27/BenchReduceContractStEph.rs"
harness = false

[[bench]]
name = "BenchScanContractStEph"
path = "benches/Chap27/BenchScanContractStEph.rs"
harness = false

[[bench]]
name = "BenchReduceContractMtEph"
path = "benches/Chap27/BenchReduceContractMtEph.rs"
harness = false

[[bench]]
name = "BenchScanContractMtEph"
path = "benches/Chap27/BenchScanContractMtEph.rs"
harness = false
```

---

## Implementation Notes

### Contraction Pattern
All implementations follow the explicit contraction pattern:
1. **Base Case:** Handle small instances (n ≤ 1)
2. **Contract:** Map to geometrically smaller instance
3. **Solve:** Recurse on smaller instance
4. **Expand:** Build final result from recursive solution

### Algorithmic Differences from Chap18/19
- These implementations **explicitly demonstrate contraction**
- Single recursive call (not divide-and-conquer with multiple calls)
- Shows contract→solve→expand pattern clearly
- Educational value in showing the technique

### Non-Power-of-2 Handling
For sequences not power-of-2:
- **Reduce:** Handle odd-length sequences by keeping last element separate
- **Scan:** Similar strategy, ensure all positions computed correctly

---

## Time Estimate

### Source Files (~80 minutes)
- ReduceContractStEph.rs: 15 min (simpler algorithm)
- ScanContractStEph.rs: 25 min (more complex expansion)
- ReduceContractMtEph.rs: 15 min (parallel contraction)
- ScanContractMtEph.rs: 25 min (parallel expansion)

### Test Files (~60 minutes)
- TestReduceContractStEph.rs: 15 min
- TestScanContractStEph.rs: 15 min
- TestReduceContractMtEph.rs: 15 min
- TestScanContractMtEph.rs: 15 min

### Benchmark Files (~40 minutes)
- 4 benchmarks × 10 min each

### Integration (~20 minutes)
- Cargo.toml updates: 3 min
- src/lib.rs updates: 2 min
- Build and test all: 10 min
- Format and final checks: 5 min

### **Total: ~200 minutes (~3.3 hours)**

---

## Parallel Analysis

**Parallel Modules:** 2 out of 4
- ✅ `ReduceContractMtEph` - Parallelism Θ(n/log n)
- ✅ `ScanContractMtEph` - Parallelism Θ(n/log n)

Both use:
- ParaPair! for recursive calls
- Parallel map/tabulate for contraction/expansion
- Unconditional parallelism (no thresholding)
- Increased thread stack sizes

---

## Summary

**Total Files:** 12
- 4 source modules
- 4 test files
- 4 benchmark files

**Parallel:** 2 modules (50%)

**Estimated Time:** ~3.3 hours

**Key Features:**
- ✅ Explicit contraction technique demonstration
- ✅ Zero Vec usage
- ✅ Full algorithmic analysis
- ✅ Unconditional parallelism in Mt files
- ✅ APAS benchmark compliance
- ✅ Copyright headers

