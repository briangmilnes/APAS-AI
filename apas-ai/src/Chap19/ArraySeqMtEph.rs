//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph {
    use std::sync::Mutex;
    use std::thread;

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{
        ArraySeqMtEphS as ArraySeqMtEphSChap18, ArraySeqMtEphTrait as ArraySeqMtEphTraitChap18,
    };
    use crate::Types::Types::*;

    pub type ArraySeqMtEphS<T> = ArraySeqMtEphSChap18<T>;

    pub trait ArraySeqMtEphTrait<T: StTInMtT> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>;
        fn empty() -> ArraySeqMtEphS<T>;
        fn singleton(item: T) -> ArraySeqMtEphS<T>;
        fn length(&self) -> N;
        fn nth_cloned(&self, index: N) -> T;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: MtVal, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>;
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T>;
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: F) -> ArraySeqMtEphS<T>;
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T);
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;
        fn isEmpty(a: &ArraySeqMtEphS<T>) -> bool;
        fn isSingleton(a: &ArraySeqMtEphS<T>) -> bool;
        fn update(a: &ArraySeqMtEphS<T>, index: N, item: T) -> ArraySeqMtEphS<T>;
    }

    impl<T: StTInMtT + 'static> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T> {
            // Keep as primitive - delegates to tabulate
            <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::new(length, init_value)
        }

        fn empty() -> ArraySeqMtEphS<T> {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|_| unreachable!("empty sequence has no elements"),
                0,
            )
        }

        fn singleton(item: T) -> ArraySeqMtEphS<T> {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(&|_| item.clone(), 1)
        }

        fn length(&self) -> N { ArraySeqMtEphTraitChap18::length(self) }

        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTraitChap18::nth_cloned(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {
            // Keep as primitive - subseq is one of the 7 APAS primitives
            ArraySeqMtEphTraitChap18::subseq_copy(self, start, length)
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {
            // Keep as primitive - tabulate is one of the 7 APAS primitives
            if n == 0 {
                return <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::empty();
            }
            let mut result = <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::new(n, f(0));
            for i in 1..n {
                result.set(i, f(i)).unwrap();
            }
            result
        }

        fn map<U: MtVal, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U> {
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
            let total_len = a.length() + b.length();
            if total_len == 0 {
                return <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::empty();
            }
            let first_elem = if a.length() > 0 {
                a.nth_cloned(0)
            } else {
                b.nth_cloned(0)
            };
            let mut result = <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::new(total_len, first_elem);
            for i in 1..a.length() {
                result.set(i, a.nth_cloned(i)).unwrap();
            }
            for i in 0..b.length() {
                result.set(a.length() + i, b.nth_cloned(i)).unwrap();
            }
            result
        }

        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|i| <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::select(a, b, i).unwrap(),
                a.length() + b.length(),
            )
        }

        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtEphS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) == true {
                Self::singleton(x.clone())
            } else {
                Self::empty()
            }
        }

        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, pred: F) -> ArraySeqMtEphS<T> {
            // Algorithm 19.5 with parallelism: fork thread per element + serial compaction
            if a.length() == 0 {
                return <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::empty();
            }

            // Create boolean sequence for keep/filter results
            let mut keep_results = <ArraySeqMtEphS<B> as ArraySeqMtEphTraitChap18<B>>::new(a.length(), false);

            // Fork thread per element to evaluate predicate, collect results serially
            for i in 0..a.length() {
                let value = a.nth_cloned(i);
                let pred_clone = pred.clone();

                let handle = std::thread::spawn(move || pred_clone(&value));

                let keep = handle.join().unwrap();
                keep_results.set(i, keep).unwrap();
            }

            // Serial compaction phase: count kept values
            let mut kept_count = 0;
            for i in 0..keep_results.length() {
                if keep_results.nth_cloned(i) == true {
                    kept_count += 1;
                }
            }

            if kept_count == 0 {
                return <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::empty();
            }

            // Find first kept value and create result sequence
            let mut first_kept = None;
            for i in 0..a.length() {
                if keep_results.nth_cloned(i) == true {
                    first_kept = Some(a.nth_cloned(i));
                    break;
                }
            }
            let first_kept = first_kept.unwrap();

            let mut result = <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::new(kept_count, first_kept);
            let mut result_idx = 1;

            for i in 0..a.length() {
                if keep_results.nth_cloned(i) == true && result_idx < kept_count {
                    result.set(result_idx, a.nth_cloned(i)).unwrap();
                    result_idx += 1;
                }
            }

            result
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

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T {
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
                let handle = thread::spawn(move || {
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&left, f_clone, id_clone)
                });
                let right_result = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&right, f_clone2, id);
                let left_result = handle.join().unwrap();
                f(&left_result, &right_result)
            }
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified version)
            if a.length() == 0 {
                return (<ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::empty(), id);
            }
            let mut acc = id.clone();
            let item = a.nth_cloned(0);
            acc = f(&acc, &item);
            let mut result_seq = <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::new(a.length(), acc.clone());
            for i in 1..a.length() {
                let item = a.nth_cloned(i);
                acc = f(&acc, &item);
                result_seq.set(i, acc.clone()).unwrap();
            }
            (result_seq, acc)
        }

        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {
            // Keep as primitive - flatten is one of the 7 APAS primitives
            <ArraySeqMtEphS<T> as ArraySeqMtEphTraitChap18<T>>::flatten(s)
        }

        fn isEmpty(a: &ArraySeqMtEphS<T>) -> bool {
            // Algorithm 19.7: isEmpty a = |a| = 0
            a.length() == 0
        }

        fn isSingleton(a: &ArraySeqMtEphS<T>) -> bool {
            // Algorithm 19.7: isSingleton a = |a| = 1
            a.length() == 1
        }

        fn update(a: &ArraySeqMtEphS<T>, index: N, item: T) -> ArraySeqMtEphS<T> {
            // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|j| if j == index { item.clone() } else { a.nth_cloned(j) },
                a.length(),
            )
        }
    }
}
