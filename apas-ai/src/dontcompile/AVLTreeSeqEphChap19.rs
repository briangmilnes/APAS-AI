//! Chapter 19 algorithms as trait methods over `AVLTreeS<T>`.
//!
//! Abstract:
//! - Defines trait `AVLTreeSeqChap19` mirroring array/list Chapter 19 APIs for AVL trees.
//! - Implementations delegate to Chapter 18 style sequential constructions.

use crate::Types::{B, N, O};
use crate::AVLTreeSeqEph::{AVLTreeEphS, AVLTreeSeqEphTrait};
use crate::AVLTreeSeqEphChap18::AVLTreeSeqChap18Trait;
use std::fmt::{Debug, Display};

pub trait AVLTreeSeqChap19Trait {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeEphS<T>
    where
        T: Ord + Copy + Debug + Display;
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn map<T, U>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeEphS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display;
    /// APAS (array): Work Θ(1). For trees, requires rank/select support.
    fn select<'a, T>(a: &'a AVLTreeEphS<T>, b: &'a AVLTreeEphS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug + Display;
    /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, b: &AVLTreeEphS<T>) -> AVLTreeEphS<T>;
    /// APAS: Work/Span Θ(1).
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeEphS<T>;
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeEphS<T>;
}

impl<T2: Ord + Copy + Debug + Display> AVLTreeEphSeqChap19 for AVLTreeEphS<T2> {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + lg(n) + max i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> AVLTreeEphS<T>
    where
        T: Ord + Copy + Debug + Display,
    { <AVLTreeEphS<T> as AVLTreeEphSeqChap18>::tabulate(f, n) }

    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn map<T, U>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> U) -> AVLTreeEphS<U>
    where
        T: Ord + Copy + Debug + Display,
        U: Ord + Copy + Debug + Display,
    { <AVLTreeEphS<T> as AVLTreeEphSeqChap18>::map(a, f) }

    /// APAS (array): Work Θ(1). For trees, requires rank/select support.
    fn select<'a, T>(a: &'a AVLTreeEphS<T>, b: &'a AVLTreeEphS<T>, i: N) -> Option<T>
    where
        T: Ord + Copy + Debug + Display,
    {
        let a_len = <AVLTreeEphS<T> as AVLTreeEphSeq<T>>::length(a);
        if i < a_len { Some(*<AVLTreeEphS<T> as AVLTreeEphSeq<T>>::nth(a, i)) }
        else {
            let off = i - a_len;
            let b_len = <AVLTreeEphS<T> as AVLTreeEphSeq<T>>::length(b);
            if off < b_len { Some(*<AVLTreeEphS<T> as AVLTreeEphSeq<T>>::nth(b, off)) } else { None }
        }
    }

    /// APAS: Work Θ(1 + |lg(|a|/|b|)|), Span Θ(1 + |lg(|a|/|b|)|).
    fn append<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, b: &AVLTreeEphS<T>) -> AVLTreeEphS<T> {
        <AVLTreeEphS<T> as AVLTreeEphSeqChap18>::append(a, b)
    }

    /// APAS: Work/Span Θ(1).
    fn deflate<T: Ord + Copy + Debug + Display>(f: impl Fn(&T) -> B, x: &T) -> AVLTreeEphS<T> {
        if f(x) == B::True { <AVLTreeEphS<T> as AVLTreeEphSeq<T>>::singleton(*x) } else { <AVLTreeEphS<T> as AVLTreeEphSeq<T>>::empty() }
    }

    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + lg(|a|) + max x∈a S(f(x))).
    fn filter<T: Ord + Copy + Debug + Display>(a: &AVLTreeEphS<T>, f: impl Fn(&T) -> B) -> AVLTreeEphS<T> {
        // Build filtered tree directly (avoid nested AVLTreeEphS in generics due to bounds)
        let mut out: AVLTreeEphS<T> = <AVLTreeEphS<T> as AVLTreeEphSeq<T>>::empty();
        for i in 0..<AVLTreeEphS<T> as AVLTreeEphSeq<T>>::length(a) {
            let v = <AVLTreeEphS<T> as AVLTreeEphSeq<T>>::nth(a, i);
            if f(&v) == B::True { out.insert_value(*v); }
        }
        out
    }
}


