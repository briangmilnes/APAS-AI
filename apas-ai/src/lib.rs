//! Library crate root for `apas-ai`.
//!
//! Abstract:
//! - Declares modules, re-exports core types and concrete sequence structs.
//! - Provides macros `mathseq!`, `arrayseq!`, and `listseq!` to build sequences concisely.

// Codex write-test comment.

pub mod Types;
pub use crate::Types::Types::*;

pub mod Chap03 {
    pub mod InsertionSortSt;
    pub use InsertionSortSt::*;
}

pub mod Chap05 {
    pub mod SetStEphChap5_1;
    pub mod RelationStEphChap5_2;
    pub mod MappingStEphChap5_5;
}

pub mod Chap18 {
    pub mod MathSeq;
}
pub use crate::Chap18::MathSeq::MathSeq::{MathSeqS, MathSeqTrait};

pub mod Chap19 {
    pub mod ArraySeqStPer;
    pub mod ArraySeqStPerChap18;
    pub mod ArraySeqStPerChap19;
    pub mod ArraySeqStEph;
    pub mod ArraySeqStEphChap18;
    pub mod ArraySeqStEphChap19;
    pub mod ArraySeqMtPer;
    pub mod ArraySeqMtPerChap18;
    pub mod ArraySeqMtPerChap19;
    pub mod ArraySeqMtEph;
    pub mod ArraySeqMtEphChap18;
    pub mod ArraySeqMtEphChap19;
    pub mod ArraySeqMtEphSlice;
}
pub use crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::*;
pub use crate::Chap05::RelationStEphChap5_2::RelationStEphChap5_2::*;
pub use crate::Chap05::MappingStEphChap5_5::MappingStEphChap5_5::*;

pub mod Chap06 {
    pub mod DirGraphStEph;
    pub mod UnDirGraphStEph;
    pub mod LabDirGraphStEph;
    pub mod LabUnDirGraphStEph;
    pub mod DirGraphMtEph;
    pub mod UnDirGraphMtEph;
    pub mod LabDirGraphMtEph;
    pub mod LabUnDirGraphMtEph;
    pub mod WeightedDirGraphStEphInt;
    pub mod WeightedDirGraphStEphFloat;
    pub mod WeightedDirGraphMtEphInt;
    pub mod WeightedDirGraphMtEphFloat;
    pub mod WeightedUnDirGraphStEphInt;
    pub mod WeightedUnDirGraphStEphFloat;
    pub mod WeightedUnDirGraphMtEphInt;
    pub mod WeightedUnDirGraphMtEphFloat;
}
pub use crate::Chap06::DirGraphStEph::DirGraphStEph::*;
pub use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
pub use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
pub use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
pub use crate::Chap06::DirGraphMtEph::DirGraphMtEph::*;
pub use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
pub use crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
pub use crate::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
pub use crate::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::*;
pub use crate::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
pub use crate::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::*;
pub use crate::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
pub use crate::Chap06::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::*;
pub use crate::Chap06::WeightedUnDirGraphStEphFloat::WeightedUnDirGraphStEphFloat::*;
pub use crate::Chap06::WeightedUnDirGraphMtEphInt::WeightedUnDirGraphMtEphInt::*;
pub use crate::Chap06::WeightedUnDirGraphMtEphFloat::WeightedUnDirGraphMtEphFloat::*;

pub mod Chap11 {
    pub mod FibonacciMt;
}
pub use crate::Chap11::FibonacciMt::FibonacciMt::*;

pub mod Chap12 {
    pub mod Exercise12_1;
    pub mod Exercise12_2;
    pub mod Exercise12_5;
}
pub use crate::Chap12::Exercise12_1::Exercise12_1::*;
pub use crate::Chap12::Exercise12_2::Exercise12_2::*;
pub use crate::Chap12::Exercise12_5::Exercise12_5::*;

pub mod LinkedListStPer;
pub use crate::LinkedListStPer::LinkedListStPer::*;
pub mod LinkedListStPerChap18;
pub use crate::LinkedListStPerChap18::LinkedListStPerChap18::*;
pub mod LinkedListStPerChap19;
pub use crate::LinkedListStPerChap19::LinkedListStPerChap19::*;

pub mod LinkedListStEph;
pub use crate::LinkedListStEph::LinkedListStEph::*;
pub mod LinkedListStEphChap18;
pub use crate::LinkedListStEphChap18::LinkedListStEphChap18::*;
pub mod LinkedListStEphChap19;
pub use crate::LinkedListStEphChap19::LinkedListStEphChap19::*;

// All ArraySeq modules moved to Chap19
pub use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
pub use crate::Chap19::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
pub use crate::Chap19::ArraySeqStPerChap19::ArraySeqStPerChap19::*;
pub use crate::Chap19::ArraySeqMtPer::ArraySeqMtPer::*;
pub use crate::Chap19::ArraySeqMtPerChap18::ArraySeqMtPerChap18::*;
pub use crate::Chap19::ArraySeqMtPerChap19::ArraySeqMtPerChap19::*;
pub use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
pub use crate::Chap19::ArraySeqStEphChap18::ArraySeqStEphChap18::*;
pub use crate::Chap19::ArraySeqStEphChap19::ArraySeqStEphChap19::*;
pub use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
pub use crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
pub use crate::Chap19::ArraySeqMtEphChap18::ArraySeqMtEphChap18::*;
pub use crate::Chap19::ArraySeqMtEphChap19::ArraySeqMtEphChap19::*;

pub mod AVLTreeSeqStPer;
pub use crate::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
pub mod AVLTreeSeqStPerChap18;
pub use crate::AVLTreeSeqStPerChap18::AVLTreeSeqStPerChap18::*;
pub mod AVLTreeSeqStPerChap19;
pub use crate::AVLTreeSeqStPerChap19::AVLTreeSeqStPerChap19::*;

pub mod AVLTreeSeqStEph;
pub use crate::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
pub mod AVLTreeSeqStEphChap18;
pub use crate::AVLTreeSeqStEphChap18::AVLTreeSeqStEphChap18::*;
pub mod AVLTreeSeqStEphChap19;
pub use crate::AVLTreeSeqStEphChap19::AVLTreeSeqStEphChap19::*;

// Chapter 36 quicksorts
pub mod Chapter36St;
pub use crate::Chapter36St::Chapter36St::*;
pub mod Chapter36Mt;
pub use crate::Chapter36Mt::Chapter36Mt::*;
pub mod Chapter36MtSlice;
pub use crate::Chapter36MtSlice::Chapter36MtSlice::*;

// Chapter 37 binary trees
pub mod BBTEph;
pub use crate::BBTEph::BBTEph::{BBNode, BBTree};

pub mod BSTPlainStEph;
pub use crate::BSTPlainStEph::BSTPlainStEph::{BSTPlainStEph as BSTPlainStTree, BSTPlainStEphTrait};
pub mod BSTAVLStEph;
pub use crate::BSTAVLStEph::BSTAVLStEph::{BSTAVLStEph as BSTAVLStTree, BSTAVLStEphTrait};
pub mod BSTRBStEph;
pub use crate::BSTRBStEph::BSTRBStEph::{BSTRBStEph as BSTRBStTree, BSTRBStEphTrait};
pub mod BSTBBAlphaStEph;
pub use crate::BSTBBAlphaStEph::BSTBBAlphaStEph::{BSTBBAlphaStEph as BSTBBAlphaStTree, BSTBBAlphaStEphTrait};
pub mod BSTTreapStEph;
pub use crate::BSTTreapStEph::BSTTreapStEph::{BSTTreapStEph as BSTTreapStTree, BSTTreapStEphTrait};
pub mod BSTSplayStEph;
pub use crate::BSTSplayStEph::BSTSplayStEph::{BSTSplayStEph as BSTSplayStTree, BSTSplayStEphTrait};

pub mod BSTPlainMtEph;
pub use crate::BSTPlainMtEph::BSTPlainMtEph::{BSTPlainMtEph as BSTPlainMtTree, BSTPlainMtEphTrait};
pub mod BSTAVLMtEph;
pub use crate::BSTAVLMtEph::BSTAVLMtEph::{BSTAVLMtEph as BSTAVLMtTree, BSTAVLMtEphTrait};
pub mod BSTRBMtEph;
pub use crate::BSTRBMtEph::BSTRBMtEph::{BSTRBMtEph as BSTRBMtTree, BSTRBMtEphTrait};
pub mod BSTBBAlphaMtEph;
pub use crate::BSTBBAlphaMtEph::BSTBBAlphaMtEph::{BSTBBAlphaMtEph as BSTBBAlphaMtTree, BSTBBAlphaMtEphTrait};
pub mod BSTTreapMtEph;
pub use crate::BSTTreapMtEph::BSTTreapMtEph::{BSTTreapMtEph as BSTTreapMtTree, BSTTreapMtEphTrait};
pub mod BSTSplayMtEph;
pub use crate::BSTSplayMtEph::BSTSplayMtEph::{BSTSplayMtEph as BSTSplayMtTree, BSTSplayMtEphTrait};
pub mod BSTParaMtEph;
pub use crate::BSTParaMtEph::BSTParaMtEph::{ParamBST as BSTParaMtTree, ParamBSTTrait as BSTParaMtTrait};
pub mod BSTParaTreapMtEph;
pub use crate::BSTParaTreapMtEph::BSTParaTreapMtEph::{ParamTreap as BSTParaTreapMtTree, ParamTreapTrait as BSTParaTreapMtTrait};
pub mod BSTParaStEph;
pub use crate::BSTParaStEph::BSTParaStEph::{ParamBST as BSTParaStTree, ParamBSTTrait as BSTParaStTrait};

pub mod BSTSetAVLMtEph;
pub mod BSTSetBBAlphaMtEph;
pub mod BSTSetPlainMtEph;
pub mod BSTSetRBMtEph;
pub mod BSTSetSplayMtEph;
pub mod BSTSetTreapMtEph;
