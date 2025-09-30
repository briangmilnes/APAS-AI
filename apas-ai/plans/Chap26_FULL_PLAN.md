# CHAPTER 26: DIVIDE AND CONQUER - IMPLEMENTATION PLAN
## Using PrePlanChecklist.md

## OVERVIEW
Implement 4 source files, 4 test files, 4 benchmark files for Chapter 26.
All work in Chap26 directory.

---

## FILES TO CREATE

### Source Files (in order):
1. `src/Chap26/MergeSortSt.rs` - Sequential merge sort
2. `src/Chap26/MergeSortMt.rs` - Parallel merge sort
3. `src/Chap26/DivConReduceSt.rs` - D&C via reduce pattern (sequential)
4. `src/Chap26/DivConReduceMt.rs` - D&C via reduce pattern (parallel)

### Test Files (in order):
5. `tests/Chap26/TestMergeSortSt.rs`
6. `tests/Chap26/TestMergeSortMt.rs`
7. `tests/Chap26/TestDivConReduceSt.rs`
8. `tests/Chap26/TestDivConReduceMt.rs`

### Benchmark Files (in order):
9. `benches/Chap26/BenchMergeSortSt.rs`
10. `benches/Chap26/BenchMergeSortMt.rs`
11. `benches/Chap26/BenchDivConReduceSt.rs`
12. `benches/Chap26/BenchDivConReduceMt.rs`

---

## DEPENDENCIES

### From Chap18:
- `ArraySeqStPer::ArraySeqStPerS<T>` - sequential sequences
- `ArraySeqMtPer::ArraySeqMtPerS<T>` - multi-threaded sequences
- Methods: `reduce()`, `map()`, `length()`, `nth()`, `subseq_copy()`, `append()`

### From Chap19:
- `ArraySeqStPer::ArraySeqStPerS<T>` (more complete version)
- `ArraySeqMtPer::ArraySeqMtPerS<T>` (more complete version)

### From Types:
- `ParaPair!` macro for symmetric parallelism
- `StT`, `MtT` type constraints
- `N`, `B`, `O` type aliases
- `Pair` type

### From Chap36:
- Reference pattern for parallel algorithms with unconditional thread spawning

---

## DETAILED IMPLEMENTATION SPECS

### 1. MergeSortSt.rs
**Methods:**
- `merge(left: &ArraySeqStPerS<T>, right: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>`
  - Sequential two-pointer merge
  - Work Θ(n), Span Θ(n), Parallelism Θ(1)
- `merge_sort(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>`
  - Recursive divide-and-conquer
  - Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
  - Base case: n <= 1
  - Divide: splitMid
  - Recur: sequential recursive calls
  - Combine: merge

**Trait:** `MergeSortStTrait<T: StT + Ord>`

**Data structures:** Uses `ArraySeqStPerS<T>` from Chap19

**No Vec usage** - all sequences use ArraySeqStPerS

### 2. MergeSortMt.rs
**Methods:**
- `merge_parallel(left: &ArraySeqMtPerS<T>, right: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>`
  - Parallel merge using binary search + ParaPair!
  - Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
- `merge_sort_parallel(a: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>`
  - Parallel divide-and-conquer with ParaPair!
  - Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log n)
  - Base case: n <= 1
  - Divide: splitMid
  - Recur: **UNCONDITIONALLY PARALLEL** with ParaPair! (no thresholding!)
  - Combine: merge_parallel

**Trait:** `MergeSortMtTrait<T: StT + Ord + Send + Sync>`

**Data structures:** Uses `ArraySeqMtPerS<T>` from Chap19

**Parallel operations:** merge_sort_parallel uses ParaPair! unconditionally

**No Vec usage** - all sequences use ArraySeqMtPerS

**No thresholding** - parallel at all sizes per APAS rules

### 3. DivConReduceSt.rs
**Methods (demonstrating D&C as reduce pattern):**
- `max_element(a: &ArraySeqStPerS<N>) -> Option<N>`
  - Pattern: `reduce max MIN identity`
  - Work Θ(n), Span Θ(n), Parallelism Θ(1)
- `sum(a: &ArraySeqStPerS<N>) -> N`
  - Pattern: `reduce (+) 0 identity`
  - Work Θ(n), Span Θ(n), Parallelism Θ(1)
- `product(a: &ArraySeqStPerS<N>) -> N`
  - Pattern: `reduce (*) 1 identity`
  - Work Θ(n), Span Θ(n), Parallelism Θ(1)
- `any(a: &ArraySeqStPerS<B>) -> B`
  - Pattern: `reduce (||) false identity`
  - Work Θ(n), Span Θ(n), Parallelism Θ(1)
- `all(a: &ArraySeqStPerS<B>) -> B`
  - Pattern: `reduce (&&) true identity`
  - Work Θ(n), Span Θ(n), Parallelism Θ(1)

**Trait:** `DivConReduceStTrait`

**Data structures:** Uses `ArraySeqStPerS<T>` from Chap19

**Delegates to:** `ArraySeqStPerS::reduce()` and `ArraySeqStPerS::map()`

**No Vec usage** - all sequences use ArraySeqStPerS

### 4. DivConReduceMt.rs
**Methods (same as St but parallel):**
- `max_element_parallel(a: &ArraySeqMtPerS<N>) -> Option<N>`
  - Pattern: `reduce max MIN identity` (parallel)
  - Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
- `sum_parallel(a: &ArraySeqMtPerS<N>) -> N`
  - Pattern: `reduce (+) 0 identity` (parallel)
  - Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
- `product_parallel(a: &ArraySeqMtPerS<N>) -> N`
  - Pattern: `reduce (*) 1 identity` (parallel)
  - Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
- `any_parallel(a: &ArraySeqMtPerS<B>) -> B`
  - Pattern: `reduce (||) false identity` (parallel)
  - Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
- `all_parallel(a: &ArraySeqMtPerS<B>) -> B`
  - Pattern: `reduce (&&) true identity` (parallel)
  - Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)

**Trait:** `DivConReduceMtTrait`

**Data structures:** Uses `ArraySeqMtPerS<T>` from Chap19

**Delegates to:** `ArraySeqMtPerS::reduce()` and `ArraySeqMtPerS::map()`

**Parallel operations:** All methods use parallel reduce (which uses ParaPair!)

**No Vec usage** - all sequences use ArraySeqMtPerS

**No thresholding** - parallel reduce is already unconditionally parallel

---

## BENCHMARK TIMING REQUIREMENTS
Per APAS rules (memory 9454288):
- Warm-up ≤ 1s
- Measurement ≈ 6s
- Sample size ≈ 30
- Total run ≤ 10s per benchmark group
- Explicit warmup and measurement time limits (no defaults)

---

## REGISTRATION

### Cargo.toml additions:
```toml
# Tests
[[test]]
name = "TestMergeSortSt"
path = "tests/Chap26/TestMergeSortSt.rs"

[[test]]
name = "TestMergeSortMt"
path = "tests/Chap26/TestMergeSortMt.rs"

[[test]]
name = "TestDivConReduceSt"
path = "tests/Chap26/TestDivConReduceSt.rs"

[[test]]
name = "TestDivConReduceMt"
path = "tests/Chap26/TestDivConReduceMt.rs"

# Benchmarks
[[bench]]
name = "BenchMergeSortSt"
path = "benches/Chap26/BenchMergeSortSt.rs"
harness = false

[[bench]]
name = "BenchMergeSortMt"
path = "benches/Chap26/BenchMergeSortMt.rs"
harness = false

[[bench]]
name = "BenchDivConReduceSt"
path = "benches/Chap26/BenchDivConReduceSt.rs"
harness = false

[[bench]]
name = "BenchDivConReduceMt"
path = "benches/Chap26/BenchDivConReduceMt.rs"
harness = false
```

### src/lib.rs additions:
```rust
pub mod Chap26 {
    pub mod MergeSortSt;
    pub mod MergeSortMt;
    pub mod DivConReduceSt;
    pub mod DivConReduceMt;
}
```

---

## TIME ESTIMATE

| Task | Files | Subtasks | Est. Time |
|------|-------|----------|-----------|
| Create src directories | 1 | mkdir | 1 min |
| MergeSortSt.rs | 1 | write, build, check | 20 min |
| MergeSortMt.rs | 1 | write, build, check | 25 min |
| DivConReduceSt.rs | 1 | write, build, check | 15 min |
| DivConReduceMt.rs | 1 | write, build, check | 15 min |
| Create test directories | 1 | mkdir | 1 min |
| TestMergeSortSt.rs | 1 | write, test, check | 15 min |
| TestMergeSortMt.rs | 1 | write, test, check | 15 min |
| TestDivConReduceSt.rs | 1 | write, test, check | 12 min |
| TestDivConReduceMt.rs | 1 | write, test, check | 12 min |
| Create bench directories | 1 | mkdir | 1 min |
| BenchMergeSortSt.rs | 1 | write, compile, check | 10 min |
| BenchMergeSortMt.rs | 1 | write, compile, check | 10 min |
| BenchDivConReduceSt.rs | 1 | write, compile, check | 8 min |
| BenchDivConReduceMt.rs | 1 | write, compile, check | 8 min |
| Update Cargo.toml | 1 | register tests/benchmarks | 3 min |
| Update src/lib.rs | 1 | register modules | 2 min |
| Final cargo build | 1 | full build | 2 min |
| Final cargo nextest | 1 | all tests | 3 min |
| Algorithmic analysis check | 4 | per-file review | 8 min |
| PostPlanChecklist | 1 | final review | 3 min |
| **TOTAL** | **20** | **75+** | **~190 min** |

**Estimated Total Time: ~3 hours 10 minutes**

---

## EXECUTION READINESS
- [x] Plan is clearly understood
- [x] Bottom-up ordering (St before Mt)
- [x] File-by-file incremental builds
- [x] One TODO per file per task
- [x] Dependencies identified
- [x] Data structures specified
- [x] No Vec for known-length sequences
- [x] Parallel operations identified (Mt files)
- [x] Test files planned
- [x] Benchmark files planned
- [x] No thresholding in Mt files
- [x] APAS benchmark timing rules
- [x] Algorithmic analysis included
- [x] PostPlanChecklist included
- [x] Time estimated

**Ready to execute on your command.**


---

## DETAILED TODO LIST
(To be created when execution begins)

### SOURCE FILES (4 files, ~75 minutes)
1. ✓ Create src/Chap26/ directory
2. ✓ Create src/Chap26/MergeSortSt.rs
3. ✓ Add MergeSortSt to src/lib.rs
4. ✓ Cargo build (MergeSortSt)
5. ✓ Check Vec usage in MergeSortSt
6. ✓ Run RustRules checklist on MergeSortSt
7. ✓ Run APAS checklist on MergeSortSt
8. ✓ Add algorithmic analysis to MergeSortSt
9. ✓ Create src/Chap26/MergeSortMt.rs
10. ✓ Add MergeSortMt to src/lib.rs
11. ✓ Cargo build (MergeSortMt)
12. ✓ Check Vec usage in MergeSortMt
13. ✓ Check for thresholding violations in MergeSortMt
14. ✓ Run RustRules checklist on MergeSortMt
15. ✓ Run APAS checklist on MergeSortMt
16. ✓ Add algorithmic analysis to MergeSortMt
17. ✓ Create src/Chap26/DivConReduceSt.rs
18. ✓ Add DivConReduceSt to src/lib.rs
19. ✓ Cargo build (DivConReduceSt)
20. ✓ Check Vec usage in DivConReduceSt
21. ✓ Run RustRules checklist on DivConReduceSt
22. ✓ Run APAS checklist on DivConReduceSt
23. ✓ Add algorithmic analysis to DivConReduceSt
24. ✓ Create src/Chap26/DivConReduceMt.rs
25. ✓ Add DivConReduceMt to src/lib.rs
26. ✓ Cargo build (DivConReduceMt)
27. ✓ Check Vec usage in DivConReduceMt
28. ✓ Check for thresholding violations in DivConReduceMt
29. ✓ Run RustRules checklist on DivConReduceMt
30. ✓ Run APAS checklist on DivConReduceMt
31. ✓ Add algorithmic analysis to DivConReduceMt

### TEST FILES (4 files, ~54 minutes)
32. ✓ Create tests/Chap26/ directory
33. ✓ Create tests/Chap26/TestMergeSortSt.rs
34. ✓ Add TestMergeSortSt to Cargo.toml
35. ✓ Cargo nextest --test TestMergeSortSt
36. ✓ Check Vec usage in TestMergeSortSt
37. ✓ Run RustRules checklist on TestMergeSortSt
38. ✓ Run APAS checklist on TestMergeSortSt
39. ✓ Create tests/Chap26/TestMergeSortMt.rs
40. ✓ Add TestMergeSortMt to Cargo.toml
41. ✓ Cargo nextest --test TestMergeSortMt
42. ✓ Check Vec usage in TestMergeSortMt
43. ✓ Run RustRules checklist on TestMergeSortMt
44. ✓ Run APAS checklist on TestMergeSortMt
45. ✓ Create tests/Chap26/TestDivConReduceSt.rs
46. ✓ Add TestDivConReduceSt to Cargo.toml
47. ✓ Cargo nextest --test TestDivConReduceSt
48. ✓ Check Vec usage in TestDivConReduceSt
49. ✓ Run RustRules checklist on TestDivConReduceSt
50. ✓ Run APAS checklist on TestDivConReduceSt
51. ✓ Create tests/Chap26/TestDivConReduceMt.rs
52. ✓ Add TestDivConReduceMt to Cargo.toml
53. ✓ Cargo nextest --test TestDivConReduceMt
54. ✓ Check Vec usage in TestDivConReduceMt
55. ✓ Run RustRules checklist on TestDivConReduceMt
56. ✓ Run APAS checklist on TestDivConReduceMt

### BENCHMARK FILES (4 files, ~36 minutes)
57. ✓ Create benches/Chap26/ directory
58. ✓ Create benches/Chap26/BenchMergeSortSt.rs
59. ✓ Add BenchMergeSortSt to Cargo.toml
60. ✓ Cargo bench --bench BenchMergeSortSt --no-run
61. ✓ Check Vec usage in BenchMergeSortSt
62. ✓ Verify APAS benchmark timing rules in BenchMergeSortSt
63. ✓ Run RustRules checklist on BenchMergeSortSt
64. ✓ Run APAS checklist on BenchMergeSortSt
65. ✓ Create benches/Chap26/BenchMergeSortMt.rs
66. ✓ Add BenchMergeSortMt to Cargo.toml
67. ✓ Cargo bench --bench BenchMergeSortMt --no-run
68. ✓ Check Vec usage in BenchMergeSortMt
69. ✓ Verify APAS benchmark timing rules in BenchMergeSortMt
70. ✓ Run RustRules checklist on BenchMergeSortMt
71. ✓ Run APAS checklist on BenchMergeSortMt
72. ✓ Create benches/Chap26/BenchDivConReduceSt.rs
73. ✓ Add BenchDivConReduceSt to Cargo.toml
74. ✓ Cargo bench --bench BenchDivConReduceSt --no-run
75. ✓ Check Vec usage in BenchDivConReduceSt
76. ✓ Verify APAS benchmark timing rules in BenchDivConReduceSt
77. ✓ Run RustRules checklist on BenchDivConReduceSt
78. ✓ Run APAS checklist on BenchDivConReduceSt
79. ✓ Create benches/Chap26/BenchDivConReduceMt.rs
80. ✓ Add BenchDivConReduceMt to Cargo.toml
81. ✓ Cargo bench --bench BenchDivConReduceMt --no-run
82. ✓ Check Vec usage in BenchDivConReduceMt
83. ✓ Verify APAS benchmark timing rules in BenchDivConReduceMt
84. ✓ Run RustRules checklist on BenchDivConReduceMt
85. ✓ Run APAS checklist on BenchDivConReduceMt

### FINAL STEPS (~18 minutes)
86. ✓ Cargo +nightly fmt (all files)
87. ✓ Full cargo build --lib
88. ✓ Full cargo nextest run (all Chap26 tests)
89. ✓ Verify algorithmic analysis on all 4 source files
90. ✓ Run PostPlanChecklist.md
91. ✓ Create summary of changes
92. ✓ Commit all changes

**TOTAL: 92 TODO items over ~190 minutes (~3 hours 10 minutes)**

