//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).
//!
//! Note: Uses unconditional parallelism with ParaPair! for divide-and-conquer operations (map, reduce).

pub mod ArraySeqMtEph {
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::Types::Types::*;
    use crate::ParaPair;

    /// Fixed-length sequence backed by `Mutex<Box<[T]>>` (ephemeral/mutable MT variant).
    #[derive(Debug)]
    pub struct ArraySeqMtEphS<T: StTInMtT> {
        data: Mutex<Box<[T]>>,
    }

    impl<T: StTInMtT> ArraySeqMtEphS<T> {
        pub fn empty() -> Self {
            ArraySeqMtEphS {
                data: Mutex::new(Vec::new().into_boxed_slice()),
            }
        }

        pub fn new(length: N, init_value: T) -> Self
        where
            T: Clone,
        {
            ArraySeqMtEphS::from_vec(vec![init_value; length])
        }

        pub fn singleton(item: T) -> Self {
            ArraySeqMtEphS::from_vec(vec![item])
        }

        pub fn from_vec(values: Vec<T>) -> Self {
            ArraySeqMtEphS {
                data: Mutex::new(values.into_boxed_slice()),
            }
        }

        pub fn length(&self) -> N {
            let guard = self.data.lock().unwrap();
            guard.len()
        }

        pub fn nth_cloned(&self, index: N) -> T {
            let guard = self.data.lock().unwrap();
            guard[index].clone()
        }

        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            {
                let mut guard = self.data.lock().unwrap();
                if index < guard.len() {
                    guard[index] = item;
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(self)
        }

        /// Iterator over cloned elements (due to Mutex)
        pub fn iter_cloned(&self) -> Vec<T> {
            let guard = self.data.lock().unwrap();
            guard.iter().cloned().collect()
        }

        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let guard = self.data.lock().unwrap();
            let n = guard.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            let values: Vec<T> = guard[s..e].iter().cloned().collect();
            ArraySeqMtEphS::from_vec(values)
        }

        pub fn to_vec(&self) -> Vec<T> {
            let guard = self.data.lock().unwrap();
            guard.iter().cloned().collect()
        }
    }

    impl<T: StTInMtT> Clone for ArraySeqMtEphS<T> {
        fn clone(&self) -> Self {
            ArraySeqMtEphS::from_vec(self.to_vec())
        }
    }

    impl<T: StTInMtT> PartialEq for ArraySeqMtEphS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.to_vec() == other.to_vec()
        }
    }

    impl<T: StTInMtT> Eq for ArraySeqMtEphS<T> {}

    pub trait ArraySeqMtEphTrait<T: StTInMtT> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>;
        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str>;
        fn length(&self) -> N;
        fn nth_cloned(&self, index: N) -> T;
        fn empty() -> ArraySeqMtEphS<T>;
        fn singleton(item: T) -> ArraySeqMtEphS<T>;
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F) -> ArraySeqMtEphS<U> where T: Send + 'static;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEphS<T>;
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T>;
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;
        fn collect(a: &ArraySeqMtEphS<Pair<T, T>>, cmp: fn(&T, &T) -> O) -> ArraySeqMtEphS<Pair<T, ArraySeqMtEphS<T>>>;
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T where T: Send + 'static;
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T);
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T>;
    }

    impl<T: StTInMtT> std::fmt::Display for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtEphS[")?;
            let guard = self.data.lock().unwrap();
            for (i, item) in guard.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    impl<T: StTInMtT + 'static> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphS::new(length, init_value)
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut ArraySeqMtEphS<T>, &'static str> {
            ArraySeqMtEphS::set(self, index, item)
        }

        fn length(&self) -> N {
            ArraySeqMtEphS::length(self)
        }

        fn nth_cloned(&self, index: N) -> T {
            ArraySeqMtEphS::nth_cloned(self, index)
        }

        fn empty() -> ArraySeqMtEphS<T> {
            ArraySeqMtEphS::empty()
        }

        fn singleton(item: T) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphS::singleton(item)
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static,
        {
            let n = a.length();
            if n == 0 {
                return ArraySeqMtEphS::from_vec(Vec::new());
            }
            if n == 1 {
                let val = f(&a.nth_cloned(0));
                return ArraySeqMtEphS::from_vec(vec![val]);
            }
            
            // Unconditionally parallel using ParaPair!
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);
            let f_clone = f.clone();

            let Pair(left_result, right_result) = ParaPair!(
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::map(&left, f_clone),
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::map(&right, f)
            );

            <ArraySeqMtEphS<U> as ArraySeqMtEphTrait<U>>::append(&left_result, &right_result)
        }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphS::subseq_copy(self, start, length)
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            let na = a.length();
            let nb = b.length();
            let mut values: Vec<T> = Vec::with_capacity(na + nb);
            for i in 0..na {
                values.push(a.nth_cloned(i));
            }
            for j in 0..nb {
                values.push(b.nth_cloned(j));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEphS<T> {
            let mut kept: Vec<T> = Vec::new();
            let n = a.length();
            for i in 0..n {
                let value = a.nth_cloned(i);
                if pred(&value) == true {
                    kept.push(value);
                }
            }
            ArraySeqMtEphS::from_vec(kept)
        }

        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {
            let _ = a.set(index, item);
            a
        }

        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            let mut out = a.clone();
            let mut seen: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth_cloned(i);
                if seen.insert(idx) {
                    let _ = out.set(idx, val);
                }
            }
            out
        }

        fn isEmpty(&self) -> B {
            if self.length() == 0 { true } else { false }
        }

        fn isSingleton(&self) -> B {
            if self.length() == 1 { true } else { false }
        }

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner = ss.nth_cloned(i);
                for j in 0..inner.length() {
                    values.push(inner.nth_cloned(j));
                }
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn collect(a: &ArraySeqMtEphS<Pair<T, T>>, cmp: fn(&T, &T) -> O) -> ArraySeqMtEphS<Pair<T, ArraySeqMtEphS<T>>> {
            if a.length() == 0 {
                return ArraySeqMtEphS::from_vec(vec![]);
            }
            let mut groups: Vec<Pair<T, ArraySeqMtEphS<T>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(key, value) = a.nth_cloned(i);
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(&key, &group.0) == O::Equal {
                        let mut values: Vec<T> = Vec::with_capacity(group.1.length() + 1);
                        for j in 0..group.1.length() {
                            values.push(group.1.nth_cloned(j));
                        }
                        values.push(value.clone());
                        group.1 = ArraySeqMtEphS::from_vec(values);
                        found_group = true;
                        break;
                    }
                }
                if !found_group {
                    groups.push(Pair(key.clone(), ArraySeqMtEphS::from_vec(vec![value])));
                }
            }
            ArraySeqMtEphS::from_vec(groups)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
        where
            T: Send + 'static,
        {
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth_cloned(0);
            }
            
            // Unconditionally parallel using ParaPair!
            let mid = a.length() / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, a.length() - mid);
            let f_clone = f.clone();
            let f_clone2 = f.clone();
            let id_clone = id.clone();

            let Pair(l, r) = ParaPair!(
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&left, f_clone, id_clone),
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&right, f_clone2, id)
            );
            f(&l, &r)
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T) {
            let mut acc = id.clone();
            let mut values: Vec<T> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
                values.push(acc.clone());
            }
            (ArraySeqMtEphS::from_vec(values), acc)
        }

        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            let mut out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth_cloned(i);
                let _ = out.set(idx, val);
            }
            out
        }
    }

    #[macro_export]
    macro_rules! ArraySeqMtEphSLit {
        () => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqMtEphSLit_type_checks() {
        let _ = ArraySeqMtEphSLit![1];
        let _ = ArraySeqMtEphSLit![0; 2];
        let _: ArraySeqMtEphS<i32> = ArraySeqMtEphSLit![];
    }
}
