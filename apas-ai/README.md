# APAS-AI: Algorithms Parallel and Sequential in Rust

**Rust implementation of algorithms from "Algorithms: Parallel and Sequential" by Umut A. Acar, Guy E. Blelloch, and Brian G. Milnes**

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.88.0-orange.svg)](https://www.rust-lang.org/)
[![Edition](https://img.shields.io/badge/edition-2024-green.svg)](https://doc.rust-lang.org/edition-guide/)

## Overview

This project implements the algorithms and data structures from the textbook "Algorithms: Parallel and Sequential" in Rust, with both sequential (St) and parallel (Mt) variants, and both ephemeral (Eph) and persistent (Per) versions where applicable.

The implementation emphasizes:
- **Correctness**: Comprehensive test coverage (96%+ for most modules)
- **Performance**: Optimal work and span complexity as specified in APAS
- **Type Safety**: Leveraging Rust's type system for compile-time guarantees
- **Parallelism**: Using `thread::spawn/join` for explicit parallel execution
- **Persistence**: Functional data structures with structural sharing

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program. If not, see <https://www.gnu.org/licenses/>.

**Copyright (C) 2025 Umut A. Acar, Guy E. Blelloch, and Brian G. Milnes**

## Implemented Algorithms

### Chapter 3: Asymptotic Analysis
- **Insertion Sort** (Sequential)

### Chapter 5: Basic Data Structures
- **Set** (Ephemeral, Sequential/Persistent)
- **Mapping** (Ephemeral)
- **Relation** (Ephemeral)

### Chapter 6: Graph Representations
- **Directed Graphs** (Ephemeral/Persistent, Sequential/Multi-threaded)
- **Undirected Graphs** (Ephemeral/Persistent, Sequential/Multi-threaded)
- **Labeled Graphs** (Directed/Undirected, Ephemeral, Sequential/Multi-threaded)
- **Weighted Graphs** (Directed/Undirected, Float/Int, Ephemeral, Sequential/Multi-threaded)

### Chapter 11: Dynamic Programming Basics
- **Fibonacci** (Multi-threaded, Persistent)

### Chapter 12: Recurrence Relations
- **Exercises 12.1, 12.2, 12.5** (Dynamic Programming examples)

### Chapter 17: Sequences (Abstract)
- **MathSeq** (Mathematical sequence with dense domain)

### Chapter 18: Sequences (Array-based)
- **ArraySeq** (Multiple variants: Sequential/Multi-threaded, Ephemeral/Persistent)
- **LinkedList** (Sequential, Ephemeral/Persistent)

### Chapter 19: Sequences (Advanced Array)
- **ArraySeqMtEph** (Multi-threaded with slice operations)
- **ArraySeqStEph/StPer** (Single-threaded optimized variants)

### Chapter 21: Higher-Order Functions
- **Exercises 21.5-21.9** (Map, filter, reduce examples)

### Chapter 23: Sorting Basics
- **Comparison-based Sorting** (Foundations)

### Chapter 26: Divide-and-Conquer Sorting
- **Merge Sort** (Sequential/Multi-threaded, Persistent)
- **Divide-and-Conquer Reduce** (Sequential/Multi-threaded, Persistent)

### Chapter 27: Quicksort
- **Quicksort** (Sequential/Multi-threaded, Ephemeral)

### Chapter 28: Integer Sorting
- **Radix Sort** and related integer sorting algorithms

### Chapter 35: Order Statistics
- **Selection** (Sequential/Multi-threaded, Ephemeral/Persistent)

### Chapter 36: Parallel Quicksort
- **Quicksort** (Multi-threaded with slice operations)

### Chapter 37: Binary Search Trees
- **BST Implementations**:
  - AVL Trees (Sequential/Multi-threaded, Ephemeral)
  - Red-Black Trees (Sequential/Multi-threaded, Ephemeral)
  - Splay Trees (Sequential/Multi-threaded, Ephemeral)
  - Plain BST (Sequential/Multi-threaded, Ephemeral)
  - BB-Alpha Trees (Sequential/Multi-threaded, Ephemeral)
- **Tree Sequences** (AVL-based, Sequential/Multi-threaded, Ephemeral/Persistent)
- **BST Sets** (All tree variants with set operations)

### Chapter 38: Join-based Algorithms
- **Parallel Join** algorithms for BSTs

### Chapter 39: Treaps
- **Treap** (Multi-threaded, Ephemeral)
- **Treap Set** (Multi-threaded, Ephemeral)

### Chapter 40: Maximum Contiguous Subsequence Sum
- **Brute Force** (Sequential, Ephemeral)
- **Divide-and-Conquer** (Sequential/Multi-threaded, Ephemeral)
- **Optimized** (Sequential/Multi-threaded, Ephemeral)
- **Reduced** (Sequential, Ephemeral)

### Chapter 41: Tables (Abstract)
- **Table** (Ephemeral/Persistent, Sequential/Multi-threaded)

### Chapter 42: Augmented Tables
- **Augmented Table** examples and exercises

### Chapter 43: Ordered Tables
- **Ordered Table** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Ordered Set** (Sequential, Ephemeral/Persistent)
- **Augmented Ordered Table** (Sequential/Multi-threaded, Ephemeral/Persistent)

### Chapter 44: Staged Computation
- **Document Index** (example of staged computation patterns)

### Chapter 45: Priority Queues
- **Leftist Heap** Priority Queue
- **Binary Heap** Priority Queue
- **Balanced Tree** Priority Queue
- **Sorted List** Priority Queue
- **Unsorted List** Priority Queue

### Chapter 47: Hash Tables
- **Linear Probing** (Sequential, Ephemeral)
- **Quadratic Probing** (Sequential, Ephemeral)
- **Double Hashing** (Sequential, Ephemeral)
- **Chained Hashing** (Vec-based and LinkedList-based, Sequential, Ephemeral)
- **Struct Chained** Hash Table

### Chapter 49: Balanced Trees (Advanced)
- **Balanced Tree Sequences** and variations

### Chapter 50: Optimal Binary Search Trees
- **OBST** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Probability** type
- **Matrix Chain Multiplication** (Sequential/Multi-threaded, Ephemeral/Persistent)

### Chapter 51: Dynamic Programming Frameworks
- **Top-Down DP** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Bottom-Up DP** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Minimum Edit Distance** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Subset Sum** (Sequential/Multi-threaded, Ephemeral/Persistent)

### Chapter 52: Graph Representations (Advanced)
- **Adjacency Matrix** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Adjacency Sequence** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Adjacency Table** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Edge Set** (Sequential/Multi-threaded, Ephemeral/Persistent)

### Chapter 53: Priority Queue Minimum Search
- **PQ Min** (Sequential, Ephemeral/Persistent)

### Chapter 54: Breadth-First Search
- **BFS** (Sequential/Multi-threaded, Ephemeral/Persistent)

### Chapter 55: Depth-First Search
- **DFS** (Sequential, Ephemeral/Persistent)
- **Topological Sort** (Sequential, Ephemeral/Persistent)
- **Strongly Connected Components** (Sequential, Ephemeral/Persistent)
- **Cycle Detection** (Sequential, Ephemeral/Persistent)

### Chapter 56: Dijkstra's Algorithm
- **Dijkstra** (Sequential, Ephemeral, Float/Int)

### Chapter 57: Graph Search Framework
- **Generic Graph Search** (Sequential/Multi-threaded, Ephemeral/Persistent)
- **Stack** (Sequential, Ephemeral)

### Chapter 58: Bellman-Ford Algorithm
- **Bellman-Ford** (Sequential, Ephemeral, Float/Int)
- **Single-Source Shortest Path Results** (Sequential, Ephemeral/Persistent, Float/Int)

### Chapter 59: Johnson's Algorithm
- **Johnson's APSP** (Sequential/Multi-threaded, Ephemeral, Float/Int)
- **All-Pairs Shortest Path Results** (Sequential, Ephemeral/Persistent, Float/Int)

### Chapter 61: Edge Contraction & Vertex Matching
- **Edge Contraction** (Sequential/Multi-threaded, Ephemeral)
- **Vertex Matching** (Sequential/Multi-threaded, Ephemeral)

### Chapter 62: Star Contraction
- **Star Contraction** (Sequential/Multi-threaded, Ephemeral)
- **Star Partition** (Sequential/Multi-threaded, Ephemeral)

### Chapter 63: Graph Connectivity
- **Connectivity** (Sequential/Multi-threaded, Ephemeral)
- **Connected Components** (using star contraction)

### Chapter 64: Spanning Trees
- **Spanning Tree** (Sequential/Multi-threaded, Ephemeral) via star contraction
- **TSP Approximation** (Sequential, Ephemeral)

### Chapter 65: Minimum Spanning Trees
- **Kruskal's Algorithm** (Sequential, Ephemeral)
- **Prim's Algorithm** (Sequential, Ephemeral)
- **Union-Find** (Sequential, Ephemeral)

### Chapter 66: Borůvka's Algorithm
- **Borůvka's MST** (Sequential/Multi-threaded, Ephemeral)

## Naming Conventions

The project uses a systematic naming convention:

- **St**: Single-threaded (sequential)
- **Mt**: Multi-threaded (parallel)
- **Eph**: Ephemeral (in-place modifications)
- **Per**: Persistent (functional, immutable)
- **S**: Suffix for structs (e.g., `ArraySeqStEphS`)
- **Float/Int**: Weight type for graph algorithms

Examples:
- `BSTAVLStEph`: Single-threaded, Ephemeral AVL Tree
- `ArraySeqMtPer`: Multi-threaded, Persistent Array Sequence
- `DijkstraStEphFloat`: Single-threaded, Ephemeral Dijkstra with float weights

## Building and Testing

### Prerequisites

- Rust 1.88.0 or later
- Edition 2024

### Build

```bash
# Build all modules
cargo build -j 10

# Build in release mode
cargo build --release -j 10
```

### Run Tests

```bash
# Run all tests with nextest (recommended)
cargo nextest run -j 10

# Run specific module tests
cargo nextest run -j 10 --test TestBSTAVLMtEph

# Run with verbose output
cargo nextest run -j 10 --no-capture
```

### Run Benchmarks

```bash
# Compile benchmarks
cargo bench --no-run -j 10

# Run specific benchmark
cargo bench --bench BenchBSTAVLMtEph
```

### Code Coverage

```bash
# Generate HTML coverage report
cargo llvm-cov nextest -j 10 --ignore-filename-regex '(tests|benches)/' --html

# View report at target/llvm-cov/html/index.html
```

Current coverage: **96.98% functions, 96.21% lines** (with inlining disabled)

## Project Structure

```
apas-ai/
├── src/           # Source code organized by chapter
│   ├── Chap03/    # Asymptotic Analysis
│   ├── Chap05/    # Basic Data Structures
│   ├── Chap06/    # Graphs
│   ├── ...
│   └── Chap66/    # Borůvka's Algorithm
├── tests/         # Test files mirroring src/ structure
├── benches/       # Benchmark files
├── rules/         # Coding standards and guidelines
├── scripts/       # Build and analysis scripts
└── analyses/      # Coverage and code quality reports
```

## Performance Notes

- All parallel algorithms use `thread::spawn/join` for explicit parallelism
- No `rayon` iterators (by design choice for explicit control)
- Work and span complexities match APAS specifications
- Benchmarks verify parallel speedup on multi-core systems

## Contributing

This is an academic implementation following the APAS textbook. Contributions should:
1. Maintain work/span complexity specifications from APAS
2. Follow the naming conventions (St/Mt, Eph/Per)
3. Include comprehensive tests (aim for 100% coverage)
4. Document complexity in comments
5. Run `cargo clippy` and fix warnings

## References

- **Textbook**: "Algorithms: Parallel and Sequential" by Umut A. Acar, Guy E. Blelloch
- **Original Website**: [APAS Book](http://www.parallel-algorithms-book.com/)

## Acknowledgments

This Rust implementation is based on the algorithms described in "Algorithms: Parallel and Sequential" by Umut A. Acar and Guy E. Blelloch, with implementation by Brian G. Milnes.

---

**For bug reports, feature requests, or questions, please open an issue on GitHub.**

