//! Per (immutable, structurally shared semantics) Array sequence variants.
//!
//! Abstract:
//! - Defines `ArrayPerS<T>` backed by `Box<[T]>` with immutable APIs.
//! - Provides trait `ArraySeqPer<T>` mirroring `ArraySeq` but all updates return new values.
//! - Uses only safe Rust. Builders may allocate via `Vec` and convert to boxed slices.

pub mod ArraySeqPer {
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::slice::{Iter, IterMut};

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Box<[T]>` (persistent/immutable variant).
    #[derive(Clone)]
    pub struct ArrayPerS<T: MtT> { data: Box<[T]> }

    /// Sequence trait for `ArrayPerS<T>` with immutable operations.
    pub trait ArraySeqPerTrait<T: MtT> {
        /// APAS: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T) -> Self where T: Clone;
        /// APAS: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N) -> &T;
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// APAS: Work Θ(1) in ephemeral arrays; persistent update requires copy. Work Θ(|a|), Span Θ(1)
        /// gpt-5-hard: Work Θ(|a|), Span Θ(1)
        /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone, Self: Sized;
        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B;
        /// APAS: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone, Self: Sized;
    }

    impl<T: MtT> ArrayPerS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        pub fn subseq(&self, start: N, length: N) -> &[T] {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            &self.data[s..e]
        }

        /// Convenience: build from a Vec without extra copies when capacity==len.
        /// APAS: Work Θ(n) worst case, Span Θ(1)
        pub fn from_vec(v: Vec<T>) -> Self { ArrayPerS { data: v.into_boxed_slice() } }

        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }

        pub fn empty() -> Self { ArrayPerS { data: Vec::new().into_boxed_slice() } }
        pub fn singleton(item: T) -> Self { ArrayPerS { data: vec![item].into_boxed_slice() } }
        pub fn new(length: N, init_value: T) -> Self where T: Clone { Self::from_vec(vec![init_value; length]) }
        pub fn length(&self) -> N { self.data.len() }
        pub fn nth(&self, index: N) -> &T { &self.data[index] }
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {
            if index >= self.data.len() { return Err("Index out of bounds"); }
            let mut v: Vec<T> = self.data.to_vec();
            v[index] = item;
            Ok(Self::from_vec(v))
        }
    }

    impl<T: MtT + Eq> PartialEq for ArrayPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.length() != other.length() { return false; }
            for i in 0..self.length() { if self.nth(i) != other.nth(i) { return false; } }
            true
        }
    }
    impl<T: MtT + Eq> Eq for ArrayPerS<T> {}

    impl<T: MtT + Debug> Debug for ArrayPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: MtT + Display> Display for ArrayPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for (i, x) in self.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", x)?;
            }
            write!(f, "]")
        }
    }

    impl<T: MtT> ArraySeqPerTrait<T> for ArrayPerS<T> {
        fn new(length: N, init_value: T) -> Self where T: Clone {
            Self::from_vec(vec![init_value; length])
        }
        fn length(&self) -> N { self.data.len() }
        fn nth(&self, index: N) -> &T { &self.data[index] }
        fn empty() -> Self { Self::from_vec(Vec::new()) }
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> where T: Clone {
            if index >= self.data.len() { return Err("Index out of bounds"); }
            let mut v: Vec<T> = self.data.to_vec();
            v[index] = item;
            Ok(Self::from_vec(v))
        }
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }
        fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }
        fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }
        fn subseq_copy(&self, start: N, length: N) -> Self where T: Clone {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s { return Self::empty(); }
            let slice = &self.data[s..e];
            Self::from_vec(slice.to_vec())
        }

    }

    #[macro_export]
    macro_rules! ArraySeqPer {
        () => { $crate::ArraySeqPer::ArraySeqPer::ArrayPerS::empty() };
        ($x:expr; $n:expr) => { $crate::ArraySeqPer::ArraySeqPer::ArrayPerS::new($n, $x) };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqPer::ArraySeqPer::ArrayPerS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqPer_macro_type_checks() {
        let _ = ArraySeqPer![1];
        let _: crate::ArraySeqPer::ArraySeqPer::ArrayPerS<i32> = ArraySeqPer![];
    }
}
