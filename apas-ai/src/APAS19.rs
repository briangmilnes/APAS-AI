//! APAS19: Chapter 19 algorithms as trait methods over `S<T>` (no free functions).
//!
//! Abstract:
//! - Provides trait `APAS19` implemented for `S<T>` with Chapter 19 algorithms.
//! - Uses only allowed primitives: `length`, `nth`, `tabulate`, `flatten`, `inject`, `ninject`.
//! - Work/Span are documented per APAS; deviations are marked with "BUG: not right performance".
//!   The main deviation is `subseq`: expected Θ(1) with views, but ours copies due to `Box<[T]>`.
//!
//! Contents:
//! - Trait `APAS19`: `tabulate`, `map`, `select`, `append`, `append2`, `deflate`, `filter`,
//!   `iterate`, `reduce`, `scan`, `flatten`, `inject`, `atomicWrite`, `inject_parallel2`,
//!   `AtomicWriteLowestChangeNumberWins`, `ninject_parallel2`, `AtomicWriteHighestChangeNumberWins`.
//! - Impl `APAS19 for S<T>`: allocation-safe, no `Vec` or `unsafe`, consistent naming (`ChangeNumber`).
//! - Complexity notation uses Θ and Σ with explicit indices (e.g., Σ i=0..n-1).

use crate::S::{S, B, N, O, Sequence};
use crate::APAS18::APAS18;
use std::sync::Mutex;

/// Algorithms from APAS Chapter 19 as associated functions.
pub trait APAS19 {
    // Base/unnumbered utilities used by chapter 19
    /// Tabulate: build sequence of length `n` with i-th element `f(i)`. <br/>
    /// Work: Θ(n + Σ i=0..n-1 W(f(i))), Span: Θ(1 + max i=0..n-1 S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> S<T>;

    // 19.1–19.7
    /// Map: apply `f` to each element. <br/>
    /// Work: Θ(|a| + Σ x∈a W(f(x))), Span: Θ(1 + max x∈a S(f(x))).
    fn map<T, U>(a: &S<T>, f: impl Fn(&T) -> U) -> S<U>;
    /// Select: return reference to i-th element of `a ++ b` if exists. <br/>
    /// Work Θ(1), Span Θ(1).
    fn select<'a, T>(a: &'a S<T>, b: &'a S<T>, i: N) -> Option<&'a T>;
    /// Append: concatenate `a` and `b`. <br/>
    /// Work Θ(|a|+|b|), Span Θ(1).
    fn append<T: Clone + Eq>(a: &S<T>, b: &S<T>) -> S<T>;
    /// Append2: same as append, different bound on `T`. <br/>
    /// Work Θ(|a|+|b|), Span Θ(1).
    fn append2<T: Clone>(a: &S<T>, b: &S<T>) -> S<T>;
    /// Deflate: keep `x` if predicate true, else empty. <br/>
    /// Work Θ(1), Span Θ(1).
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> S<T>;
    /// Filter: keep elements where `f(x)` is True. <br/>
    /// Work: Θ(|a| + Σ i=0..|a|-1 W(f(a[i]))), Span: Θ(1 + max i=0..|a|-1 S(f(a[i]))).
    fn filter<T: Clone + Eq>(a: &S<T>, f: impl Fn(&T) -> B) -> S<T>;
 
    // 19.8–19.10 and others
    /// Iterate (left fold): sequential accumulation with `f`. <br/>
    /// Expected: Work Θ(|a| + Σ i=0..|a|-1 W(f)), Span Θ(|a| + max S(f)). <br/>
    /// BUG: not right performance — recursion uses subseq that copies, yielding Θ(|a|^2) work.
    fn iterate<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    /// Reduce (associative `f`, identity `id`): divide-and-conquer. <br/>
    /// Expected: Work Θ(|a|), Span Θ(lg|a|). <br/>
    /// BUG: not right performance — building subproblems via copying subseq causes Θ(|a| lg|a|) work.
    fn reduce<T: Clone + Eq, F>(a: &S<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    /// Scan: parallel prefix. <br/>
    /// Work Θ(|a|), Span Θ(lg|a|) under standard assumptions (nth/length O(1)).
    fn scan<T: Clone + Eq, F>(a: &S<T>, f: &F, id: T) -> (S<T>, T) where F: Fn(&T, &T) -> T;
    /// Flatten: concatenate all inner sequences. <br/>
    /// Work: Θ(Σ x∈s |x|), Span: Θ(1).
    fn flatten<T: Clone + Eq>(s: &S<S<T>>) -> S<T>;

    // 19.16/19.17 parallel injection helpers
    /// Inject (first-wins): apply updates into values. <br/>
    /// Work Θ(|values|+|changes|). Span Θ(|changes|) in this sequential impl; see parallel variant below.
    fn inject<T: Clone + Eq>(values: &S<T>, changes: &S<(N, T)>) -> S<T>;
    /// Atomic write primitive for parallel injection. <br/>
    /// Work Θ(1), Span Θ(1).
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut S<(T, N)>, changes: &S<(N, T)>, change_index: N);
    /// Parallel inject (lowest change number wins): <br/>
    /// Work Θ(|values|+|changes|), Span Θ(1) in PRAM model (ignoring thread scheduling overhead).
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &S<T>, changes: &S<(N, T)>) -> S<T>;
    /// Atomic write (lowest wins): constant work/span per update under idealized model.
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &S<Mutex<(T, N)>>, changes: &S<(N, T)>, change_index: N);
    /// Parallel ninject (highest change number wins): <br/>
    /// Work Θ(|values|+|changes|), Span Θ(1) in PRAM model.
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &S<T>, changes: &S<(N, T)>) -> S<T>;
    /// Atomic write (highest wins): constant work/span per update under idealized model.
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &S<Mutex<(T, N)>>, changes: &S<(N, T)>, change_index: N);
}

impl<T2> APAS19 for S<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> S<T> { <S<T> as APAS18>::tabulate(f, n) }

    fn map<T, U>(a: &S<T>, f: impl Fn(&T) -> U) -> S<U> { <S<U> as APAS18>::tabulate(|index| f(a.nth(index)), a.length()) }
    fn select<'a, T>(a: &'a S<T>, b: &'a S<T>, index: N) -> Option<&'a T> {
        if index < a.length() { Some(a.nth(index)) } else { let offset = index - a.length(); if offset < b.length() { Some(b.nth(offset)) } else { None } }
    }
    fn append<T: Clone + Eq>(a: &S<T>, b: &S<T>) -> S<T> {
        <S<T> as APAS18>::tabulate(|index| Self::select(a, b, index).unwrap().clone(), a.length() + b.length())
    }
    fn append2<T: Clone>(a: &S<T>, b: &S<T>) -> S<T> {
        <S<T> as APAS18>::tabulate(|index| Self::select(a, b, index).unwrap().clone(), a.length() + b.length())
    }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> S<T> {
        let keep = f(x) == B::True;
        <S<T> as APAS18>::tabulate(|_| x.clone(), if keep { 1 } else { 0 })
    }
    fn filter<T: Clone + Eq>(a: &S<T>, f: impl Fn(&T) -> B) -> S<T> {
        let mapped: S<S<T>> = <S<S<T>> as APAS18>::tabulate(|index| Self::deflate(|elt| f(elt), a.nth(index)), a.length());
        <S<T> as APAS18>::flatten(&mapped)
    }

    fn iterate<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let input_length = a.length();
        if input_length == 0 { x } else if input_length == 1 { f(&x, a.nth(0)) } else { let first_step_result = f(&x, a.nth(0)); let rest_subseq = a.subseq(1, input_length - 1); <S<T2> as crate::APAS19::APAS19>::iterate(&rest_subseq, f, first_step_result) }
    }
    fn reduce<T: Clone + Eq, F>(a: &S<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T {
        let input_length = a.length();
        if input_length == 0 { id } else if input_length == 1 { a.nth(0).clone() } else {
            let mid_index = input_length / 2; let left_subseq = a.subseq(0, mid_index); let right_subseq = a.subseq(mid_index, input_length - mid_index);
            let left_result = <S<T2> as crate::APAS19::APAS19>::reduce(&left_subseq, f, id.clone()); let right_result = <S<T2> as crate::APAS19::APAS19>::reduce(&right_subseq, f, id); f(&left_result, &right_result)
        }
    }
    fn scan<T: Clone + Eq, F>(a: &S<T>, f: &F, id: T) -> (S<T>, T) where F: Fn(&T, &T) -> T {
        let input_length = a.length();
        if input_length == 0 { (<S<T> as APAS18>::tabulate(|_| id.clone(), 0), id) }
        else if input_length == 1 { (<S<T> as APAS18>::tabulate(|_| id.clone(), 1), a.nth(0).clone()) }
        else {
            let half_length = input_length / 2;
            let pairwise_reductions = <S<T> as APAS18>::tabulate(|index| { let left_elt = a.nth(2 * index); let right_elt = a.nth(2 * index + 1); f(left_elt, right_elt) }, half_length);
            let (reductions, total) = <S<T2> as crate::APAS19::APAS19>::scan(&pairwise_reductions, f, id);
            let prefixes = <S<T> as APAS18>::tabulate(|index| { if index % 2 == 0 { reductions.nth(index / 2).clone() } else { let prev_elt = a.nth(index - 1); let base = reductions.nth(index / 2); f(base, prev_elt) } }, input_length);
            (prefixes, total)
        }
    }
    fn flatten<T: Clone + Eq>(s: &S<S<T>>) -> S<T> { <S<T> as APAS18>::flatten(s) }

    fn inject<T: Clone + Eq>(values: &S<T>, changes: &S<(N, T)>) -> S<T> {
        let sequence_length = values.length();
        let mut values_with_change_number: S<(T, N)> = <S<(T, N)> as APAS18>::tabulate(|index| (values.nth(index).clone(), sequence_length), sequence_length);
        for change_index in 0..changes.length() { Self::atomicWrite(&mut values_with_change_number, changes, change_index); }
        <S<T> as APAS18>::tabulate(|index| values_with_change_number.nth(index).0.clone(), sequence_length)
    }
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut S<(T, N)>, changes: &S<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location < values_with_change_number.length() {
            let (_, change_number) = values_with_change_number.nth(change_location).clone();
            if change_index < change_number { let _ = values_with_change_number.set(change_location, (change_value, change_index)); }
        }
    }
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &S<T>, changes: &S<(N, T)>) -> S<T> {
        let sequence_length = values.length();
        let values_with_change_number: S<Mutex<(T, N)>> = <S<Mutex<(T, N)>> as APAS18>::tabulate(|index| Mutex::new((values.nth(index).clone(), sequence_length)), sequence_length);
        std::thread::scope(|scope| {
            for change_index in 0..changes.length() {
                let values_ref: &S<Mutex<(T, N)>> = &values_with_change_number;
                let changes_ref: &S<(N, T)> = changes;
                scope.spawn(move || { Self::AtomicWriteLowestChangeNumberWins(values_ref, changes_ref, change_index); });
            }
        });
        <S<T> as APAS18>::tabulate(|index| { let guard = values_with_change_number.nth(index).lock().unwrap(); guard.0.clone() }, sequence_length)
    }
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &S<Mutex<(T, N)>>, changes: &S<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap();
        let (ref mut current_value, ref mut current_change_number) = *guard;
        if change_index < *current_change_number { *current_value = change_value; *current_change_number = change_index; }
    }
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &S<T>, changes: &S<(N, T)>) -> S<T> {
        let sequence_length = values.length();
        let values_with_change_number: S<Mutex<(T, N)>> = <S<Mutex<(T, N)>> as APAS18>::tabulate(|index| Mutex::new((values.nth(index).clone(), 0)), sequence_length);
        std::thread::scope(|scope| {
            for change_index in 0..changes.length() {
                let values_ref: &S<Mutex<(T, N)>> = &values_with_change_number;
                let changes_ref: &S<(N, T)> = changes;
                scope.spawn(move || { Self::AtomicWriteHighestChangeNumberWins(values_ref, changes_ref, change_index); });
            }
        });
        <S<T> as APAS18>::tabulate(|index| { let guard = values_with_change_number.nth(index).lock().unwrap(); guard.0.clone() }, sequence_length)
    }
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &S<Mutex<(T, N)>>, changes: &S<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap();
        let (ref mut current_value, ref mut current_change_number) = *guard;
        if change_index >= *current_change_number { *current_value = change_value; *current_change_number = change_index; }
    }
}


