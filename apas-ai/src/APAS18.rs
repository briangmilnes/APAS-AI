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

    /// Algorithm 19.13 (Function subseq). Return the subsequence in half-open range `[start, end)`. <br/>
    /// Work: Θ(end-start), Span: Θ(1) in this owning representation.
    fn subseq<T: Clone + Eq>(a: &S<T>, start: N, end: N) -> S<T>;

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
        let n = a.length();
        if n == 0 { return <S<U> as Sequence<U>>::empty(); }
        let first = f(a.nth(0));
        let mut out = <S<U> as Sequence<U>>::new(n, first.clone());
        let _ = out.set(0, first);
        for i in 1..n { let _ = out.set(i, f(a.nth(i))); }
        out
    }

    /// Work: Θ(end−start), Span: Θ(1).
    fn subseq<T: Clone + Eq>(a: &S<T>, start: N, end: N) -> S<T> {
        let a_len = a.length();
        let from = start.min(a_len);
        let to = end.min(a_len);
        if to <= from { return <S<T> as Sequence<T>>::empty(); }
        let k = to - from;
        let first = a.nth(from).clone();
        let mut out = <S<T> as Sequence<T>>::new(k, first.clone());
        let _ = out.set(0, first);
        for i in 1..k { let _ = out.set(i, a.nth(from + i).clone()); }
        out
    }

    /// Work: Θ(|a|+|b|), Span: Θ(1).
    fn append<T: Clone + Eq>(a: &S<T>, b: &S<T>) -> S<T> {
        let a_len = a.length();
        let b_len = b.length();
        let n = a_len + b_len;
        if n == 0 { return <S<T> as Sequence<T>>::empty(); }
        let seed = if a_len > 0 { a.nth(0).clone() } else { b.nth(0).clone() };
        let mut out = <S<T> as Sequence<T>>::new(n, seed.clone());
        for i in 0..a_len { let _ = out.set(i, a.nth(i).clone()); }
        for j in 0..b_len { let _ = out.set(a_len + j, b.nth(j).clone()); }
        out
    }

    /// Work: Θ(|a| + Σ W(pred(x))), Span: Θ(lg|a| + max S(pred(x))).
    fn filter<T: Clone + Eq>(a: &S<T>, pred: impl Fn(&T) -> B) -> S<T> {
        let n = a.length();
        if n == 0 { return <S<T> as Sequence<T>>::empty(); }
        // Compute mask to evaluate pred once per element
        let mask: S<B> = <S<B> as APAS18>::map(a, |x| pred(x));
        // Count kept
        let mut m: N = 0;
        for i in 0..n { if *mask.nth(i) == B::True { m += 1; } }
        if m == 0 { return <S<T> as Sequence<T>>::empty(); }
        // Seed allocation with first kept element
        let mut j: N = 0;
        while j < n && *mask.nth(j) != B::True { j += 1; }
        let seed = a.nth(j).clone();
        let mut out = <S<T> as Sequence<T>>::new(m, seed.clone());
        // Fill output
        let mut k: N = 0;
        for i in 0..n { if *mask.nth(i) == B::True { let _ = out.set(k, a.nth(i).clone()); k += 1; } }
        out
    }

    /// Work: Θ(1), Span: Θ(1).
    fn update<T: Clone + Eq>(a: &mut S<T>, (index, item): (N, T)) -> &mut S<T> { a.update((index, item)) }

    /// Work: Θ(|a|+|updates|), Span: Θ(1).
    fn inject<T: Clone + Eq>(a: &S<T>, updates: &S<(N, T)>) -> S<T> {
        let mut new_data = a.data.clone();
        let mut seen = std::collections::HashSet::new();
        for i in 0..updates.length() {
            let (idx, val) = updates.nth(i);
            if *idx < new_data.len() && !seen.contains(idx) {
                new_data[*idx] = val.clone();
                seen.insert(*idx);
            }
        }
        S { data: new_data }
    }

    /// Work: Θ(|a|+|updates|), Span: Θ(1).
    fn ninject<T: Clone + Eq>(a: &S<T>, updates: &S<(N, T)>) -> S<T> {
        let mut new_data = a.data.clone();
        for i in 0..updates.length() {
            let (idx, val) = updates.nth(i);
            if *idx < new_data.len() { new_data[*idx] = val.clone(); }
        }
        S { data: new_data }
    }

    /// Work: Θ(|a| + Σ W(f)), Span: Θ(|a| + max S(f)).
    fn iterate<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x;
        for i in 0..a.length() { acc = f(&acc, a.nth(i)); }
        acc
    }

    /// Work: Θ(|a|), Span: Θ(|a|).
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(a: &S<T>, f: impl Fn(&A, &T) -> A, x: A) -> (S<A>, A) {
        let n = a.length();
        let mut current = x;
        if n == 0 { return (<S<A> as Sequence<A>>::empty(), current); }
        let mut prefixes = <S<A> as Sequence<A>>::new(n, current.clone());
        for i in 0..n {
            let _ = prefixes.set(i, current.clone());
            current = f(&current, a.nth(i));
        }
        (prefixes, current)
    }

    /// BUG: VEC — APAS: Work Θ(Σ (y,z)∈T(−) W(f(y, z))), Span Θ(lg|a| · max (y,z)∈T(−) S(f(y, z))).
    fn reduce<T: Clone + Eq>(a: &S<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        let len = a.length();
        if len == 0 { return id; }
        if len == 1 { return a.nth(0).clone(); }
        let mid = len / 2;
        let left = Self::subseq(a, 0, mid);
        let right = Self::subseq(a, mid, len);
        let l = Self::reduce(&left, f, id.clone());
        let r = Self::reduce(&right, f, id);
        f(&l, &r)
    }

    /// Work: Θ(|a|·lg|a|), Span: Θ(lg|a|).
    fn scan<T: Clone + Eq>(a: &S<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (S<T>, T) {
        let n = a.length();
        let mut prefixes = if n == 0 { <S<T> as Sequence<T>>::empty() } else { <S<T> as Sequence<T>>::new(n, id.clone()) };
        for i in 0..n {
            let p = Self::subseq(a, 0, i);
            let r = Self::reduce(&p, f, id.clone());
            let _ = prefixes.set(i, r);
        }
        let total = Self::reduce(a, f, id);
        (prefixes, total)
    }

    /// Work: Θ(∑_{x∈ss} |x|), Span: Θ(1).
    fn flatten<T: Clone + Eq>(ss: &S<S<T>>) -> S<T> {
        let outer = ss.length();
        let mut total: N = 0;
        for i in 0..outer { total += ss.nth(i).length(); }
        if total == 0 { return <S<T> as Sequence<T>>::empty(); }
        // find first element to seed allocation
        let mut seed_opt: Option<T> = None;
        'find: for i in 0..outer {
            let s = ss.nth(i);
            if s.length() > 0 { seed_opt = Some(s.nth(0).clone()); break 'find; }
        }
        let seed = seed_opt.expect("total > 0 implies some inner element exists");
        let mut out = <S<T> as Sequence<T>>::new(total, seed.clone());
        let mut idx: N = 0;
        for i in 0..outer {
            let s = ss.nth(i);
            for j in 0..s.length() { let _ = out.set(idx, s.nth(j).clone()); idx += 1; }
        }
        out
    }

    /// BUG: VEC — APAS: Work Θ(|a| · W(f) · lg|a|), Span Θ(S(f) · lg^2|a|).
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(a: &S<(A, Bv)>, cmp: impl Fn(&A, &A) -> O) -> S<(A, S<Bv>)> {
        let mut groups: Vec<(A, Vec<Bv>)> = Vec::new();
        for i in 0..a.length() {
            let (key, value) = a.nth(i);
            let mut found = false;
            for (gk, gv) in &mut groups {
                if cmp(key, gk) == O::Equal { gv.push(value.clone()); found = true; break; }
            }
            if !found { groups.push((key.clone(), vec![value.clone()])); }
        }
        let result: Vec<(A, S<Bv>)> = groups.into_iter().map(|(k, v)| (k, S { data: v.into_boxed_slice() })).collect();
        S { data: result.into_boxed_slice() }
    }
}


