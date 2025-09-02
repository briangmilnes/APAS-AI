//! Library crate root for `apas-ai`.

pub mod Types;
pub mod MathSeq;
pub mod ArraySeq;
pub mod ArraySeqChap18;
pub mod ArraySeqChap19;


pub use crate::Types::{N, B, O};
pub use crate::MathSeq::MathS;
pub use crate::ArraySeq::ArrayS;
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




