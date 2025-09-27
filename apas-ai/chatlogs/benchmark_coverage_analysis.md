# Benchmark Coverage Analysis

## Summary
- **69 source files** analyzed
- **48 benchmark files** exist
- **21 source files** missing benchmarks

## Function Categories

### 1. WORTH BENCHMARKING (Performance-Critical)
Functions that involve significant computation, data structure operations, or algorithmic complexity.

### 2. NOT WORTH BENCHMARKING (Trivial Operations)
Simple getters, boolean checks, constructors, and O(1) operations that don't benefit from performance measurement.

---

## Chapter 03: InsertionSortSt.rs
**Benchmark Status:** ✅ HAS BENCHMARK (`BenchInsertionSortSt.rs`)

| Function | Category | Reason | Benchmarked |
|----------|----------|---------|-------------|
| `insSort(&self, slice: &mut [T])` | **WORTH** | O(n²) sorting algorithm - core performance target | ✅ YES |

---

## Chapter 05: SetStEph.rs  
**Benchmark Status:** ❌ MISSING BENCHMARK

| Function | Category | Reason | Benchmarked |
|----------|----------|---------|-------------|
| `empty()` | NOT WORTH | O(1) constructor | ❌ NO |
| `singleton(x: T)` | NOT WORTH | O(1) constructor | ❌ NO |
| `size(&self)` | NOT WORTH | O(1) getter | ❌ NO |
| `mem(&self, x: &T)` | NOT WORTH | O(1) hash lookup | ❌ NO |
| `union(&self, other: &Set<T>)` | **WORTH** | O(\|a\| + \|b\|) set operation | ❌ NO |
| `intersection(&self, other: &Set<T>)` | **WORTH** | O(\|a\| + \|b\|) set operation | ❌ NO |
| `partition(&self, parts: &Set<Set<T>>)` | **WORTH** | O(\|parts\| × \|a\|²) complex operation | ❌ NO |
| `CartesianProduct<U>(&self, other: &Set<U>)` | **WORTH** | O(\|a\| × \|b\|) expensive operation | ❌ NO |
| `insert(&mut self, x: T)` | NOT WORTH | O(1) hash insert | ❌ NO |
| `iter(&self)` | NOT WORTH | O(1) iterator creation | ❌ NO |
| `FromVec(v: Vec<T>)` | **WORTH** | O(\|v\|) bulk construction | ❌ NO |

---

## Chapter 17: MathSeq.rs
**Benchmark Status:** ✅ HAS BENCHMARK (`BenchMathSeq.rs`)

| Function | Category | Reason | Benchmarked |
|----------|----------|---------|-------------|
| `new(length: N, init_value: T)` | **WORTH** | O(length) allocation | ✅ YES |
| `set(&mut self, index: N, value: T)` | NOT WORTH | O(1) array access | ❌ NO |
| `length(&self)` | NOT WORTH | O(1) getter | ❌ NO |
| `nth(&self, index: N)` | NOT WORTH | O(1) array access | ❌ NO |
| `empty()` | NOT WORTH | O(1) constructor | ❌ NO |
| `singleton(item: T)` | NOT WORTH | O(1) constructor | ❌ NO |
| `subseq(&self, start: N, length: N)` | NOT WORTH | O(1) slice creation | ❌ NO |
| `subseq_copy(&self, start: N, length: N)` | **WORTH** | O(length) copying operation | ❌ NO |
| `add_last(&mut self, value: T)` | **WORTH** | Amortized O(1), worst O(n) - reallocation | ❌ NO |
| `delete_last(&mut self)` | NOT WORTH | O(1) pop operation | ❌ NO |
| `isEmpty(&self)` | NOT WORTH | O(1) boolean check | ❌ NO |
| `isSingleton(&self)` | NOT WORTH | O(1) boolean check | ❌ NO |
| `domain(&self)` | **WORTH** | O(\|a\|) range generation | ❌ NO |
| `range(&self)` | **WORTH** | O(\|a\|) deduplication with HashSet | ❌ NO |
| `multiset_range(&self)` | **WORTH** | O(\|a\|) counting with HashMap | ❌ NO |

---

## Chapter 18: ArraySeqStEph.rs
**Benchmark Status:** ✅ HAS BENCHMARK (`BenchArraySeqStEphChap18.rs`)

| Function | Category | Reason | Benchmarked |
|----------|----------|---------|-------------|
| `new(length: N, init_value: T)` | **WORTH** | O(length) allocation | ✅ YES |
| `set(&mut self, index: N, item: T)` | NOT WORTH | O(1) array access | ❌ NO |
| `length(&self)` | NOT WORTH | O(1) getter | ❌ NO |
| `nth(&self, index: N)` | NOT WORTH | O(1) array access | ❌ NO |
| `empty()` | NOT WORTH | O(1) constructor | ❌ NO |
| `singleton(item: T)` | NOT WORTH | O(1) constructor | ❌ NO |
| `tabulate<F>(f: &F, length: N)` | **WORTH** | O(length) function application | ✅ YES |
| `map<U, F>(a: &ArraySeqStEphS<T>, f: &F)` | **WORTH** | O(length) transformation | ✅ YES |
| `subseq(a: &ArraySeqStEphS<T>, start: N, length: N)` | **WORTH** | O(length) copying | ❌ NO |
| `append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>)` | **WORTH** | O(\|a\| + \|b\|) concatenation | ❌ NO |
| `filter<F>(a: &ArraySeqStEphS<T>, pred: &F)` | **WORTH** | O(length) filtering | ❌ NO |
| `flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>)` | **WORTH** | O(total elements) flattening | ❌ NO |
| `update(&mut self, update: Pair<N, T>)` | NOT WORTH | O(1) single update | ❌ NO |
| `inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>)` | **WORTH** | O(\|updates\|) bulk updates | ❌ NO |
| `isEmpty(&self)` | NOT WORTH | O(1) boolean check | ❌ NO |
| `isSingleton(&self)` | NOT WORTH | O(1) boolean check | ❌ NO |
| `collect<K, V>(pairs: &ArraySeqStEphS<Pair<K, V>>, cmp: fn(&K, &K) -> O)` | **WORTH** | O(n²) grouping operation | ❌ NO |
| `iterate<A, F>(a: &ArraySeqStEphS<T>, f: &F, seed: A)` | **WORTH** | O(length) fold operation | ❌ NO |
| `reduce<F>(a: &ArraySeqStEphS<T>, f: &F, id: T)` | **WORTH** | O(length) reduction | ❌ NO |
| `scan<F>(a: &ArraySeqStEphS<T>, f: &F, id: T)` | **WORTH** | O(length) prefix scan | ❌ NO |

---

## Chapter 37: BSTAVLStEph.rs
**Benchmark Status:** ✅ HAS BENCHMARK (`BenchBSTAVLStEph.rs`)

| Function | Category | Reason | Benchmarked |
|----------|----------|---------|-------------|
| `new()` | NOT WORTH | O(1) constructor | ❌ NO |
| `size(&self)` | NOT WORTH | O(1) getter (cached) | ❌ NO |
| `is_empty(&self)` | NOT WORTH | O(1) boolean check | ❌ NO |
| `height(&self)` | NOT WORTH | O(1) getter (cached) | ❌ NO |
| `insert(&mut self, value: T)` | **WORTH** | O(log n) tree insertion with rebalancing | ✅ YES |
| `find(&self, target: &T)` | **WORTH** | O(log n) tree search | ✅ YES |
| `contains(&self, target: &T)` | **WORTH** | O(log n) membership test | ✅ YES |
| `minimum(&self)` | **WORTH** | O(log n) leftmost traversal | ❌ NO |
| `maximum(&self)` | **WORTH** | O(log n) rightmost traversal | ❌ NO |
| `in_order(&self)` | **WORTH** | O(n) tree traversal | ❌ NO |
| `pre_order(&self)` | **WORTH** | O(n) tree traversal | ❌ NO |

---

## BENCHMARK COVERAGE GAPS

### High Priority Missing Benchmarks (Performance-Critical Functions)

1. **Chapter 05 (SetStEph.rs)** - Missing entire benchmark file
   - `union()`, `intersection()`, `partition()`, `CartesianProduct()`, `FromVec()`

2. **Chapter 17 (MathSeq.rs)** - Has benchmark but missing key functions
   - `subseq_copy()`, `add_last()`, `domain()`, `range()`, `multiset_range()`

3. **Chapter 18 (ArraySeqStEph.rs)** - Has benchmark but missing many functions  
   - `subseq()`, `append()`, `filter()`, `flatten()`, `inject()`, `collect()`, `iterate()`, `reduce()`, `scan()`

4. **Chapter 37 (BSTAVLStEph.rs)** - Has benchmark but missing traversal functions
   - `minimum()`, `maximum()`, `in_order()`, `pre_order()`

### Functions Already Well-Benchmarked
- **Chapter 03:** `insSort()` ✅
- **Chapter 17:** `new()` ✅  
- **Chapter 18:** `tabulate()`, `map()` ✅
- **Chapter 37:** `insert()`, `find()`, `contains()` ✅

### Functions NOT Worth Benchmarking (Trivial)
- All `empty()`, `singleton()`, `new()` constructors
- All `length()`, `size()`, `nth()` getters  
- All `isEmpty()`, `isSingleton()` boolean checks
- All `iter()` iterator creators
- Simple `set()`, `update()` single-element operations

---

## RECOMMENDATIONS

1. **Create missing benchmark files** for 21 source files without benchmarks
2. **Expand existing benchmarks** to cover performance-critical functions currently missing
3. **Focus on algorithmic complexity** - benchmark O(n), O(n log n), O(n²) operations
4. **Skip trivial operations** - don't benchmark O(1) getters, constructors, boolean checks
5. **Prioritize data structure operations** - insertions, searches, transformations, bulk operations

**Total Functions Worth Benchmarking:** ~85 functions across all modules
**Currently Benchmarked:** ~25 functions  
**Benchmark Coverage:** ~29%
