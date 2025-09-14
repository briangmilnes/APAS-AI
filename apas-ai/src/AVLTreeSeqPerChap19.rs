//! Chapter 19 algorithms for AVLTreeSeqPerS.

pub mod AVLTreeSeqPerChap19 {
use crate::AVLTreeSeqPer::AVLTreeSeqPer::*;
use crate::AVLTreeSeqPerChap18::AVLTreeSeqPerChap18Trait;
use crate::Types::Types::*;
use std::fmt::{Debug, Display};

pub trait AVLTreeSeqPerChap19Trait {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>
    where
        T: Ord + Copy + Debug + Display;
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display;
    fn select<T>(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug + Display;
    fn append<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        b: &AVLTreeSeqPerS<T>,
    ) -> AVLTreeSeqPerS<T>;
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T>;
    fn filter<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        f: impl Fn(&T) -> B,
    ) -> AVLTreeSeqPerS<T>;
}

impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqPerChap19Trait for AVLTreeSeqPerS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>
    where
        T: Ord + Copy + Debug + Display,
    {
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait>::tabulate(f, n)
    }
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display,
    {
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait>::map(a, f)
    }
    fn select<T>(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug + Display,
    {
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
    fn append<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        b: &AVLTreeSeqPerS<T>,
    ) -> AVLTreeSeqPerS<T> {
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait>::append(a, b)
    }
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqPerS<T> {
        if f(x) == B::True {
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::singleton(*x)
        } else {
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::empty()
        }
    }
    fn filter<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        f: impl Fn(&T) -> B,
    ) -> AVLTreeSeqPerS<T> {
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerChap18Trait>::filter(a, f)
    }
}

}

pub use AVLTreeSeqPerChap19::AVLTreeSeqPerChap19Trait;
