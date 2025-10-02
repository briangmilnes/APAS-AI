//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqStPer.

pub mod ArraySeqStPer {
    use crate::Chap18::ArraySeq::ArraySeq::ArraySeq;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::{
        ArraySeqStPerS as ArraySeqStPerSChap18, ArraySeqStPerTrait as ArraySeqStPerTraitChap18,
    };
    use crate::Types::Types::*;

    pub type ArraySeqStPerS<T> = ArraySeqStPerSChap18<T>;

    pub trait ArraySeqStPerTrait<T: StT> {
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N) -> &T;
        /// claude-4-sonet: Work Θ(length), Span Θ(length), Parallelism Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T>;

        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        /// claude-4-sonet: Work Θ(n + Σᵢ W(f(i))), Span Θ(n + maxᵢ S(f(i))), Parallelism Θ(1)
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        /// claude-4-sonet: Work Θ(|a| + Σₓ W(f(x))), Span Θ(|a| + maxₓ S(f(x))), Parallelism Θ(1)
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(|a| + |b|), Parallelism Θ(1)
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(|a| + |b|), Parallelism Θ(1)
        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(1 + max i S(f(a[i])))
        /// claude-4-sonet: Work Θ(|a| + Σᵢ W(f(aᵢ))), Span Θ(|a| + maxᵢ S(f(aᵢ))), Parallelism Θ(1)
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(|a| × W(f)), Span Θ(|a| × S(f)), Parallelism Θ(1)
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, x: A) -> A;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T);
        /// claude-4-sonet: Work Θ(Σᵢ |sᵢ|), Span Θ(Σᵢ |sᵢ|), Parallelism Θ(1)
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(|a| + |updates|), Parallelism Θ(1)
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(|a| + |updates|), Parallelism Θ(1)
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(a: &ArraySeqStPerS<T>) -> bool;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(a: &ArraySeqStPerS<T>) -> bool;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn update(a: &ArraySeqStPerS<T>, index: N, item: T) -> ArraySeqStPerS<T>;
    }

    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T> {
            // Keep as primitive - delegates to tabulate
            <ArraySeqStPerS<T> as ArraySeqStPerTraitChap18<T>>::new(length, init_value)
        }

        fn empty() -> ArraySeqStPerS<T> {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::tabulate(
                &|_| unreachable!("empty sequence has no elements"),
                0,
            )
        }

        fn singleton(item: T) -> ArraySeqStPerS<T> {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::tabulate(&|_| item.clone(), 1)
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
            <ArraySeqStPerS<U> as ArraySeqStPerTrait<U>>::tabulate(&|i| f(a.nth(i)), a.length())
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
            let sequences = <ArraySeqStPerS<ArraySeqStPerS<T>> as ArraySeqStPerTrait<ArraySeqStPerS<T>>>::tabulate(
                &|i| if i == 0 { a.clone() } else { b.clone() },
                2,
            );
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::flatten(&sequences)
        }

        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::tabulate(
                &|i| {
                    <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::select(a, b, i)
                        .unwrap()
                        .clone()
                },
                a.length() + b.length(),
            )
        }

        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStPerS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) == true {
                <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::singleton(x.clone())
            } else {
                <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty()
            }
        }

        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {
            // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
            let deflated = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::map(a, &|x| {
                <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::deflate(pred, x)
            });
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::flatten(&deflated)
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

        fn isEmpty(a: &ArraySeqStPerS<T>) -> bool {
            // Algorithm 19.7: isEmpty a = |a| = 0
            a.length() == 0
        }

        fn isSingleton(a: &ArraySeqStPerS<T>) -> bool {
            // Algorithm 19.7: isSingleton a = |a| = 1
            a.length() == 1
        }

        fn update(a: &ArraySeqStPerS<T>, index: N, item: T) -> ArraySeqStPerS<T> {
            // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
            <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::tabulate(
                &|j| if j == index { item.clone() } else { a.nth(j).clone() },
                a.length(),
            )
        }
    }

    #[macro_export]
    macro_rules! ArraySeqStPerSLit {
        () => { $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$($x),*]) };
    }
}
