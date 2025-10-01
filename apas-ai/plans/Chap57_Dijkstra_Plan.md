# Chapter 57: Dijkstra's Algorithm - Implementation Plan (Ephemeral Only)

## Overview

**Chapter:** 57 - Dijkstra's Algorithm  
**Algorithms:** 
1. Algorithm 57.1: Dijkstra's Algorithm (conceptual priority-first search)
2. Algorithm 57.2: Dijkstra's Algorithm using Priority Queues
3. Exercise 57.1: Dijkstra with decreaseKey variant

**Parallelism:** NONE - "It is a sequential algorithm" per textbook  
**Variants:** StEph ONLY (no Mt, no Per)  
**Weight Types:** Integer (i64) and Float (OrderedFloat<f64>)

---

## Infinity Representation

**Float weights:** Use `OrderedFloat(f64::INFINITY)` for unreachable vertices  
**Integer weights:** Use `i64::MAX` as sentinel for unreachable vertices  
**Note:** Dijkstra requires non-negative edge weights only (SSSP+ problem)

---

## Files to Create: 19 Total

### Source Files (6)
1. `src/Chap57/StackStEph.rs`
2. `src/Chap57/DijkstraStEphInt.rs`
3. `src/Chap57/DijkstraStEphFloat.rs`
4. `src/Chap57/DijkstraDecreaseKeyStEphInt.rs`
5. `src/Chap57/DijkstraDecreaseKeyStEphFloat.rs`
6. `src/Chap45/BinaryHeapPQDecreaseKey.rs` (if needed for decreaseKey support)
7. Update `src/lib.rs` with Chap57 exports

### Test Files (5)
8. `tests/Chap57/TestStackStEph.rs`
9. `tests/Chap57/TestDijkstraStEphInt.rs`
10. `tests/Chap57/TestDijkstraStEphFloat.rs`
11. `tests/Chap57/TestDijkstraDecreaseKeyStEphInt.rs`
12. `tests/Chap57/TestDijkstraDecreaseKeyStEphFloat.rs`

### Benchmark Files (5)
13. `benches/Chap57/BenchStackStEph.rs`
14. `benches/Chap57/BenchDijkstraStEphInt.rs`
15. `benches/Chap57/BenchDijkstraStEphFloat.rs`
16. `benches/Chap57/BenchDijkstraDecreaseKeyStEphInt.rs`
17. `benches/Chap57/BenchDijkstraDecreaseKeyStEphFloat.rs`

---

## Dependencies

### Existing Modules to Use
- **Graphs:** `WeightedDirGraphStEphInt`, `WeightedDirGraphStEphFloat` from Chap06
- **Results:** `SSSPResultStEphInt`, `SSSPResultStEphFloat` from Chap56
- **Priority Queue:** `BinaryHeapPQ` from Chap45 (O(log n) insert/deleteMin)
- **Sequences:** `ArraySeqStEphS` from Chap18
- **Sets:** `AVLTreeSetStEph` from Chap41 for visited tracking

### Modules That May Need Extension
- `BinaryHeapPQ` - may need decreaseKey operation added for Exercise 57.1

---

## Detailed TODO List

### Setup
- [ ] **Task 0.1**: Create `src/Chap57/` directory
- [ ] **Task 0.2**: Create `tests/Chap57/` directory
- [ ] **Task 0.3**: Create `benches/Chap57/` directory

---

## Phase 1: Stack Module

### Source File

- [ ] **Task 1.1**: Create `src/Chap57/StackStEph.rs`
  - **Purpose:** Ephemeral (mutable) stack implementation
  - **Data Structure:** Use `ArraySeqStEphS<T>` as backing storage
  - **Methods to implement:**
    - `new() -> Self` - Create empty stack
    - `push(&mut self, item: T)` - Push item onto stack
    - `pop(&mut self) -> Option<T>` - Pop and return top item
    - `peek(&self) -> Option<&T>` - View top without removing
    - `is_empty(&self) -> bool` - Check if empty
    - `size(&self) -> N` - Get number of elements
  - **Algorithmic Analysis:**
    - All ops: Work O(1) amortized, Span O(1) amortized
    - Note: May require occasional array resizing
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - Check for Vec usage - must use sequences
    - `cargo build --release` and fix all warnings/errors

- [ ] **Task 1.2**: Update `src/lib.rs` to export Chap57::StackStEph
  - Add `pub mod Chap57;`
  - Ensure StackStEph is public
  - `cargo build --release` and verify compilation

### Test File

- [ ] **Task 1.3**: Create `tests/Chap57/TestStackStEph.rs`
  - **Test cases to implement:**
    - Test empty stack creation and is_empty()
    - Test push single element
    - Test push/pop sequence
    - Test peek without modification
    - Test pop from empty stack (should return None)
    - Test multiple push/pop operations
    - Test size() after various operations
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo nextest run TestStackStEph` until all pass

### Benchmark File

- [ ] **Task 1.4**: Create `benches/Chap57/BenchStackStEph.rs`
  - **Benchmarks to implement:**
    - `bench_push` - Push n elements: n=100, 500, 1000
    - `bench_pop` - Pop n elements: n=100, 500, 1000
    - `bench_push_pop_mixed` - Mixed operations
  - **Timing:** warmup=1s, measurement=6s, samples=30 (per APAS rules)
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo bench --bench BenchStackStEph --no-run` until clean

---

## Phase 2: Dijkstra Standard Algorithm (Algorithm 57.2)

### Source Files

- [ ] **Task 2.1**: Create `src/Chap57/DijkstraStEphInt.rs`
  - **Purpose:** Dijkstra's algorithm for integer weights, ephemeral style
  - **Algorithm:** Algorithm 57.2 from textbook (priority queue based)
  - **Function signature:** `pub fn dijkstra_pq<V: StT + Hash + Ord>(graph: &WeightedDirGraphStEphInt<V>, source: V) -> SSSPResultStEphInt`
  - **Dependencies:**
    - `WeightedDirGraphStEphInt<V>` from Chap06
    - `BinaryHeapPQ` from Chap45
    - `SSSPResultStEphInt` from Chap56
    - `AVLTreeSetStEph<V>` for visited set (from Chap41)
  - **Algorithm outline (from textbook lines 1-15):**
    ```
    1  dijkstraPQ G s =
    2    let
    3      dijkstra X Q =
    4        case PQ.deleteMin Q of
    5          (None, _) => X
    6        | (Some (d, v), Q') =>
    7          if (v, _) in X then dijkstra X Q'
    8          else
    9            let
    10             X' = X ∪ {(v, d)}
    11             relax (Q, (u, w)) = PQ.insert (d + w, u) Q
    12             Q'' = iterate relax Q' (N_G^+(v))
    13           in dijkstra X' Q'' end
    14     Q0 = PQ.insert (0, s) PQ.empty
    15   in dijkstra {} Q0 end
    ```
  - **Implementation details:**
    1. Initialize: visited map X = {}, Q = empty PQ, insert (0, source) to Q
    2. Loop: deleteMin from Q to get (distance, vertex)
    3. If vertex already visited (line 7), skip (handles duplicates in Q)
    4. Mark vertex visited in X, set distance in result
    5. Relax all out-neighbors (lines 11-12): add (d + w, neighbor) to Q
    6. Return SSSP result with distances and predecessors
  - **Algorithmic Analysis:**
    - Work: O(m log n) where m=|E|, n=|V|
    - Span: O(m log n) - sequential algorithm
    - Space: O(n + m) for data structures
  - **Key implementation notes:**
    - Priority queue may contain duplicates (same vertex, different priorities)
    - Use visited check (line 7) to skip duplicates
    - Maintain distances and predecessors in SSSPResultStEphInt
    - Use out_neighbors_weighted() from graph to get N_G^+(v)
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - Check for Vec usage - must use sequences
    - `cargo build --release` and fix all warnings/errors

- [ ] **Task 2.2**: Create `src/Chap57/DijkstraStEphFloat.rs`
  - **Purpose:** Dijkstra's algorithm for float weights, ephemeral style
  - **Differences from Int version:**
    - Use `WeightedDirGraphStEphFloat<V>`
    - Use `SSSPResultStEphFloat`
    - Use `OrderedFloat<f64>` for weights and priorities
    - Use `OrderedFloat(f64::INFINITY)` for unreachable
  - **Algorithm:** Same as DijkstraStEphInt
  - **Algorithmic Analysis:** Same as Int version
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - Check for Vec usage
    - `cargo build --release` and fix all warnings/errors

- [ ] **Task 2.3**: Update `src/lib.rs` to export Dijkstra modules
  - Export DijkstraStEphInt and DijkstraStEphFloat
  - `cargo build --release` and verify all compiles

### Test Files

- [ ] **Task 2.4**: Create `tests/Chap57/TestDijkstraStEphInt.rs`
  - **Test cases to implement:**
    - **Test Example 57.1:** 3-vertex graph from textbook
      - Vertices: s, a, b
      - Edges: s→a (weight 1), s→b (weight 4), a→b (weight 2)
      - Expected: shortest path s→b is 3 (via a)
      - BFS would incorrectly give 4 (direct path)
    - **Test Example 57.3:** Execution trace verification
      - Build graph from Example 57.3
      - Verify distances match expected trace
      - Verify visit order matches algorithm
    - **Test unreachable vertices:**
      - Graph with isolated vertex
      - Verify distance = i64::MAX (unreachable)
    - **Test single vertex:**
      - Graph with only source vertex
      - Distance to self should be 0
    - **Test path reconstruction:**
      - Use extract_path from SSSP result
      - Verify path correctness and weights sum correctly
    - **Test uniform weights:**
      - All edges weight 1 (should match BFS)
    - **Test larger graph:**
      - 10+ vertices with known shortest paths
      - Verify all distances correct
    - **Test chain graph:**
      - Linear chain: 0→1→2→3→4
      - Verify distances: 0, 1, 2, 3, 4
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo nextest run TestDijkstraStEphInt` until all pass

- [ ] **Task 2.5**: Create `tests/Chap57/TestDijkstraStEphFloat.rs`
  - **Test cases:** Same as TestDijkstraStEphInt
  - **Additional test:** Fractional weights (e.g., 1.5, 2.7)
  - **Note:** Use OrderedFloat for comparisons
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo nextest run TestDijkstraStEphFloat` until all pass

### Benchmark Files

- [ ] **Task 2.6**: Create `benches/Chap57/BenchDijkstraStEphInt.rs`
  - **Benchmarks to implement:**
    - **Sparse graphs (m ≈ n):**
      - n=100 vertices, ~100 edges
      - n=500 vertices, ~500 edges
      - n=1000 vertices, ~1000 edges
    - **Dense graphs (m ≈ n²):**
      - n=50 vertices, ~2500 edges
      - n=100 vertices, ~10000 edges
      - n=200 vertices, ~40000 edges
    - **Weight distribution:** Random weights in [1, 100]
    - **Source:** Random source vertex
  - **Timing:** warmup=1s, measurement=6s, samples=30
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo bench --bench BenchDijkstraStEphInt --no-run` until clean

- [ ] **Task 2.7**: Create `benches/Chap57/BenchDijkstraStEphFloat.rs`
  - **Benchmarks:** Same as BenchDijkstraStEphInt with float weights
  - **Weight range:** [1.0, 100.0]
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo bench --bench BenchDijkstraStEphFloat --no-run` until clean

---

## Phase 3: DecreaseKey Variant (Exercise 57.1)

### Priority Queue Extension (If Needed)

- [ ] **Task 3.0**: Investigate/Extend Priority Queue for decreaseKey
  - **Options:**
    1. Check if `BinaryHeapPQ` can be extended with decreaseKey
    2. Create new `src/Chap45/BinaryHeapPQDecreaseKey.rs`
    3. Use different PQ implementation that supports decreaseKey
  - **Requirements for decreaseKey:**
    - Maintain map from keys to heap positions
    - `decrease_key(key: K, new_priority: P)` operation
    - Update heap position when priority decreased
  - **Implementation notes:**
    - More complex than standard PQ
    - Requires O(n) space for position map
    - decreaseKey is O(log n) work
  - **If new file created:**
    - Run RustRules checklist
    - Run APAS rules checklist
    - Build and test the new PQ implementation
    - `cargo build --release` and fix warnings/errors

### Source Files

- [ ] **Task 3.1**: Create `src/Chap57/DijkstraDecreaseKeyStEphInt.rs`
  - **Purpose:** Dijkstra with decreaseKey for integer weights
  - **Algorithm:** Exercise 57.1 variant
  - **Key differences from standard version:**
    - Check if neighbor already in PQ before inserting
    - If in PQ with higher priority, use decrease_key instead
    - If not in PQ, insert normally
    - No duplicate entries in priority queue
  - **Function signature:** `pub fn dijkstra_decrease_key<V: StT + Hash + Ord>(graph: &WeightedDirGraphStEphInt<V>, source: V) -> SSSPResultStEphInt`
  - **Algorithmic Analysis:**
    - Work: O(m + n log n) - better than standard O(m log n)!
    - Span: O(m + n log n) - sequential
    - More efficient on dense graphs where m >> n
  - **Implementation outline:**
    1. Initialize: distances map, predecessors, PQ with source
    2. Loop: deleteMin from Q
    3. For each out-neighbor:
      - Calculate new distance through current vertex
      - If neighbor not visited and not in PQ: insert
      - If neighbor not visited and in PQ with higher priority: decrease_key
      - Update predecessor if distance improved
    4. Return SSSP result
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - Check for Vec usage
    - `cargo build --release` and fix all warnings/errors

- [ ] **Task 3.2**: Create `src/Chap57/DijkstraDecreaseKeyStEphFloat.rs`
  - **Purpose:** DecreaseKey variant for float weights
  - **Differences:** Use float types and OrderedFloat
  - **Algorithm:** Same as DecreaseKeyStEphInt
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - Check for Vec usage
    - `cargo build --release` and fix all warnings/errors

- [ ] **Task 3.3**: Update `src/lib.rs` to export DecreaseKey modules
  - Export DijkstraDecreaseKeyStEphInt and DijkstraDecreaseKeyStEphFloat
  - `cargo build --release` and verify

### Test Files

- [ ] **Task 3.4**: Create `tests/Chap57/TestDijkstraDecreaseKeyStEphInt.rs`
  - **Test cases:** Same as TestDijkstraStEphInt (Task 2.4)
  - **Additional verification:**
    - Results should match standard Dijkstra exactly
    - Verify no incorrect distances
    - Verify correctness on all test cases
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo nextest run TestDijkstraDecreaseKeyStEphInt` until all pass

- [ ] **Task 3.5**: Create `tests/Chap57/TestDijkstraDecreaseKeyStEphFloat.rs`
  - **Test cases:** Same as TestDijkstraStEphFloat
  - **Verify:** Results match standard version
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo nextest run TestDijkstraDecreaseKeyStEphFloat` until all pass

### Benchmark Files

- [ ] **Task 3.6**: Create `benches/Chap57/BenchDijkstraDecreaseKeyStEphInt.rs`
  - **Benchmarks:** Same as BenchDijkstraStEphInt (Task 2.6)
  - **Key comparison:** Compare with standard Dijkstra performance
  - **Expected:** Should be faster on dense graphs (m ≈ n²)
  - **Analysis:** O(m + n log n) vs O(m log n)
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo bench --bench BenchDijkstraDecreaseKeyStEphInt --no-run` until clean

- [ ] **Task 3.7**: Create `benches/Chap57/BenchDijkstraDecreaseKeyStEphFloat.rs`
  - **Benchmarks:** Same as BenchDijkstraStEphFloat
  - **Comparison:** With standard version
  - **Checklist steps:**
    - Run RustRules checklist on this file
    - Run APAS rules checklist on this file
    - `cargo bench --bench BenchDijkstraDecreaseKeyStEphFloat --no-run` until clean

---

## Phase 4: Final Verification and Documentation

### Build and Test Verification

- [ ] **Task 4.1**: Run full `cargo build --release`
  - Must compile with ZERO warnings
  - Fix any errors or warnings
  - Verify all modules properly exported

- [ ] **Task 4.2**: Run full `cargo nextest run`
  - All tests must pass
  - Document any test failures
  - Fix failing tests before proceeding

### Analysis and Documentation

- [ ] **Task 4.3**: Run Algorithmic Analysis checklist
  - **File:** `rules/AlgorithmicAnalysisRules.md`
  - **Verify for each implementation:**
    - Work analysis correct
    - Span analysis correct
    - Comparison with textbook Section 3 (Cost Analysis)
  - **Expected complexities:**
    - Standard Dijkstra: Work O(m log n), Span O(m log n)
    - DecreaseKey Dijkstra: Work O(m + n log n), Span O(m + n log n)
    - Stack operations: Work O(1) amortized, Span O(1) amortized
  - **Document any discrepancies**

- [ ] **Task 4.4**: Run PostPlanChecklist
  - **File:** `checklists/PostPlanChecklist.md`
  - Verify all planned items completed
  - Note any deviations from original plan
  - Document reasons for changes

- [ ] **Task 4.5**: Create implementation summary document
  - **File:** `apas-ai/chatlogs/Chap57_Implementation_Summary.md`
  - **Contents:**
    - List of all files created (19 total)
    - Module structure overview
    - Test coverage summary
      - Number of test cases per module
      - Example tests from textbook verified
    - Benchmark results summary
      - Performance comparison: Standard vs DecreaseKey
      - Sparse vs Dense graph performance
    - Algorithm complexity verification
    - Outstanding issues or future work
    - Lessons learned

---

## Time Estimates (Revised)

### Phase 1: Stack Module
- **Source file (1):** 40 minutes
- **Test file (1):** 15 minutes
- **Benchmark file (1):** 15 minutes
- **Subtotal:** 1.25 hours

### Phase 2: Standard Dijkstra
- **Source files (2):** 2 hours (complex algorithm implementation)
- **Test files (2):** 1 hour (comprehensive test cases)
- **Benchmark files (2):** 20 minutes
- **Subtotal:** 3.5 hours

### Phase 3: DecreaseKey Dijkstra
- **PQ extension:** 1 hour (if needed)
- **Source files (2):** 1.5 hours (building on standard version)
- **Test files (2):** 40 minutes
- **Benchmark files (2):** 20 minutes
- **Subtotal:** 3.25 hours

### Phase 4: Verification
- **Build/test verification:** 15 minutes
- **Analysis checklists:** 15 minutes
- **Documentation:** 15 minutes
- **Subtotal:** 45 minutes

### **Total Estimated Time: 8.75 hours (~9 hours)**

---

## Summary of Changes from Original Plan

**Removed:**
- All Persistent (*Per) variants
- StackStPer
- DijkstraStPerInt/Float
- DijkstraDecreaseKeyStPerInt/Float
- All Mt (multi-threaded) variants

**Kept:**
- All Ephemeral (StEph) variants
- Stack module (requested by user)
- Both standard and decreaseKey implementations
- Both integer and float weight variants
- Comprehensive test and benchmark coverage

**Result:**
- Reduced from 36+ files to 19 files
- Reduced from 42 tasks to 26 tasks
- Reduced from 12 hours to ~9 hours
- Focused on most practical/efficient variants

---

## Key Implementation Notes

### Algorithm Details
- **Algorithm 57.2** is the main implementation (lines 1-15 in textbook)
- Priority queue may contain **duplicate entries** for same vertex
- Line 7 check handles duplicates by skipping already-visited vertices
- **Relax operation** (line 11) adds all neighbors to PQ without checking if already present

### Data Structure Choices
- **Priority Queue:** BinaryHeapPQ from Chap45 (O(log n) operations)
- **Visited Set:** AVLTreeSetStEph for efficient lookup
- **Graph Representation:** WeightedDirGraph from Chap06
- **Result:** SSSP Result structures from Chap56

### DecreaseKey Optimization
- Avoids duplicate entries in priority queue
- Requires more complex PQ with position tracking
- Theoretically better: O(m + n log n) vs O(m log n)
- Practically better on dense graphs where m >> n

### Stack Module Purpose
- User requested separate stack module
- Useful for auxiliary algorithms
- May be used in future chapters
- Good practice for data structure implementation

### Infinity Handling
- **Float:** `OrderedFloat(f64::INFINITY)` for unreachable
- **Integer:** `i64::MAX` as sentinel value
- Dijkstra requires **non-negative weights only**

### No Parallelism
- Textbook explicitly states: "It is a sequential algorithm"
- Vertices must be visited in priority order
- Creates inherent sequential dependencies
- No Mt versions needed

---

## PrePlanChecklist Compliance

✅ Is the plan clearly understood?  
✅ Are plan steps written as TODO items down to specific files?  
✅ Is it clear which new `src` files will be created?  
✅ Is it clear which existing `src` files the work depends upon?  
✅ Does the plan schedule creating each *Per file after the *Eph files? - N/A (no Per files)  
✅ Does the plan schedule creating each file in its own step?  
✅ For every `src` file, does the plan list which methods will be implemented?  
✅ Are tasks ordered bottom-up for incremental builds?  
✅ Is it clear which data structures to create and their types?  
✅ Do not use Vec for anything of known length, use a sequence type - CHECKED  
✅ Does the plan schedule running checklists after each file?  
✅ Does the plan schedule `cargo build` after each `src` file?  
✅ Is it clear which test files will be created?  
✅ Does the plan schedule running checklists after each test file?  
✅ Does the plan schedule `cargo nextest` after each test file?  
✅ Is it clear which benchmark files will be created?  
✅ Does the plan schedule running checklists after each benchmark file?  
✅ Does the plan schedule `cargo bench --no-run` after each benchmark?  
✅ Does the plan include full `cargo build` at end?  
✅ Does the plan include full `cargo nextest` at end?  
✅ Does the plan reserve final step to summarize changes?  
✅ Estimate the time to execute this plan - 9 hours  
✅ Add algorithmic analysis step - Task 4.3  
✅ Add PostPlanChecklist step - Task 4.4  
✅ Todo list detailed to file with each task per file  

---

## Next Steps

**When ready to execute:**
1. User reviews this revised plan
2. User provides feedback or approval
3. Upon approval, execute relentlessly following the TODO list
4. Complete all 26 tasks in order
5. Deliver summary document

**Questions or concerns should be addressed before execution begins.**
