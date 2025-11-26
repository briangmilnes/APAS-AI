# APAS Algorithms

**Naming Convention:**
- **St** = Single-threaded
- **Mt** = Multi-threaded
- **Eph** = Ephemeral (mutable)
- **Per** = Persistent (immutable)

## Chapter 3 - Sorting Introduction

| Algorithm | Description |
|-----------|-------------|
| InsertionSortStEph | Classic insertion sort over mutable slices with O(n²) work; single-threaded ephemeral. |

## Chapter 5 - Sets and Relations

| Algorithm | Description |
|-----------|-------------|
| SetStEph | Set built on HashSet with union, intersection, partition, and Cartesian product; single-threaded ephemeral. |
| RelationStEph | Binary relation as a set of pairs with domain, range, and composition operations; single-threaded ephemeral. |
| MappingStEph | Mathematical function (total/partial mapping) built on relations; single-threaded ephemeral. |

## Chapter 6 - Graphs

| Algorithm | Description |
|-----------|-------------|
| DirGraphStEph | Directed graph with vertices, arcs, and neighbor operations; single-threaded ephemeral. |
| DirGraphMtEph | Directed graph with parallel neighbor operations; multi-threaded ephemeral. |
| UnDirGraphStEph | Undirected graph with symmetric edge representation; single-threaded ephemeral. |
| UnDirGraphMtEph | Undirected graph with parallel operations; multi-threaded ephemeral. |
| LabDirGraphStEph | Labeled directed graph with vertex and edge labels; single-threaded ephemeral. |
| LabDirGraphMtEph | Labeled directed graph; multi-threaded ephemeral. |
| LabUnDirGraphStEph | Labeled undirected graph with vertex and edge labels; single-threaded ephemeral. |
| LabUnDirGraphMtEph | Labeled undirected graph; multi-threaded ephemeral. |
| WeightedDirGraphStEphInt | Directed graph with integer edge weights; single-threaded ephemeral. |
| WeightedDirGraphStEphFloat | Directed graph with floating-point edge weights; single-threaded ephemeral. |
| WeightedDirGraphMtEphInt | Directed graph with integer edge weights; multi-threaded ephemeral. |
| WeightedDirGraphMtEphFloat | Directed graph with floating-point edge weights; multi-threaded ephemeral. |
| WeightedUnDirGraphStEphInt | Undirected graph with integer edge weights; single-threaded ephemeral. |
| WeightedUnDirGraphStEphFloat | Undirected graph with floating-point edge weights; single-threaded ephemeral. |
| WeightedUnDirGraphMtEphInt | Undirected graph with integer edge weights; multi-threaded ephemeral. |
| WeightedUnDirGraphMtEphFloat | Undirected graph with floating-point edge weights; multi-threaded ephemeral. |

## Chapter 11 - Fibonacci

| Algorithm | Description |
|-----------|-------------|
| FibonacciMtPer | Parallel Fibonacci using binary recursion via ParaPair with Θ(φⁿ) work; multi-threaded persistent. |

## Chapter 12 - Exercises

| Algorithm | Description |
|-----------|-------------|
| Exercise12_1 | Spin-lock via fetch-and-add tickets for mutual exclusion. |
| Exercise12_2 | Implement fetch-and-add using compare-and-swap retry loop. |
| Exercise12_5 | Lock-free concurrent stack using Treiber-style CAS list. |

## Chapter 17 - Mathematical Sequences

| Algorithm | Description |
|-----------|-------------|
| MathSeq | Dense-domain mathematical sequence (0..n-1) backed by a growable vector. |

## Chapter 18 - Sequences

| Algorithm | Description |
|-----------|-------------|
| ArraySeq | Core array sequence abstraction with standard sequence operations. |
| ArraySeqStEph | Array sequence; single-threaded ephemeral. |
| ArraySeqStPer | Array sequence; single-threaded persistent. |
| ArraySeqMtEph | Array sequence with parallel operations; multi-threaded ephemeral. |
| ArraySeqMtPer | Array sequence with parallel operations; multi-threaded persistent. |
| LinkedListStEph | Singly-linked list; single-threaded ephemeral. |
| LinkedListStPer | Singly-linked list; single-threaded persistent. |

## Chapter 19 - Sequence Operations

| Algorithm | Description |
|-----------|-------------|
| ArraySeqStEph | Extended array sequence operations (filter, map, reduce, scan); single-threaded ephemeral. |
| ArraySeqStPer | Array sequence with functional operations; single-threaded persistent. |
| ArraySeqMtEph | Parallel array sequence with map, reduce, and scan; multi-threaded ephemeral. |
| ArraySeqMtEphSlice | Slice-based parallel array operations for zero-copy parallelism; multi-threaded ephemeral. |

## Chapter 21 - Algorithm Examples

| Algorithm | Description |
|-----------|-------------|
| Algorithm21_1 | 2D Points using tabulate + flatten with Θ(n²) work and Θ(lg n) span. |
| Algorithm21_2 | 3D Points using flatten of nested tabulates with Θ(n³) work and Θ(lg n) span. |
| Algorithm21_5 | Brute Force Solution to the Primes Problem with Θ(n^{3/2}) work. |
| Algorithm21_6 | Prime Sieve using sieve of Eratosthenes with Θ(n lg n) work. |
| Exercise21_5 | Generate all contiguous subsequences using nested tabulate + flatten. |
| Exercise21_6 | Cost analysis of all contiguous subsequences (theoretical exercise). |
| Exercise21_7 | Comprehension with conditionals: even elements paired with vowels. |
| Exercise21_8 | Brute Force Primality Test (isPrime) with Θ(√n) work. |
| Exercise21_9 | Composite generation proof (documentation placeholder for Verus). |
| Problem21_1 | Points in 2D using imperative nested loops with Θ(n²) work and span. |
| Problem21_3 | Points in 3D using imperative triple loop with Θ(n³) work and span. |
| Problem21_4 | Cartesian Product using imperative loops vs functional tabulate + flatten. |

## Chapter 23 - Trees

| Algorithm | Description |
|-----------|-------------|
| BalBinTreeStEph | Full binary tree utilities with balance operations; single-threaded ephemeral. |
| PrimTreeSeqStPer | Primitive tree sequence with expose/join operations for balanced splits; single-threaded persistent. |

## Chapter 26 - Divide and Conquer

| Algorithm | Description |
|-----------|-------------|
| DivConReduceStPer | Divide-and-conquer reduction with O(n) work and O(log n) span; single-threaded persistent. |
| DivConReduceMtPer | Divide-and-conquer reduction; multi-threaded persistent. |
| MergeSortStPer | Merge sort with O(n log n) work; single-threaded persistent. |
| MergeSortMtPer | Merge sort using parallel merge with O(log n) span; multi-threaded persistent. |

## Chapter 27 - Contraction

| Algorithm | Description |
|-----------|-------------|
| ReduceContractStEph | Contraction-based reduction; single-threaded ephemeral. |
| ReduceContractMtEph | Contraction-based reduction with O(log n) span; multi-threaded ephemeral. |
| ScanContractStEph | Contraction-based prefix scan; single-threaded ephemeral. |
| ScanContractMtEph | Contraction-based prefix scan with O(log n) span; multi-threaded ephemeral. |

## Chapter 28 - Maximum Contiguous Subsequence Sum

| Algorithm | Description |
|-----------|-------------|
| MaxContigSubSumBruteStEph | Brute-force O(n³) algorithm for maximum contiguous subsequence sum; single-threaded ephemeral. |
| MaxContigSubSumDivConStEph | Divide-and-conquer MCSS with O(n log n) work; single-threaded ephemeral. |
| MaxContigSubSumDivConMtEph | Divide-and-conquer MCSS; multi-threaded ephemeral. |
| MaxContigSubSumDivConOptStEph | Optimized divide-and-conquer MCSS with linear work; single-threaded ephemeral. |
| MaxContigSubSumDivConOptMtEph | Optimized divide-and-conquer MCSS; multi-threaded ephemeral. |
| MaxContigSubSumOptStEph | Optimal O(n) Kadane's algorithm for MCSS; single-threaded ephemeral. |
| MaxContigSubSumOptMtEph | Optimal MCSS algorithm; multi-threaded ephemeral. |
| MaxContigSubSumReducedStEph | Reduction-based MCSS using monoid structure; single-threaded ephemeral. |

## Chapter 35 - Order Statistics Selection

| Algorithm | Description |
|-----------|-------------|
| OrderStatSelectStEph | Randomized selection for kth order statistic with O(n) expected work; single-threaded ephemeral. |
| OrderStatSelectStPer | Randomized selection algorithm; single-threaded persistent. |
| OrderStatSelectMtEph | Randomized selection with O(log² n) expected span; multi-threaded ephemeral. |
| OrderStatSelectMtPer | Randomized selection algorithm; multi-threaded persistent. |

## Chapter 36 - QuickSort

| Algorithm | Description |
|-----------|-------------|
| QuickSortStEph | Quicksort with first, median-of-3, and random pivot strategies; single-threaded ephemeral. |
| QuickSortMtEph | Quicksort with O(n log n) expected work; multi-threaded ephemeral. |
| QuickSortMtEphSlice | Slice-based quicksort for in-place sorting; multi-threaded ephemeral. |

## Chapter 37 - Binary Search Trees

| Algorithm | Description |
|-----------|-------------|
| AVLTreeSeq | AVL tree sequence abstraction. |
| AVLTreeSeqStEph | AVL tree sequence; single-threaded ephemeral. |
| AVLTreeSeqStPer | AVL tree sequence; single-threaded persistent. |
| AVLTreeSeqMtPer | AVL tree sequence; multi-threaded persistent. |
| BSTPlainStEph | Unbalanced binary search tree; single-threaded ephemeral. |
| BSTPlainMtEph | Unbalanced binary search tree; multi-threaded ephemeral. |
| BSTAVLStEph | AVL-balanced binary search tree; single-threaded ephemeral. |
| BSTAVLMtEph | AVL-balanced binary search tree; multi-threaded ephemeral. |
| BSTRBStEph | Red-black balanced binary search tree; single-threaded ephemeral. |
| BSTRBMtEph | Red-black balanced binary search tree; multi-threaded ephemeral. |
| BSTBBAlphaStEph | Weight-balanced (BB[α]) binary search tree; single-threaded ephemeral. |
| BSTBBAlphaMtEph | Weight-balanced (BB[α]) binary search tree; multi-threaded ephemeral. |
| BSTSplayStEph | Splay tree with amortized O(log n) operations; single-threaded ephemeral. |
| BSTSplayMtEph | Splay tree; multi-threaded ephemeral. |
| BSTSetPlainMtEph | Set interface on unbalanced BST; multi-threaded ephemeral. |
| BSTSetAVLMtEph | Set interface on AVL tree; multi-threaded ephemeral. |
| BSTSetRBMtEph | Set interface on red-black tree; multi-threaded ephemeral. |
| BSTSetBBAlphaMtEph | Set interface on weight-balanced tree; multi-threaded ephemeral. |
| BSTSetSplayMtEph | Set interface on splay tree; multi-threaded ephemeral. |

## Chapter 38 - Parallel BST

| Algorithm | Description |
|-----------|-------------|
| BSTParaStEph | BST with parallel-ready interface; single-threaded ephemeral. |
| BSTParaMtEph | BST with concurrent union and intersection; multi-threaded ephemeral. |

## Chapter 39 - Treaps

| Algorithm | Description |
|-----------|-------------|
| BSTTreapStEph | Randomized treap with expected O(log n) height; single-threaded ephemeral. |
| BSTTreapMtEph | Randomized treap; multi-threaded ephemeral. |
| BSTParaTreapMtEph | Treap with concurrent split and join; multi-threaded ephemeral. |
| BSTSetTreapMtEph | Set interface on treap; multi-threaded ephemeral. |

## Chapter 40 - Augmented BST

| Algorithm | Description |
|-----------|-------------|
| BSTKeyValueStEph | Key-value augmented BST for dictionary operations; single-threaded ephemeral. |
| BSTSizeStEph | Size-augmented BST for order statistics; single-threaded ephemeral. |
| BSTReducedStEph | Reduction-augmented BST for range queries; single-threaded ephemeral. |

## Chapter 41 - Sets

| Algorithm | Description |
|-----------|-------------|
| ArraySetStEph | Array-based set with linear search; single-threaded ephemeral. |
| ArraySetEnumMtEph | Array-based set enumeration; multi-threaded ephemeral. |
| AVLTreeSetStEph | AVL tree-based ordered set; single-threaded ephemeral. |
| AVLTreeSetStPer | AVL tree-based ordered set; single-threaded persistent. |
| AVLTreeSetMtEph | AVL tree-based ordered set; multi-threaded ephemeral. |
| AVLTreeSetMtPer | AVL tree-based ordered set; multi-threaded persistent. |
| Example41_3 | Demonstrating set operations from Example 41.1 using ArraySet and AVLTreeSet. |

## Chapter 42 - Tables

| Algorithm | Description |
|-----------|-------------|
| TableStEph | Key-value table with insert, delete, and lookup; single-threaded ephemeral. |
| TableStPer | Key-value table; single-threaded persistent. |
| TableMtEph | Key-value table; multi-threaded ephemeral. |
| Example42_1 | Basic table operations demonstration comparing implementations. |

## Chapter 43 - Ordered Tables and Sets

| Algorithm | Description |
|-----------|-------------|
| OrderedSetStEph | Ordered set with rank, select, split, and join operations; single-threaded ephemeral. |
| OrderedSetStPer | Ordered set; single-threaded persistent. |
| OrderedSetMtEph | Ordered set with parallel operations; multi-threaded ephemeral. |
| OrderedTableStEph | Ordered key-value table with range queries; single-threaded ephemeral. |
| OrderedTableStPer | Ordered key-value table; single-threaded persistent. |
| OrderedTableMtEph | Ordered key-value table; multi-threaded ephemeral. |
| OrderedTableMtPer | Ordered key-value table; multi-threaded persistent. |
| AugOrderedTableStEph | Augmented ordered table with reduction queries; single-threaded ephemeral. |
| AugOrderedTableStPer | Augmented ordered table; single-threaded persistent. |
| AugOrderedTableMtEph | Augmented ordered table; multi-threaded ephemeral. |
| Example43_1 | Ordered set operations demonstration with lexicographic ordering. |

## Chapter 44 - Document Indexing

| Algorithm | Description |
|-----------|-------------|
| DocumentIndex | Inverted index for document search with term frequency. |
| Example44_1 | Tweet document collection demonstrating inverted index operations. |

## Chapter 45 - Priority Queues

| Algorithm | Description |
|-----------|-------------|
| UnsortedListPQ | Priority queue using unsorted list with O(n) deleteMin. |
| SortedListPQ | Priority queue using sorted list with O(n) insert. |
| BinaryHeapPQ | Binary heap priority queue with O(log n) operations. |
| BalancedTreePQ | Balanced tree priority queue with O(log n) operations. |
| LeftistHeapPQ | Leftist heap with efficient meld operation. |
| HeapsortExample | Heapsort demonstration using binary heap. |
| Example45_2 | Heapsort algorithm demonstrations with various input orderings. |

## Chapter 47 - Hash Tables

| Algorithm | Description |
|-----------|-------------|
| ChainedHashTable | Hash table with separate chaining for collision resolution. |
| VecChainedHashTableStEph | Vector-based chained hash table; single-threaded ephemeral. |
| LinkedListChainedHashTableStEph | Linked-list chained hash table; single-threaded ephemeral. |
| StructChainedHashTable | Struct-based chained hash table. |
| FlatHashTable | Open addressing hash table abstraction. |
| LinProbFlatHashTableStEph | Linear probing hash table; single-threaded ephemeral. |
| QuadProbFlatHashTableStEph | Quadratic probing hash table; single-threaded ephemeral. |
| DoubleHashFlatHashTableStEph | Double hashing hash table; single-threaded ephemeral. |
| ParaHashTableStEph | Hash table with concurrent operations; single-threaded ephemeral. |

## Chapter 49 - Dynamic Programming

| Algorithm | Description |
|-----------|-------------|
| MinEditDistStEph | Minimum edit distance (Levenshtein) between sequences; single-threaded ephemeral. |
| MinEditDistStPer | Minimum edit distance; single-threaded persistent. |
| MinEditDistMtEph | Minimum edit distance; multi-threaded ephemeral. |
| MinEditDistMtPer | Minimum edit distance; multi-threaded persistent. |
| SubsetSumStEph | Subset sum decision problem using DP; single-threaded ephemeral. |
| SubsetSumStPer | Subset sum; single-threaded persistent. |
| SubsetSumMtEph | Subset sum; multi-threaded ephemeral. |
| SubsetSumMtPer | Subset sum; multi-threaded persistent. |

## Chapter 50 - More Dynamic Programming

| Algorithm | Description |
|-----------|-------------|
| MatrixChainStEph | Optimal matrix chain multiplication order using DP; single-threaded ephemeral. |
| MatrixChainStPer | Matrix chain multiplication; single-threaded persistent. |
| MatrixChainMtEph | Matrix chain multiplication; multi-threaded ephemeral. |
| MatrixChainMtPer | Matrix chain multiplication; multi-threaded persistent. |
| OptBinSearchTreeStEph | Optimal binary search tree construction using DP; single-threaded ephemeral. |
| OptBinSearchTreeStPer | Optimal binary search tree; single-threaded persistent. |
| OptBinSearchTreeMtEph | Optimal binary search tree; multi-threaded ephemeral. |
| OptBinSearchTreeMtPer | Optimal binary search tree; multi-threaded persistent. |
| Probability | Probability type with arithmetic operations for OBST. |

## Chapter 51 - DP Implementations

| Algorithm | Description |
|-----------|-------------|
| TopDownDPStEph | Top-down (memoized) dynamic programming framework; single-threaded ephemeral. |
| TopDownDPStPer | Top-down dynamic programming; single-threaded persistent. |
| TopDownDPMtEph | Top-down dynamic programming; multi-threaded ephemeral. |
| TopDownDPMtPer | Top-down dynamic programming; multi-threaded persistent. |
| BottomUpDPStEph | Bottom-up (tabulation) dynamic programming framework; single-threaded ephemeral. |
| BottomUpDPStPer | Bottom-up dynamic programming; single-threaded persistent. |
| BottomUpDPMtEph | Bottom-up dynamic programming; multi-threaded ephemeral. |
| BottomUpDPMtPer | Bottom-up dynamic programming; multi-threaded persistent. |

## Chapter 52 - Graph Representations

| Algorithm | Description |
|-----------|-------------|
| AdjMatrixGraphStEph | Adjacency matrix graph representation; single-threaded ephemeral. |
| AdjMatrixGraphStPer | Adjacency matrix graph; single-threaded persistent. |
| AdjMatrixGraphMtEph | Adjacency matrix graph; multi-threaded ephemeral. |
| AdjMatrixGraphMtPer | Adjacency matrix graph; multi-threaded persistent. |
| AdjSeqGraphStEph | Adjacency sequence (list) graph representation; single-threaded ephemeral. |
| AdjSeqGraphStPer | Adjacency sequence graph; single-threaded persistent. |
| AdjSeqGraphMtEph | Adjacency sequence graph; multi-threaded ephemeral. |
| AdjSeqGraphMtPer | Adjacency sequence graph; multi-threaded persistent. |
| AdjTableGraphStEph | Adjacency table graph representation; single-threaded ephemeral. |
| AdjTableGraphStPer | Adjacency table graph; single-threaded persistent. |
| AdjTableGraphMtPer | Adjacency table graph; multi-threaded persistent. |
| EdgeSetGraphStEph | Edge set graph representation; single-threaded ephemeral. |
| EdgeSetGraphStPer | Edge set graph; single-threaded persistent. |
| EdgeSetGraphMtPer | Edge set graph; multi-threaded persistent. |

## Chapter 53 - Graph Search

| Algorithm | Description |
|-----------|-------------|
| GraphSearchStEph | Generic graph search framework; single-threaded ephemeral. |
| GraphSearchStPer | Graph search framework; single-threaded persistent. |
| GraphSearchMtPer | Graph search framework; multi-threaded persistent. |
| PQMinStEph | Priority queue-based minimum-first graph search; single-threaded ephemeral. |
| PQMinStPer | Priority queue-based graph search; single-threaded persistent. |

## Chapter 54 - Breadth-First Search

| Algorithm | Description |
|-----------|-------------|
| BFSStEph | Queue-based BFS for distances and reachability with O(V+E) work; single-threaded ephemeral. |
| BFSStPer | Breadth-first search; single-threaded persistent. |
| BFSMtEph | BFS with frontier expansion; multi-threaded ephemeral. |
| BFSMtPer | Breadth-first search; multi-threaded persistent. |

## Chapter 55 - Depth-First Search

| Algorithm | Description |
|-----------|-------------|
| DFSStEph | Recursive DFS for reachability with O(V+E) work; single-threaded ephemeral. |
| DFSStPer | Depth-first search; single-threaded persistent. |
| CycleDetectStEph | Cycle detection in directed graphs using DFS; single-threaded ephemeral. |
| CycleDetectStPer | Cycle detection; single-threaded persistent. |
| TopoSortStEph | Topological sort for DAGs using DFS with O(V+E) work; single-threaded ephemeral. |
| TopoSortStPer | Topological sort; single-threaded persistent. |
| SCCStEph | Strongly connected components using Kosaraju's algorithm; single-threaded ephemeral. |
| SCCStPer | Strongly connected components; single-threaded persistent. |

## Chapter 56 - Shortest Paths

| Algorithm | Description |
|-----------|-------------|
| SSSPResultStEphInt | Single-source shortest path result container (integer weights); single-threaded ephemeral. |
| SSSPResultStEphFloat | Single-source shortest path result container (float weights); single-threaded ephemeral. |
| SSSPResultStPerInt | SSSP result (integer weights); single-threaded persistent. |
| SSSPResultStPerFloat | SSSP result (float weights); single-threaded persistent. |
| AllPairsResultStEphInt | All-pairs shortest path result container (integer weights); single-threaded ephemeral. |
| AllPairsResultStEphFloat | All-pairs shortest path result container (float weights); single-threaded ephemeral. |
| AllPairsResultStPerInt | APSP result (integer weights); single-threaded persistent. |
| AllPairsResultStPerFloat | APSP result (float weights); single-threaded persistent. |
| PathWeightUtilsStEph | Utilities for path weight computation and validation; single-threaded ephemeral. |
| PathWeightUtilsStPer | Path weight utilities; single-threaded persistent. |
| Example56_1 | Path weight computation demonstrating simple paths with positive and negative weights. |
| Example56_3 | Negative weight cycles and their effect on shortest path calculations. |

## Chapter 57 - Dijkstra's Algorithm

| Algorithm | Description |
|-----------|-------------|
| DijkstraStEphInt | Dijkstra's SSSP for non-negative integer weights with O(m log n) work; single-threaded ephemeral. |
| DijkstraStEphFloat | Dijkstra's SSSP for non-negative float weights; single-threaded ephemeral. |
| StackStEph | Stack data structure for path reconstruction; single-threaded ephemeral. |

## Chapter 58 - Bellman-Ford

| Algorithm | Description |
|-----------|-------------|
| BellmanFordStEphInt | Bellman-Ford SSSP handling negative weights with O(nm) work; single-threaded ephemeral. |
| BellmanFordStEphFloat | Bellman-Ford SSSP with negative cycle detection; single-threaded ephemeral. |

## Chapter 59 - Johnson's Algorithm

| Algorithm | Description |
|-----------|-------------|
| JohnsonStEphInt | Johnson's APSP for sparse graphs with negative weights; single-threaded ephemeral. |
| JohnsonStEphFloat | Johnson's APSP for float weights; single-threaded ephemeral. |
| JohnsonMtEphInt | Johnson's APSP; multi-threaded ephemeral. |
| JohnsonMtEphFloat | Johnson's APSP; multi-threaded ephemeral. |

## Chapter 61 - Edge Contraction

| Algorithm | Description |
|-----------|-------------|
| EdgeContractionStEph | Edge contraction for graph algorithms; single-threaded ephemeral. |
| EdgeContractionMtEph | Edge contraction; multi-threaded ephemeral. |
| VertexMatchingStEph | Greedy vertex matching for contraction; single-threaded ephemeral. |
| VertexMatchingMtEph | Vertex matching; multi-threaded ephemeral. |

## Chapter 62 - Star Contraction

| Algorithm | Description |
|-----------|-------------|
| StarPartitionStEph | Randomized star partition for graph contraction; single-threaded ephemeral. |
| StarPartitionMtEph | Star partition; multi-threaded ephemeral. |
| StarContractionStEph | Star-based graph contraction; single-threaded ephemeral. |
| StarContractionMtEph | Star contraction; multi-threaded ephemeral. |

## Chapter 63 - Connectivity

| Algorithm | Description |
|-----------|-------------|
| ConnectivityStEph | Connected components using contraction with O(m log n) work; single-threaded ephemeral. |
| ConnectivityMtEph | Connected components with O(log² n) span; multi-threaded ephemeral. |

## Chapter 64 - Spanning Trees

| Algorithm | Description |
|-----------|-------------|
| SpanTreeStEph | Spanning tree computation; single-threaded ephemeral. |
| SpanTreeMtEph | Spanning tree; multi-threaded ephemeral. |
| TSPApproxStEph | 2-approximation for Traveling Salesman using MST; single-threaded ephemeral. |

## Chapter 65 - MST Algorithms

| Algorithm | Description |
|-----------|-------------|
| UnionFindStEph | Union-Find with path compression and union-by-rank; single-threaded ephemeral. |
| KruskalStEph | Kruskal's MST using sorted edges and Union-Find with O(m log m) work; single-threaded ephemeral. |
| PrimStEph | Prim's MST using priority-first search with O(m log n) work; single-threaded ephemeral. |

## Chapter 66 - Borůvka's Algorithm

| Algorithm | Description |
|-----------|-------------|
| BoruvkaStEph | Borůvka's MST using vertex bridges and graph contraction; single-threaded ephemeral. |
| BoruvkaMtEph | Borůvka's MST with O(m log n) work and O(log² n) span; multi-threaded ephemeral. |
