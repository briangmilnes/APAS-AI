//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 persistent sequence implementation for array-backed sequences.

pub mod ArraySeqStPer {
    use std::collections::HashSet;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

    use crate::Types::Types::*;

    /// Persistent array-backed sequence (`Per` variant) that never mutates in place.
    #[derive(Clone)]
    pub struct ArraySeqStPerS<T: StT> {
        data: Box<[T]>,
    }

    pub type ArrayStPer<T> = ArraySeqStPerS<T>;

    impl<T: StT> ArraySeqStPerS<T> {
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }
        pub fn length(&self) -> N { self.data.len() }
        pub fn nth(&self, index: N) -> &T { &self.data[index] }
        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let total = self.data.len();
            let begin = start.min(total);
            let end = start.saturating_add(length).min(total);
            let slice: Vec<T> = self.data[begin..end].iter().cloned().collect();
            Self::from_vec(slice)
        }
    }

    impl<T: StT> PartialEq for ArraySeqStPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }
    }

    impl<T: StT> Eq for ArraySeqStPerS<T> {}

    impl<T: StT> Debug for ArraySeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.iter()).finish() }
    }

    impl<T: StT> Display for ArraySeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    /// Persistent ArraySeq API matching the APAS ordering (no `set` or `update`).
    pub trait ArraySeqStPerTrait<T: StT> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn empty() -> ArraySeqStPerS<T>;
        fn singleton(item: T) -> ArraySeqStPerS<T>;
        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStPerS<T>;
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U>;
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T>;
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T>;
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;
        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStPerS<Pair<K, V>>,
            cmp: impl Fn(&K, &K) -> O,
        ) -> ArraySeqStPerS<Pair<K, ArraySeqStPerS<V>>>;
        fn iterate<A>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A;
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, T);
    }

    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::new(length, init_value) }
        fn length(&self) -> N { ArraySeqStPerS::length(self) }
        fn nth(&self, index: N) -> &T { ArraySeqStPerS::nth(self, index) }
        fn empty() -> ArraySeqStPerS<T> { ArraySeqStPerS::empty() }
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::singleton(item) }

        fn tabulate(f: impl Fn(N) -> T, length: N) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(length);
            for i in 0..length {
                values.push(f(i));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U> {
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T> {
            a.subseq_copy(start, length)
        }

        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let total = a.length() + b.length();
            let mut values: Vec<T> = Vec::with_capacity(total);
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T> {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) == B::True {
                    kept.push(value.clone());
                }
            }
            ArraySeqStPerS::from_vec(kept)
        }

        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let inner = a.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            let mut seen: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i).clone();
                if index < values.len() && seen.insert(index) {
                    values[index] = value;
                }
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }

        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }

        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStPerS<Pair<K, V>>,
            cmp: impl Fn(&K, &K) -> O,
        ) -> ArraySeqStPerS<Pair<K, ArraySeqStPerS<V>>> {
            let mut groups: Vec<Pair<K, Vec<V>>> = Vec::new();
            'outer: for i in 0..pairs.length() {
                let Pair(key, value) = pairs.nth(i).clone();
                for group in groups.iter_mut() {
                    if cmp(&group.0, &key) == O::Equal {
                        group.1.push(value.clone());
                        continue 'outer;
                    }
                }
                groups.push(Pair(key, vec![value]));
            }
            let collected: Vec<Pair<K, ArraySeqStPerS<V>>> = groups
                .into_iter()
                .map(|Pair(key, bucket)| Pair(key, ArraySeqStPerS::from_vec(bucket)))
                .collect();
            ArraySeqStPerS::from_vec(collected)
        }

        fn iterate<A>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, seed: A) -> A {
            let mut acc = seed;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, T) {
            let mut prefixes: Vec<T> = Vec::with_capacity(a.length());
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                prefixes.push(acc.clone());
            }
            (ArraySeqStPerS::from_vec(prefixes), acc)
        }
    }

    #[macro_export]
    macro_rules! ArraySeqStPerS {
        () => {
            $crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(Vec::new())
        };
        ($elem:expr; $len:expr) => {
            $crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$elem; $len])
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$($elem),+])
        };
    }

    #[allow(dead_code)]
    fn _arrayseqstpers_macro_type_checks() {
        let _: ArraySeqStPerS<i32> = ArraySeqStPerS![];
        let _: ArraySeqStPerS<&str> = ArraySeqStPerS!["alpha", "beta"];
        let _: ArraySeqStPerS<i64> = ArraySeqStPerS![1; 2];
    }
}
