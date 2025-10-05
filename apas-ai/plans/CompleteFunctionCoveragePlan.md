# Complete Function Coverage Plan
## Goal: Achieve 100% Function Coverage Across All 133 Modules

**Current Status:** 81.92% function coverage (3,750 functions, 678 untested)
**Target:** 100% function coverage

---

## Phase 1: Types.rs (25 untested functions) - PRIORITY 1
**File:** src/Types.rs
**Test File:** tests/TestTypes.rs
**Untested Functions:** 25
**Coverage:** 49.72%

**Tasks:**
- Identify all 25 untested functions
- Add comprehensive tests for each
- Compile and run: `cargo test --test TestTypes`

---

## Phase 2: Chap37 BSTSet Modules (6 files, ~120 untested functions) - PRIORITY 2

### 2.1 BSTSetPlainMtEph.rs (23 untested)
**File:** src/Chap37/BSTSetPlainMtEph.rs
**Test File:** tests/Chap37/TestBSTSetPlainMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSetPlainMtEph`

### 2.2 BSTSetAVLMtEph.rs (20 untested)
**File:** src/Chap37/BSTSetAVLMtEph.rs
**Test File:** tests/Chap37/TestBSTSetAVLMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSetAVLMtEph`

### 2.3 BSTSetBBAlphaMtEph.rs (20 untested)
**File:** src/Chap37/BSTSetBBAlphaMtEph.rs
**Test File:** tests/Chap37/TestBSTSetBBAlphaMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSetBBAlphaMtEph`

### 2.4 BSTSetRBMtEph.rs (20 untested)
**File:** src/Chap37/BSTSetRBMtEph.rs
**Test File:** tests/Chap37/TestBSTSetRBMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSetRBMtEph`

### 2.5 BSTSetSplayMtEph.rs (20 untested)
**File:** src/Chap37/BSTSetSplayMtEph.rs
**Test File:** tests/Chap37/TestBSTSetSplayMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSetSplayMtEph`

### 2.6 BSTSetTreapMtEph.rs (20 untested)
**File:** src/Chap39/BSTSetTreapMtEph.rs
**Test File:** tests/Chap39/TestBSTSetTreapMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSetTreapMtEph`

---

## Phase 3: Chap19 ArraySeq Modules (4 files, ~56 untested functions)

### 3.1 ArraySeqMtEphSlice.rs (19 untested)
**File:** src/Chap19/ArraySeqMtEphSlice.rs
**Test File:** tests/Chap19/TestArraySeqMtEphSlice.rs
**Compile/Run:** `cargo test --test TestArraySeqMtEphSlice`

### 3.2 ArraySeqMtPer.rs (11 untested)
**File:** src/Chap19/ArraySeqMtPer.rs
**Test File:** tests/Chap19/TestArraySeqMtPerChap19.rs
**Compile/Run:** `cargo test --test TestArraySeqMtPerChap19`

### 3.3 ArraySeqStEph.rs (8 untested)
**File:** src/Chap19/ArraySeqStEph.rs
**Test File:** tests/Chap19/TestArraySeqStEphChap19.rs
**Compile/Run:** `cargo test --test TestArraySeqStEphChap19`

### 3.4 ArraySeqStPer.rs (6 untested)
**File:** src/Chap19/ArraySeqStPer.rs
**Test File:** tests/Chap19/TestArraySeqStPerChap19.rs
**Compile/Run:** `cargo test --test TestArraySeqStPerChap19`

### 3.5 ArraySeqMtEph.rs (7 untested)
**File:** src/Chap19/ArraySeqMtEph.rs
**Test File:** tests/Chap19/TestArraySeqMtEph.rs
**Compile/Run:** `cargo test --test TestArraySeqMtEph`

---

## Phase 4: Chap18 ArraySeq/LinkedList Modules (5 files, ~50 untested functions)

### 4.1 ArraySeqMtPer.rs (18 untested)
**File:** src/Chap18/ArraySeqMtPer.rs
**Test File:** tests/Chap18/TestArraySeqMtPerChap18.rs
**Compile/Run:** `cargo test --test TestArraySeqMtPerChap18`

### 4.2 ArraySeq.rs (15 untested)
**File:** src/Chap18/ArraySeq.rs
**Test File:** tests/Chap18/TestArraySeqMacro.rs
**Compile/Run:** `cargo test --test TestArraySeqMacro`

### 4.3 ArraySeqMtEph.rs (12 untested)
**File:** src/Chap18/ArraySeqMtEph.rs
**Test File:** tests/Chap18/TestArraySeqMtEphChap18.rs
**Compile/Run:** `cargo test --test TestArraySeqMtEphChap18`

### 4.4 LinkedListStPer.rs (8 untested)
**File:** src/Chap18/LinkedListStPer.rs
**Test File:** tests/Chap18/TestLinkedListStPerChap18.rs
**Compile/Run:** `cargo test --test TestLinkedListStPerChap18`

### 4.5 ArraySeqStEph.rs (8 untested)
**File:** src/Chap18/ArraySeqStEph.rs
**Test File:** tests/Chap18/TestArraySeqStEphChap18.rs
**Compile/Run:** `cargo test --test TestArraySeqStEphChap18`

### 4.6 LinkedListStEph.rs (5 untested)
**File:** src/Chap18/LinkedListStEph.rs
**Test File:** tests/Chap18/TestLinkedListStEphChap18.rs
**Compile/Run:** `cargo test --test TestLinkedListStEphChap18`

### 4.7 ArraySeqStPer.rs (2 untested)
**File:** src/Chap18/ArraySeqStPer.rs
**Test File:** tests/Chap18/TestArraySeqStPerChap18.rs
**Compile/Run:** `cargo test --test TestArraySeqStPerChap18`

---

## Phase 5: Chap06 Graph Modules (8 files, ~67 untested functions)

### 5.1 DirGraphMtEph.rs (18 untested)
**File:** src/Chap06/DirGraphMtEph.rs
**Test File:** tests/Chap06/TestDirGraphMtEph.rs
**Compile/Run:** `cargo test --test TestDirGraphMtEph`

### 5.2 SetStEph.rs (11 untested)
**File:** src/Chap05/SetStEph.rs
**Test File:** tests/Chap05/TestSetStEph.rs
**Compile/Run:** `cargo test --test TestSetStEph`

### 5.3 LabDirGraphMtEph.rs (8 untested)
**File:** src/Chap06/LabDirGraphMtEph.rs
**Test File:** tests/Chap06/TestLabDirGraphMtEph.rs
**Compile/Run:** `cargo test --test TestLabDirGraphMtEph`

### 5.4 LabUnDirGraphStEph.rs (7 untested)
**File:** src/Chap06/LabUnDirGraphStEph.rs
**Test File:** tests/Chap06/TestLabUnDirGraphStEph.rs
**Compile/Run:** `cargo test --test TestLabUnDirGraphStEph`

### 5.5 LabUnDirGraphMtEph.rs (7 untested)
**File:** src/Chap06/LabUnDirGraphMtEph.rs
**Test File:** tests/Chap06/TestLabUnDirGraphMtEph.rs
**Compile/Run:** `cargo test --test TestLabUnDirGraphMtEph`

### 5.6 WeightedDirGraphMtEphFloat.rs (7 untested)
**File:** src/Chap06/WeightedDirGraphMtEphFloat.rs
**Test File:** tests/Chap06/TestWeightedDirGraphMtEphFloat.rs
**Compile/Run:** `cargo test --test TestWeightedDirGraphMtEphFloat`

### 5.7 WeightedDirGraphMtEphInt.rs (7 untested)
**File:** src/Chap06/WeightedDirGraphMtEphInt.rs
**Test File:** tests/Chap06/TestWeightedDirGraphMtEphInt.rs
**Compile/Run:** `cargo test --test TestWeightedDirGraphMtEphInt`

### 5.8 MappingStEph.rs (4 untested)
**File:** src/Chap05/MappingStEph.rs
**Test File:** tests/Chap05/TestMappingStEph.rs
**Compile/Run:** `cargo test --test TestMappingStEph`

### 5.9 RelationStEph.rs (3 untested)
**File:** src/Chap05/RelationStEph.rs
**Test File:** tests/Chap05/TestRelationStEph.rs
**Compile/Run:** `cargo test --test TestRelationStEph`

### 5.10 DirGraphStEph.rs (3 untested)
**File:** src/Chap06/DirGraphStEph.rs
**Test File:** tests/Chap06/TestDirGraphStEph.rs
**Compile/Run:** `cargo test --test TestDirGraphStEph`

### 5.11 UnDirGraphMtEph.rs (3 untested)
**File:** src/Chap06/UnDirGraphMtEph.rs
**Test File:** tests/Chap06/TestUnDirGraphMtEph.rs
**Compile/Run:** `cargo test --test TestUnDirGraphMtEph`

### 5.12 WeightedUnDirGraphMtEphFloat.rs (3 untested)
**File:** src/Chap06/WeightedUnDirGraphMtEphFloat.rs
**Test File:** tests/Chap06/TestWeightedUnDirGraphMtEphFloat.rs
**Compile/Run:** `cargo test --test TestWeightedUnDirGraphMtEphFloat`

### 5.13 WeightedUnDirGraphMtEphInt.rs (3 untested)
**File:** src/Chap06/WeightedUnDirGraphMtEphInt.rs
**Test File:** tests/Chap06/TestWeightedUnDirGraphMtEphInt.rs
**Compile/Run:** `cargo test --test TestWeightedUnDirGraphMtEphInt`

---

## Phase 6: Chap37 BST Modules (9 files, ~43 untested functions)

### 6.1 BSTBBAlphaStEph.rs (13 untested)
**File:** src/Chap37/BSTBBAlphaStEph.rs
**Test File:** tests/Chap37/TestBSTBBAlphaStEph.rs
**Compile/Run:** `cargo test --test TestBSTBBAlphaStEph`

### 6.2 AVLTreeSeq.rs (5 untested)
**File:** src/Chap37/AVLTreeSeq.rs
**Test File:** tests/Chap37/TestAVLTreeSeq.rs
**Compile/Run:** `cargo test --test TestAVLTreeSeq`

### 6.3 BSTRBStEph.rs (5 untested)
**File:** src/Chap37/BSTRBStEph.rs
**Test File:** tests/Chap37/TestBSTRBStEph.rs
**Compile/Run:** `cargo test --test TestBSTRBStEph`

### 6.4 AVLTreeSeqMtPer.rs (4 untested)
**File:** src/Chap37/AVLTreeSeqMtPer.rs
**Test File:** tests/Chap37/TestAVLTreeSeqMtPer.rs
**Compile/Run:** `cargo test --test TestAVLTreeSeqMtPer`

### 6.5 AVLTreeSeqStPer.rs (3 untested)
**File:** src/Chap37/AVLTreeSeqStPer.rs
**Test File:** tests/Chap37/TestAVLTreeSeqStPer.rs
**Compile/Run:** `cargo test --test TestAVLTreeSeqStPer`

### 6.6 BSTAVLMtEph.rs (3 untested)
**File:** src/Chap37/BSTAVLMtEph.rs
**Test File:** tests/Chap37/TestBSTAVLMtEph.rs
**Compile/Run:** `cargo test --test TestBSTAVLMtEph`

### 6.7 BSTRBMtEph.rs (3 untested)
**File:** src/Chap37/BSTRBMtEph.rs
**Test File:** tests/Chap37/TestBSTRBMtEph.rs
**Compile/Run:** `cargo test --test TestBSTRBMtEph`

### 6.8 BSTSplayMtEph.rs (3 untested)
**File:** src/Chap37/BSTSplayMtEph.rs
**Test File:** tests/Chap37/TestBSTSplayMtEph.rs
**Compile/Run:** `cargo test --test TestBSTSplayMtEph`

### 6.9 AVLTreeSeqStEph.rs (2 untested)
**File:** src/Chap37/AVLTreeSeqStEph.rs
**Test File:** tests/Chap37/TestAVLTreeSeqStEphChap37.rs
**Compile/Run:** `cargo test --test TestAVLTreeSeqStEphChap37`

### 6.10 BSTBBAlphaMtEph.rs (1 untested)
**File:** src/Chap37/BSTBBAlphaMtEph.rs
**Test File:** tests/Chap37/TestBSTBBAlphaMtEph.rs
**Compile/Run:** `cargo test --test TestBSTBBAlphaMtEph`

### 6.11 BSTPlainStEph.rs (1 untested)
**File:** src/Chap37/BSTPlainStEph.rs
**Test File:** tests/Chap37/TestBSTPlainStEph.rs
**Compile/Run:** `cargo test --test TestBSTPlainStEph`

### 6.12 BSTSplayStEph.rs (1 untested)
**File:** src/Chap37/BSTSplayStEph.rs
**Test File:** tests/Chap37/TestBSTSplayStEph.rs
**Compile/Run:** `cargo test --test TestBSTSplayStEph`

---

## Phase 7: Chap47 Hash Modules (6 files, ~21 untested functions)

### 7.1 NestedHashTable.rs (12 untested)
**File:** src/Chap47/NestedHashTable.rs
**Test File:** tests/Chap47/TestNestedHashTable.rs
**Compile/Run:** `cargo test --test TestNestedHashTable`

### 7.2 ClusteringAnalysis.rs (3 untested)
**File:** src/Chap47/ClusteringAnalysis.rs
**Test File:** tests/Chap47/TestClusteringAnalysis.rs
**Compile/Run:** `cargo test --test TestClusteringAnalysis`

### 7.3 ProbeSequenceExamples.rs (2 untested)
**File:** src/Chap47/ProbeSequenceExamples.rs
**Test File:** tests/Chap47/TestProbeSequenceExamples.rs
**Compile/Run:** `cargo test --test TestProbeSequenceExamples`

### 7.4 AdvancedDoubleHashing.rs (2 untested)
**File:** src/Chap47/AdvancedDoubleHashing.rs
**Test File:** tests/Chap47/TestAdvancedDoubleHashing.rs
**Compile/Run:** `cargo test --test TestAdvancedDoubleHashing`

### 7.5 AdvancedLinearProbing.rs (1 untested)
**File:** src/Chap47/AdvancedLinearProbing.rs
**Test File:** tests/Chap47/TestAdvancedLinearProbing.rs
**Compile/Run:** `cargo test --test TestAdvancedLinearProbing`

### 7.6 HashFunctionTraits.rs (1 untested)
**File:** src/Chap47/HashFunctionTraits.rs
**Test File:** tests/Chap47/TestHashFunctionTraits.rs
**Compile/Run:** `cargo test --test TestHashFunctionTraits`

---

## Phase 8: Chap45 Priority Queue Modules (5 files, ~27 untested functions)

### 8.1 SortedListPQ.rs (11 untested)
**File:** src/Chap45/SortedListPQ.rs
**Test File:** tests/Chap45/TestSortedListPQ.rs
**Compile/Run:** `cargo test --test TestSortedListPQ`

### 8.2 Example45_2.rs (8 untested)
**File:** src/Chap45/Example45_2.rs
**Test File:** Need to create test file
**Compile/Run:** `cargo test --test TestExample45_2`

### 8.3 LeftistHeapPQ.rs (4 untested)
**File:** src/Chap45/LeftistHeapPQ.rs
**Test File:** tests/Chap45/TestLeftistHeapPQ.rs
**Compile/Run:** `cargo test --test TestLeftistHeapPQ`

### 8.4 UnsortedListPQ.rs (2 untested)
**File:** src/Chap45/UnsortedListPQ.rs
**Test File:** tests/Chap45/TestUnsortedListPQ.rs
**Compile/Run:** `cargo test --test TestUnsortedListPQ`

### 8.5 BalancedTreePQ.rs (1 untested)
**File:** src/Chap45/BalancedTreePQ.rs
**Test File:** tests/Chap45/TestBalancedTreePQ.rs
**Compile/Run:** `cargo test --test TestBalancedTreePQ`

### 8.6 BinaryHeapPQ.rs (1 untested)
**File:** src/Chap45/BinaryHeapPQ.rs
**Test File:** tests/Chap45/TestBinaryHeapPQ.rs
**Compile/Run:** `cargo test --test TestBinaryHeapPQ`

---

## Phase 9: Chap50 Dynamic Programming Modules (8 files, ~25 untested functions)

### 9.1 OptBinSearchTreeStEph.rs (4 untested)
**File:** src/Chap50/OptBinSearchTreeStEph.rs
**Test File:** tests/Chap50/TestOptBinSearchTreeStEph.rs
**Compile/Run:** `cargo test --test TestOptBinSearchTreeStEph`

### 9.2 MatrixChainMtEph.rs (4 untested)
**File:** src/Chap50/MatrixChainMtEph.rs
**Test File:** tests/Chap50/TestMatrixChainMtEph.rs
**Compile/Run:** `cargo test --test TestMatrixChainMtEph`

### 9.3 MatrixChainMtPer.rs (4 untested)
**File:** src/Chap50/MatrixChainMtPer.rs
**Test File:** tests/Chap50/TestMatrixChainMtPer.rs
**Compile/Run:** `cargo test --test TestMatrixChainMtPer`

### 9.4 MatrixChainStEph.rs (4 untested)
**File:** src/Chap50/MatrixChainStEph.rs
**Test File:** tests/Chap50/TestMatrixChainStEph.rs
**Compile/Run:** `cargo test --test TestMatrixChainStEph`

### 9.5 MatrixChainStPer.rs (4 untested)
**File:** src/Chap50/MatrixChainStPer.rs
**Test File:** tests/Chap50/TestMatrixChainStPer.rs
**Compile/Run:** `cargo test --test TestMatrixChainStPer`

### 9.6 Probability.rs (4 untested)
**File:** src/Chap50/Probability.rs
**Test File:** tests/Chap50/TestProbability.rs
**Compile/Run:** `cargo test --test TestProbability`

### 9.7 OptBinSearchTreeMtPer.rs (3 untested)
**File:** src/Chap50/OptBinSearchTreeMtPer.rs
**Test File:** tests/Chap50/TestOptBinSearchTreeMtPer.rs
**Compile/Run:** `cargo test --test TestOptBinSearchTreeMtPer`

### 9.8 OptBinSearchTreeStPer.rs (2 untested)
**File:** src/Chap50/OptBinSearchTreeStPer.rs
**Test File:** tests/Chap50/TestOptBinSearchTreeStPer.rs
**Compile/Run:** `cargo test --test TestOptBinSearchTreeStPer`

### 9.9 OptBinSearchTreeMtEph.rs (1 untested)
**File:** src/Chap50/OptBinSearchTreeMtEph.rs
**Test File:** tests/Chap50/TestOBSTMtEph.rs
**Compile/Run:** `cargo test --test TestOBSTMtEph`

---

## Phase 10: Chap49 Dynamic Programming Modules (8 files, ~36 untested functions)

### 10.1 MinEditDistStEph.rs (7 untested)
**File:** src/Chap49/MinEditDistStEph.rs
**Test File:** tests/Chap49/TestMinEditDistStEph.rs
**Compile/Run:** `cargo test --test TestMinEditDistStEph`

### 10.2 SubsetSumStEph.rs (7 untested)
**File:** src/Chap49/SubsetSumStEph.rs
**Test File:** tests/Chap49/TestSubsetSumStEph.rs
**Compile/Run:** `cargo test --test TestSubsetSumStEph`

### 10.3 SubsetSumMtEph.rs (7 untested)
**File:** src/Chap49/SubsetSumMtEph.rs
**Test File:** tests/Chap49/TestSubsetSumMtEph.rs
**Compile/Run:** `cargo test --test TestSubsetSumMtEph`

### 10.4 MinEditDistMtPer.rs (6 untested)
**File:** src/Chap49/MinEditDistMtPer.rs
**Test File:** tests/Chap49/TestMinEditDistMtPer.rs
**Compile/Run:** `cargo test --test TestMinEditDistMtPer`

### 10.5 SubsetSumMtPer.rs (5 untested)
**File:** src/Chap49/SubsetSumMtPer.rs
**Test File:** tests/Chap49/TestSubsetSumMtPer.rs
**Compile/Run:** `cargo test --test TestSubsetSumMtPer`

### 10.6 MinEditDistStPer.rs (3 untested)
**File:** src/Chap49/MinEditDistStPer.rs
**Test File:** tests/Chap49/TestMinEditDistStPer.rs
**Compile/Run:** `cargo test --test TestMinEditDistStPer`

### 10.7 SubsetSumStPer.rs (2 untested)
**File:** src/Chap49/SubsetSumStPer.rs
**Test File:** tests/Chap49/TestSubsetSumStPer.rs
**Compile/Run:** `cargo test --test TestSubsetSumStPer`

### 10.8 MinEditDistMtEph.rs (1 untested)
**File:** src/Chap49/MinEditDistMtEph.rs
**Test File:** tests/Chap49/TestMinEditDistMtEph.rs
**Compile/Run:** `cargo test --test TestMinEditDistMtEph`

---

## Phase 11: Chap43 Ordered Table/Set Modules (7 files, ~16 untested functions)

### 11.1 OrderedTableMtEph.rs (3 untested)
**File:** src/Chap43/OrderedTableMtEph.rs
**Test File:** tests/Chap43/TestOrderedTableMtEph.rs
**Compile/Run:** `cargo test --test TestOrderedTableMtEph`

### 11.2 OrderedTableMtPer.rs (3 untested)
**File:** src/Chap43/OrderedTableMtPer.rs
**Test File:** Need to create test file
**Compile/Run:** `cargo test --test TestOrderedTableMtPer`

### 11.3 OrderedTableStEph.rs (3 untested)
**File:** src/Chap43/OrderedTableStEph.rs
**Test File:** tests/Chap43/TestOrderedTableStEph.rs
**Compile/Run:** `cargo test --test TestOrderedTableStEph`

### 11.4 OrderedSetMtEph.rs (2 untested)
**File:** src/Chap43/OrderedSetMtEph.rs
**Test File:** tests/Chap43/TestOrderedSetMtEph.rs
**Compile/Run:** `cargo test --test TestOrderedSetMtEph`

### 11.5 AugOrderedTableMtEph.rs (1 untested)
**File:** src/Chap43/AugOrderedTableMtEph.rs
**Test File:** tests/Chap43/TestAugOrderedTableMtEph.rs
**Compile/Run:** `cargo test --test TestAugOrderedTableMtEph`

### 11.6 AugOrderedTableStEph.rs (1 untested)
**File:** src/Chap43/AugOrderedTableStEph.rs
**Test File:** tests/Chap43/TestAugOrderedTableStEph.rs
**Compile/Run:** `cargo test --test TestAugOrderedTableStEph`

### 11.7 OrderedSetStEph.rs (1 untested)
**File:** src/Chap43/OrderedSetStEph.rs
**Test File:** tests/Chap43/TestOrderedSetStEph.rs
**Compile/Run:** `cargo test --test TestOrderedSetStEph`

### 11.8 OrderedTableStPer.rs (1 untested)
**File:** src/Chap43/OrderedTableStPer.rs
**Test File:** tests/Chap43/TestOrderedTableStPer.rs
**Compile/Run:** `cargo test --test TestOrderedTableStPer`

### 11.9 Example43_1.rs (1 untested)
**File:** src/Chap43/Example43_1.rs
**Test File:** tests/Chap43/TestExample43_1.rs
**Compile/Run:** `cargo test --test TestExample43_1`

---

## Phase 12: Chap52 Graph Representation Modules (9 files, ~23 untested functions)

### 12.1 EdgeSetGraphMtPer.rs (4 untested)
**File:** src/Chap52/EdgeSetGraphMtPer.rs
**Test File:** tests/Chap52/TestEdgeSetGraphMtPer.rs
**Compile/Run:** `cargo test --test TestEdgeSetGraphMtPer`

### 12.5 AdjTableGraphStEph.rs (3 untested)
**File:** src/Chap52/AdjTableGraphStEph.rs
**Test File:** tests/Chap52/TestAdjTableGraphStEph.rs
**Compile/Run:** `cargo test --test TestAdjTableGraphStEph`

### 12.6 EdgeSetGraphStEph.rs (3 untested)
**File:** src/Chap52/EdgeSetGraphStEph.rs
**Test File:** tests/Chap52/TestEdgeSetGraphStEph.rs
**Compile/Run:** `cargo test --test TestEdgeSetGraphStEph`

### 12.7 EdgeSetGraphStPer.rs (3 untested)
**File:** src/Chap52/EdgeSetGraphStPer.rs
**Test File:** tests/Chap52/TestEdgeSetGraphStPer.rs
**Compile/Run:** `cargo test --test TestEdgeSetGraphStPer`

### 12.8 AdjSeqGraphStEph.rs (2 untested)
**File:** src/Chap52/AdjSeqGraphStEph.rs
**Test File:** tests/Chap52/TestAdjSeqGraphStEph.rs
**Compile/Run:** `cargo test --test TestAdjSeqGraphStEph`

### 12.9 AdjTableGraphMtPer.rs (2 untested)
**File:** src/Chap52/AdjTableGraphMtPer.rs
**Test File:** tests/Chap52/TestAdjTableGraphMtPer.rs
**Compile/Run:** `cargo test --test TestAdjTableGraphMtPer`

### 12.10 AdjMatrixGraphStEph.rs (1 untested)
**File:** src/Chap52/AdjMatrixGraphStEph.rs
**Test File:** tests/Chap52/TestAdjMatrixGraphStEph.rs
**Compile/Run:** `cargo test --test TestAdjMatrixGraphStEph`

### 12.11 AdjMatrixGraphStPer.rs (1 untested)
**File:** src/Chap52/AdjMatrixGraphStPer.rs
**Test File:** tests/Chap52/TestAdjMatrixGraphStPer.rs
**Compile/Run:** `cargo test --test TestAdjMatrixGraphStPer`

### 12.12 AdjSeqGraphStPer.rs (1 untested)
**File:** src/Chap52/AdjSeqGraphStPer.rs
**Test File:** tests/Chap52/TestAdjSeqGraphStPer.rs
**Compile/Run:** `cargo test --test TestAdjSeqGraphStPer`

---

## Phase 13: Chap40-41 BST/Set Modules (8 files, 29 untested functions)

### 13.1 AVLTreeSetMtPer.rs (15 untested)
**File:** src/Chap41/AVLTreeSetMtPer.rs
**Test File:** tests/Chap41/TestAVLTreeSetMtPer.rs
**Compile/Run:** `cargo test --test TestAVLTreeSetMtPer`

### 13.2 AVLTreeSetMtEph.rs (3 untested)
**File:** src/Chap41/AVLTreeSetMtEph.rs
**Test File:** tests/Chap41/TestAVLTreeSetMtEph.rs
**Compile/Run:** `cargo test --test TestAVLTreeSetMtEph`

### 13.3 AVLTreeSetStEph.rs (3 untested)
**File:** src/Chap41/AVLTreeSetStEph.rs
**Test File:** tests/Chap41/TestAVLTreeSetStEph.rs
**Compile/Run:** `cargo test --test TestAVLTreeSetStEph`

### 13.4 AVLTreeSetStPer.rs (3 untested)
**File:** src/Chap41/AVLTreeSetStPer.rs
**Test File:** tests/Chap41/TestAVLTreeSetStPer.rs
**Compile/Run:** `cargo test --test TestAVLTreeSetStPer`

### 13.5 BSTReducedStEph.rs (2 untested)
**File:** src/Chap40/BSTReducedStEph.rs
**Test File:** tests/Chap40/TestBSTReducedStEph.rs
**Compile/Run:** `cargo test --test TestBSTReducedStEph`

### 13.6 ArraySetStEph.rs (1 untested)
**File:** src/Chap41/ArraySetStEph.rs
**Test File:** tests/Chap41/TestArraySetStEph.rs
**Compile/Run:** `cargo test --test TestArraySetStEph`

### 13.7 BSTKeyValueStEph.rs (1 untested)
**File:** src/Chap40/BSTKeyValueStEph.rs
**Test File:** tests/Chap40/TestBSTKeyValueStEph.rs
**Compile/Run:** `cargo test --test TestBSTKeyValueStEph`

### 13.8 BSTSizeStEph.rs (1 untested)
**File:** src/Chap40/BSTSizeStEph.rs
**Test File:** tests/Chap40/TestBSTSizeStEph.rs
**Compile/Run:** `cargo test --test TestBSTSizeStEph`

---

## Phase 14: Chap51 Dynamic Programming Top-Down/Bottom-Up (4 files, 9 untested functions)

### 14.1 TopDownDPMtEph.rs (3 untested)
**File:** src/Chap51/TopDownDPMtEph.rs
**Test File:** tests/Chap51/TestTopDownDPMtEph.rs
**Compile/Run:** `cargo test --test TestTopDownDPMtEph`

### 14.2 TopDownDPStEph.rs (3 untested)
**File:** src/Chap51/TopDownDPStEph.rs
**Test File:** tests/Chap51/TestTopDownDPStEph.rs
**Compile/Run:** `cargo test --test TestTopDownDPStEph`

### 14.3 BottomUpDPMtEph.rs (2 untested)
**File:** src/Chap51/BottomUpDPMtEph.rs
**Test File:** tests/Chap51/TestBottomUpDPMtEph.rs
**Compile/Run:** `cargo test --test TestBottomUpDPMtEph`

### 14.4 TopDownDPMtPer.rs (1 untested)
**File:** src/Chap51/TopDownDPMtPer.rs
**Test File:** tests/Chap51/TestTopDownDPMtPer.rs
**Compile/Run:** `cargo test --test TestTopDownDPMtPer`

---

## Phase 15: Chap55-59 Graph Algorithms (10 files, 23 untested functions)

### 15.1 JohnsonMtEphFloat.rs (4 untested)
**File:** src/Chap59/JohnsonMtEphFloat.rs
**Test File:** tests/Chap59/TestJohnsonMtEphFloat.rs
**Compile/Run:** `cargo test --test TestJohnsonMtEphFloat`

### 15.2 JohnsonMtEphInt.rs (3 untested)
**File:** src/Chap59/JohnsonMtEphInt.rs
**Test File:** tests/Chap59/TestJohnsonMtEphInt.rs
**Compile/Run:** `cargo test --test TestJohnsonMtEphInt`

### 15.3 JohnsonStEphFloat.rs (3 untested)
**File:** src/Chap59/JohnsonStEphFloat.rs
**Test File:** tests/Chap59/TestJohnsonStEphFloat.rs
**Compile/Run:** `cargo test --test TestJohnsonStEphFloat`

### 15.4 JohnsonStEphInt.rs (3 untested)
**File:** src/Chap59/JohnsonStEphInt.rs
**Test File:** tests/Chap59/TestJohnsonStEphInt.rs
**Compile/Run:** `cargo test --test TestJohnsonStEphInt`

### 15.5 ConnectivityMtEph.rs (3 untested)
**File:** src/Chap63/ConnectivityMtEph.rs
**Test File:** tests/Chap63/TestConnectivityMtEph.rs
**Compile/Run:** `cargo test --test TestConnectivityMtEph`

### 15.6 Example56_1.rs (3 untested)
**File:** src/Chap56/Example56_1.rs
**Test File:** tests/Chap56/TestExample56_1.rs
**Compile/Run:** `cargo test --test TestExample56_1`

### 15.7 TopoSortStEph.rs (2 untested)
**File:** src/Chap55/TopoSortStEph.rs
**Test File:** tests/Chap55/TestTopoSortStEph.rs
**Compile/Run:** `cargo test --test TestTopoSortStEph`

### 15.8 TopoSortStPer.rs (2 untested)
**File:** src/Chap55/TopoSortStPer.rs
**Test File:** tests/Chap55/TestTopoSortStPer.rs
**Compile/Run:** `cargo test --test TestTopoSortStPer`

### 15.9 Example56_3.rs (2 untested)
**File:** src/Chap56/Example56_3.rs
**Test File:** tests/Chap56/TestExample56_3.rs
**Compile/Run:** `cargo test --test TestExample56_3`

### 15.10 DijkstraStEphFloat.rs (1 untested)
**File:** src/Chap57/DijkstraStEphFloat.rs
**Test File:** tests/Chap57/TestDijkstraStEphFloat.rs
**Compile/Run:** `cargo test --test TestDijkstraStEphFloat`

### 15.11 DijkstraStEphInt.rs (1 untested)
**File:** src/Chap57/DijkstraStEphInt.rs
**Test File:** tests/Chap57/TestDijkstraStEphInt.rs
**Compile/Run:** `cargo test --test TestDijkstraStEphInt`

---

## Phase 16: Chap63-66 Spanning Trees & Connectivity (6 files, 14 untested functions)

### 16.1 UnionFindStEph.rs (7 untested)
**File:** src/Chap65/UnionFindStEph.rs
**Test File:** tests/Chap65/TestUnionFindStEph.rs
**Compile/Run:** `cargo test --test TestUnionFindStEph`

### 16.2 SpanTreeMtEph.rs (2 untested)
**File:** src/Chap64/SpanTreeMtEph.rs
**Test File:** tests/Chap64/TestSpanTreeMtEph.rs
**Compile/Run:** `cargo test --test TestSpanTreeMtEph`

### 16.3 BoruvkaMtEph.rs (2 untested)
**File:** src/Chap66/BoruvkaMtEph.rs
**Test File:** tests/Chap66/TestBoruvkaMtEph.rs
**Compile/Run:** `cargo test --test TestBoruvkaMtEph`

### 16.4 BoruvkaStEph.rs (2 untested)
**File:** src/Chap66/BoruvkaStEph.rs
**Test File:** tests/Chap66/TestBoruvkaStEph.rs
**Compile/Run:** `cargo test --test TestBoruvkaStEph`

### 16.5 PrimStEph.rs (1 untested)
**File:** src/Chap65/PrimStEph.rs
**Test File:** tests/Chap65/TestPrimStEph.rs
**Compile/Run:** `cargo test --test TestPrimStEph`

---

## Phase 17: Chap17, 23, 39 Sequences & Trees (5 files, 23 untested functions)

### 17.1 PrimTreeSeqSt.rs (8 untested)
**File:** src/Chap23/PrimTreeSeqSt.rs
**Test File:** tests/Chap23/TestPrimTreeSeqSt.rs
**Compile/Run:** `cargo test --test TestPrimTreeSeqSt`

### 17.2 BalBinTreeStEph.rs (7 untested)
**File:** src/Chap23/BalBinTreeStEph.rs
**Test File:** tests/Chap23/TestBalBinTreeStEph.rs
**Compile/Run:** `cargo test --test TestBalBinTreeStEph`

### 17.3 MathSeq.rs (6 untested)
**File:** src/Chap17/MathSeq.rs
**Test File:** tests/Chap18/TestMathSeq.rs
**Compile/Run:** `cargo test --test TestMathSeq`

### 17.4 BSTParaTreapMtEph.rs (2 untested)
**File:** src/Chap39/BSTParaTreapMtEph.rs
**Test File:** tests/Chap39/TestBSTParaTreapMtEph.rs
**Compile/Run:** `cargo test --test TestBSTParaTreapMtEph`

---

## Phase 18: Chap42, 44 Tables & Document Index (3 files, 13 untested functions)

### 18.1 DocumentIndex.rs (7 untested)
**File:** src/Chap44/DocumentIndex.rs
**Test File:** tests/Chap44/TestDocumentIndex.rs
**Compile/Run:** `cargo test --test TestDocumentIndex`

### 18.2 Example42_1.rs (6 untested)
**File:** src/Chap42/Example42_1.rs
**Test File:** tests/Chap42/TestExample42_1.rs
**Compile/Run:** `cargo test --test TestExample42_1`

---

## Phase 19: Chap12, 21, 26 Exercises & Problems (7 files, 11 untested functions)

### 19.1 Problem21_4.rs (4 untested)
**File:** src/Chap21/Problem21_4.rs
**Test File:** tests/Chap21/TestProblem_21_4.rs
**Compile/Run:** `cargo test --test TestProblem_21_4`

### 19.2 Exercise12_1.rs (4 untested)
**File:** src/Chap12/Exercise12_1.rs
**Test File:** tests/Chap12/TestExercise12_1.rs
**Compile/Run:** `cargo test --test TestExercise12_1`

### 19.3 Exercise12_5.rs (1 untested)
**File:** src/Chap12/Exercise12_5.rs
**Test File:** tests/Chap12/TestExercise12_5.rs
**Compile/Run:** `cargo test --test TestExercise12_5`

### 19.4 Problem21_1.rs (1 untested)
**File:** src/Chap21/Problem21_1.rs
**Test File:** tests/Chap21/TestProblem_21_1.rs
**Compile/Run:** `cargo test --test TestProblem_21_1`

### 19.5 Problem21_3.rs (1 untested)
**File:** src/Chap21/Problem21_3.rs
**Test File:** tests/Chap21/TestProblem_21_3.rs
**Compile/Run:** `cargo test --test TestProblem_21_3`

### 19.6 MergeSortMt.rs (1 untested)
**File:** src/Chap26/MergeSortMt.rs
**Test File:** tests/Chap26/TestMergeSortMt.rs
**Compile/Run:** `cargo test --test TestMergeSortMt`

---

## Phase 20: Main & Final Cleanup (1 file, 1 untested function)

### 20.1 main.rs (1 untested)
**File:** src/main.rs
**Test File:** Integration test or skip (main entry point)
**Compile/Run:** `cargo test` or mark as acceptable skip

---

## Execution Strategy

1. **Work in phases** - Complete one phase before moving to next
2. **For each file:**
   - Identify untested functions using coverage report
   - Read source file to understand function signatures
   - Add comprehensive tests to test file
   - Compile with `cargo test --test <TestFileName>`
   - Fix any compilation errors
   - Verify all tests pass
   - Update coverage

3. **Track progress:**
   - Mark each file complete in todos
   - Run `cargo llvm-cov report` after each phase
   - Document any issues or blockers

4. **Final verification:**
   - Run full `cargo llvm-cov --html`
   - Verify 100% function coverage achieved
   - Generate final report

---

## Estimated Effort

- **Phase 1-2:** ~3-4 hours (Types + BSTSet modules)
- **Phase 3-5:** ~4-5 hours (ArraySeq + Graph modules)
- **Phase 6-8:** ~3-4 hours (BST + Hash + PQ modules)
- **Phase 9-11:** ~3-4 hours (DP + Ordered modules)
- **Phase 12-14:** ~4-5 hours (Graph repr + remaining)

**Total:** ~17-22 hours estimated effort

---

## Notes

- Some functions may be internal/private and untestable from test files
- Some functions may be automatically tested through other function calls
- Edge cases and error conditions should be thoroughly tested
- Multithreaded (Mt) versions may require concurrent testing
- Persistent (Per) versions may require immutability testing
- Ephemeral (Eph) versions may require mutation testing
