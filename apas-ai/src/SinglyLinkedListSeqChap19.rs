//! Chapter 19 algorithms as trait methods over `SinglyLinkedListS<T>`.
//!
//! Abstract:
//! - Defines trait `SinglyLinkedListSeqChap19` mirroring `ArraySeqChap19` over `SinglyLinkedListS<T>`.
//! - Uses `tabulate`, `map`, `append`, `deflate`, `filter`, `iterate`, `reduce`, `scan`, `flatten`.
//! - Includes sequential “parallel injection” analogs using `Mutex` to mirror semantics.

use crate::Types::{B, N, O};
use crate::SinglyLinkedListSeq::{SinglyLinkedListS, SinglyLinkedListSeq};
use crate::SinglyLinkedListSeqChap18::SinglyLinkedListSeqChap18;
use std::sync::Mutex;

pub trait SinglyLinkedListSeqChap19 {
    /// APAS 20.7 (tabulate f n): Work Θ(1 + Σ i W(f(i))), Span Θ(1 + Σ i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (map f a): Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<T, U>(a: &SinglyLinkedListS<T>, f: impl Fn(&T) -> U) -> SinglyLinkedListS<U>;
    /// APAS 20.7 (select): for arrays Θ(1); for lists, Θ(1+i) by traversal.
    fn select<'a, T>(a: &'a SinglyLinkedListS<T>, b: &'a SinglyLinkedListS<T>, i: N) -> Option<&'a T>;
    /// APAS 20.7 (append a b): Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append<T: Clone + Eq>(a: &SinglyLinkedListS<T>, b: &SinglyLinkedListS<T>) -> SinglyLinkedListS<T>;
    /// Variant with weaker bound on T.
    fn append2<T: Clone>(a: &SinglyLinkedListS<T>, b: &SinglyLinkedListS<T>) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (deflate): Work/Span Θ(1).
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (filter f a): Work Θ(1 + Σ x W(f(x))), Span Θ(1 + Σ x S(f(x))).
    fn filter<T: Clone + Eq>(a: &SinglyLinkedListS<T>, f: impl Fn(&T) -> B) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (iterate): Work/Span sums over trace T(−).
    fn iterate<T: Clone + Eq, A: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    /// APAS 20.7 (reduce/scan): linear work/span under list traversal assumptions.
    fn reduce<T: Clone + Eq, F>(a: &SinglyLinkedListS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    fn scan<T: Clone + Eq, F>(a: &SinglyLinkedListS<T>, f: &F, id: T) -> (SinglyLinkedListS<T>, T) where F: Fn(&T, &T) -> T;
    /// APAS 20.7 (flatten): Work Θ(1 + |a| + Σ x |x|), Span Θ(1 + |a| + Σ x |x|).
    fn flatten<T: Clone + Eq>(s: &SinglyLinkedListS<SinglyLinkedListS<T>>) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (inject/ninject): Work Θ(1 + |a| + |b|), Span Θ(1 + |a| + |b|).
    fn inject<T: Clone + Eq>(values: &SinglyLinkedListS<T>, changes: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T>;
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut SinglyLinkedListS<(T, N)>, changes: &SinglyLinkedListS<(N, T)>, change_index: N);
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &SinglyLinkedListS<T>, changes: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T>;
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &SinglyLinkedListS<Mutex<(T, N)>>,
        changes: &SinglyLinkedListS<(N, T)>, change_index: N);
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &SinglyLinkedListS<T>, changes: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T>;
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &SinglyLinkedListS<Mutex<(T, N)>>,
        changes: &SinglyLinkedListS<(N, T)>, change_index: N);
}

impl<T2> SinglyLinkedListSeqChap19 for SinglyLinkedListS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SinglyLinkedListS<T> { <SinglyLinkedListS<T> as SinglyLinkedListSeqChap18>::tabulate(f, n) }
    fn map<T, U>(a: &SinglyLinkedListS<T>, f: impl Fn(&T) -> U) -> SinglyLinkedListS<U> { <SinglyLinkedListS<U> as SinglyLinkedListSeqChap18>::tabulate(|i| f(a.nth(i)), a.length()) }
    fn select<'a, T>(a: &'a SinglyLinkedListS<T>, b: &'a SinglyLinkedListS<T>, index: N) -> Option<&'a T> {
        if index < a.length() { Some(a.nth(index)) } else { let offset = index - a.length(); if offset < b.length() { Some(b.nth(offset)) } else { None } }
    }
    fn append<T: Clone + Eq>(a: &SinglyLinkedListS<T>, b: &SinglyLinkedListS<T>) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(x.clone()); }
        for x in b.data.iter() { out.push_back(x.clone()); }
        SinglyLinkedListS { data: out }
    }
    fn append2<T: Clone>(a: &SinglyLinkedListS<T>, b: &SinglyLinkedListS<T>) -> SinglyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(x.clone()); }
        for x in b.data.iter() { out.push_back(x.clone()); }
        SinglyLinkedListS { data: out }
    }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> SinglyLinkedListS<T> { let keep = f(x) == B::True; if keep { <SinglyLinkedListS<T> as SinglyLinkedListSeq<T>>::singleton(x.clone()) } else { <SinglyLinkedListS<T> as SinglyLinkedListSeq<T>>::empty() } }
    fn filter<T: Clone + Eq>(a: &SinglyLinkedListS<T>, f: impl Fn(&T) -> B) -> SinglyLinkedListS<T> { let mapped: SinglyLinkedListS<SinglyLinkedListS<T>> = <SinglyLinkedListS<SinglyLinkedListS<T>> as SinglyLinkedListSeqChap19>::tabulate(|i| Self::deflate(|elt| f(elt), a.nth(i)), a.length()); <SinglyLinkedListS<T> as SinglyLinkedListSeqChap18>::flatten(&mapped) }
    fn iterate<T: Clone + Eq, A: Clone>(a: &SinglyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { <SinglyLinkedListS<T2> as SinglyLinkedListSeqChap18>::iterate(a, f, x) }
    fn reduce<T: Clone + Eq, F>(a: &SinglyLinkedListS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { <SinglyLinkedListS<T2> as SinglyLinkedListSeqChap18>::reduce(a, f, id) }
    fn scan<T: Clone + Eq, F>(a: &SinglyLinkedListS<T>, f: &F, id: T) -> (SinglyLinkedListS<T>, T) where F: Fn(&T, &T) -> T { <SinglyLinkedListS<T2> as SinglyLinkedListSeqChap18>::scan(a, f, id) }
    fn flatten<T: Clone + Eq>(s: &SinglyLinkedListS<SinglyLinkedListS<T>>) -> SinglyLinkedListS<T> { <SinglyLinkedListS<T> as SinglyLinkedListSeqChap18>::flatten(s) }
    fn inject<T: Clone + Eq>(values: &SinglyLinkedListS<T>, changes: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T> { <SinglyLinkedListS<T2> as SinglyLinkedListSeqChap18>::inject(values, changes) }
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut SinglyLinkedListS<(T, N)>, changes: &SinglyLinkedListS<(N, T)>, change_index: N) { let (loc, val) = changes.nth(change_index).clone(); if loc < values_with_change_number.length() { let (_, cn) = values_with_change_number.nth(loc).clone(); if change_index < cn { let _ = values_with_change_number.set(loc, (val, change_index)); } } }
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &SinglyLinkedListS<T>, changes: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T> { let len = values.length(); let values_with_cn: SinglyLinkedListS<Mutex<(T, N)>> = <SinglyLinkedListS<Mutex<(T, N)>> as SinglyLinkedListSeqChap19>::tabulate(|i| Mutex::new((values.nth(i).clone(), len)), len); std::thread::scope(|scope| { for ci in 0..changes.length() { let v = &values_with_cn; let c = changes; scope.spawn(move || { Self::AtomicWriteLowestChangeNumberWins(v, c, ci); }); } }); <SinglyLinkedListS<T> as SinglyLinkedListSeqChap19>::tabulate(|i| { let guard = values_with_cn.nth(i).lock().unwrap(); guard.0.clone() }, len) }
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &SinglyLinkedListS<Mutex<(T, N)>>, changes: &SinglyLinkedListS<(N, T)>, change_index: N) { let (loc, val) = changes.nth(change_index).clone(); if loc >= values_with_change_number.length() { return; } let mut guard = values_with_change_number.nth(loc).lock().unwrap(); let (ref mut cur, ref mut cn) = *guard; if change_index < *cn { *cur = val; *cn = change_index; } }
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &SinglyLinkedListS<T>, changes: &SinglyLinkedListS<(N, T)>) -> SinglyLinkedListS<T> { let len = values.length(); let values_with_cn: SinglyLinkedListS<Mutex<(T, N)>> = <SinglyLinkedListS<Mutex<(T, N)>> as SinglyLinkedListSeqChap19>::tabulate(|i| Mutex::new((values.nth(i).clone(), 0)), len); std::thread::scope(|scope| { for ci in 0..changes.length() { let v = &values_with_cn; let c = changes; scope.spawn(move || { Self::AtomicWriteHighestChangeNumberWins(v, c, ci); }); } }); <SinglyLinkedListS<T> as SinglyLinkedListSeqChap19>::tabulate(|i| { let guard = values_with_cn.nth(i).lock().unwrap(); guard.0.clone() }, len) }
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &SinglyLinkedListS<Mutex<(T, N)>>, changes: &SinglyLinkedListS<(N, T)>, change_index: N) { let (loc, val) = changes.nth(change_index).clone(); if loc >= values_with_change_number.length() { return; } let mut guard = values_with_change_number.nth(loc).lock().unwrap(); let (ref mut cur, ref mut cn) = *guard; if change_index >= *cn { *cur = val; *cn = change_index; } }
}


