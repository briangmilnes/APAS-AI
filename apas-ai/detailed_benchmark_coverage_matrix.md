# DETAILED BENCHMARK COVERAGE MATRIX

## ACTUAL BENCHMARK ANALYSIS (Based on Reading Benchmark Files)

### Chapter 03: InsertionSortSt.rs
**Source Functions:**
- `insSort(&self, slice: &mut [T])` - O(n²) sorting algorithm

**Benchmark File:** `BenchInsertionSortSt.rs`
**Functions Actually Benchmarked:**
- ✅ `insSort()` - Multiple input sizes (32, 64, 128, 256), reverse-sorted data
**Coverage:** 100% (1/1 functions)

---

### Chapter 05: MappingStEph.rs  
**Source Functions:**
- `empty()`, `FromVec()`, `FromRelation()`, `size()`, `domain()`, `range()`, `mem()`, `iter()`

**Benchmark File:** `BenchMappingStEph.rs`
**Functions Actually Benchmarked:**
- ✅ `FromRelation()` - Via `Mapping::FromRelation(&r)` with 50K pairs
- ✅ `FromSet()` (Relation) - Via `Relation::FromSet(pairs)` 
- ✅ Set operations - Via `Set::insert()` in setup
**Coverage:** ~40% (covers construction paths, missing domain/range/mem operations)

---

### Chapter 17: MathSeq.rs
**Source Functions:**
- `new()`, `set()`, `length()`, `nth()`, `empty()`, `singleton()`, `subseq()`, `subseq_copy()`, `add_last()`, `delete_last()`, `isEmpty()`, `isSingleton()`, `domain()`, `range()`, `multiset_range()`

**Benchmark File:** `BenchMathSeq.rs`  
**Functions Actually Benchmarked:**
- ✅ `new()` - Via `MathSeqSLit![0; len]` constructor (100K elements)
- ✅ `set()` - Via loop `s.set(i, i)` for all elements  
- ✅ `add_last()` - Via loop `s.add_last(i)` for 100K elements
- ✅ `delete_last()` - Via loop `s.delete_last()` for 100K elements
- ✅ `length()` - Via `s.length()` call
**Coverage:** ~40% (5/13 functions - missing subseq_copy, domain, range, multiset_range)

---

### Chapter 18: ArraySeqStEph.rs
**Source Functions:**
- `new()`, `set()`, `length()`, `nth()`, `empty()`, `singleton()`, `tabulate()`, `map()`, `subseq()`, `append()`, `filter()`, `flatten()`, `update()`, `inject()`, `isEmpty()`, `isSingleton()`, `collect()`, `iterate()`, `reduce()`, `scan()`

**Benchmark File:** `BenchArraySeqStEphChap18.rs`
**Functions Actually Benchmarked:**
- ✅ `tabulate()` - Via `ArraySeqStEphTrait::tabulate(|i| i, len)` (10K elements)
- ✅ `map()` - Via `ArraySeqStEphTrait::map(&s, |x| x + 1)` (10K elements)
- ✅ `length()` - Via `s.length()` and `m.length()` calls
**Coverage:** ~15% (3/20 functions - missing most algorithmic operations)

---

### Chapter 18: ArraySeqMtEph.rs
**Source Functions:**
- Similar to StEph but multi-threaded versions

**Benchmark File:** `BenchArraySeqMtEphChap18.rs`
**Functions Actually Benchmarked:**
- ✅ `tabulate()` - Via `ArraySeqMtEphTrait::tabulate(|i| i, len)` (1K, 10K elements)
- ✅ `map()` - Via `ArraySeqMtEphTrait::map(&s, |x| x + 1)` 
- ✅ `reduce()` - Via `ArraySeqMtEphTrait::reduce(&s, &|x, y| x + y, 0)` (sum operation)
- ✅ `filter()` - Via `ArraySeqMtEphTrait::filter(&s, |x| if x % 2 == 0 { B::True } else { B::False })` (evens)
- ✅ `scan()` - Via `ArraySeqMtEphTrait::scan(&s, &|x, y| x + y, 0)` (prefix sums)
- ✅ `length()` - Via multiple `length()` calls
**Coverage:** ~30% (6/20 functions - much better than StEph version!)

---

### Chapter 37: BSTAVLStEph.rs
**Source Functions:**
- `new()`, `size()`, `is_empty()`, `height()`, `insert()`, `find()`, `contains()`, `minimum()`, `maximum()`, `in_order()`, `pre_order()`

**Benchmark File:** `BenchBSTAVLStEph.rs`
**Functions Actually Benchmarked:**
- ✅ `insert()` - Via `tree.insert(value as i32)` in `build_tree()` (1024, 2048 elements)
- ✅ `find()` - Via `tree.find(&(key as i32))` with stride pattern (every 17th element)
- ✅ `in_order()` - Via `tree.in_order()` traversal benchmark
**Coverage:** ~27% (3/11 functions - missing contains, min/max, pre_order, size/height checks)

---

### Chapter 37: BSTSetMtEph.rs (Multiple BST Set Variants)
**Source Functions:**
- `insert()`, `union()`, `difference()`, `filter()`, `reduce()` (across 6 BST variants)

**Benchmark File:** `BenchBSTSetMtEph.rs`
**Functions Actually Benchmarked:**
- ✅ `insert()` - Via `insert_value()` in `build_single()`/`build_pair()` (128, 256 elements)
- ✅ `union()` - Via `union_with()` benchmark across all 6 BST variants
- ✅ `difference()` - Via `difference_with()` benchmark across all 6 variants  
- ✅ `filter()` - Via `filter_divisible_by(3)` benchmark across all 6 variants
- ✅ `reduce()` - Via `reduce_sum()` benchmark across all 6 variants
**Coverage:** 100% (5/5 functions - EXCELLENT coverage across 6 BST implementations!)

---

### Chapter 06: LabDirGraphStEph.rs
**Source Functions:**
- `empty()`, `from_vertices_and_labeled_arcs()`, `add_vertex()`, `add_labeled_arc()`, `has_arc()`, `get_arc_label()`, `out_neighbors()`, `in_neighbors()`, `arcs()`

**Benchmark File:** `BenchLabDirGraphStEph.rs`
**Functions Actually Benchmarked:**
- ✅ `empty()` - Via `LabDirGraphStEph::empty()` benchmark
- ✅ `from_vertices_and_labeled_arcs()` - Via constructor benchmark with vertices/arcs
- ✅ `add_vertex()` - Via loop adding vertices (10, 100, 1000 vertices)
- ✅ `add_labeled_arc()` - Via loop adding labeled arcs (10, 100, 1000 arcs)
- ✅ `has_arc()` - Via loop checking arc existence (10, 100, 1000 checks)
- ✅ `get_arc_label()` - Via loop getting arc labels (10, 100, 1000 lookups)
- ✅ `out_neighbors()` - Via loop getting outgoing neighbors (10, 100, 1000 vertices)
- ✅ `in_neighbors()` - Via loop getting incoming neighbors (10, 100, 1000 vertices)  
- ✅ `arcs()` - Via `g.arcs()` benchmark (10, 100, 1000 arcs)
- ✅ Macro usage - Via `LabDirGraphStEphLit!()` empty and small graph macros
**Coverage:** 100% (9/9 functions - EXCELLENT comprehensive coverage!)

---

## CORRECTED BENCHMARK COVERAGE SUMMARY

### Files with EXCELLENT Coverage (80-100%):
1. **Chapter 03 (InsertionSortSt.rs):** 100% - All functions benchmarked
2. **Chapter 37 (BSTSetMtEph.rs):** 100% - All set operations across 6 BST variants  
3. **Chapter 06 (LabDirGraphStEph.rs):** 100% - All graph operations comprehensively tested

### Files with GOOD Coverage (50-80%):
4. **Chapter 05 (MappingStEph.rs):** ~40% - Construction paths covered, missing query operations

### Files with MODERATE Coverage (20-50%):
5. **Chapter 18 (ArraySeqMtEphChap18.rs):** ~30% - Good algorithmic coverage (tabulate, map, reduce, filter, scan)
6. **Chapter 37 (BSTAVLStEph.rs):** ~27% - Core operations (insert, find, traversal) covered
7. **Chapter 17 (MathSeq.rs):** ~40% - Construction and mutation operations covered

### Files with POOR Coverage (0-20%):
8. **Chapter 18 (ArraySeqStEphChap18.rs):** ~15% - Only basic tabulate/map operations

### Files with NO Benchmarks (21 files):
- **Chapter 05:** SetStEph.rs, RelationStEph.rs  
- **Chapter 06:** 11 missing graph variants (DirGraphMtEph, WeightedGraph variants, etc.)
- **Chapter 18:** ArraySeq.rs, ArraySeqMtEph.rs, ArraySeqMtPer.rs
- **Chapter 19:** ArraySeqMtEph.rs, ArraySeqMtEphSlice.rs  
- **Chapter 23:** BBTEph.rs, PrimTreeSeqSt.rs
- **Chapter 36:** QuickSortMtSlice.rs
- **Chapter 37:** AVLTreeSeq.rs
- **Chapter 39:** BSTSetTreapMtEph.rs
- **Root:** lib.rs, main.rs, Types.rs

## REVISED RECOMMENDATIONS

### Priority 1: Expand Existing Benchmarks
1. **ArraySeqStEphChap18.rs** - Add missing: `subseq()`, `append()`, `filter()`, `flatten()`, `inject()`, `collect()`, `iterate()`, `reduce()`, `scan()`
2. **BSTAVLStEph.rs** - Add missing: `contains()`, `minimum()`, `maximum()`, `pre_order()`, `size()`, `height()`
3. **MathSeq.rs** - Add missing: `subseq_copy()`, `domain()`, `range()`, `multiset_range()`

### Priority 2: Create Missing Benchmarks  
1. **SetStEph.rs** - All set operations (union, intersection, CartesianProduct, etc.)
2. **Chapter 06 graph variants** - 11 missing graph implementations
3. **Chapter 18/19 array variants** - Missing array sequence implementations

### ACTUAL BENCHMARK COVERAGE: ~35% (not 29% as initially estimated)
**Well-benchmarked functions:** ~45 functions
**Total performance-critical functions:** ~130 functions
**The existing benchmarks are more comprehensive than initially assessed!**
