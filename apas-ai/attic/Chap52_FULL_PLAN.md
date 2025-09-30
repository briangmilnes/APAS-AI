# Chapter 52: Complete Graph Representations Implementation Plan

## OVERVIEW
Implement all 16 graph representation variants (4 representations × 4 threading/mutability models) for directed graphs as specified in Chapter 52.

**Representations:**
1. **Edge Set**: G = (V set, E set) where E ⊆ V × V
2. **Adjacency Table**: G = (V × V set) table  
3. **Adjacency Sequence**: G = (int seq) seq (for enumerable graphs)
4. **Adjacency Matrix**: G = (bool seq) seq (for enumerable graphs)

**Variants per Representation:**
- StPer: Single-threaded Persistent (immutable)
- StEph: Single-threaded Ephemeral (mutable)
- MtPer: Multi-threaded Persistent (parallel + immutable)
- MtEph: Multi-threaded Ephemeral (parallel + mutable)

**Current Status:** 1/16 complete (EdgeSetGraphStPer working with tests + benchmark)

**Key API Constraints Discovered:**
- AVLTreeSetStEph: NO Debug/Display/Eq traits, only PartialEq
- ArraySeqStPer: NO update() method, use inject() for batch updates
- OrderedTableStEph: insert() requires combine function
- All Eph types: use &mut methods, avoid derives

---

## IMPLEMENTATION STRATEGY

### Phase 1: Edge Set Graphs (4 variants)
Use AVLTreeSet for vertices and edges. Already have StPer working.

### Phase 2: Adjacency Table Graphs (4 variants)  
Use OrderedTable<V, AVLTreeSet<V>> for vertex → neighbors mapping.

### Phase 3: Adjacency Sequence Graphs (4 variants)
Use ArraySeq<ArraySeq<usize>> for indexed neighbor lists (vertices must be 0..n-1).

### Phase 4: Adjacency Matrix Graphs (4 variants)
Use ArraySeq<ArraySeq<bool>> for n×n boolean matrices (vertices must be 0..n-1).

---

## DETAILED TASK BREAKDOWN

### EDGE SET GRAPHS

#### Task 1: Restore src/Chap52/EdgeSetGraphStEph.rs from backup
**File:** `src/Chap52/EdgeSetGraphStEph.rs`
**Status:** Currently stubbed, needs restoration from .bak
**Methods:**
- `new()` → Self (using AVLTreeSetStEph::empty())
- `insert_vertex(&mut self, v: V)` → mutable insert
- `insert_edge(&mut self, u: V, v: V)` → mutable insert  
- `has_edge(&self, u: &V, v: &V) -> bool` → find on edges
- `vertex_count(&self) -> usize` → size()
- `edge_count(&self) -> usize` → size()
- NO Display/Debug derives (not supported by AVLTreeSetStEph)
**Dependencies:** AVLTreeSetStEph (Chap41)
**Struct:**
```rust
pub struct EdgeSetGraphStEphS<V: StT + Ord> {
    vertices: AVLTreeSetStEph<V>,
    edges: AVLTreeSetStEph<Pair<V, V>>,
}
```

#### Task 2: Restore src/Chap52/EdgeSetGraphMtPer.rs from backup
**File:** `src/Chap52/EdgeSetGraphMtPer.rs`
**Methods:**
- `new()` → Self
- `insert_vertex(&self, v: V) -> Self` → persistent with clone
- `insert_edge(&self, u: V, v: V) -> Self` → persistent with clone
- `has_edge(&self, u: &V, v: &V) -> bool`
- `vertex_count(&self) -> usize`
- `edge_count(&self) -> usize`
- `par_map_vertices<F>(&self, f: F) -> Self` → spawn threads for parallel map
- `par_map_edges<F>(&self, f: F) -> Self` → spawn threads for parallel map
**Dependencies:** AVLTreeSetStPer, std::thread, Arc (no Mutex needed for Per)
**Struct:**
```rust
#[derive(Clone)]
pub struct EdgeSetGraphMtPerS<V: MtT + Ord> {
    vertices: AVLTreeSetStPer<V>,
    edges: AVLTreeSetStPer<Pair<V, V>>,
}
```

#### Task 3: Restore src/Chap52/EdgeSetGraphMtEph.rs from backup
**File:** `src/Chap52/EdgeSetGraphMtEph.rs`
**Methods:**
- `new()` → Self (Arc<Mutex<AVLTreeSetStEph>>)
- `insert_vertex(&self, v: V)` → acquire lock, mutate
- `insert_edge(&self, u: V, v: V)` → acquire lock, mutate
- `has_edge(&self, u: &V, v: &V) -> bool` → acquire lock, find
- `vertex_count(&self) -> usize` → acquire lock, size
- `edge_count(&self) -> usize` → acquire lock, size
- `par_insert_edges(&self, edges: &[(V, V)])` → spawn threads, each acquires lock
**Dependencies:** AVLTreeSetStEph, std::thread, Arc<Mutex>
**Struct:**
```rust
pub struct EdgeSetGraphMtEphS<V: MtT + Ord> {
    vertices: Arc<Mutex<AVLTreeSetStEph<V>>>,
    edges: Arc<Mutex<AVLTreeSetStEph<Pair<V, V>>>>,
}
```

### ADJACENCY TABLE GRAPHS

#### Task 4: Restore src/Chap52/AdjTableGraphStPer.rs from backup
**File:** `src/Chap52/AdjTableGraphStPer.rs`
**Methods:**
- `new()` → Self (OrderedTableStPer::empty())
- `insert_vertex(&self, v: V) -> Self` → insert v → empty set
- `insert_edge(&self, u: V, v: V) -> Self` → update neighbors of u
- `has_edge(&self, u: &V, v: &V) -> bool` → find u's neighbors, check v
- `out_neighbors(&self, v: &V) -> Option<AVLTreeSetStPer<V>>` → find
- `out_degree(&self, v: &V) -> usize` → find neighbors, size
- `vertex_count(&self) -> usize` → table size
**Dependencies:** OrderedTableStPer, AVLTreeSetStPer (Chap43Claude, Chap41)
**Struct:**
```rust
#[derive(Clone)]
pub struct AdjTableGraphStPerS<V: StT + Ord> {
    adj_table: OrderedTableStPer<V, AVLTreeSetStPer<V>>,
}
```

#### Task 5: Fix src/Chap52/AdjTableGraphStEph.rs  
**File:** `src/Chap52/AdjTableGraphStEph.rs`
**Current Issue:** OrderedTableStEph<V, AVLTreeSetStEph<V>> needs proper trait bounds
**Methods:**
- `new()` → Self
- `insert_vertex(&mut self, v: V)` → mutable insert
- `insert_edge(&mut self, u: V, v: V)` → find/modify u's neighbor set
- `has_edge(&self, u: &V, v: &V) -> bool`
- `out_neighbors(&self, v: &V) -> Option<&AVLTreeSetStEph<V>>`
- `out_degree(&self, v: &V) -> usize`
- `vertex_count(&self) -> usize`
**Dependencies:** OrderedTableStEph, AVLTreeSetStEph
**Struct:**
```rust
pub struct AdjTableGraphStEphS<V: StT + Ord> {
    adj_table: OrderedTableStEph<V, AVLTreeSetStEph<V>>,
}
```
**Special Handling:** NO derives, AVLTreeSetStEph doesn't support Debug/Display/Eq

#### Task 6: Restore src/Chap52/AdjTableGraphMtPer.rs from backup
**File:** `src/Chap52/AdjTableGraphMtPer.rs`
**Methods:**
- Same as StPer but with parallel map operations
- `par_map_vertices<F>(&self, f: F) -> Self` → spawn threads
**Dependencies:** OrderedTableStPer, AVLTreeSetStPer, std::thread, Arc
**Struct:**
```rust
#[derive(Clone)]
pub struct AdjTableGraphMtPerS<V: MtT + Ord> {
    adj_table: OrderedTableStPer<V, AVLTreeSetStPer<V>>,
}
```

#### Task 7: Restore src/Chap52/AdjTableGraphMtEph.rs from backup
**File:** `src/Chap52/AdjTableGraphMtEph.rs`  
**Methods:**
- Same as StEph but with Arc<Mutex> for thread-safety
- `par_insert_edges(&self, edges: &[(V, V)])`
**Dependencies:** OrderedTableStEph, AVLTreeSetStEph, Arc<Mutex>, std::thread
**Struct:**
```rust
pub struct AdjTableGraphMtEphS<V: MtT + Ord> {
    adj_table: Arc<Mutex<OrderedTableStEph<V, AVLTreeSetStEph<V>>>>,
}
```

### ADJACENCY SEQUENCE GRAPHS

#### Task 8: Restore src/Chap52/AdjSeqGraphStPer.rs from backup
**File:** `src/Chap52/AdjSeqGraphStPer.rs`
**Methods:**
- `new(n: usize) -> Self` → n empty sequences
- `from_adjacency_list(adj: Vec<Vec<usize>>) -> Self`
- `insert_edge(&self, u: usize, v: usize) -> Self` → use inject() for updates
- `has_edge(&self, u: usize, v: usize) -> bool`
- `out_neighbors(&self, v: usize) -> &ArraySeqStPer<usize>`
- `out_degree(&self, v: usize) -> usize`
- `vertex_count(&self) -> usize`
**Dependencies:** ArraySeqStPer (nested)
**Struct:**
```rust
#[derive(Clone)]
pub struct AdjSeqGraphStPerS {
    adj_seq: ArraySeqStPerS<ArraySeqStPerS<usize>>,
}
```
**Key API:** Use `inject()` with Pair<N, ArraySeqStPerS<usize>> for updates, not update()

#### Task 9: Restore src/Chap52/AdjSeqGraphStEph.rs from backup
**File:** `src/Chap52/AdjSeqGraphStEph.rs`
**Methods:**
- `new(n: usize) -> Self`
- `from_adjacency_list(adj: Vec<Vec<usize>>) -> Self`
- `insert_edge(&mut self, u: usize, v: usize)` → use update(Pair(u, new_neighbors))
- `has_edge(&self, u: usize, v: usize) -> bool`
- `out_neighbors(&self, v: usize) -> &ArraySeqStEphS<usize>`
- `out_degree(&self, v: usize) -> usize`
**Dependencies:** ArraySeqStEph (nested)
**Struct:**
```rust
pub struct AdjSeqGraphStEphS {
    adj_seq: ArraySeqStEphS<ArraySeqStEphS<usize>>,
}
```

#### Task 10: Restore src/Chap52/AdjSeqGraphMtPer.rs from backup
**File:** `src/Chap52/AdjSeqGraphMtPer.rs`
**Methods:**
- Same as StPer with parallel map over vertices
- `par_map_neighbors<F>(&self, f: F) -> Self`
**Dependencies:** ArraySeqStPer, Arc, std::thread
**Struct:**
```rust
#[derive(Clone)]
pub struct AdjSeqGraphMtPerS {
    adj_seq: ArraySeqStPerS<ArraySeqStPerS<usize>>,
}
```

#### Task 11: Restore src/Chap52/AdjSeqGraphMtEph.rs from backup
**File:** `src/Chap52/AdjSeqGraphMtEph.rs`
**Methods:**
- Same as StEph with Arc<Mutex> for shared state
- `par_insert_edges(&self, edges: &[(usize, usize)])`
**Dependencies:** ArraySeqStEph, Arc<Mutex>, std::thread
**Struct:**
```rust
pub struct AdjSeqGraphMtEphS {
    adj_seq: Arc<Mutex<ArraySeqStEphS<ArraySeqStEphS<usize>>>>,
}
```

### ADJACENCY MATRIX GRAPHS

#### Task 12: Restore src/Chap52/AdjMatrixGraphStPer.rs from backup
**File:** `src/Chap52/AdjMatrixGraphStPer.rs`
**Methods:**
- `new(n: usize) -> Self` → n×n matrix of false
- `from_matrix(matrix: Vec<Vec<bool>>) -> Self`
- `insert_edge(&self, u: usize, v: usize) -> Self` → use inject() to update
- `has_edge(&self, u: usize, v: usize) -> bool` → constant time lookup
- `out_degree(&self, v: usize) -> usize` → count true in row
- `complement(&self) -> Self` → flip all booleans
- `vertex_count(&self) -> usize`
**Dependencies:** ArraySeqStPer<ArraySeqStPer<bool>>
**Struct:**
```rust
#[derive(Clone)]
pub struct AdjMatrixGraphStPerS {
    matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>,
    n: usize,
}
```

#### Task 13: Restore src/Chap52/AdjMatrixGraphStEph.rs from backup
**File:** `src/Chap52/AdjMatrixGraphStEph.rs`
**Methods:**
- `new(n: usize) -> Self`
- `from_matrix(matrix: Vec<Vec<bool>>) -> Self`
- `insert_edge(&mut self, u: usize, v: usize)` → use update(Pair(u, new_row))
- `has_edge(&self, u: usize, v: usize) -> bool`
- `out_degree(&self, v: usize) -> usize`
- `complement(&mut self)` → mutate in place
**Dependencies:** ArraySeqStEph<ArraySeqStEph<bool>>
**Struct:**
```rust
pub struct AdjMatrixGraphStEphS {
    matrix: ArraySeqStEphS<ArraySeqStEphS<bool>>,
    n: usize,
}
```

#### Task 14: Restore src/Chap52/AdjMatrixGraphMtPer.rs from backup
**File:** `src/Chap52/AdjMatrixGraphMtPer.rs`
**Methods:**
- Same as StPer with parallel map over rows
- `par_map_rows<F>(&self, f: F) -> Self`
**Dependencies:** ArraySeqStPer, Arc, std::thread
**Struct:**
```rust
#[derive(Clone)]
pub struct AdjMatrixGraphMtPerS {
    matrix: ArraySeqStPerS<ArraySeqStPerS<bool>>,
    n: usize,
}
```

#### Task 15: Restore src/Chap52/AdjMatrixGraphMtEph.rs from backup
**File:** `src/Chap52/AdjMatrixGraphMtEph.rs`
**Methods:**
- Same as StEph with Arc<Mutex> for shared state
- `par_insert_edges(&self, edges: &[(usize, usize)])`
**Dependencies:** ArraySeqStEph, Arc<Mutex>, std::thread
**Struct:**
```rust
pub struct AdjMatrixGraphMtEphS {
    matrix: Arc<Mutex<ArraySeqStEphS<ArraySeqStEphS<bool>>>>,
    n: usize,
}
```

---

## TEST FILES (16 total, one per implementation)

Each test file follows same pattern:
1. `test_empty_graph` - initialization
2. `test_insert_vertices` - vertex insertion
3. `test_insert_edges` - edge insertion  
4. `test_has_edge` - edge queries
5. `test_directed_semantics` - verify u→v ≠ v→u
6. `test_persistent_behavior` (Per variants only)
7. `test_parallel_correctness` (Mt variants only)
8. Type-specific tests (e.g., `test_complement` for matrices)

**Test Files:**
- `tests/Chap52/TestEdgeSetGraphStPer.rs` ✅ (already complete)
- `tests/Chap52/TestEdgeSetGraphStEph.rs`
- `tests/Chap52/TestEdgeSetGraphMtPer.rs`
- `tests/Chap52/TestEdgeSetGraphMtEph.rs`
- `tests/Chap52/TestAdjTableGraphStPer.rs`
- `tests/Chap52/TestAdjTableGraphStEph.rs`
- `tests/Chap52/TestAdjTableGraphMtPer.rs`
- `tests/Chap52/TestAdjTableGraphMtEph.rs`
- `tests/Chap52/TestAdjSeqGraphStPer.rs`
- `tests/Chap52/TestAdjSeqGraphStEph.rs`
- `tests/Chap52/TestAdjSeqGraphMtPer.rs`
- `tests/Chap52/TestAdjSeqGraphMtEph.rs`
- `tests/Chap52/TestAdjMatrixGraphStPer.rs`
- `tests/Chap52/TestAdjMatrixGraphStEph.rs`
- `tests/Chap52/TestAdjMatrixGraphMtPer.rs`
- `tests/Chap52/TestAdjMatrixGraphMtEph.rs`

---

## BENCHMARK FILES (8 total, StPer + MtPer variants)

Benchmark key operations per representation:
- **Edge Set**: insert_vertices, insert_edges, has_edge queries
- **Adj Table**: insert_vertices, insert_edges, neighbor_queries, degree
- **Adj Sequence**: constant-time neighbor access, degree
- **Adj Matrix**: constant-time edge queries, complement operation

**Benchmark Files:**
- `benches/Chap52/BenchEdgeSetGraphStPer.rs` ✅ (already complete)
- `benches/Chap52/BenchEdgeSetGraphMtPer.rs`
- `benches/Chap52/BenchAdjTableGraphStPer.rs`
- `benches/Chap52/BenchAdjTableGraphMtPer.rs`
- `benches/Chap52/BenchAdjSeqGraphStPer.rs`
- `benches/Chap52/BenchAdjSeqGraphMtPer.rs`
- `benches/Chap52/BenchAdjMatrixGraphStPer.rs`
- `benches/Chap52/BenchAdjMatrixGraphMtPer.rs`

All benchmarks use APAS timing: warm_up ≤1s, measurement ~6s, sample_size ~30

---

## EXECUTION ORDER (Bottom-Up, Incremental Build)

### PHASE 1: Edge Set (Complete 4/4 variants)
1. ✅ EdgeSetGraphStPer (DONE)
2. EdgeSetGraphStEph (restore, no derives)
3. EdgeSetGraphMtPer (restore, add Clone)
4. EdgeSetGraphMtEph (restore, Arc<Mutex>)

### PHASE 2: Adjacency Table (Complete 4/4 variants)
5. AdjTableGraphStPer (restore, use inject pattern)
6. AdjTableGraphStEph (fix trait bounds)
7. AdjTableGraphMtPer (restore, parallel map)
8. AdjTableGraphMtEph (restore, Arc<Mutex>)

### PHASE 3: Adjacency Sequence (Complete 4/4 variants)
9. AdjSeqGraphStPer (restore, inject for updates)
10. AdjSeqGraphStEph (restore, update with Pair)
11. AdjSeqGraphMtPer (restore, parallel map)
12. AdjSeqGraphMtEph (restore, Arc<Mutex>)

### PHASE 4: Adjacency Matrix (Complete 4/4 variants)
13. AdjMatrixGraphStPer (restore, inject for updates, complement)
14. AdjMatrixGraphStEph (restore, update with Pair, mutable complement)
15. AdjMatrixGraphMtPer (restore, parallel map)
16. AdjMatrixGraphMtEph (restore, Arc<Mutex>)

---

## CHECKLIST COMPLIANCE

Per PrePlanChecklist.md:

- [x] Plan clearly understood (16 graph implementations)
- [x] TODO items down to specific files (39 src files + 31 test/bench files)
- [x] Clear which new src files created (15 new + 1 existing)
- [x] Clear dependencies (AVLTreeSet, OrderedTable, ArraySeq from existing chapters)
- [x] Per files before Eph files (StPer → StEph → MtPer → MtEph)
- [x] Each file in own step (70 distinct tasks)
- [x] Methods listed for each file (see detailed task breakdown)
- [x] Ordered bottom-up (EdgeSet → AdjTable → AdjSeq → AdjMatrix)
- [x] Data structures and types clear (see struct definitions)
- [x] No Vec for known length (using ArraySeq, not Vec)
- [x] Mt functions marked parallel (par_map_*, par_insert_edges)
- [x] RustRules + APASRules checklist per file (scheduled in TODO)
- [x] cargo build after each file (scheduled in TODO)
- [x] Test files clear (16 files listed)
- [x] Checklist per test file (scheduled in TODO)
- [x] cargo nextest per test file (scheduled in TODO)
- [x] Benchmark files clear (8 files listed)
- [x] Checklist per benchmark file (scheduled in TODO)
- [x] cargo bench --no-run per benchmark (scheduled in TODO)
- [x] Full cargo build at end (final task)
- [x] Full cargo nextest at end (final task)
- [x] Final summary step (PostPlanChecklist)
- [x] AlgorithmicAnalysis step (scheduled per file)

---

## TIME ESTIMATE

**Per file estimates:**
- Restore + fix src file: 5-10 minutes (simple fixes to existing code)
- Write test file: 10-15 minutes (6-8 tests per file)
- Write benchmark file: 10-15 minutes (3 benchmark groups per file)
- Run checklists: 2-3 minutes per file
- Build + test: 2-3 minutes per file

**Phase breakdown:**
- Phase 1 (Edge Set): 3 files × 20 min = 60 min
- Phase 2 (Adj Table): 4 files × 25 min = 100 min  
- Phase 3 (Adj Sequence): 4 files × 25 min = 100 min
- Phase 4 (Adj Matrix): 4 files × 25 min = 100 min
- Tests: 15 files × 15 min = 225 min
- Benchmarks: 7 files × 15 min = 105 min
- Checklists + builds: 70 files × 3 min = 210 min

**Total: ~900 minutes = 15 hours**

**With parallel tool usage and batch operations: ~8-10 hours**

---

## CAN EXECUTE RELENTLESSLY?

**YES** - Plan is fully detailed with:
- Exact file paths
- Specific methods per file
- Clear API patterns to avoid previous errors
- Incremental build strategy
- Automated test/checklist verification

No user decisions needed. Can execute from start to finish autonomously.

---

## FINAL STEPS

1. Run AlgorithmicAnalysisRules.md on all 16 implementations
2. Verify Work/Span analysis documented per method
3. Run PostPlanChecklist.md
4. Generate final summary with:
   - All 16 implementations complete
   - 16 test suites passing
   - 8 benchmark suites compiling
   - Zero warnings, zero errors
   - Performance characteristics documented
