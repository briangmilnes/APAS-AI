# Chapter 59: Johnson's Algorithm - Implementation Plan

**Date:** October 1, 2025  
**Estimated Time:** ~120 minutes (~2 hours)

---

## ALGORITHMS IDENTIFIED

### Algorithm 59.1: Johnson's All-Pairs Shortest Paths
- **Purpose:** Solve APSP problem with negative weights allowed
- **Method:** 
  1. Add dummy source, run Bellman-Ford to compute potentials
  2. Reweight edges to eliminate negative weights: w'(u,v) = w(u,v) + p(u) - p(v)
  3. Run Dijkstra from each vertex in parallel on reweighted graph
  4. Adjust final distances: δG(u,v) = δG'(u,v) - p(u) + p(v)
- **Complexity:**
  - Work: O(mn log n)
  - Span: O(m log n)
  - Parallelism: Θ(n) - significant parallelism in Phase 2
- **Graph Type:** **DIRECTED ONLY**
- **Weights:** Negative weights allowed, no negative infinity needed

---

## UNDIRECTED GRAPH NEEDED?

**NO.** Johnson's algorithm is for directed graphs only (APSP problem).

---

## NEGATIVE WEIGHTS?

**YES.** Our existing `WeightedDirGraphStEphInt` and `WeightedDirGraphStEphFloat` already support negative weights (i32 and OrderedFloat<f64>).

**NO NEGATIVE INFINITY** needed per user requirement.

---

## PARALLELISM ANALYSIS

### Phase 1: Bellman-Ford with Dummy Source
- **Parallelism:** O(n log n) span (already implemented in Chap58)
- **Sequential:** Single Bellman-Ford run

### Phase 2: Dijkstra from Each Vertex
- **Parallelism:** Θ(n) - **SIGNIFICANT!**
- **Each Dijkstra:** Independent, can run in parallel
- **Mt Implementation:** Use rayon to parallelize over vertices

**Total Parallelism:** Θ(n) - excellent for large graphs

---

## DEPENDENCIES

### Existing Modules (Chap06)
- ✓ `WeightedDirGraphStEphInt<V>` (i32 weights, negative OK)
- ✓ `WeightedDirGraphStEphFloat<V>` (OrderedFloat<f64> weights, negative OK)
- ✓ `WeightedDirGraphMtEphInt<V>` (for parallel version)
- ✓ `WeightedDirGraphMtEphFloat<V>` (for parallel version)

### Existing Modules (Chap56)
- ✓ `AllPairsResultStEphInt` (reuse for APSP results)
- ✓ `AllPairsResultStEphFloat` (reuse for APSP results)

### Existing Modules (Chap57)
- ✓ `DijkstraStEphInt` (Algorithm 57.2)
- ✓ `DijkstraStEphFloat` (Algorithm 57.2)

### Existing Modules (Chap58)
- ✓ `BellmanFordStEphInt` (Algorithm 58.2)
- ✓ `BellmanFordStEphFloat` (Algorithm 58.2)

---

## FILES TO CREATE

### Source Files (4)

1. **`src/Chap59/JohnsonStEphInt.rs`** (St, Eph, Int)
   - `johnson_apsp(graph, vertex_list) -> AllPairsResultStEphInt`
   - Helper: `reweight_graph(graph, potentials) -> WeightedDirGraphStEphInt`
   - Helper: `adjust_distances(dijkstra_result, potentials) -> distances`

2. **`src/Chap59/JohnsonStEphFloat.rs`** (St, Eph, Float)
   - `johnson_apsp(graph, vertex_list) -> AllPairsResultStEphFloat`
   - Helper: `reweight_graph(graph, potentials) -> WeightedDirGraphStEphFloat`
   - Helper: `adjust_distances(dijkstra_result, potentials) -> distances`

3. **`src/Chap59/JohnsonMtEphInt.rs`** (Mt, Eph, Int)
   - `johnson_apsp_parallel(graph, vertex_list) -> AllPairsResultStEphInt`
   - **PARALLEL:** Run Dijkstra from each vertex using rayon
   - Reuses St helpers

4. **`src/Chap59/JohnsonMtEphFloat.rs`** (Mt, Eph, Float)
   - `johnson_apsp_parallel(graph, vertex_list) -> AllPairsResultStEphFloat`
   - **PARALLEL:** Run Dijkstra from each vertex using rayon
   - Reuses St helpers

5. **`src/Chap59/mod.rs`**
   - Module declarations

### Test Files (4)

6. **`tests/Chap59/TestJohnsonStEphInt.rs`**
   - Test Example 59.1 (textbook example)
   - Test negative weights
   - Test dummy source addition
   - Test reweighting correctness
   - Test against Bellman-Ford baseline

7. **`tests/Chap59/TestJohnsonStEphFloat.rs`**
   - Similar tests with float weights

8. **`tests/Chap59/TestJohnsonMtEphInt.rs`**
   - Verify Mt matches St results
   - Test parallelism correctness

9. **`tests/Chap59/TestJohnsonMtEphFloat.rs`**
   - Verify Mt matches St results

### Benchmark Files (4)

10. **`benches/Chap59/BenchJohnsonStEphInt.rs`**
    - Benchmark vs naive Bellman-Ford approach
    - Various graph sizes (n=10, 20, 30)
    - Sparse and dense graphs

11. **`benches/Chap59/BenchJohnsonStEphFloat.rs`**
    - Similar benchmarks with float weights

12. **`benches/Chap59/BenchJohnsonMtEphInt.rs`**
    - Compare St vs Mt speedup
    - Measure parallel efficiency

13. **`benches/Chap59/BenchJohnsonMtEphFloat.rs`**
    - Compare St vs Mt speedup

---

## DETAILED TASK BREAKDOWN

### PHASE 1: Source Files (48 tasks = 4 files × 12 tasks/file)

#### Task Group: JohnsonStEphInt (12 tasks)
1. Create `src/Chap59/JohnsonStEphInt.rs` with `johnson_apsp()` implementation
2. Run RustRules checklist on JohnsonStEphInt.rs
3. Run APAS checklist on JohnsonStEphInt.rs
4. Check for Vec usage in JohnsonStEphInt.rs
5. Run cargo build (JohnsonStEphInt.rs)
6. Create `src/Chap59/mod.rs` with JohnsonStEphInt declaration
7. Run RustRules checklist on mod.rs
8. Run cargo build (mod.rs)
9. Update `src/lib.rs` to expose Chap59
10. Run cargo build (lib.rs)
11. Run AlgorithmicAnalysis on JohnsonStEphInt.rs
12. Verify zero warnings

#### Task Group: JohnsonStEphFloat (12 tasks)
13. Create `src/Chap59/JohnsonStEphFloat.rs` with `johnson_apsp()` implementation
14. Run RustRules checklist on JohnsonStEphFloat.rs
15. Run APAS checklist on JohnsonStEphFloat.rs
16. Check for Vec usage in JohnsonStEphFloat.rs
17. Run cargo build (JohnsonStEphFloat.rs)
18. Update `src/Chap59/mod.rs` with JohnsonStEphFloat declaration
19. Run RustRules checklist on mod.rs
20. Run cargo build (mod.rs)
21. Update `src/lib.rs` if needed
22. Run cargo build (lib.rs)
23. Run AlgorithmicAnalysis on JohnsonStEphFloat.rs
24. Verify zero warnings

#### Task Group: JohnsonMtEphInt (12 tasks)
25. Create `src/Chap59/JohnsonMtEphInt.rs` with parallel `johnson_apsp_parallel()`
26. Run RustRules checklist on JohnsonMtEphInt.rs
27. Run APAS checklist on JohnsonMtEphInt.rs
28. Check for Vec usage in JohnsonMtEphInt.rs
29. Run cargo build (JohnsonMtEphInt.rs)
30. Update `src/Chap59/mod.rs` with JohnsonMtEphInt declaration
31. Run RustRules checklist on mod.rs
32. Run cargo build (mod.rs)
33. Update `src/lib.rs` if needed
34. Run cargo build (lib.rs)
35. Run AlgorithmicAnalysis on JohnsonMtEphInt.rs
36. Verify zero warnings

#### Task Group: JohnsonMtEphFloat (12 tasks)
37. Create `src/Chap59/JohnsonMtEphFloat.rs` with parallel `johnson_apsp_parallel()`
38. Run RustRules checklist on JohnsonMtEphFloat.rs
39. Run APAS checklist on JohnsonMtEphFloat.rs
40. Check for Vec usage in JohnsonMtEphFloat.rs
41. Run cargo build (JohnsonMtEphFloat.rs)
42. Update `src/Chap59/mod.rs` with JohnsonMtEphFloat declaration
43. Run RustRules checklist on mod.rs
44. Run cargo build (mod.rs)
45. Update `src/lib.rs` if needed
46. Run cargo build (lib.rs)
47. Run AlgorithmicAnalysis on JohnsonMtEphFloat.rs
48. Verify zero warnings

### PHASE 2: Test Files (48 tasks = 4 files × 12 tasks/file)

#### Task Group: TestJohnsonStEphInt (12 tasks)
49. Create `tests/Chap59/TestJohnsonStEphInt.rs` with Example 59.1
50. Run RustRules checklist on TestJohnsonStEphInt.rs
51. Run APAS checklist on TestJohnsonStEphInt.rs
52. Check for Vec usage in TestJohnsonStEphInt.rs
53. Run cargo build (TestJohnsonStEphInt.rs)
54. Run cargo nextest run TestJohnsonStEphInt
55. Fix any test failures
56. Verify all tests pass
57. Add test for negative weights
58. Add test for dummy source correctness
59. Add test for reweighting
60. Run cargo nextest run TestJohnsonStEphInt (final)

#### Task Group: TestJohnsonStEphFloat (12 tasks)
61-72. Same as tasks 49-60 but for TestJohnsonStEphFloat

#### Task Group: TestJohnsonMtEphInt (12 tasks)
73-84. Same as tasks 49-60 but for TestJohnsonMtEphInt (verify Mt = St results)

#### Task Group: TestJohnsonMtEphFloat (12 tasks)
85-96. Same as tasks 49-60 but for TestJohnsonMtEphFloat

### PHASE 3: Benchmark Files (52 tasks = 4 files × 13 tasks/file)

#### Task Group: BenchJohnsonStEphInt (13 tasks)
97. Create `benches/Chap59/BenchJohnsonStEphInt.rs`
98. Run RustRules checklist on BenchJohnsonStEphInt.rs
99. Run APAS checklist on BenchJohnsonStEphInt.rs
100. Check for Vec usage in BenchJohnsonStEphInt.rs
101. Update Cargo.toml with bench entry
102. Run cargo build --release --benches (BenchJohnsonStEphInt)
103. Run cargo bench --bench BenchJohnsonStEphInt --no-run
104. Fix any compilation issues
105. Add warmup=300ms, measurement=1s timing config
106. Add sparse graph benchmarks
107. Add dense graph benchmarks
108. Verify benchmark runs successfully
109. Verify zero warnings

#### Task Group: BenchJohnsonStEphFloat (13 tasks)
110-122. Same as tasks 97-109 but for BenchJohnsonStEphFloat

#### Task Group: BenchJohnsonMtEphInt (13 tasks)
123-135. Same as tasks 97-109 but for BenchJohnsonMtEphInt (add St vs Mt comparison)

#### Task Group: BenchJohnsonMtEphFloat (13 tasks)
136-148. Same as tasks 97-109 but for BenchJohnsonMtEphFloat

### PHASE 4: Final Verification (5 tasks)

149. Run full cargo build (verify entire codebase)
150. Run full cargo nextest run (verify all tests pass)
151. Run cargo build --release --benches (verify all benchmarks compile)
152. Verify zero warnings across entire codebase
153. Run PostPlanChecklist.md verification

### PHASE 5: Documentation (1 task)

154. Create summary document: `CodeReviews/Chap59_Johnson_Summary.md`

---

## IMPLEMENTATION NOTES

### Key Algorithm Steps

1. **Add Dummy Source:** Create graph G+ with new vertex s and zero-weight edges to all vertices
2. **Run Bellman-Ford:** Compute potentials p(v) = δG+(s, v) for all v
3. **Reweight Edges:** w'(u,v) = w(u,v) + p(u) - p(v) (eliminates negative weights)
4. **Run Dijkstra from Each Vertex:** On reweighted graph G'
5. **Adjust Distances:** δG(u,v) = δG'(u,v) - p(u) + p(v)

### Infinity Representation
- **Int:** `i64::MAX` for unreachable vertices
- **Float:** `OrderedFloat(f64::INFINITY)` for unreachable vertices
- **NO NEGATIVE INFINITY** per user requirement

### Parallelization Strategy (Mt versions)
```rust
use rayon::prelude::*;

// Parallel Dijkstra from each vertex
let results: Vec<_> = vertices.par_iter()
    .map(|&u| {
        let dijkstra_result = dijkstra_st_eph_int::dijkstra(&reweighted_graph, u);
        adjust_distances(dijkstra_result, &potentials, u)
    })
    .collect();
```

### Data Structures
- Use `ArraySeqStEphS<T>` for distance/predecessor arrays (NOT Vec)
- Use `SetStEph<V>` for vertex sets
- Use existing `AllPairsResultStEphInt/Float` for return types

---

## TIME ESTIMATE

### Breakdown by Phase
- **Phase 1: Source Files** (4 files × 12 tasks) = 48 minutes (1 min/task)
- **Phase 2: Test Files** (4 files × 12 tasks) = 40 minutes (0.83 min/task)
- **Phase 3: Benchmark Files** (4 files × 13 tasks) = 26 minutes (0.5 min/task)
- **Phase 4: Final Verification** = 5 minutes
- **Phase 5: Documentation** = 1 minute

**TOTAL ESTIMATED TIME: ~120 minutes (~2 hours)**

---

## READY TO EXECUTE?

**Waiting for user instruction to proceed.**

Per user requirement: "I will tell you when to execute. Wait for me to tell you when to fix, build and test."

---

## PREPLAN CHECKLIST COMPLIANCE

✓ Plan clearly understood  
✓ File-by-file TODO items created (154 tasks)  
✓ New src files identified (4 + mod.rs)  
✓ Dependencies on existing modules identified (Chap06, Chap56, Chap57, Chap58)  
✓ No *Per files needed (only *Eph versions)  
✓ One file at a time scheduled  
✓ Methods identified for each file  
✓ Bottom-up ordering (St before Mt)  
✓ Data structures specified (ArraySeqStEphS, not Vec)  
✓ Parallel functions identified (Mt versions)  
✓ RustRules + APAS checklists scheduled per file  
✓ cargo build scheduled per file  
✓ Test files identified (4)  
✓ Vec checks scheduled per file  
✓ cargo nextest scheduled per test file  
✓ Benchmark files identified (4)  
✓ cargo bench --no-run scheduled per benchmark  
✓ Full cargo build at end  
✓ Full cargo nextest at end  
✓ Final summary step included  
✓ Time estimated (120 minutes)  
✓ AlgorithmicAnalysis scheduled  
✓ PostPlanChecklist scheduled  
✓ Detailed file-level TODOs created  

