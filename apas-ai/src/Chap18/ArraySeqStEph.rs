//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral array sequence (mutable) implementation.

pub mod ArraySeqStEph {
    use std::collections::HashSet;
    use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct ArraySeqStEphS<T: StT> {
        data: Box<[T]>,
    }

    pub type ArrayStEph<T> = ArraySeqStEphS<T>;

    impl<T: StT> ArraySeqStEphS<T> {
        pub fn from_vec(elts: Vec<T>) -> Self { Self { data: elts.into_boxed_slice() } }

        pub fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }

        pub fn empty() -> Self { Self::from_vec(Vec::new()) }

        pub fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }

        pub fn length(&self) -> N { self.data.len() }

        pub fn nth(&self, index: N) -> &T { &self.data[index] }

        /// Iterator over references to elements
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }

        pub fn subseq(&self, start: N, length: N) -> Self {
            let total = self.data.len();
            let begin = start.min(total);
            let end = start.saturating_add(length).min(total);
            Self::from_vec(self.data[begin..end].iter().cloned().collect())
        }

        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            if index < self.data.len() {
                self.data[index] = item;
                Ok(self)
            } else {
                Err("Index out of bounds")
            }
        }

        pub fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {
            let _ = self.set(index, item);
            self
        }

        pub fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut Self {
            let mut last_values: std::collections::HashMap<N, T> = std::collections::HashMap::new();
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
        type IntoIter = std::slice::Iter<'a, T>;
        
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<T: StT> IntoIterator for ArraySeqStEphS<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;
        
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_vec().into_iter()
        }
    }

    impl<T: StT> Display for ArraySeqStEphS<T> {
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

    pub trait ArraySeqStEphTrait<T: StT> {
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>;
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn empty() -> ArraySeqStEphS<T>;
        fn singleton(item: T) -> ArraySeqStEphS<T>;
        fn tabulate<F: Fn(N) -> T>(f: &F, length: N) -> ArraySeqStEphS<T>;
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;
        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T>;
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T>;
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;
        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T>;
        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T>;
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;
        fn collect<K: StT, V: StT>(
            pairs: &ArraySeqStEphS<Pair<K, V>>,
            cmp: fn(&K, &K) -> O,
        ) -> ArraySeqStEphS<Pair<K, ArraySeqStEphS<V>>>;
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T);
    }

    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::new(length, init_value) }

        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqStEphS<T>, &'static str> {
            ArraySeqStEphS::set(self, index, item)
        }

        fn length(&self) -> N { ArraySeqStEphS::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeqStEphS::nth(self, index) }

        fn empty() -> ArraySeqStEphS<T> { ArraySeqStEphS::empty() }

        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeqStEphS::singleton(item) }

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

        fn subseq(a: &ArraySeqStEphS<T>, start: N, length: N) -> ArraySeqStEphS<T> { a.subseq(start, length) }

        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            let total = a.length() + b.length();
            let mut values: Vec<T> = Vec::with_capacity(total);
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T> {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) == true {
                    kept.push(value.clone());
                }
            }
            ArraySeqStEphS::from_vec(kept)
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

        fn update(&mut self, update: Pair<N, T>) -> &mut ArraySeqStEphS<T> { ArraySeqStEphS::update(self, update) }

        fn inject(&mut self, updates: &ArraySeqStEphS<Pair<N, T>>) -> &mut ArraySeqStEphS<T> {
            ArraySeqStEphS::inject(self, updates)
        }

        fn isEmpty(&self) -> B { if self.length() == 0 { true } else { false } }

        fn isSingleton(&self) -> B { if self.length() == 1 { true } else { false } }

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
            prefixes.push(acc.clone()); // Include initial value
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                if i < a.length() - 1 {
                    prefixes.push(acc.clone());
                }
            }
            (ArraySeqStEphS::from_vec(prefixes), acc)
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
