//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph {
    use std::sync::Mutex;
    use std::thread;

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS as ArraySeqMtEphSChap18, ArraySeqMtEphTrait as ArraySeqMtEphTraitChap18};
    use crate::Types::Types::*;

    pub type ArraySeqMtEphS<T> = ArraySeqMtEphSChap18<T>;

    pub trait ArraySeqMtEphTrait<T: StTInMtT> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>
        where
            T: Clone;
        fn empty() -> ArraySeqMtEphS<T>;
        fn singleton(item: T) -> ArraySeqMtEphS<T>;
        fn length(&self) -> N;
        fn nth_cloned(&self, index: N) -> T;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F) -> ArraySeqMtEphS<U> where T: 'static;
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T>;
        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEphS<T>;
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T where T: 'static;
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T);
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;
    }

    impl<T: StTInMtT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>
        where
            T: Clone,
        {
            // Keep as primitive - delegates to tabulate
            <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::new(length, init_value)
        }

        fn empty() -> ArraySeqMtEphS<T> {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(&|_| unreachable!("empty sequence has no elements"), 0)
        }

        fn singleton(item: T) -> ArraySeqMtEphS<T> {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            // Note: With function pointers, we can't capture `item`, so we use a different approach
            let mut result = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::empty();
            result.set(0, item).unwrap();
            result
        }

        fn length(&self) -> N { ArraySeqMtEphTraitChap18::length(self) }

        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTraitChap18::nth_cloned(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {
            // Keep as primitive - subseq is one of the 7 APAS primitives
            ArraySeqMtEphTraitChap18::subseq_copy(self, start, length)
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {
            // Keep as primitive - tabulate is one of the 7 APAS primitives
            // Implement directly to handle closures (can't delegate to Chap18 fn pointers)
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F) -> ArraySeqMtEphS<U> where T: 'static {
            // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
            // Sequential implementation to maintain APAS algorithmic fidelity with closures
            // Can't use tabulate with closure capture, implement directly
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(&a.nth_cloned(i)));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {
            let len_a = a.length();
            if index < len_a {
                return Some(a.nth_cloned(index));
            }
            let offset = index - len_a;
            let len_b = b.length();
            if offset < len_b {
                Some(b.nth_cloned(offset))
            } else {
                None
            }
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.4: append a b = flatten([a, b])
            // Implement directly since we can't capture with &F
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth_cloned(i));
            }
            for i in 0..b.length() {
                values.push(b.nth_cloned(i));
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            // Alternative append using tabulate with select helper
            // Implement directly since we can't capture with &F
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..(a.length() + b.length()) {
                if i < a.length() {
                    values.push(a.nth_cloned(i));
                } else {
                    values.push(b.nth_cloned(i - a.length()));
                }
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) == B::True {
                Self::singleton(x.clone())
            } else {
                Self::empty()
            }
        }

        fn filter<F: Fn(&T) -> B + Send + Sync>(a: &ArraySeqMtEphS<T>, pred: &F) -> ArraySeqMtEphS<T> {
            // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
            // Implement directly since we can't capture with owned F
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth_cloned(i);
                if pred(&value) == B::True {
                    kept.push(value);
                }
            }
            ArraySeqMtEphS::from_vec(kept)
        }

        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A {
            // Algorithm 19.8: iterate f x a = left-to-right traversal
            let mut acc = x;
            for i in 0..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T where T: 'static {
            // Algorithm 19.9 with parallelism: always parallel divide-and-conquer
            if a.length() == 0 {
                id
            } else if a.length() == 1 {
                a.nth_cloned(0)
            } else {
                // Always parallel for MT - divide and conquer
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let id_clone = id.clone();
                let f_clone = f.clone();
                let f_clone2 = f.clone();
                let handle = thread::spawn(move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&left, f_clone, id_clone));
                let right_result = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&right, f_clone2, id);
                let left_result = handle.join().unwrap();
                f(&left_result, &right_result)
            }
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified version)
            let mut acc = id.clone();
            let mut results = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
                results.push(acc.clone());
            }
            // Implement directly since we can't capture with &F
            let result_seq = ArraySeqMtEphS::from_vec(results);
            (result_seq, acc)
        }

        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {
            // Keep as primitive - flatten is one of the 7 APAS primitives
            <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::flatten(s)
        }
    }
}