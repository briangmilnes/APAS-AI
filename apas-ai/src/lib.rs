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
    pub mod SetStEphClean;
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
    pub mod ArraySeqStEphClean;
    pub mod ArraySeqStEphMinimal;
    pub mod ArraySeqStEphSimple;
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
    pub mod BalBinTreeStEph;
    pub mod PrimTreeSeqSt;
}

pub mod Chap26 {
    pub mod DivConReduceMt;
    pub mod DivConReduceSt;
    pub mod MergeSortMt;
    pub mod MergeSortSt;
}

pub mod Chap27 {
    pub mod ReduceContractMtEph;
    pub mod ReduceContractStEph;
    pub mod ScanContractMtEph;
    pub mod ScanContractStEph;
}

pub mod Chap28 {
    pub mod MaxContigSubSumBruteStEph;
    pub mod MaxContigSubSumDivConMtEph;
    pub mod MaxContigSubSumDivConOptMtEph;
    pub mod MaxContigSubSumDivConOptStEph;
    pub mod MaxContigSubSumDivConStEph;
    pub mod MaxContigSubSumOptMtEph;
    pub mod MaxContigSubSumOptStEph;
    pub mod MaxContigSubSumReducedStEph;
}

pub mod Chap35 {
    pub mod OrderStatSelectMtEph;
    pub mod OrderStatSelectMtPer;
    pub mod OrderStatSelectStEph;
    pub mod OrderStatSelectStPer;
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

pub mod Chap42 {
    pub mod Example42_1;
    pub mod TableMtEph;
    pub mod TableStEph;
    pub mod TableStPer;
}

pub mod Chap43 {
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
    pub mod Example45_2;
    pub mod HeapsortExample;
    pub mod LeftistHeapPQ;
    pub mod SortedListPQ;
    pub mod UnsortedListPQ;
}

pub mod Chap47 {
    pub mod ChainedHashTable;
    pub mod DoubleHashFlatHashTableStEph;
    pub mod FlatHashTable;
    pub mod LinProbFlatHashTableStEph;
    pub mod LinkedListChainedHashTableStEph;
    pub mod ParaHashTableStEph;
    pub mod QuadProbFlatHashTableStEph;
    pub mod StructChainedHashTable;
    pub mod VecChainedHashTableStEph;
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
    pub mod AdjTableGraphMtPer;
    pub mod AdjTableGraphStEph;
    pub mod AdjTableGraphStPer;
    pub mod EdgeSetGraphMtPer;
    pub mod EdgeSetGraphStEph;
    pub mod EdgeSetGraphStPer;
    // pub mod AdjSeqGraphMtEph; // API mismatch - ArraySeqMtEphS lacks nth method
    pub mod AdjSeqGraphMtPer;
    pub mod AdjSeqGraphStEph;
    pub mod AdjSeqGraphStPer;
    // pub mod AdjMatrixGraphMtEph; // API mismatch - ArraySeqMtEphS lacks nth method
    pub mod AdjMatrixGraphMtPer;
    pub mod AdjMatrixGraphStEph;
    pub mod AdjMatrixGraphStPer;
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

pub mod Chap54 {
    pub mod BFSMtEph;
    pub mod BFSMtPer;
    pub mod BFSStEph;
    pub mod BFSStPer;
}

pub mod Chap55 {
    pub mod CycleDetectStEph;
    pub mod CycleDetectStPer;
    pub mod DFSStEph;
    pub mod DFSStPer;
    pub mod SCCStEph;
    pub mod SCCStPer;
    pub mod TopoSortStEph;
    pub mod TopoSortStPer;
}

pub mod Chap56 {
    pub mod AllPairsResultStEphFloat;
    pub mod AllPairsResultStEphInt;
    pub mod AllPairsResultStPerFloat;
    pub mod AllPairsResultStPerInt;
    pub mod Example56_1;
    pub mod Example56_3;
    pub mod PathWeightUtilsStEph;
    pub mod PathWeightUtilsStPer;
    pub mod SSSPResultStEphFloat;
    pub mod SSSPResultStEphInt;
    pub mod SSSPResultStPerFloat;
    pub mod SSSPResultStPerInt;
}

pub mod Chap57 {
    pub mod DijkstraStEphFloat;
    pub mod DijkstraStEphInt;
    pub mod StackStEph;
}

pub mod Chap58 {
    pub mod BellmanFordStEphFloat;
    pub mod BellmanFordStEphInt;
}

pub mod Chap59 {
    pub mod JohnsonMtEphFloat;
    pub mod JohnsonMtEphInt;
    pub mod JohnsonStEphFloat;
    pub mod JohnsonStEphInt;
}

pub mod Chap61 {
    pub mod EdgeContractionMtEph;
    pub mod EdgeContractionStEph;
    pub mod VertexMatchingMtEph;
    pub mod VertexMatchingStEph;
}
pub mod Chap62 {
    pub mod StarContractionMtEph;
    pub mod StarContractionStEph;
    pub mod StarPartitionMtEph;
    pub mod StarPartitionStEph;
}
pub mod Chap63 {
    pub mod ConnectivityMtEph;
    pub mod ConnectivityStEph;
}
pub mod Chap64 {
    pub mod SpanTreeMtEph;
    pub mod SpanTreeStEph;
    pub mod TSPApproxMtEph;
    pub mod TSPApproxStEph;
}
pub mod Chap65 {
    pub mod KruskalStEph;
    pub mod PrimStEph;
    pub mod UnionFindStEph;
}
pub mod Chap66 {
    pub mod BoruvkaMtEph;
    pub mod BoruvkaStEph;
}
