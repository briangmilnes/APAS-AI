//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Mathematical sequence backed by a growable vector. Dense domain 0..len-1.
//!
//! Abstract: Definition 17.1 (Sequence) — runtime-sized, dense-domain sequence (0..n-1),
//! using rust vector which is dense.

pub mod MathSeq {

    use std::collections::hash_map::Entry;
    use std::collections::{HashMap, HashSet};
    use std::fmt::{Debug, Display, Formatter};
    use std::hash::Hash;
    use std::slice::Iter;
    use std::slice::IterMut;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct MathSeqS<T: StT> {
        data: Vec<T>,
    }

    /// Core API for `MathSeqS<T>`.
    pub trait MathSeqTrait<T: StT + Hash> {
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T)           -> Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn set(&mut self, index: N, value: T)      -> Result<&mut Self, &'static str>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self)                           -> N;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N)                    -> &T;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                 -> Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T)                      -> Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn subseq(&self, start: N, length: N)      -> &[T];

        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> Self;

        /// APAS: Work amortized Θ(1), worst case Θ(n), Span amortized Θ(1), worst case Θ(n)
        /// claude-4-sonet: Work amortized Θ(1), worst case Θ(n), Span amortized Θ(1), worst case Θ(n)
        fn add_last(&mut self, value: T)           -> &mut Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn delete_last(&mut self)                  -> Option<T>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self)                          -> B;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self)                      -> B;

        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn domain(&self)                           -> Vec<N>;

        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn range(&self)                            -> Vec<T>;

        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn multiset_range(&self)                   -> Vec<(N, T)>;

        // Iterator and construction methods
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self) -> Iter<'_, T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter_mut(&mut self) -> IterMut<'_, T>;
        /// APAS: Work Θ(|data|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|data|), Span Θ(1)
        fn from_vec(data: Vec<T>) -> Self;
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn with_len(length: N, init_value: T) -> Self;
    }





    impl<T: StT + Hash> MathSeqTrait<T> for MathSeqS<T> {
        fn new(length: N, init_value: T) -> Self {
            MathSeqS {
                data: vec![init_value; length],
            }
        }

        fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {
            if index < self.data.len() {
                self.data[index] = value;
                Ok(self)
            } else {
                Err("Index out of bounds")
            }
        }

        fn length(&self) -> N { self.data.len() }

        fn nth(&self, index: N) -> &T { &self.data[index] }

        fn empty() -> Self { MathSeqS { data: Vec::new() } }

        fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }

        fn subseq(&self, start: N, length: N) -> &[T] {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            &self.data[s..e]
        }

        fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return MathSeqS { data: Vec::new() };
            }
            MathSeqS {
                data: self.data[s..e].to_vec(),
            }
        }

        fn add_last(&mut self, value: T) -> &mut Self {
            self.data.push(value);
            self
        }

        fn delete_last(&mut self) -> Option<T> { self.data.pop() }

        fn isEmpty(&self) -> B { self.data.is_empty() }

        fn isSingleton(&self) -> B { self.data.len() == 1 }

        fn domain(&self) -> Vec<N> { (0..self.data.len()).collect() }

        fn range(&self) -> Vec<T> {
            let mut seen: HashSet<T> = HashSet::with_capacity(self.data.len());
            let mut out: Vec<T> = Vec::with_capacity(self.data.len());
            for x in self.data.iter() {
                if seen.insert(x.clone()) {
                    out.push(x.clone());
                }
            }
            out
        }

        fn multiset_range(&self) -> Vec<(N, T)> {
            let mut counts: HashMap<T, N> = HashMap::with_capacity(self.data.len());
            let mut order: Vec<T> = Vec::new();
            for x in self.data.iter() {
                match counts.entry(x.clone()) {
                    | Entry::Vacant(e) => {
                        e.insert(1);
                        order.push(x.clone());
                    }
                    | Entry::Occupied(mut e) => {
                        *e.get_mut() += 1;
                    }
                }
            }
            order.into_iter().map(|x| (*counts.get(&x).unwrap(), x)).collect()
        }

        fn iter(&self) -> Iter<'_, T> { self.data.iter() }

        fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }

        fn from_vec(data: Vec<T>) -> Self { Self { data } }

        fn with_len(length: N, init_value: T) -> Self {
            Self {
                data: vec![init_value; length],
            }
        }
    }

    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }
    }

    impl<T: StT> IntoIterator for MathSeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
    }

    impl<T: StT> PartialEq for MathSeqS<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }
    impl<T: StT> Eq for MathSeqS<T> {}
    impl<T: StT> Debug for MathSeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { f.debug_list().entries(self.data.iter()).finish() }
    }
    impl<T: StT> Display for MathSeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            for x in &self.data {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{x}")?;
            }
            write!(f, "]")
        }
    }

    #[macro_export]
    macro_rules! MathSeqSLit {
        () => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::empty()
        };
        ($x:expr; $n:expr) => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::with_len($n, $x)
        };
        ($($x:expr),* $(,)?) => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::from_vec(vec![$($x),*])
        };
    }
}
