//! Chapter 18 algorithms for AVLTreeStPerS.

pub mod AVLTreeSeqStPerChap18 {
    use crate::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Types::Types::*;
    use std::fmt::{Debug, Display};

    pub trait AVLTreeSeqStPerChap18Trait<T: StT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U>;
        /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
        fn filter(a: &AVLTreeSeqStPerS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T>;
    }

    impl<T: StT> AVLTreeSeqStPerChap18Trait<T> for AVLTreeSeqStPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStPerS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::from_vec(v)
        }
        fn map<U: StT>(a: &AVLTreeSeqStPerS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStPerS<U> {
            let vals = <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::values_in_order(a);
            let mapped: Vec<U> = vals.iter().map(|x| f(x)).collect();
            <AVLTreeSeqStPerS<U> as AVLTreeSeqStPerTrait<U>>::from_vec(mapped)
        }
        fn append(a: &AVLTreeSeqStPerS<T>, b: &AVLTreeSeqStPerS<T>) -> AVLTreeSeqStPerS<T> {
            let mut vals = <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::values_in_order(a);
            let bv = <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::values_in_order(b);
            for x in bv {
                if !vals.contains(&x) {
                    vals.push(x);
                }
            }
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::from_vec(vals)
        }
        fn filter(a: &AVLTreeSeqStPerS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStPerS<T> {
            let vals = <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::values_in_order(a);
            let mut out: Vec<T> = Vec::new();
            for x in vals.iter() {
                if pred(x) == B::True {
                    out.push(x.clone());
                }
            }
            <AVLTreeSeqStPerS<T> as AVLTreeSeqStPerTrait<T>>::from_vec(out)
        }
    }
}
