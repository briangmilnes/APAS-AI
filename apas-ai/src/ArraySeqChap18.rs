//! Chapter 18 algorithms as trait methods over `ArrayS<T>` (no free functions).
//!
//! Abstract:
//! - Defines trait `ArraySeqChap18` with Chapter 18 operations over `ArrayS<T>`.
//! - Provides sequential implementations using only allowed primitives from `ArraySeq`.
//! - Focus on clarity and correctness; some routines are Θ(n^2) where noted.

use crate::Types::{B, N, O};
use crate::ArraySeq::{ArrayS, ArraySeq};

/// Algorithms from APAS Chapter 18 built from Array Sequences.
pub trait ArraySeqChap18 {
    /// Definition 18.1 (tabulate). Build a sequence of length `n` where the i-th element is `f(i)`. <br/>
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayS<T>;

    /// Definition 18.1 (map). Apply `f` to each element, returning a new sequence of results. <br/>
    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x))).
    fn map<T, U: Clone>(a: &ArrayS<T>, f: impl Fn(&T) -> U) -> ArrayS<U>;

    /// Definition 18.1 (append). Concatenate `a` and `b` preserving order. <br/>
    /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1).
    fn append<T: Clone + Eq>(a: &ArrayS<T>, b: &ArrayS<T>) -> ArrayS<T>;

    /// Definition 18.1 (filter). Keep elements `x` for which `pred(x)` returns `True`. <br/>
    /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i=0..|a|-1 S(pred(a[i]))).
    fn filter<T: Clone + Eq>(a: &ArrayS<T>, pred: impl Fn(&T) -> B) -> ArrayS<T>;

    /// Update `self[index]` to `item` in place if in bounds, and return `self` for chaining. <br/>
    /// APAS: Work Θ(1 + |a|), Span Θ(1).
    fn update<T: Clone + Eq>(a: &mut ArrayS<T>, item_at: (N, T)) -> &mut ArrayS<T>;

    /// Definition 18.1 (inject). Apply all updates, keeping the first update to any index. <br/>
    /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates))).
    fn inject<T: Clone + Eq>(a: &ArrayS<T>, updates: &ArrayS<(N, T)>) -> ArrayS<T>;

    /// Definition 18.1 (ninject). Apply all updates, last write per index wins. <br/>
    /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1).
    fn ninject<T: Clone + Eq>(a: &ArrayS<T>, updates: &ArrayS<(N, T)>) -> ArrayS<T>;

    /// Left-to-right fold using `f`, starting from seed `x`. <br/>
    /// APAS: Work Θ(1 + Σ (y,z)∈T(−) W(f(y, z))), Span Θ(1 + Σ (y,z)∈T(−) S(f(y, z))).
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

    /// Produce the sequence of prefix accumulations and the final value using `f`. <br/>
    /// APAS: Work Θ(|a|), Span Θ(|a|).
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &ArrayS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayS<A>, A);

    /// Divide-and-conquer reduction using associative `f` with identity `id`. <br/>
    /// APAS: Work Θ(1 + Σ (y,z)∈T(−) W(f(y, z))), Span Θ(lg|a| · max (y,z)∈T(−) S(f(y, z))).
    fn reduce<T: Clone + Eq>(a: &ArrayS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

    /// Sequential scan producing all prefixes under `f` together with the total. <br/>
    /// APAS: Work Θ(|a|), Span Θ(lg|a|).
    fn scan<T: Clone + Eq>(a: &ArrayS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayS<T>, T);

    /// Flatten one level of nested sequences by concatenating all inner sequences. <br/>
    /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|).
    fn flatten<T: Clone + Eq>(ss: &ArrayS<ArrayS<T>>) -> ArrayS<T>;

    /// Group pairs by key according to `cmp`, collecting values into sequences per key. <br/>
    /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|).
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(
        a: &ArrayS<(A, Bv)>,
        cmp: impl Fn(&A, &A) -> O,
    ) -> ArrayS<(A, ArrayS<Bv>)>;
}

impl<T2> ArraySeqChap18 for ArrayS<T2> {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i))).
    /// ChatGPT-5-hard: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + Σ i=0..n-1 S(f(i))).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayS<T> {
        let data: Vec<T> = (0..n).map(|i| f(i)).collect();
        ArrayS { data: data.into_boxed_slice() }
    }

    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x))).
    /// ChatGPT-5-hard: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + Σ x∈a S(f(x))).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn map<T, U: Clone>(a: &ArrayS<T>, f: impl Fn(&T) -> U) -> ArrayS<U> {
        let input_length = a.length();
        if input_length == 0 { return <ArrayS<U> as ArraySeq<U>>::empty(); }
        let first_mapped_elt = f(a.nth(0));
        let mut result_seq = <ArrayS<U> as ArraySeq<U>>::new(input_length, first_mapped_elt.clone());
        let _ = result_seq.set(0, first_mapped_elt);
        for index in 1..input_length { let _ = result_seq.set(index, f(a.nth(index))); }
        result_seq
    }

    /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1).
    /// ChatGPT-5-hard: Work Θ(1 + |a| + |b|), Span Θ(1 + |a| + |b|).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn append<T: Clone + Eq>(a: &ArrayS<T>, b: &ArrayS<T>) -> ArrayS<T> {
        let left_length = a.length();
        let right_length = b.length();
        let combined_length = left_length + right_length;
        if combined_length == 0 { return <ArrayS<T> as ArraySeq<T>>::empty(); }
        let initial_elt = if left_length > 0 { a.nth(0).clone() } else { b.nth(0).clone() };
        let mut result_seq = <ArrayS<T> as ArraySeq<T>>::new(combined_length, initial_elt.clone());
        for left_index in 0..left_length { let _ = result_seq.set(left_index, a.nth(left_index).clone()); }
        for right_index in 0..right_length {
            let _ = result_seq.set(
                left_length + right_index,
                b.nth(right_index).clone(),
            );
        }
        result_seq
    }

    /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i=0..|a|-1 S(pred(a[i]))).
    /// ChatGPT-5-hard: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(1 + Σ i=0..|a|-1 S(pred(a[i]))).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn filter<T: Clone + Eq>(a: &ArrayS<T>, pred: impl Fn(&T) -> B) -> ArrayS<T> {
        let input_length = a.length();
        if input_length == 0 { return <ArrayS<T> as ArraySeq<T>>::empty(); }
        // Compute mask to evaluate pred once per elt
        let keep_mask: ArrayS<B> = <ArrayS<B> as ArraySeqChap18>::map(a, |x| pred(x));
        // Count kept
        let mut kept_count: N = 0;
        for index in 0..input_length { if *keep_mask.nth(index) == B::True { kept_count += 1; } }
        if kept_count == 0 { return <ArrayS<T> as ArraySeq<T>>::empty(); }
        // Seed allocation with first kept elt
        let mut first_kept_index: N = 0;
        while first_kept_index < input_length && *keep_mask.nth(first_kept_index) != B::True { first_kept_index += 1; }
        let initial_elt = a.nth(first_kept_index).clone();
        let mut result_seq = <ArrayS<T> as ArraySeq<T>>::new(kept_count, initial_elt.clone());
        // Fill output
        let mut write_index: N = 0;
        for index in 0..input_length {
            if *keep_mask.nth(index) == B::True {
                let _ = result_seq.set(write_index, a.nth(index).clone());
                write_index += 1;
            }
        }
        result_seq
    }

    /// APAS: Work Θ(1 + |a|), Span Θ(1).
    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn update<T: Clone + Eq>(a: &mut ArrayS<T>, (index, item): (N, T)) -> &mut ArrayS<T> { a.update((index, item)) }

    /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates))).
    /// ChatGPT-5-hard: Work Θ(1 + |a| + |updates|), Span Θ(1).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn inject<T: Clone + Eq>(a: &ArrayS<T>, updates: &ArrayS<(N, T)>) -> ArrayS<T> {
        let mut new_elts = a.data.clone();
        let mut seen_indices = std::collections::HashSet::new();
        for update_iter in 0..updates.length() {
            let (update_index, update_value) = updates.nth(update_iter);
            if *update_index < new_elts.len() && !seen_indices.contains(update_index) {
                new_elts[*update_index] = update_value.clone();
                seen_indices.insert(*update_index);
            }
        }
        ArrayS { data: new_elts }
    }

    /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1).
    fn ninject<T: Clone + Eq>(a: &ArrayS<T>, updates: &ArrayS<(N, T)>) -> ArrayS<T> {
        let mut new_elts = a.data.clone();
        for update_iter in 0..updates.length() {
            let (update_index, update_value) = updates.nth(update_iter);
            if *update_index < new_elts.len() { new_elts[*update_index] = update_value.clone(); }
        }
        ArrayS { data: new_elts }
    }

    /// APAS: Work Θ(1 + Σ (y,z)∈T(−) W(f(y, z))), Span Θ(1 + Σ (y,z)∈T(−) S(f(y, z))).
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut accumulator = x;
        for index in 0..a.length() { accumulator = f(&accumulator, a.nth(index)); }
        accumulator
    }

    /// APAS: Work Θ(|a|), Span Θ(|a|).
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &ArrayS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayS<A>, A) {
        let input_length = a.length();
        let mut prefix_value = x;
        if input_length == 0 { return (<ArrayS<A> as ArraySeq<A>>::empty(), prefix_value); }
        let mut prefix_values = <ArrayS<A> as ArraySeq<A>>::new(input_length, prefix_value.clone());
        for index in 0..input_length {
            let _ = prefix_values.set(index, prefix_value.clone());
            prefix_value = f(&prefix_value, a.nth(index));
        }
        (prefix_values, prefix_value)
    }

    /// APAS: Work Θ(1 + Σ (y,z)∈T(−) W(f(y, z))), Span Θ(lg|a| · max (y,z)∈T(−) S(f(y, z))).
    /// ChatGPT-5-hard: Work Θ(Σ (y,z)∈T(−) W(f(y, z))), Span Θ(lg|a| · max (y,z)∈T(−) S(f(y, z))).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn reduce<T: Clone + Eq>(a: &ArrayS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        fn reduce_slice<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = s.len();
            if n == 0 { return id; }
            if n == 1 { return s[0].clone(); }
            let mid = n / 2;
            let l = reduce_slice(&s[..mid], f, id.clone());
            let r = reduce_slice(&s[mid..], f, id);
            f(&l, &r)
        }
        reduce_slice(&a.data[..], f, id)
    }

    /// APAS: Work Θ(|a|), Span Θ(lg|a|).
    /// ChatGPT-5-hard: Work Θ(|a| lg|a|), Span Θ(lg|a|).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn scan<T: Clone + Eq>(a: &ArrayS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayS<T>, T) {
        fn reduce_slice<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = s.len();
            if n == 0 { return id; }
            if n == 1 { return s[0].clone(); }
            let mid = n / 2;
            let l = reduce_slice(&s[..mid], f, id.clone());
            let r = reduce_slice(&s[mid..], f, id);
            f(&l, &r)
        }
        let input_length = a.length();
        let mut prefix_values = if input_length == 0 {
            <ArrayS<T> as ArraySeq<T>>::empty()
        } else {
            <ArrayS<T> as ArraySeq<T>>::new(input_length, id.clone())
        };
        for index in 0..input_length {
            let prefix_result = reduce_slice(&a.data[..index], f, id.clone());
            let _ = prefix_values.set(index, prefix_result);
        }
        let total_result = reduce_slice(&a.data[..], f, id);
        (prefix_values, total_result)
    }

    /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|).
    /// ChatGPT-5-hard: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + |a| + Σ x∈a |x|).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn flatten<T: Clone + Eq>(ss: &ArrayS<ArrayS<T>>) -> ArrayS<T> {
        let outer_length = ss.length();
        let mut total_length: N = 0;
        for outer_index in 0..outer_length { total_length += ss.nth(outer_index).length(); }
        if total_length == 0 { return <ArrayS<T> as ArraySeq<T>>::empty(); }
        // find first elt to seed allocation
        let mut first_elt_opt: Option<T> = None;
        'find: for outer_index in 0..outer_length {
            let inner_seq = ss.nth(outer_index);
            if inner_seq.length() > 0 { first_elt_opt = Some(inner_seq.nth(0).clone()); break 'find; }
        }
        let initial_elt = first_elt_opt.expect("total_length > 0 implies some inner elt exists");
        let mut result_seq = <ArrayS<T> as ArraySeq<T>>::new(total_length, initial_elt.clone());
        let mut write_index: N = 0;
        for outer_index in 0..outer_length {
            let inner_seq = ss.nth(outer_index);
            for inner_index in 0..inner_seq.length() {
                let _ = result_seq.set(write_index, inner_seq.nth(inner_index).clone());
                write_index += 1;
            }
        }
        result_seq
    }

    /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|).
    /// ChatGPT-5-hard: Work Θ(|a| · W(f) · lg|a|), Span Θ(S(f) · lg^2|a|).
    /// BUG: APAS and ChatGPT-5-hard algorithmic analyses differ.
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(
        a: &ArrayS<(A, Bv)>,
        cmp: impl Fn(&A, &A) -> O,
    ) -> ArrayS<(A, ArrayS<Bv>)> {
        let mut grouped_values: Vec<(A, Vec<Bv>)> = Vec::new();
        for index in 0..a.length() {
            let (key, value) = a.nth(index);
            let mut found = false;
            for (group_key, group_values) in &mut grouped_values {
                if cmp(key, group_key) == O::Equal {
                    group_values.push(value.clone());
                    found = true;
                    break;
                }
            }
            if !found { grouped_values.push((key.clone(), vec![value.clone()])); }
        }
        let grouped_seq: Vec<(A, ArrayS<Bv>)> = grouped_values
            .into_iter()
            .map(|(key, values)| (key, ArrayS { data: values.into_boxed_slice() }))
            .collect();
        ArrayS { data: grouped_seq.into_boxed_slice() }
    }
}


