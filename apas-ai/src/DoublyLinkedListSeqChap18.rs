//! Chapter 18 algorithms as trait methods over `DoublyLinkedListS<T>`.
//!
//! Abstract:
//! - Defines trait `DoublyLinkedListSeqChap18` with Chapter 18 operations over `DoublyLinkedListS<T>`.
//! - Sequential implementations using `DoublyLinkedListSeq` primitives; iteration is linear over nodes.
//! - Intended for API parity with arrays; performance differs due to linked-list traversal costs.

use crate::Types::{B, N, O};
use crate::DoublyLinkedListSeq::{DoublyLinkedListS, DoublyLinkedListSeq};

/// Algorithms from APAS Chapter 18 built from Linked-List Sequences.
pub trait DoublyLinkedListSeqChap18 {
    /// Definition 18.1 (tabulate). Build a sequence of length `n` where the i-th element is `f(i)`. <br/>
    /// Derived (DLL): Work Θ(1 + Σ i W(f(i))), Span Θ(1 + Σ i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> DoublyLinkedListS<T>;

    /// Definition 18.1 (map). Apply `f` to each element, returning a new sequence of results. <br/>
    /// Derived (DLL): Work Θ(1 + Σ x W(f(x))), Span Θ(1 + Σ x S(f(x))).
    fn map<T, U: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&T) -> U) -> DoublyLinkedListS<U>;

    /// Definition 18.1 (append). Concatenate `a` and `b` preserving order. <br/>
    /// Derived (DLL): Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append<T: Clone + Eq>(a: &DoublyLinkedListS<T>, b: &DoublyLinkedListS<T>) -> DoublyLinkedListS<T>;

    /// Definition 18.1 (filter). Keep elements `x` for which `pred(x)` returns `True`. <br/>
    /// Derived (DLL): Work Θ(1 + Σ x W(pred(x))), Span Θ(1 + Σ x S(pred(x))).
    fn filter<T: Clone + Eq>(a: &DoublyLinkedListS<T>, pred: impl Fn(&T) -> B) -> DoublyLinkedListS<T>;

    /// Update `self[index]` to `item` in place if in bounds, and return `self` for chaining. <br/>
    /// Derived (DLL): Work/Span Θ(i).
    fn update<T: Clone + Eq>(a: &mut DoublyLinkedListS<T>, item_at: (N, T)) -> &mut DoublyLinkedListS<T>;

    /// Definition 18.1 (inject). Apply all updates, keeping the first update to any index. <br/>
    /// Derived (DLL): Work Θ(1 + |a| + |updates|), Span Θ(1 + |a| + |updates|).
    fn inject<T: Clone + Eq>(a: &DoublyLinkedListS<T>, updates: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T>;

    /// Definition 18.1 (ninject). Apply all updates, last write per index wins. <br/>
    /// Derived (DLL): Work Θ(1 + |a| + |updates|), Span Θ(1 + |a| + |updates|).
    fn ninject<T: Clone + Eq>(a: &DoublyLinkedListS<T>, updates: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T>;

    /// Left-to-right fold using `f`, starting from seed `x`. <br/>
    /// Derived (DLL): Work Θ(1 + Σ f-works), Span Θ(1 + Σ f-spans).
    fn iterate<T: Clone + Eq, A: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

    /// Produce the sequence of prefix accumulations and the final value using `f`. <br/>
    /// Derived (DLL): Work Θ(|a|), Span Θ(|a|).
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (DoublyLinkedListS<A>, A);

    /// Divide-and-conquer reduction using associative `f` with identity `id`. <br/>
    /// Derived (DLL): Work Θ(|a|), Span Θ(|a|).
    fn reduce<T: Clone + Eq>(a: &DoublyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

    /// Sequential scan producing all prefixes under `f` together with the total. <br/>
    /// Derived (DLL): Work Θ(|a|), Span Θ(|a|) under constant-time `f`.
    fn scan<T: Clone + Eq>(a: &DoublyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (DoublyLinkedListS<T>, T);

    /// Flatten one level of nested sequences by concatenating all inner sequences. <br/>
    /// Derived (DLL): Work Θ(1 + |a| + Σ x |x|), Span Θ(1 + |a| + Σ x |x|).
    fn flatten<T: Clone + Eq>(ss: &DoublyLinkedListS<DoublyLinkedListS<T>>) -> DoublyLinkedListS<T>;

    /// Group pairs by key according to `cmp`, collecting values into sequences per key. <br/>
    /// Derived (DLL): Work Θ(1 + W(cmp)·|a| lg|a|), Span Θ(1 + S(cmp)·|a| lg|a|).
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &DoublyLinkedListS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> DoublyLinkedListS<(A, DoublyLinkedListS<Bv>)>;
}

impl<T2> DoublyLinkedListSeqChap18 for DoublyLinkedListS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for i in 0..n { out.push_back(f(i)); }
        DoublyLinkedListS { data: out }
    }

    fn map<T, U: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&T) -> U) -> DoublyLinkedListS<U> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(f(x)); }
        DoublyLinkedListS { data: out }
    }

    fn append<T: Clone + Eq>(a: &DoublyLinkedListS<T>, b: &DoublyLinkedListS<T>) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(x.clone()); }
        for x in b.data.iter() { out.push_back(x.clone()); }
        DoublyLinkedListS { data: out }
    }

    fn filter<T: Clone + Eq>(a: &DoublyLinkedListS<T>, pred: impl Fn(&T) -> B) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() {
            if pred(x) == B::True { out.push_back(x.clone()); }
        }
        DoublyLinkedListS { data: out }
    }

    fn update<T: Clone + Eq>(a: &mut DoublyLinkedListS<T>, (index, item): (N, T)) -> &mut DoublyLinkedListS<T> { a.update((index, item)) }

    fn inject<T: Clone + Eq>(a: &DoublyLinkedListS<T>, updates: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for (i, x) in a.data.iter().enumerate() {
            let mut written = false;
            for (u_index, u_val) in updates.data.iter() {
                if *u_index == i { out.push_back(u_val.clone()); written = true; break; }
            }
            if !written { out.push_back(x.clone()); }
        }
        DoublyLinkedListS { data: out }
    }

    fn ninject<T: Clone + Eq>(a: &DoublyLinkedListS<T>, updates: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for (i, x) in a.data.iter().enumerate() {
            let mut last: Option<T> = None;
            for (u_index, u_val) in updates.data.iter() { if *u_index == i { last = Some(u_val.clone()); } }
            match last { Some(v) => out.push_back(v), None => out.push_back(x.clone()) }
        }
        DoublyLinkedListS { data: out }
    }

    fn iterate<T: Clone + Eq, A: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x;
        for elt in a.data.iter() { acc = f(&acc, elt); }
        acc
    }

    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (DoublyLinkedListS<A>, A) {
        let mut prefixes = std::collections::LinkedList::new();
        let mut acc = x;
        for elt in a.data.iter() { prefixes.push_back(acc.clone()); acc = f(&acc, elt); }
        (DoublyLinkedListS { data: prefixes }, acc)
    }

    fn reduce<T: Clone + Eq>(a: &DoublyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        let mut acc = id;
        for elt in a.data.iter() { acc = f(&acc, elt); }
        acc
    }

    fn scan<T: Clone + Eq>(a: &DoublyLinkedListS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (DoublyLinkedListS<T>, T) {
        let (prefixes, total) = <DoublyLinkedListS<T2> as DoublyLinkedListSeqChap18>::iteratePrefixes(a, |p, x| f(p, x), id);
        (prefixes, total)
    }

    fn flatten<T: Clone + Eq>(ss: &DoublyLinkedListS<DoublyLinkedListS<T>>) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for inner in ss.data.iter() { for x in inner.data.iter() { out.push_back(x.clone()); } }
        DoublyLinkedListS { data: out }
    }

    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &DoublyLinkedListS<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> DoublyLinkedListS<(A, DoublyLinkedListS<Bv>)> {
        let mut groups: std::collections::LinkedList<(A, DoublyLinkedListS<Bv>)> = std::collections::LinkedList::new();
        'outer: for (key, value) in a.data.iter() {
            for (gk, gvals) in groups.iter_mut() {
                if cmp(key, gk) == O::Equal { gvals.data.push_back(value.clone()); continue 'outer; }
            }
            let mut ll = std::collections::LinkedList::new();
            ll.push_back(value.clone());
            groups.push_back((key.clone(), DoublyLinkedListS { data: ll }));
        }
        DoublyLinkedListS { data: groups }
    }
}


