//! Chapter 19 algorithms for AVLTreeSeqPerS.

pub mod AVLTreeSeqPerChap19 {
    use crate::AVLTreeSeqPer::AVLTreeSeqPer::*;
    use crate::AVLTreeSeqPerChap18::AVLTreeSeqPerChap18Trait;
    use crate::Types::Types::*;
    use std::fmt::{Debug, Display};

    pub trait AVLTreeSeqPerChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>;
        fn map<U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>;
        fn select(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>;
        fn append(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>) -> AVLTreeSeqPerS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T>;
        fn filter(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqPerS<T>;
    }

    impl<T: StT> AVLTreeSeqPerChap19Trait<T> for AVLTreeSeqPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T> {
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U> {
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait<T>>::map(a, f)
        }
        fn select(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T> {
            let a_len = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::length(a);
            if i < a_len {
                Some(*<AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::nth(a, i))
            } else {
                let off = i - a_len;
                let b_len = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::length(b);
                if off < b_len {
                    Some(*<AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::nth(b, off))
                } else {
                    None
                }
            }
        }
        fn append(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>) -> AVLTreeSeqPerS<T> {
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T> {
            if f(x) == B::True {
                <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::singleton(*x)
            } else {
                <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::empty()
            }
        }
        fn filter(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqPerS<T> {
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait<T>>::filter(a, f)
        }
    }
}

pub use AVLTreeSeqPerChap19::AVLTreeSeqPerChap19Trait;
