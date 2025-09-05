//! Chapter 18 algorithms as trait methods over `AVLTreeS<T>`.
//!
//! Abstract:
//! - Defines trait `AVLTreeSeqChap18` with sequential operations over `AVLTreeS<T>`.
//! - Interprets the tree as an in-order sequence; implementations use inserts/deletes.
//! - Complexity reflects tree reinsertion costs; suitable for tests and small benches.

use crate::Types::{B, N, O};
use crate::AVLTreeSeqEph::{AVLTreeEphS, AVLTreeSeqEphTrait};
//  as AVLTreeSeq};
use tree_collections::commonTrait::CommonTreeTrait;
use std::fmt::{Debug, Display};

pub trait AVLTreeSeqChap18Trait {
    /// Build a tree by inserting f(i) for i = 0..n-1.
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>
    where
        T: Ord + Copy + Debug + Display;

    /// Map over in-order traversal, inserting results into a new tree.
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display;

    /// Append (union) of two trees.
    /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T>;

    /// Keep elements x where pred(x) is True.
    /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, pred: impl Fn(&T) -> B) -> AVLTreeS<T>;
}

impl<T2: Ord + Copy + Debug + Display> AVLTreeSeqChap18 for AVLTreeS<T2> {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>
    where
        T: Ord + Copy + Debug + Display,
    {
        let mut t: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeqEph<T>>::empty();
        for i in 0..n { t.insert_value(f(i)); }
        t
    }

    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display,
    {
        let mut out: AVLTreeS<U> = <AVLTreeS<U> as AVLTreeSeq<U>>::empty();
        for x in a.iter() { out.insert_value(f(x)); }
        out
    }

    /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T> {
        let mut out: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
        for x in a.iter() { out.insert_value(*x); }
        for x in b.iter() { if out.contains_value(x) == B::False { out.insert_value(*x); } }
        out
    }

    /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeS<T>, pred: impl Fn(&T) -> B) -> AVLTreeS<T> {
        let mut out: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
        for x in a.iter() { if pred(x) == B::True { out.insert_value(*x); } }
        out
    }
}


