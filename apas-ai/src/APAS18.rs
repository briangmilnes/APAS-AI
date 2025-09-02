//! Chapter 18 algorithms as trait methods over `S<T>` (no free functions).

use crate::Sequences::{S, B, N, O, Sequence};

/// Algorithms from APAS Chapter 18 as associated functions.
pub trait APAS18 {
    /// Definition 18.1 (tabulate). Build a sequence of length `n` where the i-th element is `f(i)`. <br/>
    /// Work: Θ(n + Σ_{i=0}^{n-1} W(f(i))), Span: Θ(1 + max_{0≤i<n} S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> S<T>;

    /// Definition 18.1 (map). Apply `f` to each element, returning a new sequence of results. <br/>
    /// Work: Θ(|a| + Σ_{x∈a} W(f(x))), Span: Θ(1 + max_{x∈a} S(f(x))).
    fn map<T, U: Clone>(a: &S<T>, f: impl Fn(&T) -> U) -> S<U>;

    /// Definition 18.1 (append). Concatenate `a` and `b` preserving order. <br/>
    /// Work: Θ(|a|+|b|), Span: Θ(1).
    fn append<T: Clone + Eq>(a: &S<T>, b: &S<T>) -> S<T>;

    /// Definition 18.1 (filter). Keep elements `x` for which `pred(x)` returns `True`. <br/>
    /// Work: Θ(|a| + Σ i=0..|a|-1 W(pred(a[i]))), Span: Θ(lg|a| + max i=0..|a|-1 S(pred(a[i]))).
    /// APAS: Work Θ(|a| + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(lg|a| + max i=0..|a|-1 S(f(a[i]))).
    fn filter<T: Clone + Eq>(a: &S<T>, pred: impl Fn(&T) -> B) -> S<T>;

    /// Update `self[index]` to `item` in place if in bounds, and return `self` for chaining. <br/>
    /// Work: Θ(1 + |a|), Span: Θ(1).
    fn update<T: Clone + Eq>(a: &mut S<T>, item_at: (N, T)) -> &mut S<T>;

    /// Definition 18.1 (inject). Apply all updates, keeping the first update to any index. <br/>
    /// Work: Θ(|a|+|updates|), Span: Θ(1).
    fn inject<T: Clone + Eq>(a: &S<T>, updates: &S<(N, T)>) -> S<T>;

    /// Definition 18.1 (ninject). Apply all updates, last write per index wins. <br/>
    /// Work: Θ(|a|+|updates|), Span: Θ(1).
    fn ninject<T: Clone + Eq>(a: &S<T>, updates: &S<(N, T)>) -> S<T>;

    /// Left-to-right fold using `f`, starting from seed `x`. <br/>
    /// Work: Θ(|a| + Σ i=0..|a|-1 W(f)), Span: Θ(|a| + max S(f)).
    fn iterate<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

    /// Produce the sequence of prefix accumulations and the final value using `f`. <br/>
    /// Work: Θ(|a|), Span: Θ(|a|).
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> (S<A>, A);

    /// Divide-and-conquer reduction using associative `f` with identity `id`. <br/>
    /// Work: Θ(|a|), Span: Θ(lg|a|). <br/>
    /// APAS: Work Θ(Σ W(f(y, z))), Span Θ(lg|a| · max S(f(y, z))).
    fn reduce<T: Clone + Eq>(a: &S<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

    /// Sequential scan producing all prefixes under `f` together with the total. <br/>
    /// Work: Θ(|a|·lg|a|), Span: Θ(lg|a|).
    fn scan<T: Clone + Eq>(a: &S<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (S<T>, T);

    /// Flatten one level of nested sequences by concatenating all inner sequences. <br/>
    /// Work: Θ(∑_{x∈ss} |x|), Span: Θ(1).
    fn flatten<T: Clone + Eq>(ss: &S<S<T>>) -> S<T>;

    /// Group pairs by key according to `cmp`, collecting values into sequences per key. <br/>
    /// Work: Θ(|a|^2), Span: Θ(|a|^2). <br/>
    /// APAS: Work Θ(|a| · W(f) · lg|a|), Span Θ(S(f) · lg^2|a|).
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &S<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> S<(A, S<Bv>)>;
}

impl<T2> APAS18 for S<T2> {
    /// Work: Θ(n + Σ_{i=0}^{n-1} W(f(i))), Span: Θ(1 + max_{0≤i<n} S(f(i))).
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> S<T> {
        S { data: (0..n).map(|i| f(i)).collect::<Vec<T>>().into_boxed_slice() }
    }

    /// Work: Θ(|a| + Σ_{x∈a} W(f(x))), Span: Θ(1 + max_{x∈a} S(f(x))).
    fn map<T, U: Clone>(a: &S<T>, f: impl Fn(&T) -> U) -> S<U> {
        let input_length = a.length();
        if input_length == 0 { return <S<U> as Sequence<U>>::empty(); }
        let first_mapped_elt = f(a.nth(0));
        let mut result_seq = <S<U> as Sequence<U>>::new(input_length, first_mapped_elt.clone());
        let _ = result_seq.set(0, first_mapped_elt);
        for index in 1..input_length { let _ = result_seq.set(index, f(a.nth(index))); }
        result_seq
    }

    /// Work: Θ(|a|+|b|), Span: Θ(1).
    fn append<T: Clone + Eq>(a: &S<T>, b: &S<T>) -> S<T> {
        let left_length = a.length();
        let right_length = b.length();
        let combined_length = left_length + right_length;
        if combined_length == 0 { return <S<T> as Sequence<T>>::empty(); }
        let initial_elt = if left_length > 0 { a.nth(0).clone() } else { b.nth(0).clone() };
        let mut result_seq = <S<T> as Sequence<T>>::new(combined_length, initial_elt.clone());
        for left_index in 0..left_length { let _ = result_seq.set(left_index, a.nth(left_index).clone()); }
        for right_index in 0..right_length { let _ = result_seq.set(left_length + right_index, b.nth(right_index).clone()); }
        result_seq
    }

    /// Work: Θ(|a| + Σ W(pred(x))), Span: Θ(lg|a| + max S(pred(x))).
    fn filter<T: Clone + Eq>(a: &S<T>, pred: impl Fn(&T) -> B) -> S<T> {
        let input_length = a.length();
        if input_length == 0 { return <S<T> as Sequence<T>>::empty(); }
        // Compute mask to evaluate pred once per elt
        let keep_mask: S<B> = <S<B> as APAS18>::map(a, |x| pred(x));
        // Count kept
        let mut kept_count: N = 0;
        for index in 0..input_length { if *keep_mask.nth(index) == B::True { kept_count += 1; } }
        if kept_count == 0 { return <S<T> as Sequence<T>>::empty(); }
        // Seed allocation with first kept elt
        let mut first_kept_index: N = 0;
        while first_kept_index < input_length && *keep_mask.nth(first_kept_index) != B::True { first_kept_index += 1; }
        let initial_elt = a.nth(first_kept_index).clone();
        let mut result_seq = <S<T> as Sequence<T>>::new(kept_count, initial_elt.clone());
        // Fill output
        let mut write_index: N = 0;
        for index in 0..input_length { if *keep_mask.nth(index) == B::True { let _ = result_seq.set(write_index, a.nth(index).clone()); write_index += 1; } }
        result_seq
    }

    /// Work: Θ(1), Span: Θ(1).
    fn update<T: Clone + Eq>(a: &mut S<T>, (index, item): (N, T)) -> &mut S<T> { a.update((index, item)) }

    /// Work: Θ(|a|+|updates|), Span: Θ(1).
    fn inject<T: Clone + Eq>(a: &S<T>, updates: &S<(N, T)>) -> S<T> {
        let mut new_elts = a.data.clone();
        let mut seen_indices = std::collections::HashSet::new();
        for update_iter in 0..updates.length() {
            let (update_index, update_value) = updates.nth(update_iter);
            if *update_index < new_elts.len() && !seen_indices.contains(update_index) {
                new_elts[*update_index] = update_value.clone();
                seen_indices.insert(*update_index);
            }
        }
        S { data: new_elts }
    }

    /// Work: Θ(|a|+|updates|), Span: Θ(1).
    fn ninject<T: Clone + Eq>(a: &S<T>, updates: &S<(N, T)>) -> S<T> {
        let mut new_elts = a.data.clone();
        for update_iter in 0..updates.length() {
            let (update_index, update_value) = updates.nth(update_iter);
            if *update_index < new_elts.len() { new_elts[*update_index] = update_value.clone(); }
        }
        S { data: new_elts }
    }

    /// Work: Θ(|a| + Σ W(f)), Span: Θ(|a| + max S(f)).
    fn iterate<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut accumulator = x;
        for index in 0..a.length() { accumulator = f(&accumulator, a.nth(index)); }
        accumulator
    }

    /// Work: Θ(|a|), Span: Θ(|a|).
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> (S<A>, A) {
        let input_length = a.length();
        let mut prefix_value = x;
        if input_length == 0 { return (<S<A> as Sequence<A>>::empty(), prefix_value); }
        let mut prefix_values = <S<A> as Sequence<A>>::new(input_length, prefix_value.clone());
        for index in 0..input_length {
            let _ = prefix_values.set(index, prefix_value.clone());
            prefix_value = f(&prefix_value, a.nth(index));
        }
        (prefix_values, prefix_value)
    }

    /// BUG: VEC — APAS: Work Θ(Σ (y,z)∈T(−) W(f(y, z))), Span Θ(lg|a| · max (y,z)∈T(−) S(f(y, z))).
    fn reduce<T: Clone + Eq>(a: &S<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        let input_length = a.length();
        if input_length == 0 { return id; }
        if input_length == 1 { return a.nth(0).clone(); }
        let mid_index = input_length / 2;
        let left_subseq = a.subseq(0, mid_index);
        let right_subseq = a.subseq(mid_index, input_length - mid_index);
        let left_result = Self::reduce(&left_subseq, f, id.clone());
        let right_result = Self::reduce(&right_subseq, f, id);
        f(&left_result, &right_result)
    }

    /// Work: Θ(|a|·lg|a|), Span: Θ(lg|a|).
    fn scan<T: Clone + Eq>(a: &S<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (S<T>, T) {
        let input_length = a.length();
        let mut prefix_values = if input_length == 0 { <S<T> as Sequence<T>>::empty() } else { <S<T> as Sequence<T>>::new(input_length, id.clone()) };
        for index in 0..input_length {
            let prefix_subseq = a.subseq(0, index);
            let prefix_result = Self::reduce(&prefix_subseq, f, id.clone());
            let _ = prefix_values.set(index, prefix_result);
        }
        let total_result = Self::reduce(a, f, id);
        (prefix_values, total_result)
    }

    /// Work: Θ(∑_{x∈ss} |x|), Span: Θ(1).
    fn flatten<T: Clone + Eq>(ss: &S<S<T>>) -> S<T> {
        let outer_length = ss.length();
        let mut total_length: N = 0;
        for outer_index in 0..outer_length { total_length += ss.nth(outer_index).length(); }
        if total_length == 0 { return <S<T> as Sequence<T>>::empty(); }
        // find first elt to seed allocation
        let mut first_elt_opt: Option<T> = None;
        'find: for outer_index in 0..outer_length {
            let inner_seq = ss.nth(outer_index);
            if inner_seq.length() > 0 { first_elt_opt = Some(inner_seq.nth(0).clone()); break 'find; }
        }
        let initial_elt = first_elt_opt.expect("total_length > 0 implies some inner elt exists");
        let mut result_seq = <S<T> as Sequence<T>>::new(total_length, initial_elt.clone());
        let mut write_index: N = 0;
        for outer_index in 0..outer_length {
            let inner_seq = ss.nth(outer_index);
            for inner_index in 0..inner_seq.length() { let _ = result_seq.set(write_index, inner_seq.nth(inner_index).clone()); write_index += 1; }
        }
        result_seq
    }

    /// BUG: VEC — APAS: Work Θ(|a| · W(f) · lg|a|), Span Θ(S(f) · lg^2|a|).
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &S<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> S<(A, S<Bv>)> {
        let mut grouped_values: Vec<(A, Vec<Bv>)> = Vec::new();
        for index in 0..a.length() {
            let (key, value) = a.nth(index);
            let mut found = false;
            for (group_key, group_values) in &mut grouped_values {
                if cmp(key, group_key) == O::Equal { group_values.push(value.clone()); found = true; break; }
            }
            if !found { grouped_values.push((key.clone(), vec![value.clone()])); }
        }
        let grouped_seq: Vec<(A, S<Bv>)> = grouped_values.into_iter().map(|(key, values)| (key, S { data: values.into_boxed_slice() })).collect();
        S { data: grouped_seq.into_boxed_slice() }
    }
}


