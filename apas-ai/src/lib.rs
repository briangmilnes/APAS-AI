//! Library crate root for `apas-ai`.
//!
//! Abstract:
//! - Declares modules, re-exports core types and concrete sequence structs.
//! - Provides macros `mathseq!`, `arrayseq!`, and `listseq!` to build sequences concisely.

pub mod Types;

pub mod MathSeq;
pub use crate::Types::{N, B, O};
pub use crate::MathSeq::MathSeqS;




pub mod LinkedListPer;
pub use crate::LinkedListPer::{LinkedListPerS, LinkedListPerTrait};
pub mod LinkedListPerChap18;
pub use crate::LinkedListPerChap18::LinkedListPerChap18Trait;
pub mod LinkedListPerChap19;
pub use crate::LinkedListPerChap19::LinkedListPerChap19Trait;

pub mod LinkedListEph;
pub use crate::LinkedListEph::{LinkedListEphS, LinkedListEphTrait};
pub mod LinkedListEphChap18;
pub use crate::LinkedListEphChap18::LinkedListEphChap18Trait;
pub mod LinkedListEphChap19;
pub use crate::LinkedListEphChap19::LinkedListEphChap19Trait;

pub mod ArraySeqPer;
pub use crate::ArraySeqPer::{ArrayPerS, ArraySeqPerTrait};

pub mod ArraySeqPerChap18;
pub use crate::ArraySeqPerChap18::ArraySeqPerChap18Trait;

pub mod ArraySeqPerChap19;
pub use crate::ArraySeqPerChap19::ArraySeqPerChap19Trait;

pub mod FooArraySeqPer;

pub mod ArraySeqEph;
pub use crate::ArraySeqEph::{ArraySeqEphS, ArraySeqEphTrait};

pub mod ArraySeqEphChap18;
pub use crate::ArraySeqEphChap18::ArraySeqEphChap18Trait;
pub mod ArraySeqEphChap19;
pub use crate::ArraySeqEphChap19::ArraySeqEphChap19Trait;

pub mod AVLTreeSeqPer;
pub use crate::AVLTreeSeqPer::{AVLTreeSeqPerS, AVLTreeSeqPerTrait};
pub mod AVLTreeSeqPerChap18;
pub use crate::AVLTreeSeqPerChap18::AVLTreeSeqPerChap18Trait;
pub mod AVLTreeSeqPerChap19;
pub use crate::AVLTreeSeqPerChap19::AVLTreeSeqPerChap19Trait;

// Ephemeral AVLTree (base only; Chap18/19 not yet enabled)
pub mod AVLTreeSeqEph;
pub use crate::AVLTreeSeqEph::{AVLTreeSeqEphS, AVLTreeSeqEphTrait};
// Note: AVLTreeSeqEph Chap18/19 modules are not yet stable for export
