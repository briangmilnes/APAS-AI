//! MtPer (immutable, structurally shared semantics) Array sequence variants with multithreading types and calls.
//!
//! Abstract:
//! - Defines `ArrayMtPerS<T>` backed by `Box<[T]>` with immutable APIs.
//! - Provides trait `ArraySeqMtPer<T>` mirroring `ArraySeq` but all updates return new values.
//! - Uses only safe Rust. Builders may allocate via `Vec` and convert to boxed slices.

pub mod ArraySeqMtPer {
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::slice::{Iter, IterMut};

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Box<[T]>` (persistent/immutable variant).
    pub struct ArrayMtPerS<T> {
        data: Box<[T]>,
    }

    /// Sequence trait for `ArrayMtPerS<T>` with immutable operations.
    pub trait ArraySeqMtPerTrait<T: MtT> {
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N) -> &T;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// APAS: Work Θ(1) in ephemeral arrays; persistent update requires copy. Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>
        where
            Self: Sized;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B;
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> Self
        where
            Self: Sized;
    }

    impl<T> ArrayMtPerS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        pub fn subseq(&self, start: N, length: N) -> &[T] {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            &self.data[s..e]
        }

        /// Convenience: build from a Vec without extra copies when capacity==len.
        /// APAS: Work Θ(n) worst case, Span Θ(1)
        pub fn from_vec(v: Vec<T>) -> Self {
            ArrayMtPerS {
                data: v.into_boxed_slice(),
            }
        }

        pub fn iter(&self) -> Iter<'_, T> {
            self.data.iter()
        }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            self.data.iter_mut()
        }

        pub fn empty() -> Self {
            ArrayMtPerS {
                data: Vec::new().into_boxed_slice(),
            }
        }
        pub fn singleton(item: T) -> Self {
            ArrayMtPerS {
                data: vec![item].into_boxed_slice(),
            }
        }
        pub fn new(length: N, init_value: T) -> Self
        where
            T: Clone,
        {
            Self::from_vec(vec![init_value; length])
        }
        pub fn length(&self) -> N {
            self.data.len()
        }
        pub fn nth(&self, index: N) -> &T {
            &self.data[index]
        }
    }

    impl<T: MtT> ArrayMtPerS<T> {
        pub fn new_mt(length: N, init_value: T) -> Self {
            let mut v = Vec::with_capacity(length);
            for _ in 0..length {
                v.push(init_value.clone_mt());
            }
            Self::from_vec(v)
        }

        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str> {
            if index >= self.data.len() {
                return Err("Index out of bounds");
            }
            let mut v: Vec<T> = self.data.iter().map(|x| x.clone_mt()).collect();
            v[index] = item;
            Ok(Self::from_vec(v))
        }

        pub fn subseq_copy_mt(&self, start: N, length: N) -> Self {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            let v: Vec<T> = self.data[s..e].iter().map(|x| x.clone_mt()).collect();
            Self::from_vec(v)
        }
    }

    impl<T: MtT> Clone for ArrayMtPerS<T> {
        fn clone(&self) -> Self {
            let cloned_data: Vec<T> = self.data.iter().map(|item| item.clone_mt()).collect();
            ArrayMtPerS {
                data: cloned_data.into_boxed_slice(),
            }
        }
    }

    impl<T: MtT + StT + Send + Sync> MtT for ArrayMtPerS<T> {
        type Inner = ArrayMtPerS<T>;
        fn clone_mt(&self) -> Self {
            self.clone()
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl<T: Eq> PartialEq for ArrayMtPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.length() != other.length() {
                return false;
            }
            for i in 0..self.length() {
                if self.nth(i) != other.nth(i) {
                    return false;
                }
            }
            true
        }
    }
    impl<T: Eq> Eq for ArrayMtPerS<T> {}

    impl<T: Debug> Debug for ArrayMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: Display> Display for ArrayMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for (i, x) in self.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", x)?;
            }
            write!(f, "]")
        }
    }

    impl<T: MtT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {
        fn new(length: N, init_value: T) -> Self {
            ArrayMtPerS::new_mt(length, init_value)
        }
        fn length(&self) -> N {
            self.data.len()
        }
        fn nth(&self, index: N) -> &T {
            &self.data[index]
        }
        fn empty() -> Self {
            Self::from_vec(Vec::new())
        }
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {
            self.set(index, item)
        }
        fn singleton(item: T) -> Self {
            Self::from_vec(vec![item])
        }
        fn isEmpty(&self) -> B {
            if self.data.len() == 0 { B::True } else { B::False }
        }
        fn isSingleton(&self) -> B {
            if self.data.len() == 1 { B::True } else { B::False }
        }
        fn subseq_copy(&self, start: N, length: N) -> Self {
            self.subseq_copy_mt(start, length)
        }
    }

    #[macro_export]
    macro_rules! ArraySeqMtPer {
        () => { $crate::ArraySeqMtPer::ArraySeqMtPer::ArrayMtPerS::empty() };
        ($x:expr; $n:expr) => { $crate::ArraySeqMtPer::ArraySeqMtPer::ArrayMtPerS::new($n, $x) };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqMtPer::ArraySeqMtPer::ArrayMtPerS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqMtPer_macro_type_checks() {
        let _ = ArraySeqMtPer![1];
        let _: crate::ArraySeqMtPer::ArraySeqMtPer::ArrayMtPerS<i32> = ArraySeqMtPer![];
    }
}
