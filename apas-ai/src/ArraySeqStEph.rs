//! StEphemeral (mutable) Array sequence variants. Mirrors `ArraySeq` under distinct names.

pub mod ArraySeqStEph {

    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::slice::Iter;

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Box<[T]>` (ephemeral variant).
    #[derive(Clone)]
    pub struct ArraySeqStEphS<T: StT> {
        pub data: Box<[T]>,
    }

    /// Sequence trait for `ArraySeqStEphS<T>`.
    pub trait ArraySeqStEphTrait<T: StT> {
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
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
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
        fn subseq_copy(&self, start: N, length: N) -> Self;
    }

    impl<T: StT> ArraySeqStEphS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn subseq(&self, start: N, length: N) -> &[T] {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            &self.data[s..e]
        }
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty();
            }
            let len = e - s;
            let first = self.nth(s).clone();
            let mut out = <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::new(len, first.clone());
            let _ = out.set(0, first);
            for i in 1..len {
                let _ = out.set(i, self.nth(s + i).clone());
            }
            out
        }
        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        pub fn from_vec(v: Vec<T>) -> Self {
            ArraySeqStEphS {
                data: v.into_boxed_slice(),
            }
        }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {
            if index < self.data.len() {
                self.data[index] = item;
            }
            self
        }

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }
    }

    impl<T: StT> PartialEq for ArraySeqStEphS<T> {
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

    impl<T: StT> Eq for ArraySeqStEphS<T> {}

    impl<T: StT> Debug for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: StT> Display for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "[")?;
            for i in 0..self.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.nth(i))?;
            }
            write!(f, "]")
        }
    }

    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T) -> Self { ArraySeqStEphS::from_vec(vec![init_value; length]) }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N { self.data.len() }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N) -> &T { &self.data[index] }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Self { ArraySeqStEphS::from_vec(Vec::new()) }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            if index < self.data.len() {
                self.data[index] = item;
                Ok(self)
            } else {
                Err("Index out of bounds")
            }
        }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self { ArraySeqStEphS::from_vec(vec![item]) }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B {
            if self.data.len() == 0 {
                B::True
            } else {
                B::False
            }
        }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B {
            if self.data.len() == 1 {
                B::True
            } else {
                B::False
            }
        }
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> Self { self.subseq_copy(start, length) }
    }

    #[macro_export]
    macro_rules! ArraySeqStEph {
        () => { $crate::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(vec![$($x),*]) };
    }
}
