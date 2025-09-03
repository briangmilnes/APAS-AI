//! Chapter 18 algorithms over `SharedSinglyLinkedList<T>` (immutable, structurally shared).

use crate::Types::{B, N, O};
use crate::SharedSinglyLinkedList::{SharedSinglyLinkedList as SLL, SharedSList};

pub trait SharedSListChap18 {
    /// APAS 20.7 (tabulate f n): Work Θ(1 + Σ i W(f(i))), Span Θ(1 + Σ i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SLL<T>;
    /// APAS 20.7 (map f a): Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<T, U>(a: &SLL<T>, f: impl Fn(&T) -> U) -> SLL<U>;
    /// APAS 20.7 (append a b): Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append<T: Clone>(a: &SLL<T>, b: &SLL<T>) -> SLL<T>;
    /// APAS 20.7 (filter f a): Work Θ(1 + Σ x∈a W(p(x))), Span Θ(1 + Σ x∈a S(p(x))).
    fn filter<T: Clone>(a: &SLL<T>, pred: impl Fn(&T) -> B) -> SLL<T>;
    /// APAS 20.7 (update a (i,x)): Not applicable to immutable list; returns cloned with Θ(i).
    /// ChatGPT-5-hard: Work Θ(i), Span Θ(i).
    fn update<T: Clone>(a: &SLL<T>, item_at: (N, T)) -> SLL<T>;
    /// APAS 20.7 (inject/ninject a b): Define serially for reference.
    fn inject<T: Clone + Eq>(a: &SLL<T>, updates: &SLL<(N, T)>) -> SLL<T>;
    fn ninject<T: Clone + Eq>(a: &SLL<T>, updates: &SLL<(N, T)>) -> SLL<T>;
    /// iterate/reduce/scan
    fn iterate<T, A: Clone>(a: &SLL<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn iteratePrefixes<T, A: Clone>(a: &SLL<T>, f: impl Fn(&A, &T) -> A, x: A) -> (SLL<A>, A) where A: Clone;
    fn reduce<T: Clone>(a: &SLL<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
    fn scan<T: Clone>(a: &SLL<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (SLL<T>, T);
    /// flatten/collect
    fn flatten<T: Clone>(ss: &SLL<SLL<T>>) -> SLL<T>;
    fn collect<A: Clone + Eq, Bv: Clone>(a: &SLL<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> SLL<(A, SLL<Bv>)>;
}

impl<T2> SharedSListChap18 for SLL<T2> {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + Σ i=0..n-1 S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SLL<T> {
        let mut out: SLL<T> = SLL::new();
        // Build in reverse then return
        let mut i: N = n;
        while i > 0 { i -= 1; out = SLL::cons(f(i), &out); }
        out
    }
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<T, U>(a: &SLL<T>, f: impl Fn(&T) -> U) -> SLL<U> { SLL::map(a, f) }
    /// APAS: Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append<T: Clone>(a: &SLL<T>, b: &SLL<T>) -> SLL<T> { SLL::append(a, b) }
    /// ChatGPT-5-hard: Work Θ(i), Span Θ(i).
    fn filter<T: Clone>(a: &SLL<T>, pred: impl Fn(&T) -> B) -> SLL<T> {
        fn go<T: Clone>(n: &SLL<T>, pred: &impl Fn(&T) -> B) -> SLL<T> {
            match n.head() {
                None => SLL::new(),
                Some(h) => {
                    let t = n.tail();
                    let rest = go(&t, pred);
                    if pred(h) == B::True { SLL::cons(h.clone(), &rest) } else { rest }
                }
            }
        }
        go(a, &pred)
    }
    fn update<T: Clone>(a: &SLL<T>, (index, item): (N, T)) -> SLL<T> {
        fn go<T: Clone>(n: &SLL<T>, idx: N, item: T) -> SLL<T> {
            match n.head() {
                None => SLL::new(),
                Some(h) => if idx == 0 {
                    SLL::cons(item, &n.tail())
                } else {
                    let replaced_tail = go(&n.tail(), idx - 1, item);
                    SLL::cons(h.clone(), &replaced_tail)
                }
            }
        }
        go(a, index, item)
    }
    fn inject<T: Clone + Eq>(a: &SLL<T>, updates: &SLL<(N, T)>) -> SLL<T> {
        let first_update_for = |i: N| -> Option<T> {
            let mut cur = updates.clone();
            while let Some((idx, val)) = cur.head().map(|p| (p.0, p.1.clone())) { if idx == i { return Some(val); } cur = cur.tail(); }
            None
        };
        let mut out = SLL::new();
        let mut i: N = 0; let mut cur = (*a).clone();
        while let Some(h) = cur.head() { let v = first_update_for(i).unwrap_or_else(|| h.clone()); out = SLL::cons(v, &out); cur = cur.tail(); i += 1; }
        // reverse out to preserve order
        let mut rev = SLL::new(); let mut cur2 = out; while let Some(h) = cur2.head() { rev = SLL::cons(h.clone(), &rev); cur2 = cur2.tail(); }
        rev
    }
    fn ninject<T: Clone + Eq>(a: &SLL<T>, updates: &SLL<(N, T)>) -> SLL<T> {
        let last_update_for = |i: N| -> Option<T> {
            let mut cur = updates.clone(); let mut last: Option<T> = None;
            while let Some((idx, val)) = cur.head().map(|p| (p.0, p.1.clone())) { if idx == i { last = Some(val); } cur = cur.tail(); }
            last
        };
        let mut out = SLL::new();
        let mut i: N = 0; let mut cur = (*a).clone();
        while let Some(h) = cur.head() { let v = last_update_for(i).unwrap_or_else(|| h.clone()); out = SLL::cons(v, &out); cur = cur.tail(); i += 1; }
        let mut rev = SLL::new(); let mut cur2 = out; while let Some(h) = cur2.head() { rev = SLL::cons(h.clone(), &rev); cur2 = cur2.tail(); }
        rev
    }
    fn iterate<T, A: Clone>(a: &SLL<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x; let mut cur: SLL<T> = (*a).clone();
        loop {
            match cur.head() {
                None => break,
                Some(h) => { let next = cur.tail(); acc = f(&acc, h); cur = next; }
            }
        }
        acc
    }
    fn iteratePrefixes<T, A: Clone>(a: &SLL<T>, f: impl Fn(&A, &T) -> A, x: A) -> (SLL<A>, A) where A: Clone {
        let mut prefixes = SLL::new(); let mut acc = x; let mut cur: SLL<T> = (*a).clone();
        loop {
            match cur.head() {
                None => break,
                Some(h) => { let next = cur.tail(); prefixes = SLL::cons(acc.clone(), &prefixes); acc = f(&acc, h); cur = next; }
            }
        }
        // reverse prefixes to original order
        let mut rev = SLL::new(); let mut p = prefixes; while let Some(h) = p.head() { let next = p.tail(); rev = SLL::cons(h.clone(), &rev); p = next; }
        (rev, acc)
    }
    fn reduce<T: Clone>(a: &SLL<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        let mut acc = id; let mut cur: SLL<T> = (*a).clone();
        loop { match cur.head() { None => break, Some(h) => { let next = cur.tail(); acc = f(&acc, h); cur = next; } } }
        acc
    }
    fn scan<T: Clone>(a: &SLL<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (SLL<T>, T) {
        let mut prefixes = SLL::new(); let mut acc = id; let mut cur: SLL<T> = (*a).clone();
        loop {
            match cur.head() {
                None => break,
                Some(h) => { let next = cur.tail(); prefixes = SLL::cons(acc.clone(), &prefixes); acc = f(&acc, h); cur = next; }
            }
        }
        let mut rev = SLL::new(); let mut p = prefixes; while let Some(h) = p.head() { let next = p.tail(); rev = SLL::cons(h.clone(), &rev); p = next; }
        (rev, acc)
    }
    fn flatten<T: Clone>(ss: &SLL<SLL<T>>) -> SLL<T> {
        let mut out = SLL::new(); let mut outer: SLL<SLL<T>> = (*ss).clone();
        loop {
            match outer.head() {
                None => break,
                Some(inner) => {
                    let mut cur = inner.clone();
                    loop { match cur.head() { None => break, Some(h) => { let next = cur.tail(); out = SLL::cons(h.clone(), &out); cur = next; } } }
                    outer = outer.tail();
                }
            }
        }
        let mut rev = SLL::new(); let mut p = out; while let Some(h) = p.head() { let next = p.tail(); rev = SLL::cons(h.clone(), &rev); p = next; }
        rev
    }
    fn collect<A: Clone + Eq, Bv: Clone>(a: &SLL<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> SLL<(A, SLL<Bv>)> {
        // naive O(n^2) grouping for simplicity
        let mut groups: SLL<(A, SLL<Bv>)> = SLL::new();
        let mut cur = (*a).clone();
        while let Some((k, v)) = cur.head().map(|p| (p.0.clone(), p.1.clone())) {
            // find group
            let mut built: SLL<(A, SLL<Bv>)> = SLL::new();
            let mut inserted = false;
            let mut gcur = groups.clone();
            while let Some((gk, gv)) = gcur.head().map(|p| (p.0.clone(), p.1.clone())) {
                if !inserted && cmp(&k, &gk) == O::Equal {
                    let new_vals = SLL::cons(v.clone(), &gv);
                    built = SLL::cons((gk, new_vals), &built);
                    inserted = true;
                } else {
                    built = SLL::cons((gk, gv), &built);
                }
                gcur = gcur.tail();
            }
            if !inserted { built = SLL::cons((k, SLL::cons(v, &SLL::new())), &built); }
            // reverse built into groups
            let mut new_groups: SLL<(A, SLL<Bv>)> = SLL::new(); let mut b = built; while let Some(h) = b.head() { new_groups = SLL::cons(h.clone(), &new_groups); b = b.tail(); }
            groups = new_groups;
            cur = cur.tail();
        }
        groups
    }
}


