//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! The simplest possible version, ignoring parallelism.

pub mod ArraySeq {

    use std::collections::HashSet;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
    use std::slice::Iter;
    use std::slice::IterMut;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct ArraySeqS<T> {
        data: Box<[T]>,
    }

    /// Data Type 18.1: Generic sequence trait for array-backed sequences.
    pub trait ArraySeq<T> {
        /// Create a new sequence of length `length` with each element initialized to `init_value`. <br/>
        /// claude-4-sonet: Work Θ(length), Span Θ(1), Parallelism Θ(1).
        fn new(length: N, init_value: T)                             -> Self
        where
            T: Clone;

        /// Set the element at `index` to `item` in place. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn set(&mut self, index: N, item: T)                         -> Result<&mut ArraySeqS<T>, &'static str>;

        /// Definition 18.1 (length). Return the number of elements. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn length(&self)                                             -> N;

        /// Algorithm 19.11 (Function nth). Return a reference to the element at `index`. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn nth(&self, index: N)                                      -> &T;

        /// Definition 18.1 (empty). Construct the empty sequence. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn empty()                                                   -> ArraySeqS<T>;

        /// Definition 18.1 (singleton). Construct a singleton sequence containing `item`. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn singleton(item: T)                                        -> ArraySeqS<T>;

        /// Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index. <br/>
        /// Work: Θ(length), Span: Θ(1).
        fn tabulate<F: Fn(N)                                         -> T>(f: &F, length: N) -> ArraySeqS<T>;

        /// Algorithm 18.4 (map). Transform each element via `f`. <br/>
        /// Work: Θ(|a|), Span: Θ(1).
        fn map<U: Clone, F: Fn(&T)                                   -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U>;

        /// Definition 18.12 (subseq). Extract a contiguous subsequence, truncating out-of-bounds ranges. <br/>
        /// Work: Θ(length), Span: Θ(1).
        fn subseq(a: &ArraySeqS<T>, start: N, length: N)             -> ArraySeqS<T>
        where
            T: Clone;

        /// Definition 18.13 (append). Concatenate two sequences. <br/>
        /// Work: Θ(|a| + |b|), Span: Θ(1).
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>)                -> ArraySeqS<T>;

        /// Definition 18.14 (filter). Keep elements satisfying `pred`. <br/>
        /// Work: Θ(|a|), Span: Θ(1).
        fn filter<F: PredSt<T>>(a: &ArraySeqS<T>, pred: &F)          -> ArraySeqS<T>;

        /// Definition 18.15 (flatten). Concatenate a sequence of sequences. <br/>
        /// Work: Θ(total length), Span: Θ(1).
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>)                      -> ArraySeqS<T>;

        /// Definition 18.16 (update). Return a copy with the index replaced by the new value. <br/>
        /// Work: Θ(|a|), Span: Θ(1).
        fn update(a: &ArraySeqS<T>, update: Pair<N, T>)              -> ArraySeqS<T>;

        /// Definition 18.17 (inject). Apply updates, keeping the first update per index. <br/>
        /// Work: Θ(|a| + |updates|), Span: Θ(1).
        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T>;

        /// Definition 18.5 (isEmpty). true iff the sequence has length zero. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn isEmpty(&self)                                            -> B;

        /// Definition 18.5 (isSingleton). true iff the sequence has length one. <br/>
        /// Work: Θ(1), Span: Θ(1).
        fn isSingleton(&self)                                        -> B;

        /// Algorithm 18.21 (collect). Group values with equal keys under `cmp`. <br/>
        /// Work: Θ(|pairs|²) worst case due to linear search, Span: Θ(1).
        fn collect<K: Clone + Eq, V: Clone>(
            pairs: &ArraySeqS<Pair<K, V>>,
            cmp: impl Fn(&K, &K) -> O,
        ) -> ArraySeqS<Pair<K, ArraySeqS<V>>>;

        /// Definition 18.7 (iterate). Fold with accumulator `seed`.
        fn iterate<A, F: Fn(&A, &T)                                  -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A;

        /// Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`. <br/>
        /// Work: Θ(|a|), Span: Θ(1).
        fn reduce<F: Fn(&T, &T)                                      -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> T
        where
            T: Clone;

        /// Definition 18.19 (scan). Prefix-reduce returning partial sums and total. <br/>
        /// Work: Θ(|a|), Span: Θ(1).
        fn scan<F: Fn(&T, &T)                                        -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T)
        where
            T: Clone;
    }

    impl<T: Clone> ArraySeqS<T> {
        fn new(length: N, init_value: T) -> ArraySeqS<T> {
            let mut data = Vec::with_capacity(length);
            data.resize(length, init_value);
            ArraySeqS::from_vec(data)
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {
            if index < self.data.len() {
                self.data[index] = item;
                Ok(self)
            } else {
                Err("Index out of bounds")
            }
        }

        fn length(&self) -> N { self.data.len() }

        fn nth(&self, index: N) -> &T { &self.data[index] }

        fn empty() -> ArraySeqS<T> { ArraySeqS::from_vec(Vec::new()) }

        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::from_vec(vec![item]) }

        fn isEmpty(&self) -> B { self.data.is_empty() }

        fn isSingleton(&self) -> B { self.data.len() == 1 }

        /// Definition 18.2 (subseq view). Return a slice for the subsequence starting at `start`
        /// and of length `length` without copying or allocation (zero‑copy view). <br/>
        /// Work: Θ(1), Span: Θ(1).
        pub fn subseq(&self, start: N, length: N) -> &[T] {
            let sequence_length = self.data.len();
            let start_index = start.min(sequence_length);
            let end_exclusive = start.saturating_add(length).min(sequence_length);
            &self.data[start_index..end_exclusive]
        }
        /// Definition 18.12 (subseq). Extract a contiguous subsequence starting at `start` with length `length`. <br/>
        /// If out of bounds, returns only the in-bounds part. <br/>
        /// Work: Θ(1) to compute bounds; allocation and cloning Θ(length) in this owning representation.
        pub fn subseq_copy(&self, start: N, length: N) -> ArraySeqS<T> {
            let sequence_length = self.data.len();
            let start_index = start.min(sequence_length);
            let end_exclusive = start.saturating_add(length).min(sequence_length);
            if end_exclusive <= start_index {
                return ArraySeqS::from_vec(Vec::new());
            }
            let segment: Vec<T> = self.data[start_index..end_exclusive].to_vec();
            ArraySeqS::from_vec(segment)
        }

        /// Update `self[index]` to `item` in place if in bounds, and return `self` for chaining. <br/>
        /// Work: Θ(1), Span: Θ(1).
        pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqS<T> {
            if index < self.data.len() {
                self.data[index] = item;
            }
            self
        }

        /// Create sequence from a Vec (used by `arrayseq!` and tests). <br/>
        /// Work: Θ(n) worst case (shrink-to-fit moves), Θ(1) best case (rebrand); Span: Θ(1). <br/>
        /// Reason: `Vec<T>` owns a heap buffer; `into_boxed_slice()` reuses it when
        /// capacity==len, else shrinks and moves elements.
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> {
            ArraySeqS {
                data: elts.into_boxed_slice(),
            }
        }

        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }
    }

    impl<T: Clone> ArraySeq<T> for ArraySeqS<T> {
        fn new(length: N, init_value: T) -> ArraySeqS<T> { ArraySeqS::new(length, init_value) }

        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqS<T>, &'static str> {
            ArraySeqS::set(self, index, item)
        }

        fn length(&self) -> N { ArraySeqS::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeqS::nth(self, index) }

        fn empty() -> ArraySeqS<T> { ArraySeqS::empty() }

        fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS::singleton(item) }

        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqS<T> {
            let mut values: Vec<T> = Vec::with_capacity(length);
            for i in 0..length {
                values.push(f(i));
            }
            ArraySeqS::from_vec(values)
        }

        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U> {
            let len = a.length();
            let mut values: Vec<U> = Vec::with_capacity(len);
            for i in 0..len {
                values.push(f(a.nth(i)));
            }
            ArraySeqS::from_vec(values)
        }

        fn subseq(a: &ArraySeqS<T>, start: N, length: N) -> ArraySeqS<T> { a.subseq_copy(start, length) }

        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> ArraySeqS<T> {
            let total = a.length() + b.length();
            if total == 0 {
                return ArraySeqS::from_vec(Vec::new());
            }
            let mut values: Vec<T> = Vec::with_capacity(total);
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqS::from_vec(values)
        }

        fn filter<F: PredSt<T>>(a: &ArraySeqS<T>, pred: &F) -> ArraySeqS<T> {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) {
                    kept.push(value.clone());
                }
            }
            ArraySeqS::from_vec(kept)
        }

        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> ArraySeqS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let inner = a.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            ArraySeqS::from_vec(values)
        }

        fn update(a: &ArraySeqS<T>, Pair(index, item): Pair<N, T>) -> ArraySeqS<T> {
            let mut values: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            if index < values.len() {
                values[index] = item;
            }
            ArraySeqS::from_vec(values)
        }

        fn inject(a: &ArraySeqS<T>, updates: &ArraySeqS<Pair<N, T>>) -> ArraySeqS<T> {
            let mut values: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            let mut seen: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                if idx < values.len() && seen.insert(idx) {
                    values[idx] = val;
                }
            }
            ArraySeqS::from_vec(values)
        }

        fn isEmpty(&self) -> B { ArraySeqS::isEmpty(self) }

        fn isSingleton(&self) -> B { ArraySeqS::isSingleton(self) }

        fn collect<K: Clone + Eq, V: Clone>(
            pairs: &ArraySeqS<Pair<K, V>>,
            cmp: impl Fn(&K, &K) -> O,
        ) -> ArraySeqS<Pair<K, ArraySeqS<V>>> {
            let mut groups: Vec<Pair<K, Vec<V>>> = Vec::new();
            for i in 0..pairs.length() {
                let Pair(key, value) = pairs.nth(i).clone();
                if let Some(group) = groups.iter_mut().find(|existing| cmp(&existing.0, &key) == O::Equal) {
                    group.1.push(value);
                } else {
                    groups.push(Pair(key, vec![value]));
                }
            }
            let grouped: Vec<Pair<K, ArraySeqS<V>>> = groups
                .into_iter()
                .map(|Pair(key, bucket)| Pair(key, ArraySeqS::from_vec(bucket)))
                .collect();
            ArraySeqS::from_vec(grouped)
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A {
            let mut acc = seed;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> T {
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T) {
            let len = a.length();
            let mut prefixes: Vec<T> = Vec::with_capacity(len);
            let mut acc = id;
            for i in 0..len {
                acc = f(&acc, a.nth(i));
                prefixes.push(acc.clone());
            }
            (ArraySeqS::from_vec(prefixes), acc)
        }
    }

    impl<T: PartialEq> PartialEq for ArraySeqS<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    impl<T: Eq> Eq for ArraySeqS<T> {}

    impl<T: Debug> Debug for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.iter()).finish() }
    }

    impl<T: Display> Display for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;

        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }
    }

    impl<T> IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter { Vec::from(self.data).into_iter() }
    }

    #[macro_export]
    macro_rules! ArraySeqS {
        () => {
            $crate::Chap18::ArraySeq::ArraySeq::ArraySeqS::from_vec(Vec::new())
        };
        ($elem:expr; $len:expr) => {
            $crate::Chap18::ArraySeq::ArraySeq::ArraySeqS::from_vec(vec![$elem; $len])
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap18::ArraySeq::ArraySeq::ArraySeqS::from_vec(vec![$($elem),+])
        };
    }

    #[allow(dead_code)]
    fn _arrayseqs_macro_type_checks() {
        let _: ArraySeqS<i32> = ArraySeqS![];
        let _: ArraySeqS<&str> = ArraySeqS!["a", "b", "c"];
        let _: ArraySeqS<i64> = ArraySeqS![0; 3];
    }
}
