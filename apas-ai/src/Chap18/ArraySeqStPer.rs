//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 persistent sequence implementation for array-backed sequences.

pub mod ArraySeqStPer {

    use std::collections::HashSet;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
    use std::slice::Iter;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct ArraySeqStPerS<T: StT> {
        data: Box<[T]>,
    }

    pub type ArrayStPer<T> = ArraySeqStPerS<T>;

    pub trait ArraySeqStPerTrait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn new(length: N, init_value: T)                                        -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn length(&self)                                                        -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn nth(&self, index: N)                                                 -> &T;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                                              -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn singleton(item: T)                                                   -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn tabulate<F: Fn(N)                                                    -> T>(f: &F, length: N) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn map<U: StT, F: Fn(&T)                                                -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;
        /// APAS: Work Θ(len), Span Θ(1)
        /// claude-4-sonet: Work Θ(len), Span Θ(len), Parallelism Θ(1) - sequential copy
        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N)              -> Self;
        /// APAS: Work Θ(|a|+|b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|b|), Span Θ(|a|+|b|), Parallelism Θ(1) - sequential
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>)                 -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn filter<F: PredSt<T>>(a: &ArraySeqStPerS<T>, pred: &F)                -> Self;
        /// APAS: Work Θ(Σ|a[i]|), Span Θ(1)
        /// claude-4-sonet: Work Θ(Σ|a[i]|), Span Θ(Σ|a[i]|), Parallelism Θ(1) - sequential
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>)                       -> Self;
        /// APAS: Work Θ(|a|+|updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential with HashSet
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>)  -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn isEmpty(&self)                                                       -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn isSingleton(&self)                                                   -> B;
        /// APAS: Work Θ(|pairs|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|pairs|²), Span Θ(|pairs|²), Parallelism Θ(1) - sequential with linear search
        fn collect<K: StT, V: StT>(
            a: &ArraySeqStPerS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStPerS<Pair<K, ArraySeqStPerS<V>>>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential fold
        fn iterate<A, F: Fn(&A, &T)                                             -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential reduction
        fn reduce<F: Fn(&T, &T)                                                 -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential prefix sum
        fn scan<F: Fn(&T, &T)                                                   -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T);
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential, overwrites on conflict
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self) -> Iter<'_, T>;
    }


    impl<T: StT> ArraySeqStPerS<T> {
        pub fn from_vec(elts: Vec<T>) -> Self {
            Self {
                data: elts.into_boxed_slice(),
            }
        }
        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }
        pub fn empty() -> Self { Self::from_vec(Vec::new()) }
        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }
        pub fn length(&self) -> N { self.data.len() }
        pub fn nth(&self, index: N) -> &T { &self.data[index] }
        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let total = self.data.len();
            let begin = start.min(total);
            let end = start.saturating_add(length).min(total);
            let slice: Vec<T> = self.data[begin..end].to_vec();
            Self::from_vec(slice)
        }

        /// Iterator over references to elements
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }
    }







    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::new(length, init_value) }
        fn length(&self) -> N { ArraySeqStPerS::length(self) }
        fn nth(&self, index: N) -> &T { ArraySeqStPerS::nth(self, index) }
        fn empty() -> ArraySeqStPerS<T> { ArraySeqStPerS::empty() }
        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeqStPerS::singleton(item) }

        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(length);
            for i in 0..length {
                values.push(f(i));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn subseq_copy(a: &ArraySeqStPerS<T>, start: N, length: N) -> ArraySeqStPerS<T> { a.subseq_copy(start, length) }

        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn filter<F: PredSt<T>>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let item = a.nth(i);
                if pred(item) {
                    values.push(item.clone());
                }
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let inner_seq = a.nth(i);
                for j in 0..inner_seq.length() {
                    values.push(inner_seq.nth(j).clone());
                }
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            let mut result = a.clone();
            let mut updated: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index >= result.length() {
                    // Skip out-of-bounds indices instead of panicking
                    continue;
                }
                if updated.insert(*index) {
                    let mut new_data: Vec<T> = result.data.to_vec();
                    new_data[*index] = value.clone();
                    result = ArraySeqStPerS::from_vec(new_data);
                }
            }
            result
        }

        fn isEmpty(&self) -> B { self.data.is_empty() }

        fn isSingleton(&self) -> B { self.data.len() == 1 }

        fn collect<K: StT, V: StT>(
            a: &ArraySeqStPerS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStPerS<Pair<K, ArraySeqStPerS<V>>> {
            if a.length() == 0 {
                return ArraySeqStPerS::from_vec(vec![]);
            }
            let mut groups: Vec<Pair<K, ArraySeqStPerS<V>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(key, value) = a.nth(i);
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(key, &group.0) == O::Equal {
                        let mut values: Vec<V> = (0..group.1.length()).map(|j| group.1.nth(j).clone()).collect();
                        values.push(value.clone());
                        group.1 = ArraySeqStPerS::from_vec(values);
                        found_group = true;
                        break;
                    }
                }
                if !found_group {
                    groups.push(Pair(key.clone(), ArraySeqStPerS::from_vec(vec![value.clone()])));
                }
            }
            ArraySeqStPerS::from_vec(groups)
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A {
            let mut acc = seed;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth(0).clone();
            }
            let mid = a.length() / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, a.length() - mid);
            let l = ArraySeqStPerS::reduce(&left, f, id.clone());
            let r = ArraySeqStPerS::reduce(&right, f, id);
            f(&l, &r)
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T) {
            let mut acc = id.clone();
            let mut values: Vec<T> = Vec::with_capacity(a.length());
            values.push(acc.clone()); // Include initial value
            for i in 0..a.length() {
                let item = a.nth(i);
                acc = f(&acc, item);
                if i < a.length() - 1 {
                    values.push(acc.clone());
                }
            }
            (ArraySeqStPerS::from_vec(values), acc)
        }

        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            let mut result = a.clone();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() {
                    let mut new_data: Vec<T> = result.data.to_vec();
                    new_data[*index] = value.clone();
                    result = ArraySeqStPerS::from_vec(new_data);
                }
            }
            result
        }

        fn from_vec(elts: Vec<T>) -> Self {
            ArraySeqStPerS::from_vec(elts)
        }

        fn iter(&self) -> Iter<'_, T> {
            ArraySeqStPerS::iter(self)
        }
    }

    impl<T: StT> PartialEq for ArraySeqStPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }
    }
    impl<T: StT> Eq for ArraySeqStPerS<T> {}
    impl<T: StT> Debug for ArraySeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.iter()).finish() }
    }
    impl<'a, T: StT> IntoIterator for &'a ArraySeqStPerS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }
    impl<T: StT> IntoIterator for ArraySeqStPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter { self.data.into_vec().into_iter() }
    }
    impl<T: StT> Display for ArraySeqStPerS<T> {
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

    #[macro_export]
    macro_rules! ArraySeqStPerS {
        () => { $crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$($x),*]) };
    }
}
