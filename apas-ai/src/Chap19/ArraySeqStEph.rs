//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqStEph<T>`.

pub mod ArraySeqStEph {

    use crate::Chap18::ArraySeq::ArraySeq::ArraySeq;
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{
        ArraySeqStEphS as ArraySeqStEphSChap18, ArraySeqStEphTrait as ArraySeqStEphTraitChap18,
    };
    use crate::Types::Types::*;

    pub type ArraySeqStEphS<T> = ArraySeqStEphSChap18<T>;

    pub trait ArraySeqStEphTrait<T: StT> {
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn new(length: N, init_value: T) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> ArraySeqStEphS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> ArraySeqStEphS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth(&self, index: N) -> &T;
        /// claude-4-sonet: Work Θ(length), Span Θ(length), Parallelism Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T>;

        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        /// claude-4-sonet: Work Θ(n + Σᵢ W(f(i))), Span Θ(n + maxᵢ S(f(i))), Parallelism Θ(1)
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        /// claude-4-sonet: Work Θ(|a| + Σₓ W(f(x))), Span Θ(|a| + maxₓ S(f(x))), Parallelism Θ(1)
        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(|a| + |b|), Parallelism Θ(1)
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(|a| + |b|), Parallelism Θ(1)
        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(1 + max i S(f(a[i])))
        /// claude-4-sonet: Work Θ(|a| + Σᵢ W(f(aᵢ))), Span Θ(|a| + maxᵢ S(f(aᵢ))), Parallelism Θ(1)
        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T>;
        /// claude-4-sonet: Work Θ(|a| × W(f)), Span Θ(|a| × S(f)), Parallelism Θ(1)
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T;
        /// claude-4-sonet: Work Θ(|a|), Span Θ(|a|), Parallelism Θ(1)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T);
        /// claude-4-sonet: Work Θ(Σᵢ |sᵢ|), Span Θ(Σᵢ |sᵢ|), Parallelism Θ(1)
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(a: &ArraySeqStEphS<T>) -> bool;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(a: &ArraySeqStEphS<T>) -> bool;
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn update(a: &ArraySeqStEphS<T>, index: N, item: T) -> ArraySeqStEphS<T>;
        /// Inject updates into base sequence. Updates is a vector of (index, value) pairs.
        /// If multiple updates target the same index, the last update wins.
        /// APAS: Work Θ(|base| + |updates|), Span Θ(|base| + |updates|)
        /// claude-4-sonet: Work Θ(|base| + |updates|), Span Θ(|base| + |updates|), Parallelism Θ(1)
        fn inject(base: &ArraySeqStEphS<T>, updates: &[(N, T)]) -> ArraySeqStEphS<T>;
    }

    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T> {
            // Keep as primitive - delegates to tabulate
            <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::new(length, init_value)
        }

        fn empty() -> ArraySeqStEphS<T> {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
            <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::tabulate(
                &|_| unreachable!("empty sequence has no elements"),
                0,
            )
        }

        fn singleton(item: T) -> ArraySeqStEphS<T> {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
            <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::tabulate(&|_| item.clone(), 1)
        }

        fn length(&self) -> N { ArraySeqStEphTraitChap18::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeqStEphTraitChap18::nth(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T> {
            // Keep as primitive - subseq is one of the 7 APAS primitives
            <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::subseq(self, start, length)
        }

        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> ArraySeqStEphS<T> {
            // Keep as primitive - tabulate is one of the 7 APAS primitives
            if n == 0 {
                return <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::empty();
            }
            let mut result = <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::new(n, f(0));
            for i in 1..n {
                result.set(i, f(i)).unwrap();
            }
            result
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> ArraySeqStEphS<U> {
            // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
            <ArraySeqStEphS<U> as ArraySeqStEphTrait<U>>::tabulate(&|i| f(a.nth(i)), a.length())
        }

        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T> {
            let len_a = a.length();
            if index < len_a {
                return Some(a.nth(index).clone());
            }
            let offset = index - len_a;
            let len_b = b.length();
            if offset < len_b {
                Some(b.nth(offset).clone())
            } else {
                None
            }
        }

        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            // Algorithm 19.4: append a b = flatten([a, b])
            let total_len = a.length() + b.length();
            if total_len == 0 {
                return <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty();
            }
            let first_elem = if a.length() > 0 {
                a.nth(0).clone()
            } else {
                b.nth(0).clone()
            };
            let mut result = <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::new(total_len, first_elem);
            for i in 1..a.length() {
                result.set(i, a.nth(i).clone()).unwrap();
            }
            for i in 0..b.length() {
                result.set(a.length() + i, b.nth(i).clone()).unwrap();
            }
            result
        }

        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
            <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::tabulate(
                &|i| <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::select(a, b, i).unwrap(),
                a.length() + b.length(),
            )
        }

        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> ArraySeqStEphS<T> {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::singleton(x.clone())
            } else {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty()
            }
        }

        fn filter<F: Fn(&T) -> B>(a: &ArraySeqStEphS<T>, pred: &F) -> ArraySeqStEphS<T> {
            // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
            let deflated = <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::map(a, &|x| {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::deflate(pred, x)
            });
            <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::flatten(&deflated)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A {
            // Algorithm 19.8: iterate f x a = left-to-right traversal
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {
            // Algorithm 19.9: reduce using divide-and-conquer
            if a.length() == 0 {
                id
            } else if a.length() == 1 {
                a.nth(0).clone()
            } else {
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let left_result = <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::reduce(&left, f, id.clone());
                let right_result = <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::reduce(&right, f, id);
                f(&left_result, &right_result)
            }
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified version)
            if a.length() == 0 {
                return (<ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty(), id);
            }
            let mut acc = id.clone();
            acc = f(&acc, a.nth(0));
            let mut result_seq = <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::new(a.length(), acc.clone());
            for i in 1..a.length() {
                acc = f(&acc, a.nth(i));
                result_seq.set(i, acc.clone()).unwrap();
            }
            (result_seq, acc)
        }

        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {
            // Keep as primitive - flatten is one of the 7 APAS primitives
            <ArraySeqStEphS<T> as ArraySeqStEphTraitChap18<T>>::flatten(s)
        }

        fn isEmpty(a: &ArraySeqStEphS<T>) -> bool {
            // Algorithm 19.7: isEmpty a = |a| = 0
            a.length() == 0
        }

        fn isSingleton(a: &ArraySeqStEphS<T>) -> bool {
            // Algorithm 19.7: isSingleton a = |a| = 1
            a.length() == 1
        }

        fn update(a: &ArraySeqStEphS<T>, index: N, item: T) -> ArraySeqStEphS<T> {
            // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
            <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::tabulate(
                &|j| if j == index { item.clone() } else { a.nth(j).clone() },
                a.length(),
            )
        }

        fn inject(base: &ArraySeqStEphS<T>, updates: &[(N, T)]) -> ArraySeqStEphS<T> {
            // Used in Algorithm 62.3 (Star Partition)
            // Create a mutable copy of the base sequence
            let mut result = base.clone();
            // Apply each update
            for (index, value) in updates.iter() {
                if *index < result.length() {
                    let _ = result.set(*index, value.clone());
                }
            }
            result
        }
    }

    #[macro_export]
    macro_rules! ArraySeqStEphSLit {
        () => { $crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS::from_vec(vec![$($x),*]) };
    }
}
