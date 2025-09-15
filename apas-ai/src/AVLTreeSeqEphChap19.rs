//! Chapter 19 algorithms for `AVLTreeSeqEphS<T>`.

pub mod AVLTreeSeqEphChap19 {
use crate::Types::Types::*;
use crate::AVLTreeSeqEph::AVLTreeSeqEph::*;
use crate::AVLTreeSeqEphChap18::AVLTreeSeqEphChap18Trait;
use std::fmt::{Debug, Display};

pub trait AVLTreeSeqEphChap19Trait {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>
    where
        T: Ord + Copy + Debug + Display;
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display;
    fn select<T>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug + Display;
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> AVLTreeSeqEphS<T>;
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqEphS<T>;
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqEphS<T>;
}

impl<T: Ord + Copy + Debug + Display> AVLTreeSeqEphChap19Trait for AVLTreeSeqEphS<T> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>
    where
        T: Ord + Copy + Debug + Display,
    {
        <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::tabulate(f, n)
    }
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display,
    {
        <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::map(a, f)
    }
    fn select<T>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug + Display,
    {
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
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> AVLTreeSeqEphS<T> {
        <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::append(a, b)
    }
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeSeqEphS<T> {
        if f(x) == B::True {
            <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::singleton(*x)
        } else {
            <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::empty()
        }
    }
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeSeqEphS<T> {
        <AVLTreeSeqEphS<T> as AVLTreeSeqEphChap18Trait<T>>::filter(a, f)
    }
}

}

pub use AVLTreeSeqEphChap19::AVLTreeSeqEphChap19Trait;
