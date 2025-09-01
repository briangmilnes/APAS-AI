//! Library crate root for `apas-ai`.

pub mod S;
pub mod APAS18;
pub mod APAS19;

pub use crate::S::{N, B, O, Sequence};

pub mod Sequences {
    pub use crate::S::{S, Sequence, N, B, O};
}

#[macro_export]
macro_rules! seq {
    () => { $crate::S::S::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => { $crate::S::S::from_vec(vec![$x; $n]) };
    ($($x:expr),* $(,)?) => { $crate::S::S::from_vec(vec![$($x),*]) };
}


