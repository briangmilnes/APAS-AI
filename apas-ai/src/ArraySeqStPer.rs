//! StPer (immutable, structurally shared semantics) Array sequence variants.
//!
//! Abstract:
//! - Defines `ArrayStPerS<T>` backed by `Box<[T]>` with immutable APIs.
//! - Provides trait `ArraySeqStPer<T>` mirroring `ArraySeq` but all updates return new values.
//! - Uses only safe Rust. Builders may allocate via `Vec` and convert to boxed slices.

pub mod ArraySeqStPer {
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::slice::{Iter, IterMut};

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Box<[T]>` (persistent/immutable variant).
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
    pub struct ArrayStPerS<T: StT> {
        data: Box<[T]>,
    }

    /// Sequence trait for `ArrayStPerS<T>` with immutable operations.
    pub trait ArraySeqStPerTrait<T: StT + Clone> {
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

    impl<T: StT> ArrayStPerS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn subseq(&self, start: N, length: N) -> &[T] {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            &self.data[s..e]
        }

        /// Convenience: build from a Vec without extra copies when capacity==len.
        /// APAS: Work Θ(n) worst case, Span Θ(1)
        pub fn from_vec(v: Vec<T>) -> Self {
            ArrayStPerS {
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
            ArrayStPerS {
                data: Vec::new().into_boxed_slice(),
            }
        }
        pub fn singleton(item: T) -> Self {
            ArrayStPerS {
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
        pub fn set(&self, index: N, item: T) -> Result<Self, &'static str>
        where
            T: Clone,
        {
            if index >= self.data.len() {
                return Err("Index out of bounds");
            }
            let mut v: Vec<T> = self.data.to_vec();
            v[index] = item;
            Ok(Self::from_vec(v))
        }
    }

    impl<T: StT + Debug> Debug for ArrayStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: StT + Display> Display for ArrayStPerS<T> {
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

    impl<T: StT + Clone> ArraySeqStPerTrait<T> for ArrayStPerS<T> {
        fn new(length: N, init_value: T) -> Self {
            Self::from_vec(vec![init_value; length])
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
            if index >= self.data.len() {
                return Err("Index out of bounds");
            }
            let mut v: Vec<T> = self.data.to_vec();
            v[index] = item;
            Ok(Self::from_vec(v))
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
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return Self::empty();
            }
            let slice = &self.data[s..e];
            Self::from_vec(slice.to_vec())
        }
    }

    #[macro_export]
    macro_rules! ArraySeqStPer {
        () => { $crate::ArraySeqStPer::ArraySeqStPer::ArrayStPerS::empty() };
        ($x:expr; $n:expr) => { $crate::ArraySeqStPer::ArraySeqStPer::ArrayStPerS::new($n, $x) };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqStPer::ArraySeqStPer::ArrayStPerS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqStPer_macro_type_checks() {
        let _ = ArraySeqStPer![1];
        let _: crate::ArraySeqStPer::ArraySeqStPer::ArrayStPerS<i32> = ArraySeqStPer![];
    }
}
