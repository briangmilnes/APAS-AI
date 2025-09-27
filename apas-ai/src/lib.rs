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
        pub mod Exercise21_5;
        pub mod Exercise21_6;
        pub mod Exercise21_7;
        pub mod Exercise21_8;
        pub mod Exercise21_9;
        pub mod Algorithm21_1;
        pub mod Algorithm21_2;
        pub mod Algorithm21_5;
        pub mod Algorithm21_6;
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
