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
