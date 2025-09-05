//! Chapter 18 algorithms as trait methods over `SinglyLinkedListS<T>`.
//!
//! Abstract:
//! - Defines trait `SinglyLinkedListSeqChap18` with Chapter 18 operations over `SinglyLinkedListS<T>`.
//! - Sequential implementations using `SinglyLinkedListSeq` primitives; iteration is linear over nodes.
//! - Intended for API parity with arrays; performance differs due to linked-list traversal costs.

use crate::Types::{B, N, O};
use crate::SinglyLinkedListSeq::{SinglyLinkedListS, SinglyLinkedListSeq};

/// Algorithms from APAS Chapter 18 built from Linked-List Sequences.
pub trait SinglyLinkedListSeqChap18 {
    /// APAS 20.7 (tabulate f n): Work Θ(1 + Σ i W(f(i))), Span Θ(1 + Σ i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (map f a): Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<T, U: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&T) -> U) -> SinglyLinkedListS<U>;
    /// APAS 20.7 (append a b): Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append<T: Clone + Eq>(a: &SinglyLinkedListS<T>, b: &SinglyLinkedListS<T>) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (filter f a): Work Θ(1 + Σ x∈a W(p(x))), Span Θ(1 + Σ x∈a S(p(x))).
    fn filter<T: Clone + Eq>(a: &SinglyLinkedListS<T>, pred: impl Fn(&T) -> B) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (update a (i,x)): Work Θ(1 + |a|), Span Θ(1 + |a|). <br/>
    /// gpt-5-hard: Work Θ(i), Span Θ(i).
    /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
    fn update<T: Clone + Eq>(a: &mut SinglyLinkedListS<T>, item_at: (N, T)) -> &mut SinglyLinkedListS<T>;
    /// APAS 20.7 (inject/ninject a b): Work Θ(1 + |a| + |b|), Span Θ(1 + |a| + |b|).
    fn inject<T: Clone + Eq>(a: &SinglyLinkedListS<T>, updates: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T>;
    fn ninject<T: Clone + Eq>(a: &SinglyLinkedListS<T>, updates: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (iterate/reduce/scan): see table assumptions.
    fn iterate<T: Clone + Eq, A: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (SinglyLinkedListS<A>, A);
    fn reduce<T: Clone + Eq>(a: &SinglyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
    fn scan<T: Clone + Eq>(a: &SinglyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (SinglyLinkedListS<T>, T);
    /// APAS 20.7 (flatten a): Work Θ(1 + |a| + Σ x |x|), Span Θ(1 + |a| + Σ x |x|).
    fn flatten<T: Clone + Eq>(ss: &SinglyLinkedListS<SinglyLinkedListS<T>>) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (collect f a): Work Θ(1 + W(f)·|a| lg|a|), Span Θ(1 + S(f)·|a| lg|a|).
    /// gpt-5-hard: Work Θ(|a|^2), Span Θ(|a|^2).
    /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &SinglyLinkedListS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> SinglyLinkedListS<(A, SinglyLinkedListS<Bv>)>;
}

impl<T2> SinglyLinkedListSeqChap18 for SinglyLinkedListS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for i in 0..n { out.push_back(f(i)); }
        SinglyLinkedListS { data: out }
    }
    fn map<T, U: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&T) -> U) -> SinglyLinkedListS<U> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(f(x)); }
        SinglyLinkedListS { data: out }
    }
    fn append<T: Clone + Eq>(a: &SinglyLinkedListS<T>, b: &SinglyLinkedListS<T>) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(x.clone()); }
        for x in b.data.iter() { out.push_back(x.clone()); }
        SinglyLinkedListS { data: out }
    }
    fn filter<T: Clone + Eq>(a: &SinglyLinkedListS<T>, pred: impl Fn(&T) -> B) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { if pred(x) == B::True { out.push_back(x.clone()); } }
        SinglyLinkedListS { data: out }
    }
    fn update<T: Clone + Eq>(a: &mut SinglyLinkedListS<T>, (index, item): (N, T)) -> &mut SinglyLinkedListS<T> { a.update((index, item)) }
    fn inject<T: Clone + Eq>(a: &SinglyLinkedListS<T>, updates: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for (i, x) in a.data.iter().enumerate() {
            let mut written = false;
            for (u_index, u_val) in updates.data.iter() {
                if *u_index == i { out.push_back(u_val.clone()); written = true; break; }
            }
            if !written { out.push_back(x.clone()); }
        }
        SinglyLinkedListS { data: out }
    }
    fn ninject<T: Clone + Eq>(a: &SinglyLinkedListS<T>, updates: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for (i, x) in a.data.iter().enumerate() {
            let mut last: Option<T> = None;
            for (u_index, u_val) in updates.data.iter() { if *u_index == i { last = Some(u_val.clone()); } }
            match last { Some(v) => out.push_back(v), None => out.push_back(x.clone()) }
        }
        SinglyLinkedListS { data: out }
    }
    fn iterate<T: Clone + Eq, A: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x; for elt in a.data.iter() { acc = f(&acc, elt); } acc
    }
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (SinglyLinkedListS<A>, A) {
        let mut prefixes = std::collections::LinkedList::new(); let mut acc = x; for elt in a.data.iter() { prefixes.push_back(acc.clone()); acc = f(&acc, elt); } (SinglyLinkedListS { data: prefixes }, acc)
    }
    fn reduce<T: Clone + Eq>(a: &SinglyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T { let mut acc = id; for elt in a.data.iter() { acc = f(&acc, elt); } acc }
    fn scan<T: Clone + Eq>(a: &SinglyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (SinglyLinkedListS<T>, T) { let (p, t) = <SinglyLinkedListS<T2> as SinglyLinkedListSeqChap18>::iteratePrefixes(a, |p, x| f(p, x), id); (p, t) }
    fn flatten<T: Clone + Eq>(ss: &SinglyLinkedListS<SinglyLinkedListS<T>>) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new(); for inner in ss.data.iter() { for x in inner.data.iter() { out.push_back(x.clone()); } } SinglyLinkedListS { data: out }
    }
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &SinglyLinkedListS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> SinglyLinkedListS<(A, SinglyLinkedListS<Bv>)> {
        let mut groups: std::collections::LinkedList<(A, SinglyLinkedListS<Bv>)> = std::collections::LinkedList::new();
        'outer: for (key, value) in a.data.iter() {
            for (gk, gvals) in groups.iter_mut() { if cmp(key, gk) == O::Equal { gvals.data.push_back(value.clone()); continue 'outer; } }
            let mut ll = std::collections::LinkedList::new(); ll.push_back(value.clone()); groups.push_back((key.clone(), SinglyLinkedListS { data: ll }));
        }
        SinglyLinkedListS { data: groups }
    }
}


