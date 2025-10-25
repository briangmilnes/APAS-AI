//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 persistent sequence implementation for array-backed sequences.

pub mod ArraySeqStPer {

    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::fmt::Display;
    use std::fmt::Formatter;
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct ArraySeqStPerS<T: StT> {
        data: Box<[T]>,
    }

    pub type ArrayStPer<T> = ArraySeqStPerS<T>;

    // Base methods - never redefined in later chapters
    pub trait ArraySeqStPerBaseTrait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn new(length: N, init_value: T)                                        -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn length(&self)                                                        -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn nth(&self, index: N)                                                 -> &T;
        /// APAS: Work Θ(len), Span Θ(1)
        /// claude-4-sonet: Work Θ(len), Span Θ(len), Parallelism Θ(1) - sequential copy
        fn subseq_copy(&self, start: N, length: N)                              -> Self;
        /// APAS: Work Θ(Σ|a[i]|), Span Θ(1)
        /// claude-4-sonet: Work Θ(Σ|a[i]|), Span Θ(Σ|a[i]|), Parallelism Θ(1) - sequential
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>)                       -> Self;
        /// APAS: Work Θ(|a|+|updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential with HashSet
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>)  -> Self;
        /// APAS: Work Θ(|pairs|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|pairs|²), Span Θ(|pairs|²), Parallelism Θ(1) - sequential with linear search
        fn collect<K: StT, V: StT>(
            a: &ArraySeqStPerS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStPerS<Pair<K, ArraySeqStPerS<V>>>;
        /// claude-4-sonet: Work Θ(|a|+|updates|), Span Θ(|a|+|updates|), Parallelism Θ(1) - sequential, overwrites on conflict
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>)                                               -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self)                                                          -> Iter<'_, T>;
    }

    // Redefinable methods - may be overridden with better algorithms in later chapters
    pub trait ArraySeqStPerRedefinableTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                                               -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn singleton(item: T)                                    -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;
        /// APAS: Work Θ(|a|+|b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|b|), Span Θ(|a|+|b|), Parallelism Θ(1) - sequential
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>)  -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn filter<F: PredSt<T>>(a: &ArraySeqStPerS<T>, pred: &F) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential fold
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential reduction
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential prefix sum
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T);
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn isEmpty(&self)                                        -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn isSingleton(&self)                                    -> B;
    }

    impl<T: StT> ArraySeqStPerBaseTrait<T> for ArraySeqStPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> { Self::from_vec(vec![init_value; length]) }
        fn length(&self) -> N { self.data.len() }
        fn nth(&self, index: N) -> &T { &self.data[index] }

        fn subseq_copy(&self, start: N, length: N) -> Self {
            let total = self.data.len();
            let begin = start.min(total);
            let end = start.saturating_add(length).min(total);
            let slice: Vec<T> = self.data[begin..end].to_vec();
            Self::from_vec(slice)
        }

        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {
            let mut values = Vec::<T>::new();
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
            let mut updated = HashSet::<N>::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index >= result.length() {
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

        fn collect<K: StT, V: StT>(
            a: &ArraySeqStPerS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStPerS<Pair<K, ArraySeqStPerS<V>>> {
            if a.length() == 0 {
                return ArraySeqStPerS::from_vec(vec![]);
            }
            let mut groups = Vec::<Pair<K, ArraySeqStPerS<V>>>::new();
            for i in 0..a.length() {
                let Pair(key, value) = a.nth(i);
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(key, &group.0) == O::Equal {
                        let mut values = (0..group.1.length()).map(|j| group.1.nth(j).clone()).collect::<Vec<V>>();
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
            Self {
                data: elts.into_boxed_slice(),
            }
        }
        fn iter(&self) -> Iter<'_, T> { self.data.iter() }
    }

    impl<T: StT> ArraySeqStPerRedefinableTrait<T> for ArraySeqStPerS<T> {
        fn empty() -> ArraySeqStPerS<T> { Self::from_vec(Vec::new()) }
        fn singleton(item: T) -> ArraySeqStPerS<T> { Self::from_vec(vec![item]) }

        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStPerS<T> {
            let mut values = Vec::<T>::with_capacity(length);
            for i in 0..length {
                values.push(f(i));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {
            let mut values = Vec::<U>::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let mut values = Vec::<T>::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn filter<F: PredSt<T>>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {
            let mut values = Vec::<T>::new();
            for i in 0..a.length() {
                let item = a.nth(i);
                if pred(item) {
                    values.push(item.clone());
                }
            }
            ArraySeqStPerS::from_vec(values)
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
            let mut values = Vec::<T>::with_capacity(a.length());
            values.push(acc.clone());
            for i in 0..a.length() {
                let item = a.nth(i);
                acc = f(&acc, item);
                if i < a.length() - 1 {
                    values.push(acc.clone());
                }
            }
            (ArraySeqStPerS::from_vec(values), acc)
        }

        fn isEmpty(&self) -> B { self.data.is_empty() }
        fn isSingleton(&self) -> B { self.data.len() == 1 }
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
}
