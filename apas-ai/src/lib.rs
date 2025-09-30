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
    pub mod AVLTreeSeqStEph;
    pub mod AVLTreeSeqStPer;
    pub mod AVLTreeSeqMtPer;
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
    pub mod AVLTreeSetStEph;
    pub mod AVLTreeSetStPer;
    pub mod AVLTreeSetMtPer;
    pub mod AVLTreeSetMtEph;
    pub mod ArraySetEnumMtEph;
    pub mod ArraySetStEph;
    pub mod Example41_3;
}


pub mod Chap42Claude {
    pub mod TableStPer;
    pub mod TableStEph;
    pub mod TableMtEph;
    pub mod Example42_1;
}

pub mod Chap43Claude {
    pub mod OrderedSetStPer;
    pub mod OrderedSetStEph;
    pub mod OrderedSetMtEph;
    pub mod OrderedTableStPer;
    pub mod OrderedTableMtPer;
    pub mod OrderedTableStEph;
    pub mod OrderedTableMtEph;
    pub mod AugOrderedTableStPer;
    pub mod AugOrderedTableStEph;
    pub mod AugOrderedTableMtEph;
    pub mod Example43_1;
}

pub mod Chap44 {
    pub mod DocumentIndex;
    pub mod Example44_1;
}

pub mod Chap45 {
    pub mod UnsortedListPQ;
    pub mod SortedListPQ;
    pub mod BalancedTreePQ;
    pub mod BinaryHeapPQ;
    pub mod LeftistHeapPQ;
    pub mod HeapsortExample;
}

pub mod Chap47 {
    pub mod HashFunctionTraits;
    pub mod NestedHashTable;
    pub mod SeparateChaining;
    pub mod FlatHashTable;
    pub mod LinearProbing;
    pub mod QuadraticProbing;
    pub mod DoubleHashing;
    pub mod HashExamples;
    pub mod AdvancedLinearProbing;
    pub mod AdvancedQuadraticProbing;
    pub mod AdvancedDoubleHashing;
    pub mod ClusteringAnalysis;
    pub mod ProbeSequenceExamples;
}

pub mod Chap49 {
    pub mod SubsetSumStPer;
    pub mod SubsetSumStEph;
    pub mod SubsetSumMtPer;
    pub mod SubsetSumMtEph;
    pub mod MinEditDistStPer;
    pub mod MinEditDistStEph;
    pub mod MinEditDistMtPer;
    pub mod MinEditDistMtEph;
}

    pub mod Chap50 {
        pub mod Probability;
        pub mod OptBinSearchTreeStPer;
        pub mod OptBinSearchTreeStEph;
        pub mod OptBinSearchTreeMtPer;
        pub mod OptBinSearchTreeMtEph;
        pub mod MatrixChainStPer;
        pub mod MatrixChainStEph;
        pub mod MatrixChainMtPer;
        pub mod MatrixChainMtEph;
    }

pub mod Chap51 {
    pub mod BottomUpDPStPer;
    pub mod BottomUpDPStEph;
    pub mod BottomUpDPMtPer;
    pub mod BottomUpDPMtEph;
    pub mod TopDownDPStPer;
    pub mod TopDownDPStEph;
    pub mod TopDownDPMtPer;
    pub mod TopDownDPMtEph;
}

pub mod Chap52 {
    pub mod EdgeSetGraphStPer;
    pub mod EdgeSetGraphStEph;
    pub mod EdgeSetGraphMtPer;
    // pub mod EdgeSetGraphMtEph;
    pub mod AdjTableGraphStPer;
    pub mod AdjTableGraphStEph;
    pub mod AdjTableGraphMtPer;
    // pub mod AdjTableGraphMtEph;
    pub mod AdjSeqGraphStPer;
    pub mod AdjSeqGraphStEph;
    // pub mod AdjSeqGraphMtPer; // API mismatch - ArraySeqMtPerS lacks full interface
    // pub mod AdjSeqGraphMtEph; // API mismatch - ArraySeqMtEphS lacks full interface
    pub mod AdjMatrixGraphStPer;
    pub mod AdjMatrixGraphStEph;
    // pub mod AdjMatrixGraphMtPer; // API mismatch - ArraySeqMtPerS lacks full interface
    // pub mod AdjMatrixGraphMtEph; // API mismatch - ArraySeqMtEphS lacks full interface
}

pub mod Chap53 {
    pub mod GraphSearchStPer;
    pub mod GraphSearchStEph;
    pub mod GraphSearchMtPer;
    pub mod PQMinStPer;
    pub mod PQMinStEph;
    pub mod PQMinMtPer;
    pub mod PQMinMtEph;
}

