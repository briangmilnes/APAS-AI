//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for ArraySeqMtPer multithreaded, just tabulate needed.

pub mod ArraySeqMtPer {
    use std::collections::HashSet;
    use std::sync::Arc;
    use std::thread;

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Box<[T]>` (persistent MT variant).
    #[derive(Debug)]
    pub struct ArraySeqMtPerS<T: MtVal> {
        data: Box<[T]>,
    }

    impl<T: MtVal> ArraySeqMtPerS<T> {
        pub fn empty() -> Self {
            ArraySeqMtPerS {
                data: Vec::new().into_boxed_slice(),
            }
        }

        pub fn new(length: N, init_value: T) -> Self {
            let mut values: Vec<T> = Vec::with_capacity(length);
            for _ in 0..length {
                values.push(init_value.clone());
            }
            ArraySeqMtPerS::from_vec(values)
        }

        pub fn singleton(item: T) -> Self {
            ArraySeqMtPerS::from_vec(vec![item])
        }

        pub fn from_vec(values: Vec<T>) -> Self {
            ArraySeqMtPerS {
                data: values.into_boxed_slice(),
            }
        }

        pub fn length(&self) -> N {
            self.data.len()
        }

        pub fn nth(&self, index: N) -> &T {
            &self.data[index]
        }

        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            let values: Vec<T> = self.data[s..e].iter().cloned().collect();
            ArraySeqMtPerS::from_vec(values)
        }

        pub fn is_empty(&self) -> B {
            if self.data.is_empty() { true } else { false }
        }

        pub fn is_singleton(&self) -> B {
            if self.data.len() == 1 { true } else { false }
        }

        /// Iterator over references to elements
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.data.iter()
        }
    }

    impl<T: MtVal> Clone for ArraySeqMtPerS<T> {
        fn clone(&self) -> Self {
            let values: Vec<T> = self.data.iter().cloned().collect();
            ArraySeqMtPerS::from_vec(values)
        }
    }

    impl<T: MtVal> PartialEq for ArraySeqMtPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.data.len() != other.data.len() {
                return false;
            }
            for i in 0..self.data.len() {
                if self.data[i] != other.data[i] {
                    return false;
                }
            }
            true
        }
    }

    impl<T: MtVal + Eq> Eq for ArraySeqMtPerS<T> {}

    impl<'a, T: MtVal> IntoIterator for &'a ArraySeqMtPerS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<T: MtVal> IntoIterator for ArraySeqMtPerS<T> {
        type Item = T;
        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.data.into_vec().into_iter()
        }
    }

    impl<T: MtVal> std::fmt::Display for ArraySeqMtPerS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtPerS[")?;
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    pub trait ArraySeqMtPerTrait<T: MtVal> {
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;
        fn empty() -> ArraySeqMtPerS<T>;
        fn singleton(item: T) -> ArraySeqMtPerS<T>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;
        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str>;
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T>;
        fn map<W: MtVal, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: F) -> ArraySeqMtPerS<W>;
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtPerS<T>, pred: &F) -> ArraySeqMtPerS<T>;
        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T>;
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T>;
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, x: A) -> A;
        fn iteratePrefixes<A: StTInMtT + 'static, F: Fn(&A, &T) -> A + Send + Sync>(
            a: &ArraySeqMtPerS<T>,
            f: &F,
            x: A,
        ) -> (ArraySeqMtPerS<A>, A);
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (ArraySeqMtPerS<T>, T);
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;
        fn collect(a: &ArraySeqMtPerS<Pair<T, T>>, cmp: fn(&T, &T) -> O) -> ArraySeqMtPerS<Pair<T, ArraySeqMtPerS<T>>>;
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T>;
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;
    }

    impl<T: MtVal> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> {
            ArraySeqMtPerS::new(length, init_value)
        }
        fn empty() -> ArraySeqMtPerS<T> {
            ArraySeqMtPerS::empty()
        }
        fn singleton(item: T) -> ArraySeqMtPerS<T> {
            ArraySeqMtPerS::singleton(item)
        }
        fn length(&self) -> N {
            ArraySeqMtPerS::length(self)
        }
        fn nth(&self, index: N) -> &T {
            ArraySeqMtPerS::nth(self, index)
        }
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {
            ArraySeqMtPerS::subseq_copy(self, start, length)
        }

        fn set(&self, index: N, item: T) -> Result<ArraySeqMtPerS<T>, &'static str> {
            if index >= self.data.len() {
                return Err("Index out of bounds");
            }
            let mut new_data: Vec<T> = self.data.iter().cloned().collect();
            new_data[index] = item;
            Ok(ArraySeqMtPerS::from_vec(new_data))
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: F,
        ) -> ArraySeqMtPerS<W>
        where
            T: 'static,
        {
            if a.length() == 0 {
                return ArraySeqMtPerS::from_vec(Vec::new());
            }
            if a.length() == 1 {
                let result = f(a.nth(0));
                return ArraySeqMtPerS::from_vec(vec![result]);
            }
            let mid = a.length() / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, a.length() - mid);
            let f_clone = f.clone();
            let left_handle = thread::spawn(move || ArraySeqMtPerS::map(&left, f_clone));
            let right_result = ArraySeqMtPerS::map(&right, f);
            let left_result = left_handle.join().unwrap();
            ArraySeqMtPerS::append(&left_result, &right_result)
        }

        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtPerS<T>, pred: &F) -> ArraySeqMtPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let item = a.nth(i);
                if pred(item) == true {
                    values.push(item.clone());
                }
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn update(a: &ArraySeqMtPerS<T>, item_at: Pair<N, T>) -> ArraySeqMtPerS<T> {
            let Pair(index, item) = item_at;
            if index >= a.length() {
                return a.clone();
            }
            let mut new_data: Vec<T> = a.data.iter().cloned().collect();
            new_data[index] = item;
            ArraySeqMtPerS::from_vec(new_data)
        }

        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T> {
            let mut result = a.clone();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() {
                    result = result.set(*index, value.clone()).unwrap_or(result);
                }
            }
            result
        }

        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn iteratePrefixes<A: StTInMtT + 'static, F: Fn(&A, &T) -> A + Send + Sync>(
            a: &ArraySeqMtPerS<T>,
            f: &F,
            x: A,
        ) -> (ArraySeqMtPerS<A>, A) {
            let mut acc = x;
            let mut values: Vec<A> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                values.push(acc.clone());
            }
            (ArraySeqMtPerS::from_vec(values), acc)
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: F, id: T) -> T
        where
            T: 'static,
        {
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth(0).clone();
            }
            if a.length() < 1000 {
                // Sequential for small arrays
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let f_clone = f.clone();
                let f_clone2 = f.clone();
                let l = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&left, f_clone, id.clone());
                let r = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&right, f_clone2, id);
                f(&l, &r)
            } else {
                // Parallel for large arrays
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let f_clone = f.clone();
                let f_clone2 = f.clone();

                let id_clone = id.clone();
                let left_handle = thread::spawn(move || {
                    <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&left, f_clone, id_clone)
                });
                let r = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&right, f_clone2, id);
                let l = left_handle.join().unwrap();
                f(&l, &r)
            }
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (ArraySeqMtPerS<T>, T) {
            let mut acc = id.clone();
            let mut values: Vec<T> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                values.push(acc.clone());
            }
            (ArraySeqMtPerS::from_vec(values), acc)
        }

        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner_seq = ss.nth(i);
                for j in 0..inner_seq.length() {
                    values.push(inner_seq.nth(j).clone());
                }
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn collect(a: &ArraySeqMtPerS<Pair<T, T>>, cmp: fn(&T, &T) -> O) -> ArraySeqMtPerS<Pair<T, ArraySeqMtPerS<T>>> {
            if a.length() == 0 {
                return ArraySeqMtPerS::from_vec(vec![]);
            }
            let mut groups: Vec<Pair<T, ArraySeqMtPerS<T>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(key, value) = a.nth(i);
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(&key, &group.0) == O::Equal {
                        let mut values: Vec<T> = (0..group.1.length()).map(|j| group.1.nth(j).clone()).collect();
                        values.push(value.clone());
                        group.1 = ArraySeqMtPerS::from_vec(values);
                        found_group = true;
                        break;
                    }
                }
                if !found_group {
                    groups.push(Pair(key.clone(), ArraySeqMtPerS::from_vec(vec![value.clone()])));
                }
            }
            ArraySeqMtPerS::from_vec(groups)
        }

        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T> {
            let mut result = a.clone();
            let mut updated: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() && updated.insert(*index) {
                    result = result.set(*index, value.clone()).unwrap_or(result);
                }
            }
            result
        }

        fn isEmpty(&self) -> B {
            ArraySeqMtPerS::is_empty(self)
        }

        fn isSingleton(&self) -> B {
            ArraySeqMtPerS::is_singleton(self)
        }
    }
}
