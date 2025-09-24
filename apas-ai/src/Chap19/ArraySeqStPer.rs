//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqStPer.

pub mod ArraySeqStPer {
    use crate::Chap18::ArraySeq::ArraySeq::{ArrayS, ArraySeq};
    use crate::Types::Types::*;

    pub type ArraySeqStPerS<T> = ArrayS<T>;

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
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(1 + max i S(f(a[i])))
        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T>;
        fn iterate<A: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, T);
        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T>;
        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T>;
    }

    impl<T: StT> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStPerS<T>
        where
            T: Clone,
        {
            ArraySeq::new(length, init_value)
        }

        fn empty() -> ArraySeqStPerS<T> { ArraySeq::empty() }

        fn singleton(item: T) -> ArraySeqStPerS<T> { ArraySeq::singleton(item) }

        fn length(&self) -> N { ArraySeq::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeq::nth(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStPerS<T> { ArraySeq::subseq_copy(self, start, length) }

        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStPerS<T> { ArraySeq::tabulate(f, n) }

        fn map<U: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&T) -> U) -> ArraySeqStPerS<U> { ArraySeq::map(a, f) }

        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: N) -> Option<&'a T> {
            let len_a = ArraySeq::length(a);
            if i < len_a {
                return Some(ArraySeq::nth(a, i));
            }
            let offset = i - len_a;
            let len_b = ArraySeq::length(b);
            if offset < len_b {
                Some(ArraySeq::nth(b, offset))
            } else {
                None
            }
        }

        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> { ArraySeq::append(a, b) }

        fn append2(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> { ArraySeq::append(a, b) }

        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStPerS<T> {
            if f(x) == B::True {
                ArraySeq::singleton(x.clone())
            } else {
                ArraySeq::empty()
            }
        }

        fn filter(a: &ArraySeqStPerS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStPerS<T> { ArraySeq::filter(a, pred) }

        fn iterate<A: StT>(a: &ArraySeqStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { ArraySeq::iterate(a, f, x) }

        fn reduce(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { ArraySeq::reduce(a, f, id) }

        fn scan(a: &ArraySeqStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStPerS<T>, T) {
            ArraySeq::scan(a, f, id)
        }

        fn flatten(s: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> ArraySeqStPerS<T> { ArraySeq::flatten(s) }

        fn inject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            ArraySeq::inject(a, updates)
        }

        fn ninject(a: &ArraySeqStPerS<T>, updates: &ArraySeqStPerS<Pair<N, T>>) -> ArraySeqStPerS<T> {
            ArraySeq::ninject(a, updates)
        }
    }
}
