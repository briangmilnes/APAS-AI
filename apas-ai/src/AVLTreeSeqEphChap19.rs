//! Chapter 19 algorithms for `AVLTreeSeqEphS<T>`.

pub mod AVLTreeSeqEphChap19 {

    use std::fmt::{Debug, Display};

    use crate::AVLTreeSeqEph::AVLTreeSeqEph::*;
    use crate::AVLTreeSeqEphChap18::AVLTreeSeqEphChap18Trait;
    use crate::Types::Types::*;

    pub trait AVLTreeSeqEphChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>;
        fn map<U: StT>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>;
        fn select(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>, i: N) -> Option<T>;
        fn append(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> AVLTreeSeqEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqEphS<T>;
        fn filter(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqEphS<T>;
    }

    impl<T: StT> AVLTreeSeqEphChap19Trait<T> for AVLTreeSeqEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T> {
            <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U> {
            <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::map(a, f)
        }
        fn select(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>, i: N) -> Option<T> {
            let a_len = <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::length(a);
            if i < a_len {
                Some(*<AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::nth(a, i))
            } else {
                let off = i - a_len;
                let b_len = <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::length(b);
                if off < b_len {
                    Some(*<AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::nth(b, off))
                } else {
                    None
                }
            }
        }
        fn append(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> AVLTreeSeqEphS<T> {
            <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqEphS<T> {
            if f(x) == B::True {
                <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::singleton(*x)
            } else {
                <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::empty()
            }
        }
        fn filter(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqEphS<T> {
            <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::filter(a, f)
        }
    }
}

pub use AVLTreeSeqEphChap19::AVLTreeSeqEphChap19Trait;
