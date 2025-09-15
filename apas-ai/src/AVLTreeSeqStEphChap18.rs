//! Chapter 18 algorithms for `AVLTreeSeqStEphS<T>` (ephemeral variant).

pub mod AVLTreeSeqStEphChap18 {
    use std::fmt::{Debug, Display};

    use crate::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    pub trait AVLTreeSeqStEphChap18Trait<T: StT> {
        /// Build a tree by inserting f(i) for i = 0..n-1.
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T>;
        /// Map over in-order traversal, inserting results into a new tree.
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U>;
        /// Append (union) of two trees.
        /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T>;
        /// Keep elements x where pred(x) is True.
        /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
        fn filter(a: &AVLTreeSeqStEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T>;
    }

    impl<T: StT> AVLTreeSeqStEphChap18Trait<T> for AVLTreeSeqStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqStEphS<T> {
            let mut t: AVLTreeSeqStEphS<T> = <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::empty();
            for i in 0..n {
                t.push_back(f(i));
            }
            t
        }
        fn map<U: StT>(a: &AVLTreeSeqStEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqStEphS<U> {
            let mut out: AVLTreeSeqStEphS<U> = <AVLTreeSeqStEphS<U> as AVLTreeSeqStEphTrait<U>>::empty();
            for x in a.iter() {
                out.push_back(f(x));
            }
            out
        }
        fn append(a: &AVLTreeSeqStEphS<T>, b: &AVLTreeSeqStEphS<T>) -> AVLTreeSeqStEphS<T> {
            let mut out: AVLTreeSeqStEphS<T> = <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::empty();
            for x in a.iter() {
                out.push_back(*x);
            }
            for x in b.iter() {
                if out.contains_value(x) == B::False {
                    out.push_back(*x);
                }
            }
            out
        }
        fn filter(a: &AVLTreeSeqStEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqStEphS<T> {
            let mut out: AVLTreeSeqStEphS<T> = <AVLTreeSeqStEphS<T> as AVLTreeSeqStEphTrait<T>>::empty();
            for x in a.iter() {
                if pred(x) == B::True {
                    out.push_back(*x);
                }
            }
            out
        }
    }
}

pub use AVLTreeSeqStEphChap18::AVLTreeSeqStEphChap18Trait;
