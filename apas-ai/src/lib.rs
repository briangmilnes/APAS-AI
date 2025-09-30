//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Library crate root for `apas-ai`.
//!
//! Minimal lib.rs which does no reexporting and so requires pub use apas-ai::Chap03::Chap03::* imports.

pub mod Types;

pub mod Chap03 {
    pub mod InsertionSortSt;
}

pub mod Chap05 {
    pub mod MappingStEph;
    pub mod RelationStEph;
    pub mod SetStEph;
}

pub mod Chap06 {
    pub mod DirGraphMtEph;
    pub mod DirGraphStEph;
    pub mod LabDirGraphMtEph;
    pub mod LabDirGraphStEph;
    pub mod LabUnDirGraphMtEph;
    pub mod LabUnDirGraphStEph;
    pub mod UnDirGraphMtEph;
    pub mod UnDirGraphStEph;
    pub mod WeightedDirGraphMtEphFloat;
    pub mod WeightedDirGraphMtEphInt;
    pub mod WeightedDirGraphStEphFloat;
    pub mod WeightedDirGraphStEphInt;
    pub mod WeightedUnDirGraphMtEphFloat;
    pub mod WeightedUnDirGraphMtEphInt;
    pub mod WeightedUnDirGraphStEphFloat;
    pub mod WeightedUnDirGraphStEphInt;
}

pub mod Chap11 {
    pub mod FibonacciMt;
}

pub mod Chap12 {
    pub mod Exercise12_1;
    pub mod Exercise12_2;
    pub mod Exercise12_5;
}

pub mod Chap17 {
    pub mod MathSeq;
}

pub mod Chap21 {
    pub mod Algorithm21_1;
    pub mod Algorithm21_2;
    pub mod Algorithm21_5;
    pub mod Algorithm21_6;
    pub mod Exercise21_5;
    pub mod Exercise21_6;
    pub mod Exercise21_7;
    pub mod Exercise21_8;
    pub mod Exercise21_9;
    pub mod Problem21_1;
    pub mod Problem21_3;
    pub mod Problem21_4;
}

pub mod Chap18 {
    pub mod ArraySeq;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtPer;
    pub mod ArraySeqStEph;
    pub mod ArraySeqStPer;

    pub mod LinkedListStEph;
    pub mod LinkedListStPer;
}

pub mod Chap19 {
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtEphSlice;
    pub mod ArraySeqMtPer;
    pub mod ArraySeqStEph;
    pub mod ArraySeqStPer;
}

pub mod Chap23 {
    pub mod BBTStEph;
    pub mod PrimTreeSeqSt;
}

pub mod Chap36 {
    pub mod QuickSortMt;
    pub mod QuickSortMtSlice;
    pub mod QuickSortSt;
}

pub mod Chap37 {
    pub mod AVLTreeSeq;
    pub mod AVLTreeSeqMtPer;
    pub mod AVLTreeSeqStEph;
    pub mod AVLTreeSeqStPer;
    pub mod BSTAVLMtEph;
    pub mod BSTAVLStEph;
    pub mod BSTBBAlphaMtEph;
    pub mod BSTBBAlphaStEph;
    pub mod BSTPlainMtEph;
    pub mod BSTPlainStEph;
    pub mod BSTRBMtEph;
    pub mod BSTRBStEph;
    pub mod BSTSetAVLMtEph;
    pub mod BSTSetBBAlphaMtEph;
    pub mod BSTSetPlainMtEph;
    pub mod BSTSetRBMtEph;
    pub mod BSTSetSplayMtEph;
    pub mod BSTSplayMtEph;
    pub mod BSTSplayStEph;
}

pub mod Chap38 {
    pub mod BSTParaMtEph;
    pub mod BSTParaStEph;
}

pub mod Chap39 {
    pub mod BSTParaTreapMtEph;
    pub mod BSTSetTreapMtEph;
    pub mod BSTTreapMtEph;
    pub mod BSTTreapStEph;
}

pub mod Chap40 {
    pub mod BSTKeyValueStEph;
    pub mod BSTReducedStEph;
    pub mod BSTSizeStEph;
}

pub mod Chap41 {
    pub mod AVLTreeSetMtEph;
    pub mod AVLTreeSetMtPer;
    pub mod AVLTreeSetStEph;
    pub mod AVLTreeSetStPer;
    pub mod ArraySetEnumMtEph;
    pub mod ArraySetStEph;
    pub mod Example41_3;
}

pub mod Chap42Claude {
    pub mod Example42_1;
    pub mod TableMtEph;
    pub mod TableStEph;
    pub mod TableStPer;
}

pub mod Chap43Claude {
    pub mod AugOrderedTableMtEph;
    pub mod AugOrderedTableStEph;
    pub mod AugOrderedTableStPer;
    pub mod Example43_1;
    pub mod OrderedSetMtEph;
    pub mod OrderedSetStEph;
    pub mod OrderedSetStPer;
    pub mod OrderedTableMtEph;
    pub mod OrderedTableMtPer;
    pub mod OrderedTableStEph;
    pub mod OrderedTableStPer;
}

pub mod Chap44 {
    pub mod DocumentIndex;
    pub mod Example44_1;
}

pub mod Chap45 {
    pub mod BalancedTreePQ;
    pub mod BinaryHeapPQ;
    pub mod HeapsortExample;
    pub mod LeftistHeapPQ;
    pub mod SortedListPQ;
    pub mod UnsortedListPQ;
}

pub mod Chap47 {
    pub mod AdvancedDoubleHashing;
    pub mod AdvancedLinearProbing;
    pub mod AdvancedQuadraticProbing;
    pub mod ClusteringAnalysis;
    pub mod DoubleHashing;
    pub mod FlatHashTable;
    pub mod HashExamples;
    pub mod HashFunctionTraits;
    pub mod LinearProbing;
    pub mod NestedHashTable;
    pub mod ProbeSequenceExamples;
    pub mod QuadraticProbing;
    pub mod SeparateChaining;
}

pub mod Chap49 {
    pub mod MinEditDistMtEph;
    pub mod MinEditDistMtPer;
    pub mod MinEditDistStEph;
    pub mod MinEditDistStPer;
    pub mod SubsetSumMtEph;
    pub mod SubsetSumMtPer;
    pub mod SubsetSumStEph;
    pub mod SubsetSumStPer;
}

pub mod Chap50 {
    pub mod MatrixChainMtEph;
    pub mod MatrixChainMtPer;
    pub mod MatrixChainStEph;
    pub mod MatrixChainStPer;
    pub mod OptBinSearchTreeMtEph;
    pub mod OptBinSearchTreeMtPer;
    pub mod OptBinSearchTreeStEph;
    pub mod OptBinSearchTreeStPer;
    pub mod Probability;
}

pub mod Chap51 {
    pub mod BottomUpDPMtEph;
    pub mod BottomUpDPMtPer;
    pub mod BottomUpDPStEph;
    pub mod BottomUpDPStPer;
    pub mod TopDownDPMtEph;
    pub mod TopDownDPMtPer;
    pub mod TopDownDPStEph;
    pub mod TopDownDPStPer;
}

pub mod Chap52 {
    pub mod EdgeSetGraphMtPer;
    pub mod EdgeSetGraphStEph;
    pub mod EdgeSetGraphStPer;
    // pub mod EdgeSetGraphMtEph;
    pub mod AdjTableGraphMtPer;
    pub mod AdjTableGraphStEph;
    pub mod AdjTableGraphStPer;
    // pub mod AdjTableGraphMtEph;
    pub mod AdjSeqGraphStEph;
    pub mod AdjSeqGraphStPer;
    // pub mod AdjSeqGraphMtPer; // API mismatch - ArraySeqMtPerS lacks full interface
    // pub mod AdjSeqGraphMtEph; // API mismatch - ArraySeqMtEphS lacks full interface
    pub mod AdjMatrixGraphStEph;
    pub mod AdjMatrixGraphStPer;
    // pub mod AdjMatrixGraphMtPer; // API mismatch - ArraySeqMtPerS lacks full interface
    // pub mod AdjMatrixGraphMtEph; // API mismatch - ArraySeqMtEphS lacks full interface
}

pub mod Chap53 {
    pub mod GraphSearchMtPer;
    pub mod GraphSearchStEph;
    pub mod GraphSearchStPer;
    pub mod PQMinMtEph;
    pub mod PQMinMtPer;
    pub mod PQMinStEph;
    pub mod PQMinStPer;
}
