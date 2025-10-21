//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral array sequence (mutable) implementation.

pub mod ArraySeqStEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
    use std::slice::Iter;
    use std::vec::IntoIter;

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct ArraySeqStEphS<T: StT> {
        data: Box<[T]>,
    }

    pub type ArrayStEph<T> = ArraySeqStEphS<T>;

    // Base methods - never redefined in later chapters
    pub trait ArraySeqStEphBaseTrait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn new(length: N, init_value: T)                           -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn set(&mut self, index: N, item: T)                       -> Result<&mut ArraySeqStEphS<T>, &'static str>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn length(&self)                                           -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn nth(&self, index: N)                                    -> &T;
        /// APAS: Work Θ(len), Span Θ(1)
        /// claude-4-sonet: Work Θ(len), Span Θ(len), Parallelism Θ(1) - sequential copy
        fn subseq(&self, start: N, length: N)                      -> Self;
        /// APAS: Work Θ(Σ|a[i]|), Span Θ(1)
        /// claude-4-sonet: Work Θ(Σ|a[i]|), Span Θ(Σ|a[i]|), Parallelism Θ(1) - sequential
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>)          -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - in-place mutation
        fn update(&mut self, update: Pair<N, T>)                   -> &mut Self;
        /// APAS: Work Θ(|updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|updates|), Span Θ(|updates|), Parallelism Θ(1) - sequential in-place
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self;
        /// APAS: Work Θ(|pairs|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|pairs|²), Span Θ(|pairs|²), Parallelism Θ(1) - sequential with linear search
        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStEphS<Pair<K, ArraySeqStEphS<V>>>;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>)                                  -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self)                                             -> Iter<'_, T>;
    }

    // Redefinable methods - may be overridden with better algorithms in later chapters
    pub trait ArraySeqStEphRedefinableTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty()                               -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn singleton(item: T)                    -> Self;
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;
        /// APAS: Work Θ(|a|+|b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|+|b|), Span Θ(|a|+|b|), Parallelism Θ(1) - sequential
        fn append(&self, b: &ArraySeqStEphS<T>)  -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential
        fn filter<F: PredSt<T>>(&self, pred: &F) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential fold
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential reduction
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1) - sequential prefix sum
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T);
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn isEmpty(&self)                        -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn isSingleton(&self)                    -> B;
    }

    impl<T: StT> ArraySeqStEphBaseTrait<T> for ArraySeqStEphS<T> {
        fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }
        fn length(&self) -> N { self.data.len() }
        fn nth(&self, index: N) -> &T { &self.data[index] }

        fn subseq(&self, start: N, length: N) -> Self {
            let total = self.data.len();
            let begin = start.min(total);
            let end = start.saturating_add(length).min(total);
            Self::from_vec(self.data[begin..end].to_vec())
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            if index < self.data.len() {
                self.data[index] = item;
                Ok(self)
            } else {
                Err("Index out of bounds")
            }
        }

        fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {
            let _ = self.set(index, item);
            self
        }

        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {
            let mut last_values: HashMap<N, T> = HashMap::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i).clone();
                if index < self.data.len() {
                    last_values.insert(index, value);
                }
            }
            for (index, value) in last_values {
                let _ = self.set(index, value);
            }
            self
        }

        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let inner = a.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStEphS<Pair<K, ArraySeqStEphS<V>>> {
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
            let collected: Vec<Pair<K, ArraySeqStEphS<V>>> = groups
                .into_iter()
                .map(|Pair(key, bucket)| Pair(key, ArraySeqStEphS::from_vec(bucket)))
                .collect();
            ArraySeqStEphS::from_vec(collected)
        }

        fn from_vec(elts: Vec<T>) -> Self {
            Self {
                data: elts.into_boxed_slice(),
            }
        }
        fn iter(&self) -> Iter<'_, T> { self.data.iter() }
    }

    impl<T: StT> ArraySeqStEphRedefinableTrait<T> for ArraySeqStEphS<T> {
        fn empty() -> Self { Self::from_vec(Vec::new()) }
        fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }

        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T> {
            let mut values: Vec<T> = Vec::with_capacity(length);
            for i in 0..length {
                values.push(f(i));
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn append(&self, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            let total = self.length() + b.length();
            let mut values: Vec<T> = Vec::with_capacity(total);
            for i in 0..self.length() {
                values.push(self.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn filter<F: PredSt<T>>(&self, pred: &F) -> ArraySeqStEphS<T> {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..self.length() {
                let value = self.nth(i);
                if pred(value) {
                    kept.push(value.clone());
                }
            }
            ArraySeqStEphS::from_vec(kept)
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A {
            let mut acc = seed;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T) {
            let mut prefixes: Vec<T> = Vec::with_capacity(a.length());
            let mut acc = id.clone();
            prefixes.push(acc.clone());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                if i < a.length() - 1 {
                    prefixes.push(acc.clone());
                }
            }
            (ArraySeqStEphS::from_vec(prefixes), acc)
        }

        fn isEmpty(&self) -> B { self.length() == 0 }
        fn isSingleton(&self) -> B { self.length() == 1 }
    }

    impl<T: StT> PartialEq for ArraySeqStEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }
    }
    impl<T: StT> Eq for ArraySeqStEphS<T> {}
    impl<T: StT> Debug for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.iter()).finish() }
    }
    impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }
    impl<T: StT> IntoIterator for ArraySeqStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter { self.data.into_vec().into_iter() }
    }
    impl<T: StT> Display for ArraySeqStEphS<T> {
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
    macro_rules! ArraySeqStEphS {
        () => { $crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _arrayseqstephs_macro_type_checks() {
        let _: ArraySeqStEphS<i32> = ArraySeqStEphS![];
        let _: ArraySeqStEphS<&str> = ArraySeqStEphS!["alpha", "beta"];
        let _: ArraySeqStEphS<i64> = ArraySeqStEphS![1; 2];
    }
}
