//! Chapter 18 algorithms for `AVLTreeSeqEphS<T>` (ephemeral variant).

pub mod AVLTreeSeqEphChap18 {
use crate::Types::Types::*;
use crate::AVLTreeSeqEph::AVLTreeSeqEph::*;
use std::fmt::{Debug, Display};

pub trait AVLTreeSeqEphChap18Trait<T: StT> {
    /// Build a tree by inserting f(i) for i = 0..n-1.
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    fn tabulate(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>
    /// Map over in-order traversal, inserting results into a new tree.
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn map<U: StT>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>
    /// Append (union) of two trees.
    /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
    fn append(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> AVLTreeSeqEphS<T>;

    /// Keep elements x where pred(x) is True.
    /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
    fn filter(a: &AVLTreeSeqEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqEphS<T>;
}

impl AVLTreeSeqEphChap18Trait for AVLTreeSeqEphS<T> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeSeqEphS<T>
    where
        T: Ord + Copy + Debug + Display,
    {
        let mut t: AVLTreeSeqEphS<T> = <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::empty();
        for i in 0..n { t.push_back(f(i)); }
        t
    }
    fn map<T, U>(a: &AVLTreeSeqEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeSeqEphS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display,
    {
        let mut out: AVLTreeSeqEphS<U> = <AVLTreeSeqEphS<U> as AVLTreeSeqEphTrait<U>>::empty();
        for x in a.iter() { out.push_back(f(x)); }
        out
    }
    fn append(a: &AVLTreeSeqEphS<T>, b: &AVLTreeSeqEphS<T>) -> AVLTreeSeqEphS<T> {
        let mut out: AVLTreeSeqEphS<T> = <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::empty();
        for x in a.iter() { out.push_back(*x); }
        for x in b.iter() { if out.contains_value(x) == B::False { out.push_back(*x); } }
        out
    }
    fn filter(a: &AVLTreeSeqEphS<T>, pred: impl Fn(&T) -> B) -> AVLTreeSeqEphS<T> {
        let mut out: AVLTreeSeqEphS<T> = <AVLTreeSeqEphS<T> as AVLTreeSeqEphTrait<T>>::empty();
        for x in a.iter() { if pred(x) == B::True { out.push_back(*x); } }
        out
    }
}

}

pub use AVLTreeSeqEphChap18::AVLTreeSeqEphChap18Trait;


