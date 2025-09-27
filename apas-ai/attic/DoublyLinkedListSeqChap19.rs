//! Chapter 19 algorithms as trait methods over `DoublyLinkedListS<T>`.
//!
//! Abstract:
//! - Defines trait `DoublyLinkedListSeqChap19` mirroring `ArraySeqChap19` over `DoublyLinkedListS<T>`.
//! - Uses `tabulate`, `map`, `append`, `deflate`, `filter`, `iterate`, `reduce`, `scan`, `flatten`.
//! - Includes sequential “parallel injection” analogs using `Mutex` to mirror semantics.

use crate::Types::{B, N, O};
use crate::DoublyLinkedListSeq::{DoublyLinkedListS, DoublyLinkedListSeq};
use crate::DoublyLinkedListSeqChap18::DoublyLinkedListSeqChap18;
use std::sync::Mutex;

/// Algorithms from APAS Chapter 19 as associated functions for DoublyLinkedListS.
pub trait DoublyLinkedListSeqChap19 {
    /// gpt-5-hard: Work Θ(1 + Σ i W(f(i))), Span Θ(1 + Σ i S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> DoublyLinkedListS<T>;
    /// gpt-5-hard: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    fn map<T, U>(a: &DoublyLinkedListS<T>, f: impl Fn(&T) -> U) -> DoublyLinkedListS<U>;
    /// gpt-5-hard: Work Θ(1 + i), Span Θ(1 + i) (list traversal).
    fn select<'a, T>(a: &'a DoublyLinkedListS<T>, b: &'a DoublyLinkedListS<T>, i: N) -> Option<&'a T>;
    /// gpt-5-hard: Work Θ(1 + |a|), Span Θ(1 + |a|).
    fn append<T: Clone + Eq>(a: &DoublyLinkedListS<T>, b: &DoublyLinkedListS<T>) -> DoublyLinkedListS<T>;
    /// Variant with weaker bound on T.
    fn append2<T: Clone>(a: &DoublyLinkedListS<T>, b: &DoublyLinkedListS<T>) -> DoublyLinkedListS<T>;
    /// gpt-5-hard: Work/Span Θ(1).
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> DoublyLinkedListS<T>;
    /// gpt-5-hard: Work Θ(1 + Σ x W(f(x))), Span Θ(1 + Σ x S(f(x))).
    fn filter<T: Clone + Eq>(a: &DoublyLinkedListS<T>, f: impl Fn(&T) -> B) -> DoublyLinkedListS<T>;
    /// gpt-5-hard: Work/Span sums over trace T(−).
    fn iterate<T: Clone + Eq, A: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    /// gpt-5-hard: linear work/span under list traversal assumptions.
    fn reduce<T: Clone + Eq, F>(a: &DoublyLinkedListS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    fn scan<T: Clone + Eq, F>(a: &DoublyLinkedListS<T>, f: &F, id: T) -> (DoublyLinkedListS<T>, T) where F: Fn(&T, &T) -> T;
    /// gpt-5-hard: Work Θ(1 + |a| + Σ x |x|), Span Θ(1 + |a| + Σ x |x|).
    fn flatten<T: Clone + Eq>(s: &DoublyLinkedListS<DoublyLinkedListS<T>>) -> DoublyLinkedListS<T>;
    /// gpt-5-hard: Work Θ(1 + |a| + |b|), Span Θ(1 + |a| + |b|).
    fn inject<T: Clone + Eq>(values: &DoublyLinkedListS<T>, changes: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T>;
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut DoublyLinkedListS<(T, N)>, changes: &DoublyLinkedListS<(N, T)>, change_index: N);
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &DoublyLinkedListS<T>, changes: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T>;
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &DoublyLinkedListS<Mutex<(T, N)>>,
        changes: &DoublyLinkedListS<(N, T)>, change_index: N);
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &DoublyLinkedListS<T>, changes: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T>;
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &DoublyLinkedListS<Mutex<(T, N)>>,
        changes: &DoublyLinkedListS<(N, T)>, change_index: N);
}

impl<T2> DoublyLinkedListSeqChap19 for DoublyLinkedListS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> DoublyLinkedListS<T> { <DoublyLinkedListS<T> as DoublyLinkedListSeqChap18>::tabulate(f, n) }
    fn map<T, U>(a: &DoublyLinkedListS<T>, f: impl Fn(&T) -> U) -> DoublyLinkedListS<U> { <DoublyLinkedListS<U> as DoublyLinkedListSeqChap18>::tabulate(|i| f(a.nth(i)), a.length()) }
    fn select<'a, T>(a: &'a DoublyLinkedListS<T>, b: &'a DoublyLinkedListS<T>, index: N) -> Option<&'a T> {
        if index < a.length() { Some(a.nth(index)) } else { let offset = index - a.length(); if offset < b.length() { Some(b.nth(offset)) } else { None } }
    }
    fn append<T: Clone + Eq>(a: &DoublyLinkedListS<T>, b: &DoublyLinkedListS<T>) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(x.clone()); }
        for x in b.data.iter() { out.push_back(x.clone()); }
        DoublyLinkedListS { data: out }
    }
    fn append2<T: Clone>(a: &DoublyLinkedListS<T>, b: &DoublyLinkedListS<T>) -> DoublyLinkedListS<T> {
        let mut out = std::collections::LinkedList::new();
        for x in a.data.iter() { out.push_back(x.clone()); }
        for x in b.data.iter() { out.push_back(x.clone()); }
        DoublyLinkedListS { data: out }
    }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> DoublyLinkedListS<T> {
        let keep = f(x) == true;
        if keep { <DoublyLinkedListS<T> as DoublyLinkedListSeq<T>>::singleton(x.clone()) } else { <DoublyLinkedListS<T> as DoublyLinkedListSeq<T>>::empty() }
    }
    fn filter<T: Clone + Eq>(a: &DoublyLinkedListS<T>, f: impl Fn(&T) -> B) -> DoublyLinkedListS<T> {
        let mapped: DoublyLinkedListS<DoublyLinkedListS<T>> = <DoublyLinkedListS<DoublyLinkedListS<T>> as DoublyLinkedListSeqChap19>::tabulate(|i| Self::deflate(|elt| f(elt), a.nth(i)), a.length());
        <DoublyLinkedListS<T> as DoublyLinkedListSeqChap18>::flatten(&mapped)
    }
    fn iterate<T: Clone + Eq, A: Clone>(a: &DoublyLinkedListS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { <DoublyLinkedListS<T2> as DoublyLinkedListSeqChap18>::iterate(a, f, x) }
    fn reduce<T: Clone + Eq, F>(a: &DoublyLinkedListS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { <DoublyLinkedListS<T2> as DoublyLinkedListSeqChap18>::reduce(a, f, id) }
    fn scan<T: Clone + Eq, F>(a: &DoublyLinkedListS<T>, f: &F, id: T) -> (DoublyLinkedListS<T>, T) where F: Fn(&T, &T) -> T { <DoublyLinkedListS<T2> as DoublyLinkedListSeqChap18>::scan(a, f, id) }
    fn flatten<T: Clone + Eq>(s: &DoublyLinkedListS<DoublyLinkedListS<T>>) -> DoublyLinkedListS<T> { <DoublyLinkedListS<T> as DoublyLinkedListSeqChap18>::flatten(s) }
    fn inject<T: Clone + Eq>(values: &DoublyLinkedListS<T>, changes: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T> { <DoublyLinkedListS<T2> as DoublyLinkedListSeqChap18>::inject(values, changes) }
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut DoublyLinkedListS<(T, N)>, changes: &DoublyLinkedListS<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location < values_with_change_number.length() {
            let (_, change_number) = values_with_change_number.nth(change_location).clone();
            if change_index < change_number { let _ = values_with_change_number.set(change_location, (change_value, change_index)); }
        }
    }
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &DoublyLinkedListS<T>, changes: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T> {
        let sequence_length = values.length();
        let values_with_change_number: DoublyLinkedListS<Mutex<(T, N)>> = <DoublyLinkedListS<Mutex<(T, N)>> as DoublyLinkedListSeqChap19>::tabulate(|index| Mutex::new((values.nth(index).clone(), sequence_length)), sequence_length);
        std::thread::scope(|scope| { for change_index in 0..changes.length() { let values_ref = &values_with_change_number; let changes_ref = changes; scope.spawn(move || { Self::AtomicWriteLowestChangeNumberWins(values_ref, changes_ref, change_index); }); } });
        <DoublyLinkedListS<T> as DoublyLinkedListSeqChap19>::tabulate(|index| { let guard = values_with_change_number.nth(index).lock().unwrap(); guard.0.clone() }, sequence_length)
    }
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &DoublyLinkedListS<Mutex<(T, N)>>, changes: &DoublyLinkedListS<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone(); if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap(); let (ref mut current_value, ref mut current_change_number) = *guard; if change_index < *current_change_number { *current_value = change_value; *current_change_number = change_index; }
    }
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &DoublyLinkedListS<T>, changes: &DoublyLinkedListS<(N, T)>) -> DoublyLinkedListS<T> {
        let sequence_length = values.length();
        let values_with_change_number: DoublyLinkedListS<Mutex<(T, N)>> = <DoublyLinkedListS<Mutex<(T, N)>> as DoublyLinkedListSeqChap19>::tabulate(|index| Mutex::new((values.nth(index).clone(), 0)), sequence_length);
        std::thread::scope(|scope| { for change_index in 0..changes.length() { let values_ref = &values_with_change_number; let changes_ref = changes; scope.spawn(move || { Self::AtomicWriteHighestChangeNumberWins(values_ref, changes_ref, change_index); }); } });
        <DoublyLinkedListS<T> as DoublyLinkedListSeqChap19>::tabulate(|index| { let guard = values_with_change_number.nth(index).lock().unwrap(); guard.0.clone() }, sequence_length)
    }
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &DoublyLinkedListS<Mutex<(T, N)>>, changes: &DoublyLinkedListS<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone(); if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap(); let (ref mut current_value, ref mut current_change_number) = *guard; if change_index >= *current_change_number { *current_value = change_value; *current_change_number = change_index; }
    }
}


