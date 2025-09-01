//! Chapter 19 algorithms as trait methods over `S<T>` (no free functions).

use crate::S::{S, B, N, O, Sequence};
use crate::APAS18::APAS18;
use std::sync::Mutex;

/// Algorithms from APAS Chapter 19 as associated functions.
pub trait APAS19 {
    // Base/unnumbered utilities used by chapter 19
    /// Algorithm 19.13 (subseq). Return `[start, end)` as a view. <br/>
    /// Expected: Work Θ(1), Span Θ(1) with slice/view representation. <br/>
    /// BUG: not right performance — our impl delegates to APAS18::subseq which copies, so Work Θ(end−start), Span Θ(1).
    fn subseq<T: Clone + Eq>(s: &S<T>, start: N, end: N) -> S<T>;
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
    fn subseq<T: Clone + Eq>(s: &S<T>, start: N, end: N) -> S<T> { <S<T> as APAS18>::subseq(s, start, end) }
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> S<T> { <S<T> as APAS18>::tabulate(f, n) }

    fn map<T, U>(a: &S<T>, f: impl Fn(&T) -> U) -> S<U> { <S<U> as APAS18>::tabulate(|i| f(a.nth(i)), a.length()) }
    fn select<'a, T>(a: &'a S<T>, b: &'a S<T>, i: N) -> Option<&'a T> {
        if i < a.length() { Some(a.nth(i)) } else { let j = i - a.length(); if j < b.length() { Some(b.nth(j)) } else { None } }
    }
    fn append<T: Clone + Eq>(a: &S<T>, b: &S<T>) -> S<T> {
        <S<T> as APAS18>::tabulate(|i| Self::select(a, b, i).unwrap().clone(), a.length() + b.length())
    }
    fn append2<T: Clone>(a: &S<T>, b: &S<T>) -> S<T> {
        <S<T> as APAS18>::tabulate(|i| Self::select(a, b, i).unwrap().clone(), a.length() + b.length())
    }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> S<T> {
        let keep = f(x) == B::True;
        <S<T> as APAS18>::tabulate(|_| x.clone(), if keep { 1 } else { 0 })
    }
    fn filter<T: Clone + Eq>(a: &S<T>, f: impl Fn(&T) -> B) -> S<T> {
        let mapped: S<S<T>> = <S<S<T>> as APAS18>::tabulate(|i| Self::deflate(|y| f(y), a.nth(i)), a.length());
        <S<T> as APAS18>::flatten(&mapped)
    }

    fn iterate<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let n = a.length();
        if n == 0 { x } else if n == 1 { f(&x, a.nth(0)) } else { let x1 = f(&x, a.nth(0)); let rest = <S<T> as APAS18>::subseq(a, 1, n); <S<T2> as crate::APAS19::APAS19>::iterate(&rest, f, x1) }
    }
    fn reduce<T: Clone + Eq, F>(a: &S<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T {
        let n = a.length();
        if n == 0 { id } else if n == 1 { a.nth(0).clone() } else {
            let mid = n / 2; let b = <S<T> as APAS18>::subseq(a, 0, mid); let c = <S<T> as APAS18>::subseq(a, mid, n);
            let rb = <S<T2> as crate::APAS19::APAS19>::reduce(&b, f, id.clone()); let rc = <S<T2> as crate::APAS19::APAS19>::reduce(&c, f, id); f(&rb, &rc)
        }
    }
    fn scan<T: Clone + Eq, F>(a: &S<T>, f: &F, id: T) -> (S<T>, T) where F: Fn(&T, &T) -> T {
        let n = a.length();
        if n == 0 { (<S<T> as APAS18>::tabulate(|_| id.clone(), 0), id) }
        else if n == 1 { (<S<T> as APAS18>::tabulate(|_| id.clone(), 1), a.nth(0).clone()) }
        else {
            let half = n / 2;
            let a_prime = <S<T> as APAS18>::tabulate(|i| { let left = a.nth(2 * i); let right = a.nth(2 * i + 1); f(left, right) }, half);
            let (r, t) = <S<T2> as crate::APAS19::APAS19>::scan(&a_prime, f, id);
            let prefixes = <S<T> as APAS18>::tabulate(|i| { if i % 2 == 0 { r.nth(i / 2).clone() } else { let prev = a.nth(i - 1); let base = r.nth(i / 2); f(base, prev) } }, n);
            (prefixes, t)
        }
    }
    fn flatten<T: Clone + Eq>(s: &S<S<T>>) -> S<T> { <S<T> as APAS18>::flatten(s) }

    fn inject<T: Clone + Eq>(values: &S<T>, changes: &S<(N, T)>) -> S<T> {
        let length = values.length();
        let mut values_with_change_number: S<(T, N)> = <S<(T, N)> as APAS18>::tabulate(|index| (values.nth(index).clone(), length), length);
        for change_index in 0..changes.length() { Self::atomicWrite(&mut values_with_change_number, changes, change_index); }
        <S<T> as APAS18>::tabulate(|index| values_with_change_number.nth(index).0.clone(), length)
    }
    fn atomicWrite<T: Clone + Eq>(values_with_change_number: &mut S<(T, N)>, changes: &S<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location < values_with_change_number.length() {
            let (_, change_number) = values_with_change_number.nth(change_location).clone();
            if change_index < change_number { let _ = values_with_change_number.set(change_location, (change_value, change_index)); }
        }
    }
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &S<T>, changes: &S<(N, T)>) -> S<T> {
        let length = values.length();
        let values_with_change_number: S<Mutex<(T, N)>> = <S<Mutex<(T, N)>> as APAS18>::tabulate(|index| Mutex::new((values.nth(index).clone(), length)), length);
        std::thread::scope(|scope| {
            for changeindex in 0..changes.length() {
                let values_ref: &S<Mutex<(T, N)>> = &values_with_change_number;
                let changes_ref: &S<(N, T)> = changes;
                scope.spawn(move || { Self::AtomicWriteLowestChangeNumberWins(values_ref, changes_ref, changeindex); });
            }
        });
        <S<T> as APAS18>::tabulate(|index| { let guard = values_with_change_number.nth(index).lock().unwrap(); guard.0.clone() }, length)
    }
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &S<Mutex<(T, N)>>, changes: &S<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap();
        let (ref mut current_value, ref mut current_change_number) = *guard;
        if change_index < *current_change_number { *current_value = change_value; *current_change_number = change_index; }
    }
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &S<T>, changes: &S<(N, T)>) -> S<T> {
        let length = values.length();
        let values_with_change_number: S<Mutex<(T, N)>> = <S<Mutex<(T, N)>> as APAS18>::tabulate(|index| Mutex::new((values.nth(index).clone(), 0)), length);
        std::thread::scope(|scope| {
            for changeindex in 0..changes.length() {
                let values_ref: &S<Mutex<(T, N)>> = &values_with_change_number;
                let changes_ref: &S<(N, T)> = changes;
                scope.spawn(move || { Self::AtomicWriteHighestChangeNumberWins(values_ref, changes_ref, changeindex); });
            }
        });
        <S<T> as APAS18>::tabulate(|index| { let guard = values_with_change_number.nth(index).lock().unwrap(); guard.0.clone() }, length)
    }
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(values_with_change_number: &S<Mutex<(T, N)>>, changes: &S<(N, T)>, change_index: N) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap();
        let (ref mut current_value, ref mut current_change_number) = *guard;
        if change_index >= *current_change_number { *current_value = change_value; *current_change_number = change_index; }
    }
}


