//! Library crate root for `apas-ai`.
//!
//! Abstract:
//! - Declares modules, re-exports core types and concrete sequence structs.
//! - Provides macros `mathseq!`, `arrayseq!`, and `listseq!` to build sequences concisely.

pub mod Types;
pub mod MathSeq;
pub mod ArraySeq;
pub mod ArraySeqChap18;
pub mod ArraySeqChap19;
// attic: DoublyLinkedListSeq*, SinglyLinkedListSeq* moved to src/attic
pub mod SinglyLinkedList;
pub mod SharedSinglyLinkedList;
pub mod SharedSinglyLinkedListChap18;
pub mod SharedSinglyLinkedListChap19;
pub mod AVLTreeSeq;
pub mod AVLTreeSeqChap18;
pub mod AVLTreeSeqChap19;


pub use crate::Types::{N, B, O};
pub use crate::MathSeq::MathS;
pub use crate::ArraySeq::ArrayS;
// attic re-exports removed
pub use crate::SinglyLinkedList::SinglyLinkedList as SLList;
pub use crate::SharedSinglyLinkedList::SharedSinglyLinkedList as SharedSLList;
pub use crate::SharedSinglyLinkedList::SharedSList;
pub use crate::SharedSinglyLinkedListChap18::SharedSListChap18;
pub use crate::SharedSinglyLinkedListChap19::SharedSListChap19;
//
// Provide stable re-export naming for tree-backed sequence
pub use crate::AVLTreeSeq::AVLTreeS; // canonical name

#[macro_export]
macro_rules! avltreeseq {
    () => { $crate::AVLTreeSeq::AVLTreeS::new() };
    ($x:expr; $n:expr) => {{
        let mut t = $crate::AVLTreeSeq::AVLTreeS::new();
        for _ in 0..$n { t.push_back($x); }
        t
    }};
    ($($x:expr),* $(,)?) => {{
        let mut t = $crate::AVLTreeSeq::AVLTreeS::new();
        $( { t.push_back($x); } )*
        t
    }};
}
#[macro_export]
macro_rules! mathseq {
    () => { $crate::MathSeq::MathS { data: Vec::new() } };
    ($x:expr; $n:expr) => { $crate::MathSeq::MathS { data: vec![$x; $n] } };
    ($($x:expr),* $(,)?) => { $crate::MathSeq::MathS { data: vec![$($x),*] } };
}

#[macro_export]
macro_rules! arrayseq {
    () => { $crate::ArraySeq::ArrayS::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => { $crate::ArraySeq::ArrayS::from_vec(vec![$x; $n]) };
    ($($x:expr),* $(,)?) => { $crate::ArraySeq::ArrayS::from_vec(vec![$($x),*]) };
}




