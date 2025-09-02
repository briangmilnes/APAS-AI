//! Chapter 19 algorithms as trait methods over `ArrayS<T>` (no free functions).
//!
//! Abstract:
//! - Provides trait `ArraySeqChap19` implemented for `ArrayS<T>` with Chapter 19 algorithms.
//! - Uses only allowed primitives: `length`, `nth`, `tabulate`, `flatten`, `inject`, `ninject`.
//!   The main deviation is `subseq`: expected Θ(1) with views, but ours copies due to `Box<[T]>`.
//!
//! Contents:
//! - Trait `ArraySeqChap19`: `tabulate`, `map`, `select`, `append`, `append2`, `deflate`, `filter`,
//!   `iterate`, `reduce`, `scan`, `flatten`, `inject`, `atomicWrite`, `inject_parallel2`,
//!   `AtomicWriteLowestChangeNumberWins`, `ninject_parallel2`, `AtomicWriteHighestChangeNumberWins`.
//! - Impl `ArraySeqChap19 for ArrayS<T>`: allocation-safe, no `unsafe`, consistent naming.
//! - Complexity notation uses Θ and Σ with explicit indices (e.g., Σ i=0..n-1).

use crate::Types::{B, N, O};
use crate::ArraySeq::{ArrayS, ArraySeq};
use crate::ArraySeqChap18::ArraySeqChap18;
use std::sync::Mutex;

/// Algorithms from APAS Chapter 19 as associated functions.
pub trait ArraySeqChap19 {
    // Base utilities used by chapter 19
    /// Tabulate: build sequence of length `n` with i-th element `f(i)`. <br/>
    /// Work: Θ(n + Σ i=0..n-1 W(f(i))), Span: Θ(1 + max i=0..n-1 S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayS<T>;

    // 19.1–19.7
    /// Map: apply `f` to each element. <br/>
    /// Work: Θ(|a| + Σ x∈a W(f(x))), Span: Θ(1 + max x∈a S(f(x))).
    fn map<T, U>(a: &ArrayS<T>, f: impl Fn(&T) -> U) -> ArrayS<U>;
    /// Select: return reference to i-th element of `a ++ b` if exists. <br/>
    /// Work Θ(1), Span Θ(1).
    fn select<'a, T>(a: &'a ArrayS<T>, b: &'a ArrayS<T>, i: N) -> Option<&'a T>;
    /// Append: concatenate `a` and `b`. <br/>
    /// Work Θ(|a|+|b|), Span Θ(1).
    fn append<T: Clone + Eq>(a: &ArrayS<T>, b: &ArrayS<T>) -> ArrayS<T>;
    /// Append2: same as append, different bound on `T`. <br/>
    /// Work Θ(|a|+|b|), Span Θ(1).
    fn append2<T: Clone>(a: &ArrayS<T>, b: &ArrayS<T>) -> ArrayS<T>;
    /// Deflate: keep `x` if predicate true, else empty. <br/>
    /// Work Θ(1), Span Θ(1).
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayS<T>;
    /// Filter: keep elements where `f(x)` is True. <br/>
    /// Work: Θ(|a| + Σ i=0..|a|-1 W(f(a[i]))), Span: Θ(1 + max i=0..|a|-1 S(f(a[i]))).
    fn filter<T: Clone + Eq>(a: &ArrayS<T>, f: impl Fn(&T) -> B) -> ArrayS<T>;
 
    // 19.8–19.10 and others
    /// Iterate (left fold): sequential accumulation with `f`. <br/>
    /// Expected: Work Θ(|a| + Σ i=0..|a|-1 W(f)), Span Θ(|a| + max S(f)). <br/>
    /// BUG: not right performance — recursion uses subseq that copies, yielding Θ(|a|^2) work.
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    /// Reduce (associative `f`, identity `id`): divide-and-conquer. <br/>
    /// Expected: Work Θ(|a|), Span Θ(lg|a|). <br/>
    /// BUG: not right performance — building subproblems via copying subseq causes Θ(|a| lg|a|) work.
    fn reduce<T: Clone + Eq, F>(a: &ArrayS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    /// Scan: parallel prefix. <br/>
    /// Work Θ(|a|), Span Θ(lg|a|) under standard assumptions (nth/length O(1)).
    fn scan<T: Clone + Eq, F>(a: &ArrayS<T>, f: &F, id: T) -> (ArrayS<T>, T) where F: Fn(&T, &T) -> T;
    /// Flatten: concatenate all inner sequences. <br/>
    /// Work: Θ(Σ x∈s |x|), Span: Θ(1).
    fn flatten<T: Clone + Eq>(s: &ArrayS<ArrayS<T>>) -> ArrayS<T>;

    // 19.16/19.17 parallel injection helpers
    /// Inject (first-wins): apply updates into values. <br/>
    /// Work Θ(|values|+|changes|). Span Θ(|changes|) in this sequential impl; see parallel variant below.
    fn inject<T: Clone + Eq>(values: &ArrayS<T>, changes: &ArrayS<(N, T)>) -> ArrayS<T>;
    /// Atomic write primitive for parallel injection. <br/>
    /// Work Θ(1), Span Θ(1).
    fn atomicWrite<T: Clone + Eq>(
        values_with_change_number: &mut ArrayS<(T, N)>,
        changes: &ArrayS<(N, T)>,
        change_index: N,
    );
    /// Parallel inject (lowest change number wins): <br/>
    /// Work Θ(|values|+|changes|), Span Θ(1) in PRAM model (ignoring thread scheduling overhead).
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayS<T>, changes: &ArrayS<(N, T)>) -> ArrayS<T>;
    /// Atomic write (lowest wins): constant work/span per update under idealized model.
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(
        values_with_change_number: &ArrayS<Mutex<(T, N)>>,
        changes: &ArrayS<(N, T)>,
        change_index: N,
    );
    /// Parallel ninject (highest change number wins): <br/>
    /// Work Θ(|values|+|changes|), Span Θ(1) in PRAM model.
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayS<T>, changes: &ArrayS<(N, T)>) -> ArrayS<T>;
    /// Atomic write (highest wins): constant work/span per update under idealized model.
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(
        values_with_change_number: &ArrayS<Mutex<(T, N)>>,
        changes: &ArrayS<(N, T)>,
        change_index: N,
    );
}

impl<T2> ArraySeqChap19 for ArrayS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayS<T> {
        ArrayS {
            data: (0..n)
                .map(|i| f(i))
                .collect::<Vec<T>>()
                .into_boxed_slice(),
        }
    }

    fn map<T, U>(a: &ArrayS<T>, f: impl Fn(&T) -> U) -> ArrayS<U> {
        <ArrayS<U> as ArraySeqChap19>::tabulate(|index| f(a.nth(index)), a.length())
    }
    fn select<'a, T>(a: &'a ArrayS<T>, b: &'a ArrayS<T>, index: N) -> Option<&'a T> {
        if index < a.length() {
            Some(a.nth(index))
        } else {
            let offset = index - a.length();
            if offset < b.length() { Some(b.nth(offset)) } else { None }
        }
    }
    fn append<T: Clone + Eq>(a: &ArrayS<T>, b: &ArrayS<T>) -> ArrayS<T> {
        <ArrayS<T> as ArraySeqChap19>::tabulate(
            |index| Self::select(a, b, index).unwrap().clone(),
            a.length() + b.length(),
        )
    }
    fn append2<T: Clone>(a: &ArrayS<T>, b: &ArrayS<T>) -> ArrayS<T> {
        <ArrayS<T> as ArraySeqChap19>::tabulate(
            |index| Self::select(a, b, index).unwrap().clone(),
            a.length() + b.length(),
        )
    }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArrayS<T> {
        let keep = f(x) == B::True;
        <ArrayS<T> as ArraySeqChap18>::tabulate(|_| x.clone(), if keep { 1 } else { 0 })
    }
    fn filter<T: Clone + Eq>(a: &ArrayS<T>, f: impl Fn(&T) -> B) -> ArrayS<T> {
        let mapped: ArrayS<ArrayS<T>> = <ArrayS<ArrayS<T>> as ArraySeqChap19>::tabulate(
            |index| Self::deflate(|elt| f(elt), a.nth(index)),
            a.length(),
        );
        <ArrayS<T> as ArraySeqChap18>::flatten(&mapped)
    }

    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let input_length = a.length();
        if input_length == 0 {
            x
        } else if input_length == 1 {
            f(&x, a.nth(0))
        } else {
            let first_step_result = f(&x, a.nth(0));
            let rest_subseq = a.subseq(1, input_length - 1);
            // BUG: VEC copy — convert view to owned sequence for recursive call
            let rest_owned = ArrayS::from_vec(rest_subseq.to_vec());
            <ArrayS<T2> as crate::ArraySeqChap19::ArraySeqChap19>::iterate(
                &rest_owned,
                f,
                first_step_result,
            )
        }
    }
    fn reduce<T: Clone + Eq, F>(a: &ArrayS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T {
        let input_length = a.length();
        if input_length == 0 {
            id
        } else if input_length == 1 {
            a.nth(0).clone()
        } else {
            let mid_index = input_length / 2;
            let left_view = a.subseq(0, mid_index);
            let right_view = a.subseq(mid_index, input_length - mid_index);
            // BUG: VEC copy — convert views to owned sequences for recursive calls
            let left_owned = ArrayS::from_vec(left_view.to_vec());
            let right_owned = ArrayS::from_vec(right_view.to_vec());
            let left_result = <ArrayS<T2> as crate::ArraySeqChap19::ArraySeqChap19>::reduce(
                &left_owned,
                f,
                id.clone(),
            );
            let right_result = <ArrayS<T2> as crate::ArraySeqChap19::ArraySeqChap19>::reduce(
                &right_owned,
                f,
                id,
            );
            f(&left_result, &right_result)
        }
    }
    fn scan<T: Clone + Eq, F>(a: &ArrayS<T>, f: &F, id: T) -> (ArrayS<T>, T) where F: Fn(&T, &T) -> T {
        let input_length = a.length();
        if input_length == 0 { (<ArrayS<T> as ArraySeqChap19>::tabulate(|_| id.clone(), 0), id) }
        else if input_length == 1 { (<ArrayS<T> as ArraySeqChap19>::tabulate(|_| id.clone(), 1), a.nth(0).clone()) }
        else {
            let half_length = input_length / 2;
            let pairwise_reductions = <ArrayS<T> as ArraySeqChap19>::tabulate(
                |index| {
                    let left_elt = a.nth(2 * index);
                    let right_elt = a.nth(2 * index + 1);
                    f(left_elt, right_elt)
                },
                half_length,
            );
            let (reductions, total) = <ArrayS<T2> as crate::ArraySeqChap19::ArraySeqChap19>::scan(
                &pairwise_reductions,
                f,
                id,
            );
            let prefixes = <ArrayS<T> as ArraySeqChap19>::tabulate(
                |index| {
                    if index % 2 == 0 {
                        reductions.nth(index / 2).clone()
                    } else {
                        let prev_elt = a.nth(index - 1);
                        let base = reductions.nth(index / 2);
                        f(base, prev_elt)
                    }
                },
                input_length,
            );
            (prefixes, total)
        }
    }
    fn flatten<T: Clone + Eq>(s: &ArrayS<ArrayS<T>>) -> ArrayS<T> { <ArrayS<T> as ArraySeqChap18>::flatten(s) }

    // BUG: perf mismatch with APAS text; book uses degree, which is odd here.
    fn inject<T: Clone + Eq>(values: &ArrayS<T>, changes: &ArrayS<(N, T)>) -> ArrayS<T> {
        let sequence_length = values.length();
        let mut values_with_change_number: ArrayS<(T, N)> = <ArrayS<(T, N)> as ArraySeqChap19>::tabulate(
            |index| (values.nth(index).clone(), sequence_length),
            sequence_length,
        );
        for change_index in 0..changes.length() {
            Self::atomicWrite(&mut values_with_change_number, changes, change_index);
        }
        <ArrayS<T> as ArraySeqChap19>::tabulate(
            |index| values_with_change_number.nth(index).0.clone(),
            sequence_length,
        )
    }
    fn atomicWrite<T: Clone + Eq>(
        values_with_change_number: &mut ArrayS<(T, N)>,
        changes: &ArrayS<(N, T)>,
        change_index: N,
    ) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location < values_with_change_number.length() {
            let (_, change_number) = values_with_change_number.nth(change_location).clone();
            if change_index < change_number {
                let _ = values_with_change_number.set(change_location, (change_value, change_index));
            }
        }
    }
    // BUG: perf mismatch with APAS text; book uses degree, which is odd here.
    fn inject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayS<T>, changes: &ArrayS<(N, T)>) -> ArrayS<T> {
        let sequence_length = values.length();
        let values_with_change_number: ArrayS<Mutex<(T, N)>> = <ArrayS<Mutex<(T, N)>> as ArraySeqChap19>::tabulate(
            |index| Mutex::new((values.nth(index).clone(), sequence_length)),
            sequence_length,
        );
        std::thread::scope(|scope| {
            for change_index in 0..changes.length() {
                let values_ref: &ArrayS<Mutex<(T, N)>> = &values_with_change_number;
                let changes_ref: &ArrayS<(N, T)> = changes;
                scope.spawn(move || {
                    Self::AtomicWriteLowestChangeNumberWins(values_ref, changes_ref, change_index);
                });
            }
        });
        <ArrayS<T> as ArraySeqChap19>::tabulate(
            |index| {
                let guard = values_with_change_number.nth(index).lock().unwrap();
                guard.0.clone()
            },
            sequence_length,
        )
    }
    fn AtomicWriteLowestChangeNumberWins<T: Clone + Eq + Send>(
        values_with_change_number: &ArrayS<Mutex<(T, N)>>,
        changes: &ArrayS<(N, T)>,
        change_index: N,
    ) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap();
        let (ref mut current_value, ref mut current_change_number) = *guard;
        if change_index < *current_change_number {
            *current_value = change_value;
            *current_change_number = change_index;
        }
    }
    // BUG: perf mismatch with APAS text; book uses degree, which is odd here.
    fn ninject_parallel2<T: Clone + Eq + Send + Sync>(values: &ArrayS<T>, changes: &ArrayS<(N, T)>) -> ArrayS<T> {
        let sequence_length = values.length();
        let values_with_change_number: ArrayS<Mutex<(T, N)>> = <ArrayS<Mutex<(T, N)>> as ArraySeqChap19>::tabulate(
            |index| Mutex::new((values.nth(index).clone(), 0)),
            sequence_length,
        );
        std::thread::scope(|scope| {
            for change_index in 0..changes.length() {
                let values_ref: &ArrayS<Mutex<(T, N)>> = &values_with_change_number;
                let changes_ref: &ArrayS<(N, T)> = changes;
                scope.spawn(move || {
                    Self::AtomicWriteHighestChangeNumberWins(values_ref, changes_ref, change_index);
                });
            }
        });
        <ArrayS<T> as ArraySeqChap19>::tabulate(
            |index| {
                let guard = values_with_change_number.nth(index).lock().unwrap();
                guard.0.clone()
            },
            sequence_length,
        )
    }
    fn AtomicWriteHighestChangeNumberWins<T: Clone + Eq + Send>(
        values_with_change_number: &ArrayS<Mutex<(T, N)>>,
        changes: &ArrayS<(N, T)>,
        change_index: N,
    ) {
        let (change_location, change_value) = changes.nth(change_index).clone();
        if change_location >= values_with_change_number.length() { return; }
        let mut guard = values_with_change_number.nth(change_location).lock().unwrap();
        let (ref mut current_value, ref mut current_change_number) = *guard;
        if change_index >= *current_change_number {
            *current_value = change_value;
            *current_change_number = change_index;
        }
    }
}


