# COMPREHENSIVE BENCHMARK UPGRADE PLAN

**Created:** 26 23:11:59 PDT  
**Total Files:** 52 benchmark files + missing modules  
**Total Tasks:** 33 upgrade tasks

## PRIORITY CLASSIFICATION

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
**Expand existing working benchmarks**

#### **Chapter 05 - Sets/Relations/Mappings**
- [ ] **BenchMappingStEph.rs** (Currently 60% coverage)
  - Add benchmarks for: `domain()`, `range()`, `mem()`, `size()`, `iter()`
  - Add mapping composition and inversion operations
  - Benchmark large mapping operations

- [ ] **BenchRelationStEph.rs** (Currently 95% coverage)
  - Add `compose()` and `inverse()` operations if available
  - Add relation algebra operations

- [ ] **BenchSetStEph.rs** (Currently 90% coverage)
  - Add `difference()`, `symmetric_difference()` operations
  - Add set algebra benchmarks

#### **Chapter 06 - Graphs**
- [ ] **BenchDirGraphStEph.rs** (Currently 40% coverage)
  - Add vertex operations: `add_vertex()`, `remove_vertex()`
  - Add edge operations: `add_edge()`, `remove_edge()`, `has_edge()`
  - Add graph traversals: DFS, BFS
  - Add shortest path algorithms

- [ ] **BenchUnDirGraphStEph.rs** (Currently 40% coverage)
  - Add comprehensive vertex/edge operations
  - Add graph traversals and connectivity tests
  - Add spanning tree algorithms

- [ ] **Create 8 missing weighted graph benchmarks:**
  - `BenchWeightedDirGraphMtEphFloat.rs`
  - `BenchWeightedDirGraphMtEphInt.rs`
  - `BenchWeightedDirGraphStEphFloat.rs`
  - `BenchWeightedDirGraphStEphInt.rs`
  - `BenchWeightedUnDirGraphMtEphFloat.rs`
  - `BenchWeightedUnDirGraphMtEphInt.rs`
  - `BenchWeightedUnDirGraphStEphFloat.rs`
  - `BenchWeightedUnDirGraphStEphInt.rs`

#### **Chapter 18 - Array Sequences (Working Files)**
- [ ] **BenchArraySeqMtEph.rs** (Currently 70% coverage)
  - Add parallel higher-order functions: `tabulate()`, `map()`, `filter()`
  - Add parallel reductions: `reduce()`, `scan()`
  - Benchmark thread scaling performance

### 🆕 **MISSING MODULE BENCHMARKS**
**Create entirely new benchmark files**

#### **Chapter 12 - Exercises**
- [ ] **Create BenchExercise12_1.rs**
  - Benchmark concurrent data structure operations
  - Add thread safety performance tests

- [ ] **Create BenchExercise12_2.rs**
  - Benchmark exercise-specific algorithms
  - Add concurrent algorithm comparisons

- [ ] **Create BenchExercise12_5.rs**
  - Benchmark advanced concurrent operations
  - Add scalability tests

#### **Chapter 23 - Tree Sequences**
- [ ] **Create BenchBBTEph.rs**
  - Benchmark binary tree operations
  - Add tree construction and traversal

- [ ] **Create BenchPrimTreeSeqSt.rs**
  - Benchmark primitive tree sequence operations
  - Add sequence-specific tree operations

#### **Chapter 38 - Parametric Trees**
- [ ] **Create BenchBSTParaStEph.rs**
  - Benchmark parametric BST operations
  - Add parameter-specific performance tests

### 🔧 **OPTIMIZATION & STANDARDIZATION**
**Improve existing working benchmarks**

#### **Timing Standardization**
- [ ] **Standardize all benchmarks:**
  - Warmup: 100ms (currently varies: 50ms-1s)
  - Measurement: 1s (currently varies: 400ms-6s)
  - Sample size: 100 for most, 10 for expensive operations
  - Input sizes: Calibrated for 1-second target

#### **Enhanced Coverage for Good Files**
- [ ] **BenchFibonacciMt.rs**
  - Add sequential comparison benchmarks
  - Benchmark different thread counts (1, 2, 4, 8)
  - Add larger input sizes for scalability

- [ ] **BenchMathSeq.rs** (Currently 85% coverage)
  - Add `nth()`, `isEmpty()`, `isSingleton()` benchmarks
  - Add sequence manipulation operations

- [ ] **BenchArraySeq.rs** (Currently 80% coverage)
  - Add `flatten()`, `collect()`, `update()` operations
  - Add more complex sequence operations

- [ ] **BenchArraySeqStEphChap18.rs** (Currently 90% coverage)
  - Add `collect()`, `inject()` operations
  - Add nested operation benchmarks

#### **Chapter 37 - Tree Enhancements**
- [ ] **Review 12 BST benchmark files:**
  - Ensure comprehensive tree operation coverage
  - Add tree balancing operation benchmarks
  - Add tree rotation performance tests
  - Standardize input sizes and timing

#### **Advanced Features**
- [ ] **Add scalability stress tests:**
  - Large input sizes (10K, 100K, 1M elements)
  - Memory allocation pattern analysis
  - Performance degradation characterization

- [ ] **Add comparative benchmarks:**
  - Single-threaded vs multi-threaded variants
  - Ephemeral vs persistent variants
  - Different tree implementation comparisons

## EXECUTION STRATEGY

### **Phase 1: Critical Fixes (Week 1)**
1. Fix all 17 compilation errors
2. Get all benchmark files compiling and running
3. Standardize timing parameters

### **Phase 2: High Priority Enhancements (Week 2)**
1. Expand coverage of working benchmarks to 90%+
2. Create missing weighted graph benchmarks
3. Add comprehensive graph algorithm coverage

### **Phase 3: Missing Modules (Week 3)**
1. Create Chapter 12, 23, 38 benchmark files
2. Add concurrent data structure benchmarks
3. Complete tree sequence coverage

### **Phase 4: Optimization (Week 4)**
1. Add scalability stress tests
2. Add comparative performance analysis
3. Add memory allocation benchmarks
4. Generate comprehensive performance reports

## SUCCESS METRICS

- **Compilation Success:** 100% of benchmark files compile without errors
- **Function Coverage:** 95%+ of performance-critical functions benchmarked
- **Timing Compliance:** All benchmarks use standardized 100ms/1s timing
- **Algorithmic Coverage:** All complexity classes (O(1) to O(n²)) represented
- **Concurrency Coverage:** All MT variants properly benchmarked

**Total Estimated Effort:** 4 weeks of systematic benchmark enhancement
