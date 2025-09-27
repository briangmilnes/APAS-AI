//! Chapter 19 algorithms for `AVLTreeSeqStEphS<T>`.

pub mod AVLTreeSeqStEphChap19 {

    use std::fmt::{Debug, Display};

    use crate::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::AVLTreeSeqStEphChap18::AVLTreeSeqStEphChap18Trait;
    use crate::Types::Types::*;

    pub trait AVLTreeSeqStEphChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;
        fn select(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>, i: N) -> Option<T>;
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T>;
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;
    }

    impl<T: StT> AVLTreeSeqStEphChap19Trait<T> for AVLTreeSeqStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphChap18Trait<T>>::map(a, f)
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
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStEphS<T> {
            if f(x) == true {
                <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::singleton(x.clone())
            } else {
                <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::empty()
            }
        }
        fn filter(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {
            <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphChap18Trait<T>>::filter(a, f)
        }
    }
}

pub use AVLTreeSeqStEphChap19::AVLTreeSeqStEphChap19Trait;
