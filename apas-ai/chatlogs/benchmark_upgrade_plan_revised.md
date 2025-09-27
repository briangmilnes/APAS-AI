# REVISED BENCHMARK UPGRADE PLAN

**Updated:** Based on feedback to exclude trivial operations  
**Focus:** Performance-critical operations only (no isEmpty, isSingleton, simple nth)

## REVISED PRIORITY CLASSIFICATION

### 🔥 **CRITICAL FIXES (Must Fix First)**
**17 files with compilation errors**

#### **Chapter 03 - Sorting**
- [ ] **BenchInsertionSortSt.rs**
  - Fix import path: `Chap3` → `Chap03`
  - Add more input sizes (512, 1024, 2048)
  - Benchmark best/average/worst case scenarios
  - Standardize timing (100ms warmup, 1s measurement)

#### **Chapter 18 - Array Sequences (6 broken files)**
- [ ] **BenchArraySeqMtEphChap18.rs**
  - Fix compilation errors (import paths, function signatures)
  - Add missing parallel higher-order functions
  - Benchmark tabulate(), map(), filter(), reduce(), scan()

- [ ] **BenchArraySeqMtPerChap18.rs**
  - Fix compilation errors
  - Benchmark persistent parallel operations
  - Add comprehensive MT persistent coverage

- [ ] **BenchArraySeqStPerChap18.rs**
  - Fix compilation errors
  - Benchmark persistent operations (set, update, subseq)
  - Add persistent-specific performance tests

- [ ] **BenchLinkedListStEph.rs**
  - Fix import paths: add `Chap18::` namespace
  - Add comprehensive linked list operations
  - Benchmark insert, delete, search, traversal

- [ ] **BenchLinkedListStPer.rs**
  - Fix import paths: add `Chap18::` namespace
  - Add persistent linked list operations
  - Benchmark functional update operations

- [ ] **BenchLinkedListStEph19.rs & BenchLinkedListStPer19.rs**
  - Fix compilation errors
  - Ensure proper Chapter 19 variant coverage

#### **Chapter 19 - Advanced Arrays (6 broken files)**
- [ ] **BenchArraySeqMtPer.rs**
  - Fix function signature errors (closure parameters)
  - Add comprehensive MT persistent benchmarks

- [ ] **BenchArraySeqMtPer18.rs**
  - Fix compilation errors
  - Benchmark Chapter 18 MT persistent variants

- [ ] **BenchArraySeqStEph.rs & BenchArraySeqStEph18.rs**
  - Fix import path errors
  - Add missing ephemeral operations

- [ ] **BenchArraySeqStPer.rs & BenchArraySeqStPer18.rs**
  - Fix compilation errors
  - Complete persistent operation coverage

#### **Chapter 36 - Quicksort**
- [ ] **BenchChapter36Mt.rs**
  - Fix import paths: `Chapter36Mt` → `Chap36::QuickSortMt::Chapter36Mt`
  - Fix return type specifications
  - Add more quicksort variants and input patterns

#### **Chapter 37 - Trees (6 broken files)**
- [ ] **BenchAVLTreeSeqStEph*.rs variants**
  - Fix import path errors
  - Fix macro name errors (`AVLTreeSeqStEphSLit` → `AVLTreeSeqStEphLit`)
  - Ensure comprehensive AVL tree coverage

#### **Chapter 39 - Advanced Trees**
- [ ] **BenchBSTTreapMtEph.rs**
  - Fix import paths: add `Chap39::` namespace
  - Expand treap-specific operations

- [ ] **BenchBSTParaTreapMtEph.rs**
  - Fix import paths: add `Chap39::` namespace
  - Benchmark parametric treap operations

### 📈 **HIGH PRIORITY ENHANCEMENTS**
**Focus on complex, performance-critical operations only**

#### **Chapter 05 - Sets/Relations/Mappings**
- [ ] **BenchMappingStEph.rs** (Currently 60% coverage)
  - Add benchmarks for: `domain()`, `range()`, `mem()` (only if O(n) complexity)
  - Add mapping composition and inversion operations
  - Benchmark large mapping operations with complex keys

- [ ] **BenchRelationStEph.rs** (Currently 95% coverage)
  - Add `compose()` and `inverse()` operations if available
  - Add relation algebra operations (join, project)

- [ ] **BenchSetStEph.rs** (Currently 90% coverage)
  - Add `difference()`, `symmetric_difference()` operations
  - Add complex set algebra benchmarks

#### **Chapter 06 - Graphs**
- [ ] **BenchDirGraphStEph.rs** (Currently 40% coverage)
  - **FOCUS ON COMPLEX OPERATIONS:**
    - Graph traversals: DFS, BFS (O(V+E))
    - Shortest path algorithms (O(V²) or O(E log V))
    - Strongly connected components
    - Topological sorting
  - **SKIP:** Simple vertex/edge additions (O(1) operations)

- [ ] **BenchUnDirGraphStEph.rs** (Currently 40% coverage)
  - **FOCUS ON COMPLEX OPERATIONS:**
    - Graph traversals and connectivity tests
    - Spanning tree algorithms (O(E log V))
    - Connected components (O(V+E))
  - **SKIP:** Basic vertex/edge operations

- [ ] **Create 8 missing weighted graph benchmarks:**
  - Focus on weighted algorithms: Dijkstra, Bellman-Ford, MST
  - Skip trivial weight access operations

#### **Chapter 18 - Array Sequences (Working Files)**
- [ ] **BenchArraySeqMtEph.rs** (Currently 70% coverage)
  - Add parallel higher-order functions: `tabulate()`, `map()`, `filter()`
  - Add parallel reductions: `reduce()`, `scan()` (O(n) with parallelism)
  - Benchmark thread scaling performance
  - **SKIP:** Simple length(), nth() operations

### 🆕 **MISSING MODULE BENCHMARKS**
**Create benchmarks for complex algorithms only**

#### **Chapter 12 - Exercises**
- [ ] **Create BenchExercise12_1.rs**
  - Benchmark concurrent data structure operations (O(log n) with contention)
  - Add thread safety performance tests
  - **SKIP:** Simple concurrent counters

- [ ] **Create BenchExercise12_2.rs**
  - Benchmark exercise-specific algorithms
  - Add concurrent algorithm comparisons
  - Focus on algorithms with interesting complexity

- [ ] **Create BenchExercise12_5.rs**
  - Benchmark advanced concurrent operations
  - Add scalability tests for complex operations

#### **Chapter 23 - Tree Sequences**
- [ ] **Create BenchBBTEph.rs**
  - Benchmark tree construction (O(n log n))
  - Add tree traversal and complex tree operations
  - **SKIP:** Simple tree property queries

- [ ] **Create BenchPrimTreeSeqSt.rs**
  - Benchmark sequence operations on trees (O(log n) access patterns)
  - Add tree-specific sequence operations
  - Focus on operations that benefit from tree structure

#### **Chapter 38 - Parametric Trees**
- [ ] **Create BenchBSTParaStEph.rs**
  - Benchmark parametric BST operations
  - Add parameter-specific performance tests
  - Focus on operations where parameters affect performance

### 🔧 **OPTIMIZATION & STANDARDIZATION**
**Improve existing working benchmarks**

#### **Revised Coverage for Good Files**
- [ ] **BenchFibonacciMt.rs**
  - Add sequential comparison benchmarks
  - Benchmark different thread counts (1, 2, 4, 8)
  - Add larger input sizes for scalability

- [ ] **BenchMathSeq.rs** (Currently 85% coverage)
  - **FOCUS ON COMPLEX OPERATIONS:**
    - `range()` - O(n) with deduplication
    - `multiset_range()` - O(n) with counting
    - `domain()` - O(n) index generation
  - **SKIP:** `nth()`, `isEmpty()`, `isSingleton()`

- [ ] **BenchArraySeq.rs** (Currently 80% coverage)
  - **FOCUS ON COMPLEX OPERATIONS:**
    - `flatten()` - O(n*m) nested operations
    - `collect()` - O(n) with complex aggregation
    - `update()` - O(n) functional updates
  - **SKIP:** Simple accessors

- [ ] **BenchArraySeqStEphChap18.rs** (Currently 90% coverage)
  - **FOCUS ON COMPLEX OPERATIONS:**
    - `collect()` - O(n) aggregation
    - `inject()` - O(n) injection operations
    - Nested operation benchmarks
  - **SKIP:** Simple property queries

#### **Chapter 37 - Tree Enhancements**
- [ ] **Review 12 BST benchmark files:**
  - **FOCUS ON COMPLEX OPERATIONS:**
    - Tree balancing operations (O(log n) rotations)
    - Tree construction (O(n log n))
    - Complex tree traversals
    - Tree merging/splitting operations
  - **SKIP:** Simple tree property queries, basic accessors

## REVISED EXECUTION STRATEGY

### **Phase 1: Critical Fixes (Week 1)**
1. Fix all 17 compilation errors
2. Get all benchmark files compiling and running
3. Standardize timing parameters

### **Phase 2: High Priority Enhancements (Week 2)**
1. Expand coverage focusing on O(n), O(n log n), O(n²) operations
2. Create missing weighted graph benchmarks (complex algorithms only)
3. Add comprehensive graph algorithm coverage (traversals, paths, MST)

### **Phase 3: Missing Modules (Week 3)**
1. Create Chapter 12, 23, 38 benchmark files
2. Focus on concurrent algorithms with interesting complexity
3. Complete tree sequence coverage (complex operations only)

### **Phase 4: Optimization (Week 4)**
1. Add scalability stress tests for complex operations
2. Add comparative performance analysis
3. Generate comprehensive performance reports

## REVISED SUCCESS METRICS

- **Compilation Success:** 100% of benchmark files compile without errors
- **Function Coverage:** 95%+ of **performance-critical** functions benchmarked
  - **Include:** O(n), O(n log n), O(n²) operations
  - **Exclude:** O(1) trivial operations (isEmpty, isSingleton, simple nth)
- **Timing Compliance:** All benchmarks use standardized 100ms/1s timing
- **Algorithmic Coverage:** Focus on O(n) and above complexity classes
- **Concurrency Coverage:** All MT variants with complex operations benchmarked

## OPERATIONS TO EXCLUDE FROM BENCHMARKING

### **Trivial O(1) Operations:**
- `isEmpty()` - Simple boolean check
- `isSingleton()` - Simple size check  
- `nth()` on simple structures - Direct array access
- `size()` on structures with cached size
- `length()` on simple arrays
- Basic property getters

### **Focus Instead On:**
- **O(n) Operations:** map, filter, reduce, scan, traversals
- **O(n log n) Operations:** sorting, tree construction, set operations
- **O(n²) Operations:** Cartesian products, some graph algorithms
- **Complex O(log n) Operations:** Tree operations with balancing
- **Parallel Operations:** Where parallelism provides measurable benefit

**Total Estimated Effort:** 4 weeks of systematic benchmark enhancement focusing on performance-critical operations only
