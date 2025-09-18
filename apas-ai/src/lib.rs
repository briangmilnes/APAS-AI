//! Library crate root for `apas-ai`.
//!
//! Abstract:
//! - Declares modules, re-exports core types and concrete sequence structs.
//! - Provides macros `mathseq!`, `arrayseq!`, and `listseq!` to build sequences concisely.

pub mod Types;
pub use crate::Types::Types::*;

pub mod MathSeq;
pub use crate::MathSeq::MathSeq::{MathSeqS, MathSeqTrait};

pub mod SetStEphChap5_1;
pub use crate::SetStEphChap5_1::SetStEphChap5_1::*;

pub mod RelationStEphChap5_2;
pub use crate::RelationStEphChap5_2::RelationStEphChap5_2::*;

pub mod MappingStEphChap5_5;
pub use crate::MappingStEphChap5_5::MappingStEphChap5_5::*;

pub mod DirGraphStEphChap6_1;
pub use crate::DirGraphStEphChap6_1::DirGraphStEphChap6_1::*;
pub mod UnDirGraphStEphChap6_1;
pub use crate::UnDirGraphStEphChap6_1::UnDirGraphStEphChap6_1::*;

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

pub mod ArraySeqStPer;
pub use crate::ArraySeqStPer::ArraySeqStPer::*;

pub mod ArraySeqStPerChap18;
pub use crate::ArraySeqStPerChap18::ArraySeqStPerChap18::*;

pub mod ArraySeqStPerChap19;
pub use crate::ArraySeqStPerChap19::ArraySeqStPerChap19::*;

// Multithreaded persistent arrays
pub mod ArraySeqMtPer;
pub use crate::ArraySeqMtPer::ArraySeqMtPer::*;
pub mod ArraySeqMtPerChap18;
pub use crate::ArraySeqMtPerChap18::ArraySeqMtPerChap18::*;
pub mod ArraySeqMtPerChap19;
pub use crate::ArraySeqMtPerChap19::ArraySeqMtPerChap19::*;

pub mod ArraySeqStEph;
pub use crate::ArraySeqStEph::ArraySeqStEph::*;

pub mod ArraySeqStEphChap18;
pub use crate::ArraySeqStEphChap18::ArraySeqStEphChap18::*;
pub mod ArraySeqStEphChap19;
pub use crate::ArraySeqStEphChap19::ArraySeqStEphChap19::*;

// Multithreaded ephemeral arrays
pub mod ArraySeqMtEph;
pub use crate::ArraySeqMtEph::ArraySeqMtEph::*;
pub mod ArraySeqMtEphSlice;
pub use crate::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
pub mod ArraySeqMtEphChap18;
pub use crate::ArraySeqMtEphChap18::ArraySeqMtEphChap18::*;
pub mod ArraySeqMtEphChap19;
pub use crate::ArraySeqMtEphChap19::ArraySeqMtEphChap19::*;

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
pub use crate::BSTPlainStEph::BSTPlainStEph::{BSTPlainStEphTrait, BSTree as BSTPlainStTree};
pub mod BSTAVLStEph;
pub use crate::BSTAVLStEph::BSTAVLStEph::{BSTAVLStEphTrait, BSTreeAVL as BSTAVLStTree};
pub mod BSTRBStEph;
pub use crate::BSTRBStEph::BSTRBStEph::{BSTRBStEphTrait, BSTreeRB as BSTRBStTree};
pub mod BSTBBAlphaStEph;
pub use crate::BSTBBAlphaStEph::BSTBBAlphaStEph::{BSTBBAlphaStEphTrait, BSTreeBBAlpha as BSTBBAlphaStTree};
pub mod BSTTreapStEph;
pub use crate::BSTTreapStEph::BSTTreapStEph::{BSTTreapStEphTrait, BSTreeTreap as BSTTreapStTree};
pub mod BSTSplayStEph;
pub use crate::BSTSplayStEph::BSTSplayStEph::{BSTSplayStEphTrait, BSTreeSplay as BSTSplayStTree};

pub mod BSTPlainMtEph;
pub use crate::BSTPlainMtEph::BSTPlainMtEph::{BSTPlainMtEphTrait, BSTree as BSTPlainMtTree};
pub mod BSTAVLMtEph;
pub use crate::BSTAVLMtEph::BSTAVLMtEph::{BSTAVLMtEphTrait, BSTreeAVL as BSTAVLMtTree};
pub mod BSTRBMtEph;
pub use crate::BSTRBMtEph::BSTRBMtEph::{BSTRBMtEphTrait, BSTreeRB as BSTRBMtTree};
pub mod BSTBBAlphaMtEph;
pub use crate::BSTBBAlphaMtEph::BSTBBAlphaMtEph::{BSTBBAlphaMtEphTrait, BSTreeBBAlpha as BSTBBAlphaMtTree};
pub mod BSTTreapMtEph;
pub use crate::BSTTreapMtEph::BSTTreapMtEph::{BSTTreapMtEphTrait, BSTreeTreap as BSTTreapMtTree};
pub mod BSTSplayMtEph;
pub use crate::BSTSplayMtEph::BSTSplayMtEph::{BSTSplayMtEphTrait, BSTreeSplay as BSTSplayMtTree};
