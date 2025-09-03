//! Chapter 19 algorithms as trait methods over `AVLTreeS<T>`.
//!
//! Abstract:
//! - Defines trait `AVLTreeSeqChap19` mirroring array/list Chapter 19 APIs for AVL trees.
//! - Implementations delegate to Chapter 18 style sequential constructions.

use crate::Types::{B, N, O};
use crate::AVLTreeSeq::{AVLTreeS, AVLTreeSeq};
use crate::AVLTreeSeqChap18::AVLTreeSeqChap18;
use std::fmt::Debug;

pub trait AVLTreeSeqChap19 {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    /// BUG: Our impl inserts into a tree → Work adds Θ(n·lg(n)).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>
    where
        T: Ord + Copy + Debug;
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    /// BUG: Our impl inserts results into a tree → extra Θ(|a|·lg(|a|)).
    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>
    where
        T: Ord + Copy + Debug,
        U: Ord + Copy + Debug;
    /// APAS (array): Θ(1). For trees, requires rank/select support. <br/>
    /// BUG: Ours uses nth (materializes inorder) thus Θ(n) effective.
    fn select<'a, T>(a: &'a AVLTreeS<T>, b: &'a AVLTreeS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug;
    /// APAS: Θ(1 + |lg(|a|/|b|)|) for arrays; tree union differs. <br/>
    /// BUG: Our impl reinserts all elements: Θ((|a|+|b|)·lg(|a|+|b|)).
    fn append<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T>;
    /// APAS: Θ(1). BUG: ours is Θ(1) but semantically different (set semantics).
    fn deflate<T: Ord + Copy + Debug>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeS<T>;
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    /// BUG: Our impl inserts survivors into a tree → extra Θ(|a|·lg(|a|)).
    fn filter<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, f: impl Fn(&T) -> B) -> AVLTreeS<T>;
}

impl<T2: Ord + Copy + Debug> AVLTreeSeqChap19 for AVLTreeS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeS<T>
    where
        T: Ord + Copy + Debug,
    { <AVLTreeS<T> as AVLTreeSeqChap18>::tabulate(f, n) }

    fn map<T, U>(a: &AVLTreeS<T>, f: impl Fn(&T) -> U) -> AVLTreeS<U>
    where
        T: Ord + Copy + Debug,
        U: Ord + Copy + Debug,
    { <AVLTreeS<T> as AVLTreeSeqChap18>::map(a, f) }

    fn select<'a, T>(a: &'a AVLTreeS<T>, b: &'a AVLTreeS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug,
    {
        let a_len = <AVLTreeS<T> as AVLTreeSeq<T>>::length(a);
        if i < a_len { Some(*<AVLTreeS<T> as AVLTreeSeq<T>>::nth(a, i)) }
        else {
            let off = i - a_len;
            let b_len = <AVLTreeS<T> as AVLTreeSeq<T>>::length(b);
            if off < b_len { Some(*<AVLTreeS<T> as AVLTreeSeq<T>>::nth(b, off)) } else { None }
        }
    }

    fn append<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, b: &AVLTreeS<T>) -> AVLTreeS<T> {
        <AVLTreeS<T> as AVLTreeSeqChap18>::append(a, b)
    }

    fn deflate<T: Ord + Copy + Debug>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeS<T> {
        if f(x) == B::True { <AVLTreeS<T> as AVLTreeSeq<T>>::singleton(*x) } else { <AVLTreeS<T> as AVLTreeSeq<T>>::empty() }
    }

    fn filter<T: Ord + Copy + Debug>(a: &AVLTreeS<T>, f: impl Fn(&T) -> B) -> AVLTreeS<T> {
        // Build filtered tree directly (avoid nested AVLTreeS in generics due to bounds)
        let mut out: AVLTreeS<T> = <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
        for i in 0..<AVLTreeS<T> as AVLTreeSeq<T>>::length(a) {
            let v = <AVLTreeS<T> as AVLTreeSeq<T>>::nth(a, i);
            if f(&v) == B::True { out.insert_value(*v); }
        }
        out
    }
}


