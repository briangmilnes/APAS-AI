//! Chapter 18 algorithms for AVLTreePerS.

pub mod AVLTreeSeqPerChap18 {
use crate::AVLTreeSeqPer::AVLTreeSeqPer::*;
use crate::Types::Types::*;
use std::fmt::{Debug, Display};

pub trait AVLTreeSeqPerChap18Trait {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>
    where
        T: Ord + Copy + Debug + Display;
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display;
    /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
    fn append<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        b: &AVLTreeSeqPerS<T>,
    ) -> AVLTreeSeqPerS<T>;
    /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
    fn filter<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        pred: impl Fn(&T) -> B,
    ) -> AVLTreeSeqPerS<T>;
}

impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqPerChap18Trait for AVLTreeSeqPerS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>
    where
        T: Ord + Copy + Debug + Display,
    {
        let mut v: Vec<T> = Vec::with_capacity(n);
        for i in 0..n {
            v.push(f(i));
        }
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::from_vec(v)
    }
    fn map<T, U>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display,
    {
        let vals = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(a);
        let mapped: Vec<U> = vals.iter().map(|x| f(x)).collect();
        <AVLTreeSeqPerS<U> as AVLTreeSeqPerTrait<U>>::from_vec(mapped)
    }
    fn append<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        b: &AVLTreeSeqPerS<T>,
    ) -> AVLTreeSeqPerS<T> {
        let mut vals = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(a);
        let bv = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(b);
        for x in bv {
            if !vals.contains(&x) {
                vals.push(x);
            }
        }
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::from_vec(vals)
    }
    fn filter<T: Ord + Copy + Debug + Display>(
        a: &AVLTreeSeqPerS<T>,
        pred: impl Fn(&T) -> B,
    ) -> AVLTreeSeqPerS<T> {
        let vals = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(a);
        let mut out: Vec<T> = Vec::new();
        for x in vals.iter() {
            if pred(x) == B::True {
                out.push(*x);
            }
        }
        <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::from_vec(out)
    }
}

}

pub use AVLTreeSeqPerChap18::AVLTreeSeqPerChap18Trait;
