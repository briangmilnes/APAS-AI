//! Library crate root for `apas-ai`.
//!
//! Abstract:
//! - Declares modules, re-exports core types and concrete sequence structs.
//! - Provides macros `mathseq!`, `arrayseq!`, and `listseq!` to build sequences concisely.

pub mod Types;
pub use crate::Types::Types::*;

pub mod MathSeq;
pub use crate::MathSeq::MathSeq::{MathSeqS, MathSeqTrait};

pub mod SetEphChap5_1;
pub use crate::SetEphChap5_1::SetEphChap5_1::*;

pub mod RelationEphChap5_2;
pub use crate::RelationEphChap5_2::RelationEphChap5_2::*;

pub mod MappingEphChap5_5;
pub use crate::MappingEphChap5_5::MappingEphChap5_5::*;

pub mod DirGraphEphChap6_1;
pub use crate::DirGraphEphChap6_1::DirGraphEphChap6_1::*;
pub mod UnDirGraphEphChap6_1;
pub use crate::UnDirGraphEphChap6_1::UnDirGraphEphChap6_1::*;

pub mod LinkedListPer;
pub use crate::LinkedListPer::LinkedListPer::*;
pub mod LinkedListPerChap18;
pub use crate::LinkedListPerChap18::*;
pub mod LinkedListPerChap19;
pub use crate::LinkedListPerChap19::*;

pub mod LinkedListEph;
pub use crate::LinkedListEph::LinkedListEph::*;
pub mod LinkedListEphChap18;
pub use crate::LinkedListEphChap18::*;
pub mod LinkedListEphChap19;
pub use crate::LinkedListEphChap19::*;

pub mod ArraySeqPer;
pub use crate::ArraySeqPer::ArraySeqPer::*;

pub mod ArraySeqPerChap18;
pub use crate::ArraySeqPerChap18::*;

pub mod ArraySeqPerChap19;
pub use crate::ArraySeqPerChap19::*;

pub mod ArraySeqEph;
pub use crate::ArraySeqEph::ArraySeqEph::*;

pub mod ArraySeqEphChap18;
pub use crate::ArraySeqEphChap18::ArraySeqEphChap18::*;
pub mod ArraySeqEphChap19;
pub use crate::ArraySeqEphChap19::ArraySeqEphChap19::*;

pub mod AVLTreeSeqPer;
pub use crate::AVLTreeSeqPer::AVLTreeSeqPer::*;
pub mod AVLTreeSeqPerChap18;
pub use crate::AVLTreeSeqPerChap18::AVLTreeSeqPerChap18::*;
pub mod AVLTreeSeqPerChap19;
pub use crate::AVLTreeSeqPerChap19::AVLTreeSeqPerChap19::*;

pub mod AVLTreeSeqEph;
pub use crate::AVLTreeSeqEph::AVLTreeSeqEph::*;
