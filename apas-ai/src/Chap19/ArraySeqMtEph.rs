//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph {
    use std::sync::Mutex;

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Mutex<Box<[T]>>` (ephemeral/mutable MT variant).
    #[derive(Debug)]
    pub struct ArraySeqMtEphS<T: StT> {
        data: Mutex<Box<[T]>>,
    }

    pub trait ArraySeqMtEphTrait<T: StT> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>
        where
            T: Clone;
        fn empty() -> ArraySeqMtEphS<T>;
        fn singleton(item: T) -> ArraySeqMtEphS<T>;
        fn length(&self) -> N;
        fn nth_cloned(&self, index: N) -> T;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T>;

        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, T);
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;
    }

    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtEphS<T>
        where
            T: Clone,
        {
            ArraySeqMtEphTrait::new(length, init_value)
        }

        fn empty() -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::empty() }

        fn singleton(item: T) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::singleton(item) }

        fn length(&self) -> N { ArraySeqMtEphTrait::length(self) }

        fn nth_cloned(&self, index: N) -> T { ArraySeqMtEphTrait::nth_cloned(self, index) }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphTrait::subseq_copy(self, start, length)
        }

        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::tabulate(f, n) }

        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {
            ArraySeqMtEphTrait::map(a, f)
        }

        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {
            let len_a = ArraySeqMtEphTrait::length(a);
            if index < len_a {
                return Some(ArraySeqMtEphTrait::nth_cloned(a, index));
            }
            let offset = index - len_a;
            let len_b = ArraySeqMtEphTrait::length(b);
            if offset < len_b {
                Some(ArraySeqMtEphTrait::nth_cloned(b, offset))
            } else {
                None
            }
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphTrait::append(a, b)
        }

        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphTrait::append(a, b)
        }

        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {
            if f(x) == B::True {
                ArraySeqMtEphTrait::singleton(x.clone())
            } else {
                ArraySeqMtEphTrait::empty()
            }
        }

        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {
            ArraySeqMtEphTrait::filter(a, pred)
        }

        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {
            for i in 0..ArraySeqMtEphTrait::length(a) {
                x = f(&x, &ArraySeqMtEphTrait::nth_cloned(a, i));
            }
            x
        }

        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {
            for i in 0..ArraySeqMtEphTrait::length(a) {
                id = f(&id, &ArraySeqMtEphTrait::nth_cloned(a, i));
            }
            id
        }

        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<T>, T) {
            let n = ArraySeqMtEphTrait::length(a);
            let mut out = if n == 0 {
                ArraySeqMtEphTrait::empty()
            } else {
                ArraySeqMtEphS::from_vec(vec![id.clone(); n])
            };
            for i in 0..n {
                let _ = out.set(i, id.clone());
                id = f(&id, &ArraySeqMtEphTrait::nth_cloned(a, i));
            }
            (out, id)
        }

        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> { ArraySeqMtEphTrait::flatten(s) }
    }
}
