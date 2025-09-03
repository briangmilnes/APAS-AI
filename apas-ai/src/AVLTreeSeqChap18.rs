//! Chapter 18 algorithms as trait methods over `AVLTreeS<T>`.
//!
//! Abstract:
//! - Defines trait `AVLTreeSeqChap18` with sequential operations over `AVLTreeS<T>`.
//! - Interprets the tree as an in-order sequence; implementations use inserts/deletes.
//! - Complexity reflects tree reinsertion costs; suitable for tests and small benches.

use crate::Types::{B, N, O};
use crate::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
use tree_collections::commonTrait::CommonTreeTrait;
use std::fmt::Debug;

pub trait AVLTreeSeqChap18 {
    /// Build a tree by inserting f(i) for i = 0..n-1. <br/>
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    /// BUG: Our impl inserts into a tree → Work adds Θ(n·lg(n)).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>
    where
        T: Ord + Copy + Debug;

    /// Map over in-order traversal, inserting results into a new tree. <br/>
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    /// BUG: Our impl inserts results into a tree → extra Θ(|a|·lg(|a|)).
    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>
    where
        T: Ord + Copy + Debug,
        U: Ord + Copy + Debug;

    /// Append (union) of two trees. <br/>
    /// APAS: Θ(1 + |lg(|a|/|b|)|) for arrays; tree union differs. <br/>
    /// BUG: Our impl reinserts all elements: Θ((|a|+|b|)·lg(|a|+|b|)).
    fn append<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T>;

    /// Keep elements x where pred(x) is True. <br/>
    /// APAS: Work Θ(1 + Σ x∈a W(pred(x))), Span Θ(1 + lg(|a|) + max x∈a S(pred(x))).
    /// BUG: Our impl inserts survivors into a tree → extra Θ(|a|·lg(|a|)).
    fn filter<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, pred: impl Fn(&T) -> B) -> AVLTreeS<T>;
}

impl<T2: Ord + Copy + Debug> AVLTreeSeqChap18 for AVLTreeS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>
    where
        T: Ord + Copy + Debug,
    {
        let mut t: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
        for i in 0..n { t.insert_value(f(i)); }
        t
    }

    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>
    where
        T: Ord + Copy + Debug,
        U: Ord + Copy + Debug,
    {
        let mut out: AVLTreeS<U> = <AVLTreeS<U> as AVLTreeSeq<U>>::empty();
        let vals = a.values_in_order();
        for x in vals.iter() { out.insert_value(f(x)); }
        out
    }

    fn append<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T> {
        let mut out: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
        for x in a.values_in_order().iter() { out.insert_value(*x); }
        for x in b.values_in_order().iter() {
            if out.contains_value(x) == B::False { out.insert_value(*x); }
        }
        out
    }

    fn filter<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, pred: impl Fn(&T) -> B) -> AVLTreeS<T> {
        let mut out: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
        for x in a.values_in_order().iter() { if pred(x) == B::True { out.insert_value(*x); } }
        out
    }
}


