//! Chapter 18 algorithms for `ArraySeqStEph<T>`.

pub mod ArraySeqStEphChap {
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    pub trait ArraySeqStEphChap18Trait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn update(a: &mut ArraySeqStEphS<T>, item_at: (N, T)) -> &mut ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphS<T>;
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArraySeqStEphS<A>, A);
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T);
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;
        fn collect<A: StT, Bv: StT>(
            a: &ArraySeqStEphS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> ArraySeqStEphS<Pair<A, ArraySeqStEphS<Bv>>>;
    }

    impl<T: StT> ArraySeqStEphChap18Trait<T> for ArraySeqStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            ArraySeqStEphS::from_vec(v)
        }
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {
            let n = a.length();
            if n == 0usize {
                return <ArraySeqStEphS<U> as ArraySeqStEphTrait<U>>::empty();
            }
            let first = f(a.nth(0)).clone();
            let mut out = <ArraySeqStEphS<U> as ArraySeqStEphTrait<U>>::new(n, first.clone());
            let _ = out.set(0, first);
            for i in 1..n {
                let _ = out.set(i, f(a.nth(i)));
            }
            out
        }
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            let na = a.length();
            let nb = b.length();
            let n = na + nb;
            if n == 0usize {
                return <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty();
            }
            let init = if na > 0usize { a.nth(0).clone() } else { b.nth(0).clone() };
            let mut out = <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::new(n, init.clone());
            for i in 0..na {
                let _ = out.set(i, a.nth(i).clone());
            }
            for j in 0..nb {
                let _ = out.set(na + j, b.nth(j).clone());
            }
            out
        }
        fn filter(a: &ArraySeqStEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {
            let n = a.length();
            let mut kept: Vec<T> = Vec::new();
            kept.reserve(n);
            for i in 0..n {
                let x = a.nth(i);
                if pred(x) == true {
                    kept.push(x.clone());
                }
            }
            ArraySeqStEphS::from_vec(kept)
        }
        fn update(a: &mut ArraySeqStEphS<T>, (index, item): (N, T)) -> &mut ArraySeqStEphS<T> {
            a.update((index, item))
        }
        fn inject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphS<T> {
            let mut out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                let _ = out.set(idx, val);
            }
            out
        }
        fn ninject(a: &ArraySeqStEphS<T>, updates: &ArraySeqStEphS<Pair<N, T>>) -> ArraySeqStEphS<T> {
            Self::inject(a, updates)
        }
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }
        fn iteratePrefixes<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArraySeqStEphS<A>, A) {
            let n = a.length();
            let mut acc = x;
            if n == 0usize {
                return (<ArraySeqStEphS<A> as ArraySeqStEphTrait<A>>::empty(), acc);
            }
            let mut out = <ArraySeqStEphS<A> as ArraySeqStEphTrait<A>>::new(n, acc.clone());
            for i in 0..n {
                let _ = out.set(i, acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (out, acc)
        }
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T) {
            let n = a.length();
            let mut acc = id;
            let mut out = if n == 0usize {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty()
            } else {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::new(n, acc.clone())
            };
            for i in 0..n {
                let _ = out.set(i, acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (out, acc)
        }
        fn flatten(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {
            let mut total: N = 0;
            for i in 0..ss.length() {
                total += ss.nth(i).length();
            }
            if total == 0 {
                return <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty();
            }
            let mut init: Option<T> = None;
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                if inner.length() > 0usize {
                    init = Some(inner.nth(0).clone());
                    break;
                }
            }
            let initv = init.expect("total > 0 implies some inner non-empty");
            let mut out = <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::new(total, initv.clone());
            let mut w: N = 0;
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    let _ = out.set(w, inner.nth(j).clone());
                    w += 1;
                }
            }
            out
        }
        fn collect<A: StT, Bv: StT>(
            a: &ArraySeqStEphS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> ArraySeqStEphS<Pair<A, ArraySeqStEphS<Bv>>> {
            let mut groups: Vec<Pair<A, Vec<Bv>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(k, v) = a.nth(i).clone();
                let mut found = false;
                for Pair(gk, gv) in groups.iter_mut() {
                    if cmp(&k, gk) == O::Equal {
                        gv.push(v.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    groups.push(Pair(k.clone(), vec![v.clone()]));
                }
            }
            let out: Vec<Pair<A, ArraySeqStEphS<Bv>>> = groups
                .into_iter()
                .map(|Pair(k, gv)| Pair(k, ArraySeqStEphS::from_vec(gv)))
                .collect();
            ArraySeqStEphS::from_vec(out)
        }
    }
}
