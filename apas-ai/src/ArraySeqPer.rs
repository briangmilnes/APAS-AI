//! Per (immutable, structurally shared semantics) Array sequence variants.
//!
//! Abstract:
//! - Defines `ArrayPerS<T>` backed by `Box<[T]>` with immutable APIs.
//! - Provides trait `ArraySeqPer<T>` mirroring `ArraySeq` but all updates return new values.
//! - Uses only safe Rust. Builders may allocate via `Vec` and convert to boxed slices.

pub use crate::Types::{N, B, O};

/// Fixed-length sequence backed by `Box<[T]>` (persistent/immutable variant).
#[derive(Clone)]
pub struct ArrayPerS<T> { pub data: Box<[T]> }

/// Sequence trait for `ArrayPerS<T>` with immutable operations.
pub trait ArraySeqPerTrait<T> {
    /// APAS: Work Θ(length), Span Θ(1)
    fn new(length: N, init_value: T) -> ArrayPerS<T> where T: Clone;
    /// APAS: Work Θ(1), Span Θ(1)
    fn length(&self) -> N;
    /// APAS: Work Θ(1), Span Θ(1)
    fn nth(&self, index: N) -> &T;
    /// APAS: Work Θ(1), Span Θ(1)
    fn empty() -> ArrayPerS<T>;
    /// APAS: Work Θ(1) in ephemeral arrays; persistent update requires copy. Work Θ(|a|), Span Θ(1)
    /// gpt-5-hard: Work Θ(|a|), Span Θ(1)
    /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
    fn set(&self, index: N, item: T) -> Result<ArrayPerS<T>, &'static str> where T: Clone;
    /// APAS: Work Θ(1), Span Θ(1)
    fn singleton(item: T) -> ArrayPerS<T>;
    /// APAS: Work Θ(1), Span Θ(1)
    fn isEmpty(&self) -> B;
    /// APAS: Work Θ(1), Span Θ(1)
    fn isSingleton(&self) -> B;
    /// APAS: Work Θ(length), Span Θ(1)
    fn subseq_copy(&self, start: N, length: N) -> ArrayPerS<T> where T: Clone + Eq;
}

impl<T> ArrayPerS<T> {
    /// APAS: Work Θ(1), Span Θ(1)
    pub fn subseq(&self, start: N, length: N) -> &[T] {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        &self.data[s..e]
    }

    /// Convenience: build from a Vec without extra copies when capacity==len.
    /// APAS: Work Θ(n) worst case, Span Θ(1)
    pub fn from_vec(v: Vec<T>) -> ArrayPerS<T> { ArrayPerS { data: v.into_boxed_slice() } }

    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }
}

impl<T: Eq> PartialEq for ArrayPerS<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length() != other.length() { return false; }
        for i in 0..self.length() { if self.nth(i) != other.nth(i) { return false; } }
        true
    }
}
impl<T: Eq> Eq for ArrayPerS<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for ArrayPerS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elts = (0..self.length()).map(|i| self.nth(i));
        f.debug_list().entries(elts).finish()
    }
}

impl<T> ArraySeqPerTrait<T> for ArrayPerS<T> {
    fn new(length: N, init_value: T) -> ArrayPerS<T> where T: Clone {
        ArrayPerS::from_vec(vec![init_value; length])
    }
    fn length(&self) -> N { self.data.len() }
    fn nth(&self, index: N) -> &T { &self.data[index] }
    fn empty() -> ArrayPerS<T> { ArrayPerS::from_vec(Vec::new()) }
    fn set(&self, index: N, item: T) -> Result<ArrayPerS<T>, &'static str> where T: Clone {
        if index >= self.data.len() { return Err("Index out of bounds"); }
        let mut v: Vec<T> = self.data.to_vec();
        v[index] = item;
        Ok(ArrayPerS::from_vec(v))
    }
    fn singleton(item: T) -> ArrayPerS<T> { ArrayPerS::from_vec(vec![item]) }
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }
    fn subseq_copy(&self, start: N, length: N) -> ArrayPerS<T> where T: Clone + Eq {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s { return <ArrayPerS<T> as ArraySeqPerTrait<T>>::empty(); }
        let slice = &self.data[s..e];
        ArrayPerS::from_vec(slice.to_vec())
    }

}

#[macro_export]
macro_rules! ArraySeqPer {
    () => { $crate::ArraySeqPer::ArrayPerS::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => { $crate::ArraySeqPer::ArrayPerS::from_vec(vec![$x; $n]) };
    ($($x:expr),* $(,)?) => { $crate::ArraySeqPer::ArrayPerS::from_vec(vec![$($x),*]) };
}
