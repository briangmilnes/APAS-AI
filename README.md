# APAS-AI: Algorithms Parallel and Sequential

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Copyright%20%C2%A9%202025-blue.svg)](LICENSE)

A comprehensive Rust implementation of algorithms and data structures from *"Algorithms:
Parallel and Sequential"* by Acar and Blelloch (APAS), implemented by Brian Milnes using
Open AI's Chat GPT and Anthropic's Claude.

This library provides academic style implementations of fundamental algorithms with both
sequential and parallel variants, along with ephemeral and persistent data structure
variants.

It includes algorithmic analyses that almost completely match APAS's analyses and
the solutions to many exercises and problems in the textbook.

A large amount of effort has been put into generating clean rust, not just up to
the standard of Rust's lint tool, but clean typing and type inference. The rules
directory has textual rules, which AI LLMs have a very hard time following and
the scripts directory has over 8 KLOC of Python to perform code review where
it is programatically possible.


## 🎯 Overview

APAS-AI is an educational and practical library that bridges the gap between algorithmic theory and real-world implementation. It showcases:

- **Comprehensive Coverage**: 60+ chapters covering sorting, graphs, trees, dynamic programming, and more
- **Multiple Memory Models**: Ephemeral (Eph) and Persistent (Per) data structure variants
- **Dual Execution Models**: Sequential (St) and Multi-threaded parallel (Mt) implementations
- **Test-Driven**: Extensive test suite ensuring correctness
- **Performance**: Benchmarked implementations using Criterion

## 📚 Project Structure

```
apas-ai/
├── src/              # Source code organized by chapter
│   ├── Chap03/       # Insertion Sort
│   ├── Chap05/       # Sets, Relations, Mappings
│   ├── Chap06/       # Graph Representations
│   ├── Chap18-19/    # Array Sequences & Linked Lists
│   ├── Chap26-28/    # Divide & Conquer, Merge Sort
│   ├── Chap35-36/    # Order Statistics, QuickSort
│   ├── Chap37-40/    # Binary Search Trees (AVL, RB, Splay, Treap)
│   ├── Chap41-47/    # Sets, Tables, Hash Tables, Priority Queues
│   ├── Chap49-51/    # Dynamic Programming
│   ├── Chap52-59/    # Graph Algorithms (BFS, DFS, Dijkstra, etc.)
│   ├── Chap61-66/    # Advanced Graph Algorithms (MST, Connectivity)
│   └── lib.rs        # Library root
├── tests/            # Comprehensive test suite (265+ test files)
├── benches/          # Performance benchmarks (185+ benchmark files)
├── analyses/         # Coverage reports and analysis outputs
└── scripts/          # Build, test, and analysis automation
```

## 🚀 Quick Start

### Prerequisites

- Rust 2024 edition (nightly)
- Cargo (latest)

### Installation

```bash
git clone https://github.com/briangmilnes/apas-ai.git
cd apas-ai
```

### Building

```bash
# Build the library
cargo build --lib

# Build with optimizations
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for a specific chapter
cargo test --test TestAVLTreeSeq

# Run with verbose output
cargo test -- --nocapture
```

### Benchmarking

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench BenchAVLTreeSeqStPer

# Generate HTML reports
cargo bench -- --save-baseline my-baseline
```

## 📖 Key Features

### Data Structures

#### Sets and Maps
- **Array Sets**: Enumerable and standard variants
- **Tree Sets**: AVL-based sets with ordering operations

#### Sequences
- **Array-based**: Fixed and dynamic arrays (ephemeral/persistent)
- **Linked Lists**: Singly-linked (ephemeral/persistent)
- **Tree-based**: AVL tree sequences with logarithmic operations

#### Trees
- **Binary Search Trees**: Plain, AVL, Red-Black, Splay, Treap, BB(α)
- **Augmented BSTs**: Size-augmented, Key-value, Reduced
- **Heaps**: Binary heap, Leftist heap, Balanced tree priority queues
- **Specialized**: Optimal Binary Search Trees

#### Graphs
- **Representations**: Edge sets, adjacency lists, adjacency tables, adjacency matrices
- **Variants**: Directed, undirected, labeled, weighted (int/float)
- **Execution Models**: Sequential and parallel implementations

#### Tables and Hashing
- **Hash Tables**: Separate chaining, linear probing, quadratic probing, double hashing
- **Tables**: Key-value mappings with various backing stores
- **Ordered Tables**: Order-preserving mappings


### Algorithms

#### Sorting
- Insertion Sort (sequential)
- Merge Sort (sequential & parallel)
- Quick Sort (sequential & parallel, in-place & slice-based)
- Heap Sort

#### Graph Algorithms
- **Traversal**: BFS, DFS with ephemeral/persistent variants
- **Shortest Paths**: Dijkstra, Bellman-Ford, Johnson (single-source & all-pairs)
- **Connectivity**: Union-Find, strongly connected components, cycle detection
- **Minimum Spanning Trees**: Prim, Kruskal, Borůvka (sequential & parallel)
- **Advanced**: Topological sort, TSP approximation

#### Dynamic Programming
- Subset sum problem
- Minimum edit distance
- Matrix chain multiplication
- Optimal binary search trees
- Bottom-up and top-down strategies (ephemeral/persistent)

#### Divide & Conquer
- Reduce and scan contracts
- Maximum contiguous subarray sum (multiple algorithms)
- Order statistics and selection

#### Parallel Algorithms
- Parallel reduce and scan
- Parallel graph algorithms (connectivity, spanning trees)
- Work-span analysis implementations

## 🎨 Naming Conventions

The project uses a systematic naming scheme to indicate implementation characteristics:

- **St**: Sequential (single-threaded)
- **Mt**: Multi-threaded (parallel)
- **Eph**: Ephemeral (destructive updates)
- **Per**: Persistent (functional/immutable)
- **Int**: Integer weights/values
- **Float**: Floating-point weights/values

**Examples:**
- `AVLTreeSeqStPer`: AVL tree sequence, Sequential, Persistent
- `DijkstraStEphFloat`: Dijkstra's algorithm, Sequential, Ephemeral, Float weights
- `ArraySeqMtEph`: Array sequence, Multi-threaded, Ephemeral

## 🔧 Configuration

### Lints and Warnings

The project configures specific Rust lints for educational clarity:

```toml
[lints.rust]
non_snake_case = "allow"        # Algorithm names from textbook
non_upper_case_globals = "allow"
unused_imports = "allow"        # Educational examples
dead_code = "allow"             # Chapter exercises

[lints.clippy]
module_inception = "allow"
type_complexity = "allow"       # Complex generic types
```

### Dependencies

- `bitvec`: Efficient bit vector operations
- `ordered-float`: Total ordering for floating-point types
- `rand`: Random number generation (for treaps, testing)
- `rayon`: Data parallelism
- `tree_collections`: Additional tree utilities
- `criterion`: Benchmarking framework (dev)

## 📊 Testing & Coverage

The project maintains extensive test coverage:

- **265+ test files** covering all major algorithms and data structures
- **Property-based testing** for data structure invariants
- **Coverage tracking** with `llvm-cov`
- **Automated analysis** scripts for coverage reporting

```bash
# Generate coverage report
cargo llvm-cov --html

# Run coverage analysis
python scripts/analyze/coverage_by_file.py
```

## 🏗️ Development

### Code Organization

Each chapter is self-contained with:
1. Source implementation in `src/ChapXX/`
2. Test suite in `tests/ChapXX/`
3. Benchmarks in `benches/ChapXX/`
4. Module declaration in `src/lib.rs`
5. Test/bench registration in `Cargo.toml`

### Code Style

- Follow Rust standard conventions except where textbook notation takes precedence
- Include algorithmic complexity in documentation
- Provide both iterative and recursive variants where appropriate

## 📈 Performance

All implementations are benchmarked using Criterion. Key findings:

- **Parallel speedup**: Multi-threaded variants show significant speedup on multi-core systems
- **Data structure trade-offs**: Ephemeral structures offer constant-time mutations; persistent structures enable history/backtracking
- **Graph representations**: Adjacency lists optimal for sparse graphs; matrices for dense graphs
- **Hash tables**: Linear probing fastest for high load factors; separate chaining for low

See `analyses/benchmarks/` for detailed performance reports.

## 🐛 Known Issues

- **Chap47 examples**: Some hash table examples have compilation issues (commented out in `lib.rs`)
- **Chap50**: OBST trait bound issues with `OrderedFloat<f64>` and `MtT` constraints
- **Chap52**: `AdjSeqGraphMtEph` and `AdjMatrixGraphMtEph` disabled due to API mismatches

See `PATCH_NOTES.md` for detailed issue tracking and resolution plans.

## 📝 Documentation

- **In-code documentation**: All public APIs include rustdoc comments
- **Algorithm complexity**: Time/space complexity documented for each algorithm
- **Chapter exercises**: Solutions to textbook exercises included
- **Examples**: See `src/ChapXX/ExampleXX_Y.rs` files for worked examples

Generate documentation:

```bash
cargo doc --no-deps --open
```
## 📄 License

Copyright © 2025 Acar, Blelloch, and Milnes

This implementation accompanies *"Algorithms: Parallel and Sequential"*. All rights reserved.

## 🙏 Acknowledgments

This project implements algorithms and data structures from:

**"Algorithms: Parallel and Sequential"**  
by Umut A. Acar, Guy E. Blelloch. 

The implementations aim to be faithful to the textbook while leveraging Rust's type system and performance characteristics.

## 📧 Contact

For questions about the algorithms, refer to the textbook. For implementation-specific issues, please open an issue on GitHub.

---

**Note**: This is an active educational project. Some chapters may be under development or disabled temporarily. Check `src/lib.rs` for the current module status.
