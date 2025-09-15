//! Chapter 18 algorithms for AVLTreePerS.

pub mod AVLTreeSeqPerChap18 {
    use crate::AVLTreeSeqPer::AVLTreeSeqPer::*;
    use crate::Types::Types::*;
    use std::fmt::{Debug, Display};

    pub trait AVLTreeSeqPerChap18Trait<T: StT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
        fn map<U: StT>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U>;
        /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
        fn append(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>) -> AVLTreeSeqPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
        fn filter(a: &AVLTreeSeqPerS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqPerS<T>;
    }

    impl<T: StT> AVLTreeSeqPerChap18Trait<T> for AVLTreeSeqPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqPerS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::from_vec(v)
        }
        fn map<U: StT>(a: &AVLTreeSeqPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqPerS<U> {
            let vals = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(a);
            let mapped: Vec<U> = vals.iter().map(|x| f(x)).collect();
            <AVLTreeSeqPerS<U> as AVLTreeSeqPerTrait<U>>::from_vec(mapped)
        }
        fn append(a: &AVLTreeSeqPerS<T>, b: &AVLTreeSeqPerS<T>) -> AVLTreeSeqPerS<T> {
            let mut vals = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(a);
            let bv = <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::values_in_order(b);
            for x in bv {
                if !vals.contains(&x) {
                    vals.push(x);
                }
            }
            <AVLTreeSeqPerS<T> as AVLTreeSeqPerTrait<T>>::from_vec(vals)
        }
        fn filter(a: &AVLTreeSeqPerS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqPerS<T> {
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
