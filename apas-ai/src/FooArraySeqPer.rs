//! Per (immutable, structurally shared semantics) Array sequence variants.
//!
//! Abstract:
//! - Defines `ArrayPerS<T>` backed by `Box<[T]>` with immutable APIs.
//! - Provides trait `ArraySeqPer<T>` mirroring `ArraySeq` but all updates return new values.
//! - Uses only safe Rust. Builders may allocate via `Vec` and convert to boxed slices.

pub use crate::Types::{N, B, O};

/// Fixed-length sequence backed by `Box<[T]>` (persistent/immutable variant).
#[derive(Clone)]
pub struct FooArrayPerS<T> { pub data: Box<[T]> }

/// Sequence trait for `FooArrayPerS<T>` with immutable operations.
pub trait FooArraySeqPerTrait<T> {
    fn new(length: N, init_value: T) -> FooArrayPerS<T> where T: Clone;
    fn new2(length: N, init_value: T) -> Self where T: Clone, Self: Sized;
    fn set(&self, index: N, item: T) -> Result<FooArrayPerS<T>, &'static str> where T: Clone;
    fn set2(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone, Self: Sized;
    fn singleton(item: T) -> Self where Self: Sized;
    fn empty() -> Self where Self: Sized;
}

impl<T> FooArrayPerS<T> {
    pub fn subseq(&self, start: N, length: N) -> &[T] {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        &self.data[s..e]
    }
    pub fn from_vec(v: Vec<T>) -> FooArrayPerS<T> { FooArrayPerS { data: v.into_boxed_slice() } }
    pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_value; length]) }
    pub fn new_inherent(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_value; length]) }
}


impl<T> FooArraySeqPerTrait<T> for FooArrayPerS<T> {
    fn new(length: N, init_value: T) -> FooArrayPerS<T> where T: Clone {
        FooArrayPerS::from_vec(vec![init_value; length])
    }
    fn new2(length: N, init_value: T) -> FooArrayPerS<T> where T: Clone {
        FooArrayPerS::from_vec(vec![init_value; length])
    }
    fn empty() -> Self { FooArrayPerS::from_vec(Vec::new()) }
    fn set(&self, index: N, item: T) -> Result<FooArrayPerS<T>, &'static str> where T: Clone {
        if index >= self.data.len() { return Err("Index out of bounds"); }
        let mut v: Vec<T> = self.data.to_vec();
        v[index] = item;
        Ok(FooArrayPerS::from_vec(v))
    }
    fn singleton(item: T) -> FooArrayPerS<T> { FooArrayPerS::from_vec(vec![item]) }

    fn set2(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {
        if index >= self.data.len() { return Err("Index out of bounds"); }
        let mut v: Vec<T> = self.data.to_vec();
        v[index] = item;
        Ok(FooArrayPerS::from_vec(v))
    }

}

/// Free function constructor to test inference without UFCS or type ascription
pub fn foo_new2<T: Clone>(length: N, init_value: T) -> FooArrayPerS<T> {
    FooArrayPerS::from_vec(vec![init_value; length])
}
