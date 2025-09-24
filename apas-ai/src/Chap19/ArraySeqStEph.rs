//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqStEph<T>`.

pub mod ArraySeqStEph {
    use crate::Types::Types::*;

    use crate::Chap18::ArraySeq::ArraySeq::{ArrayS, ArraySeq};

    pub type ArraySeqStEphS<T> = ArrayS<T>;

    pub trait ArraySeqStEphTrait<T: StT> {
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>
        where
            T: Clone;
        fn empty() -> ArraySeqStEphS<T>;
        fn singleton(item: T) -> ArraySeqStEphS<T>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T>;

        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T);
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;
    }

    impl<T: StT> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqStEphS<T>
        where
            T: Clone,
        {
            ArraySeq::new(length, init_value)
        }

        fn empty() -> ArraySeqStEphS<T> { ArraySeq::empty() }

        fn singleton(item: T) -> ArraySeqStEphS<T> { ArraySeq::singleton(item) }

        fn length(&self) -> N { ArraySeq::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeq::nth(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqStEphS<T> { ArraySeq::subseq_copy(self, start, length) }

        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> { ArraySeq::tabulate(f, n) }

        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> { ArraySeq::map(a, f) }

        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: N) -> Option<T> {
            let len_a = ArraySeq::length(a);
            if index < len_a {
                return Some(ArraySeq::nth(a, index).clone());
            }
            let offset = index - len_a;
            let len_b = ArraySeq::length(b);
            if offset < len_b {
                Some(ArraySeq::nth(b, offset).clone())
            } else {
                None
            }
        }

        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> { ArraySeq::append(a, b) }

        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> { ArraySeq::append(a, b) }

        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {
            if f(x) == B::True {
                ArraySeq::singleton(x.clone())
            } else {
                ArraySeq::empty()
            }
        }

        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> { ArraySeq::filter(a, pred) }

        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { ArraySeq::iterate(a, f, x) }

        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { ArraySeq::reduce(a, f, id) }

        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T) {
            ArraySeq::scan(a, f, id)
        }

        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> { ArraySeq::flatten(s) }
    }
}
