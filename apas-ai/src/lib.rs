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

pub mod ArraySeqPer;
pub use crate::ArraySeqPer::{ArrayPerS, ArraySeqPerTrait};

pub mod ArraySeqPerChap18;
pub use crate::ArraySeqPerChap18::ArraySeqPerChap18Trait;

pub mod ArraySeqPerChap19;
pub use crate::ArraySeqPerChap19::ArraySeqPerChap19Trait;

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

/*  These are messed up in ways I need to understand.

pub mod AVLTreeSeqEph;
pub use crate::AVLTreeSeqEph::{AVLTreeSeqEphS, AVLTreeSeqEphTrait};
pub mod AVLTreeSeqEphChap18;
pub use crate::AVLTreeSeqEphChap18::AVLTreeSeqEphChap18Trait;
pub mod AVLTreeSeqEphChap19; 
pub use crate::AVLTreeSeqEphChap19::AVLTreeSeqEphrChap19Trait;
*/

#[macro_export]
macro_rules! MathSeq {
    () => { $crate::MathSeq::MathS { data: Vec::new() } };
    ($x:expr; $n:expr) => { $crate::MathSeq::MathS { data: vec![$x; $n] } };
    ($($x:expr),* $(,)?) => { $crate::MathSeq::MathS { data: vec![$($x),*] } };
}

#[macro_export]
macro_rules! LinkedListEph {
    ($x:expr; $n:expr) => {{
        < $crate::LinkedListEph::LinkedListEph<_> as $crate::LinkedListEph::ListEph<_> >::new($n, $x)
    }};
    ($($x:expr),* $(,)?) => {{
        let __vals = vec![$($x),*];
        let __len = __vals.len();
        let mut __l = < $crate::LinkedListEph::LinkedListEph<_> as $crate::LinkedListEph::LinkedListEph<_> >::new(__len, __vals[0].clone());
        let mut __i: $crate::Types::N = 0;
        for __v in __vals { let _ = < $crate::LinkedListEph::LinkedListEph<_> as $crate::LinkedListEph::LinkedListEph<_> >::set(&mut __l, __i, __v); __i += 1; }
        __l
    }};
}

#[macro_export]
macro_rules! LinkedListPer {
    () => { $crate::LinkedListPer::LinkedListPer::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => {{
        < $crate::LinkedListPer::LinkedListPer<_> as $crate::LinkedListPer::ListPer<_> >::new($n, $x)
    }};
    ($($x:expr),* $(,)?) => { $crate::LinkedListPer::LinkedListPer::from_vec(vec![$($x),*]) };
}

#[macro_export]
macro_rules! ArraySeqPer {
    () => { $crate::ArraySeqPer::ArraySPer::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => { $crate::ArraySeqPer::ArraySPer::from_vec(vec![$x; $n]) };
    ($($x:expr),* $(,)?) => { $crate::ArraySeqPer::ArraySPer::from_vec(vec![$($x),*]) };
}

#[macro_export]
macro_rules! ArraySeqEph {
    () => { $crate::ArraySeqEph::ArraySEph::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => { $crate::ArraySeqEph::ArraySEph::from_vec(vec![$x; $n]) };
    ($($x:expr),* $(,)?) => { $crate::ArraySeqEph::ArraySEph::from_vec(vec![$($x),*]) };
}

#[macro_export]
macro_rules! AVLTreeSeqPer {
    () => { $crate::AVLTreeSeqPer::AVLTreeSPer::new() };
    ($x:expr; $n:expr) => {{
        let mut t = $crate::AVLTreeSeqPer::AVLTreeSPer::new();
        for _ in 0..$n { t.push_back($x); }
        t
    }};
    ($($x:expr),* $(,)?) => {{
        let mut t = $crate::AVLTreeSeqPer::AVLTreeSPer::new();
        $( { t.push_back($x); } )*
        t
    }};
}

/* Requires the right files working from above.
#[macro_export]
macro_rules! AVLTreeSeqEph {
    () => { $crate::AVLTreeSeqEph::AVLTreeSEph::new() };
    ($x:expr; $n:expr) => {{
        let mut t = $crate::AVLTreeSeqEph::AVLTreeSEph::new();
        for _ in 0..$n { t.push_back($x); }
        t
    }};
    ($($x:expr),* $(,)?) => {{
        let mut t = $crate::AVLTreeSeqEph::AVLTreeSEph::new();
        $( { t.push_back($x); } )*
        t
    }};
}
*/

