//! Chapter 19 algorithms for `AVLTreeSeqStEphS<T>`.

pub mod AVLTreeSeqStEph {

    use std::fmt::{Debug, Display};

    // Self-import removed;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    pub trait AVLTreeSeqStEphTrait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T>;
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T>;
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;
    }

    impl<T: StT> AVLTreeSeqStEphTrait<T> for AVLTreeSeqStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::map(a, f)
        }
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T> {
            let a_len = <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::length(a);
            if i < a_len {
                Some(<AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::nth(a, i).clone())
            } else {
                let off = i - a_len;
                let b_len = <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::length(b);
                if off < b_len {
                    Some(<AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::nth(b, off).clone())
                } else {
                    None
                }
            }
        }
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T> {
            if f(x) == B::True {
                <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::singleton(x.clone())
            } else {
                <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::empty()
            }
        }
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::filter(a, f)
        }
    }
}
