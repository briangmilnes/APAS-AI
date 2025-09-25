//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqStPer.

pub mod ArraySeqStPer {
    use crate::Chap18::ArraySeq::ArraySeq::ArraySeq;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS as ArraySeqStPerSChap18, ArraySeqStPerTrait as ArraySeqStPerTraitChap18};
    use crate::Types::Types::*;

    pub type ArraySeqStPerS<T> = ArraySeqStPerSChap18<T>;

    pub trait ArraySeqStPerTrait<T: StT> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>
        where
            T: Clone;
        fn empty() -> ArraySeqStPerS<T>;
        fn singleton(item: T) -> ArraySeqStPerS<T>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T>;

        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(1 + max i S(f(a[i])))
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T>;
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T);
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
    }

    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>
        where
            T: Clone,
        {
            // Keep as primitive - delegates to tabulate
            <ArraySeqStPerS<T> as ArraySeqStPerTraitChap18<T>>::new(length, init_value)
        }

        fn empty() -> ArraySeqStPerS<T> { 
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::tabulate(&|_| unreachable!("empty sequence has no elements"), 0)
        }

        fn singleton(item: T) -> ArraySeqStPerS<T> {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            // Use direct delegation to Chap18 since we can't capture with fn pointers
            ArraySeqStPerS::from_vec(vec![item])
        }

        fn length(&self) -> N { ArraySeqStPerTraitChap18::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeqStPerTraitChap18::nth(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T> {
            // Keep as primitive - subseq is one of the 7 APAS primitives
            <ArraySeqStPerS<T> as ArraySeqStPerTraitChap18<T>>::subseq_copy(self, start, length)
        }

        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T> {
            // Keep as primitive - tabulate is one of the 7 APAS primitives
            // Implement directly to handle closures (can't delegate to Chap18 fn pointers)
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {
            // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
            // Implement directly since we can't capture with &F
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T> {
            let len_a = a.length();
            if i < len_a {
                return Some(a.nth(i));
            }
            let offset = i - len_a;
            let len_b = b.length();
            if offset < len_b {
                Some(b.nth(offset))
            } else {
                None
            }
        }

        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            // Algorithm 19.4: append a b = flatten([a, b])
            // Implement directly since we can't capture with &F
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for i in 0..b.length() {
                values.push(b.nth(i).clone());
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            // Alternative append using tabulate with select helper
            // Implement directly since we can't capture with &F
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..(a.length() + b.length()) {
                if i < a.length() {
                    values.push(a.nth(i).clone());
                } else {
                    values.push(b.nth(i - a.length()).clone());
                }
            }
            ArraySeqStPerS::from_vec(values)
        }

        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) == B::True {
                <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::singleton(x.clone())
            } else {
                <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty()
            }
        }

        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {
            // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
            // Implement directly since we can't capture with &F
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) == B::True {
                    kept.push(value.clone());
                }
            }
            ArraySeqStPerS::from_vec(kept)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A {
            // Algorithm 19.8: iterate f x a = left-to-right traversal
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {
            // Algorithm 19.9: reduce using divide-and-conquer
            if a.length() == 0 {
                id
            } else if a.length() == 1 {
                a.nth(0).clone()
            } else {
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let left_result = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::reduce(&left, f, id.clone());
                let right_result = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::reduce(&right, f, id);
                f(&left_result, &right_result)
            }
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified version)
            let mut acc = id.clone();
            let mut results = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                results.push(acc.clone());
            }
            // Implement directly since we can't capture with &F
            let result_seq = ArraySeqStPerS::from_vec(results);
            (result_seq, acc)
        }

        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> {
            // Keep as primitive - flatten is one of the 7 APAS primitives
            <ArraySeqStPerS<T> as ArraySeqStPerTraitChap18<T>>::flatten(s)
        }

        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            // Keep as primitive - inject is one of the 7 APAS primitives
            <ArraySeqStPerS<T> as ArraySeqStPerTraitChap18<T>>::inject(a, updates)
        }

        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            // Keep as primitive - ninject is one of the 7 APAS primitives
            <ArraySeqStPerS<T> as ArraySeqStPerTraitChap18<T>>::ninject(a, updates)
        }
    }
}