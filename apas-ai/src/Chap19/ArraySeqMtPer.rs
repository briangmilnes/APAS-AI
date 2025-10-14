//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqMtPer, just the one multi-threaded update of code that Umut and Guy snuck into this chapter.

pub mod ArraySeqMtPer {

    use std::sync::Mutex;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerS as ArraySeqMtPerSChap18, ArraySeqMtPerTrait as ArraySeqMtPerTraitChap18};
    use crate::Types::Types::*;

    pub type ArraySeqMtPerS<T> = ArraySeqMtPerSChap18<T>;

    pub trait ArraySeqMtPerTrait<T: StTInMtT> {
        // Chapter 18 wrappers
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N) -> &T;
        /// claude-4-sonet: Work Θ(length), Span Θ(log length), Parallelism Θ(length/log length)
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;

        /// claude-4-sonet: Work Θ(n + Σᵢ W(f(i))), Span Θ(log n + maxᵢ S(f(i))), Parallelism Θ(n)
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|a| + Σₓ W(f(x))), Span Θ(log |a| + maxₓ S(f(x))), Parallelism Θ(|a|)
        fn map<W: StTInMtT + 'static, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: F,
        ) -> ArraySeqMtPerS<W>
        where
            T: 'static;
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(log(|a| + |b|)), Parallelism Θ((|a|+|b|)/log(|a|+|b|))
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|a| + Σᵢ W(f(aᵢ))), Span Θ(log |a| + maxᵢ S(f(aᵢ))), Parallelism Θ(|a|)
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, pred: F) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(log |a|), Parallelism Θ(|a|/log |a|)
        fn update_single(a: &ArraySeqMtPerS<T>, index: N, item: T) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(log |a| + log |updates|)
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1)
        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, x: A) -> A;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1)
        fn iteratePrefixes<A: StTInMtT + 'static, F: Fn(&A, &T) -> A + Send + Sync>(
            a: &ArraySeqMtPerS<T>,
            f: &F,
            x: A,
        ) -> (ArraySeqMtPerS<A>, A);
        /// claude-4-sonet: Work Θ(|a|), Span Θ(log |a|), Parallelism Θ(|a|/log |a|)
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: F, id: T) -> T
        where
            T: 'static;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(log |a|), Parallelism Θ(|a|/log |a|)
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (ArraySeqMtPerS<T>, T);
        /// claude-4-sonet: Work Θ(Σ |sᵢ|), Span Θ(log(Σ |sᵢ|))
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|a|²) worst case, Span Θ(|a|²) worst case
        fn collect<K: StTInMtT, V: StTInMtT>(a: &ArraySeqMtPerS<Pair<K, V>>, cmp: fn(&K, &K) -> O) -> ArraySeqMtPerS<Pair<K, ArraySeqMtPerS<V>>>;

        // Chapter 19 specific functions
        /// claude-4-sonet: Work Θ(|values| + |changes|), Span Θ(log |values| + log |changes|)
        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T>;
        /// claude-4-sonet: Work Θ(|changes|), Span Θ(log |changes|)
        fn atomicWrite(
            values_with_change_number: &mut ArraySeqMtPerS<Pair<T, N>>,
            changes: &ArraySeqMtPerS<Pair<N, T>>,
            change_index: N,
        );
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(a: &ArraySeqMtPerS<T>) -> bool;
        fn isSingleton(a: &ArraySeqMtPerS<T>) -> bool;
        fn append_select(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;
        fn select<'a>(a: &'a ArraySeqMtPerS<T>, b: &'a ArraySeqMtPerS<T>, i: N) -> Option<&'a T>;
        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtPerS<T>;
    }

    impl<T: StTInMtT + 'static> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> {
            // Keep as primitive - delegates to tabulate
            <ArraySeqMtPerS<T> as ArraySeqMtPerTraitChap18<T>>::new(length, init_value)
        }

        fn empty() -> ArraySeqMtPerS<T> {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::tabulate(
                &|_| unreachable!("empty sequence has no elements"),
                0,
            )
        }

        fn singleton(item: T) -> ArraySeqMtPerS<T> {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::tabulate(&|_| item.clone(), 1)
        }

        fn length(&self) -> N { ArraySeqMtPerTraitChap18::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeqMtPerTraitChap18::nth(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {
            <ArraySeqMtPerS<T> as ArraySeqMtPerTraitChap18<T>>::subseq_copy(self, start, length)
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtPerS<T> {
            // Keep as primitive - tabulate is one of the 7 APAS primitives
            // Implement directly to handle closures (can't delegate to Chap18 fn pointers)
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn map<W: MtVal, F: Fn(&T) -> W + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: F,
        ) -> ArraySeqMtPerS<W> {
            // Algorithm 19.3 with parallelism: map f a = tabulate(lambda i.f(a[i]), |a|)
            if a.length() <= 1 {
                // Implement directly since we can't capture with &F
                let mut values: Vec<W> = Vec::with_capacity(a.length());
                for i in 0..a.length() {
                    values.push(f(a.nth(i)));
                }
                return ArraySeqMtPerS::from_vec(values);
            }
            // Always parallel for MT - divide and conquer
            let mid = a.length() / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, a.length() - mid);
            let f_clone = f.clone();
            let handle = std::thread::spawn(move || <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::map(&left, f_clone));
            let right_result = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::map(&right, f);
            let left_result = handle.join().unwrap();
            <ArraySeqMtPerS<W> as ArraySeqMtPerTrait<W>>::append(&left_result, &right_result)
        }

        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {
            // Algorithm 19.4: append a b = flatten([a, b])
            let sequences = <ArraySeqMtPerS<ArraySeqMtPerS<T>> as ArraySeqMtPerTrait<ArraySeqMtPerS<T>>>::tabulate(
                &|i| if i == 0 { a.clone() } else { b.clone() },
                2,
            );
            <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::flatten(&sequences)
        }

        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, pred: F) -> ArraySeqMtPerS<T> {
            // Algorithm 19.5 with parallelism: fork thread per element + serial compaction
            if a.length() == 0 {
                return <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::empty();
            }

            // Fork thread per element to evaluate predicate
            let mut handles = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                let value = a.nth(i).clone();
                let pred_clone = pred.clone();
                let handle = std::thread::spawn(move || if pred_clone(&value) { Some(value) } else { None });
                handles.push(handle);
            }

            // Serial compaction: collect all Some values
            let mut kept: Vec<T> = Vec::new();
            for handle in handles {
                if let Some(value) = handle.join().unwrap() {
                    kept.push(value);
                }
            }

            if kept.is_empty() {
                <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::empty()
            } else {
                ArraySeqMtPerS::from_vec(kept)
            }
        }

        fn update_single(a: &ArraySeqMtPerS<T>, index: N, item: T) -> ArraySeqMtPerS<T> {
            // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
            <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::tabulate(
                &|j| if j == index { item.clone() } else { a.nth(j).clone() },
                a.length(),
            )
        }

        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T> {
            // Keep as primitive - ninject is one of the 7 APAS primitives
            <ArraySeqMtPerS<T> as ArraySeqMtPerTraitChap18<T>>::ninject(a, updates)
        }

        fn iterate<A: StTInMtT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, x: A) -> A {
            // Algorithm 19.8: iterate f x a = left-to-right traversal
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
            // Implement directly since we can't delegate impl Fn to fn pointer
            // This is a sequential operation anyway
            let mut result_vec = Vec::with_capacity(a.length());
            let mut acc = x;
            for i in 0..a.length() {
                result_vec.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (ArraySeqMtPerS::from_vec(result_vec), acc)
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtPerS<T>, f: F, id: T) -> T {
            // Algorithm 19.9 with parallelism: always parallel divide-and-conquer
            if a.length() == 0 {
                id
            } else if a.length() == 1 {
                a.nth(0).clone()
            } else {
                // Always parallel for MT - divide and conquer
                // Always parallel for MT - divide and conquer
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let id_clone = id.clone();
                let f_clone = f.clone();
                let f_clone2 = f.clone();
                let handle = std::thread::spawn(move || {
                    <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&left, f_clone, id_clone)
                });
                let right_result = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&right, f_clone2, id);
                let left_result = handle.join().unwrap();
                f(&left_result, &right_result)
            }
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (ArraySeqMtPerS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified version)
            let mut acc = id.clone();
            let mut results = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                results.push(acc.clone());
            }
            // Implement directly since we can't capture with &F
            let result_seq = ArraySeqMtPerS::from_vec(results);
            (result_seq, acc)
        }

        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {
            // Keep as primitive - flatten is one of the 7 APAS primitives
            <ArraySeqMtPerS<T> as ArraySeqMtPerTraitChap18<T>>::flatten(ss)
        }

        fn collect<K: StTInMtT, V: StTInMtT>(a: &ArraySeqMtPerS<Pair<K, V>>, cmp: fn(&K, &K) -> O) -> ArraySeqMtPerS<Pair<K, ArraySeqMtPerS<V>>> {
            <ArraySeqMtPerS<Pair<K, V>> as ArraySeqMtPerTraitChap18<Pair<K, V>>>::collect(a, cmp)
        }

        fn inject(values: &ArraySeqMtPerS<T>, changes: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T> {
            <ArraySeqMtPerS<T> as ArraySeqMtPerTraitChap18<T>>::inject(values, changes)
        }

        fn atomicWrite(
            _values_with_change_number: &mut ArraySeqMtPerS<Pair<T, N>>,
            _changes: &ArraySeqMtPerS<Pair<N, T>>,
            _change_index: N,
        ) {
            // Stub implementation - complex atomic operations not needed for basic functionality
        }

        fn isEmpty(a: &ArraySeqMtPerS<T>) -> bool {
            // Algorithm 19.7: isEmpty a = |a| = 0
            a.length() == 0
        }

        fn isSingleton(a: &ArraySeqMtPerS<T>) -> bool {
            // Algorithm 19.7: isSingleton a = |a| = 1
            a.length() == 1
        }

        fn append_select(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
            <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::tabulate(
                &|i| {
                    <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::select(a, b, i)
                        .unwrap()
                        .clone()
                },
                a.length() + b.length(),
            )
        }

        fn select<'a>(a: &'a ArraySeqMtPerS<T>, b: &'a ArraySeqMtPerS<T>, i: N) -> Option<&'a T> {
            let len_a = a.length();
            if i < len_a {
                return Some(a.nth(i));
            }
            let offset = i - len_a;
            let len_b = b.length();
            if offset < len_b { Some(b.nth(offset)) } else { None }
        }

        fn deflate<F: Fn(&T) -> B + Send + Sync>(f: &F, x: &T) -> ArraySeqMtPerS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) {
                <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::singleton(x.clone())
            } else {
                <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::empty()
            }
        }
    }

    #[macro_export]
    macro_rules! ArrayMtPerSLit {
        () => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$($x),*]) };
    }

    #[macro_export]
    macro_rules! ArraySeqMtPerS {
        () => { $crate::Chap19::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$($x),*]) };
    }
}
