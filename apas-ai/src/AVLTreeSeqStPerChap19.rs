//! Chapter 19 algorithms for AVLTreeSeqStPerS.

pub mod AVLTreeSeqStPerChap19 {
    use crate::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::AVLTreeSeqStPerChap18::AVLTreeSeqStPerChap18::*;
    use crate::Types::Types::*;
    use std::fmt::{Debug, Display};

    pub trait AVLTreeSeqStPerChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T>;
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T>;
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;
    }

    impl<T: StT> AVLTreeSeqStPerChap19Trait<T> for AVLTreeSeqStPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerChap18Trait<T>>::map(a, f)
        }
        fn select(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>, i: N) -> Option<T> {
            let a_len = <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::length(a);
            if i < a_len {
                Some(<AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::nth(a, i).clone())
            } else {
                let off = i - a_len;
                let b_len = <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::length(b);
                if off < b_len {
                    Some(<AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::nth(b, off).clone())
                } else {
                    None
                }
            }
        }
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqStPerS<T> {
            if f(x) == B::True {
                <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::singleton(x.clone())
            } else {
                <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::empty()
            }
        }
        fn filter(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerChap18Trait<T>>::filter(a, f)
        }
    }
}
