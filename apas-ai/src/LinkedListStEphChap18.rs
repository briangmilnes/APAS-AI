//! Chapter 18 algorithms for LinkedListStEph (ephemeral).

pub mod LinkedListStEphChap18 {
    use crate::LinkedListStEph::LinkedListStEph::*;
    use crate::Types::Types::*;
    use std::collections::HashSet;

    pub trait LinkedListStEphChap18Trait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T>;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T>;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEphS<T>;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEphS<T>;
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStEphS<A>, A);
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, T);
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStEphS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> LinkedListStEphS<Pair<A, LinkedListStEphS<Bv>>>;
    }

    impl<T: StT> LinkedListStEphChap18Trait<T> for LinkedListStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            LinkedListStEphS::from_vec(v)
        }
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {
            let mut v: Vec<U> = Vec::with_capacity(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a));
            for i in 0..<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a) {
                v.push(f(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i)));
            }
            LinkedListStEphS::from_vec(v)
        }
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {
            let na = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a);
            let nb = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(b);
            let mut v: Vec<T> = Vec::with_capacity(na + nb);
            for i in 0..na {
                v.push(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i).clone());
            }
            for j in 0..nb {
                v.push(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(b, j).clone());
            }
            LinkedListStEphS::from_vec(v)
        }
        fn filter(a: &LinkedListStEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListStEphS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a) {
                let x = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i);
                if pred(x) == B::True {
                    v.push(x.clone());
                }
            }
            LinkedListStEphS::from_vec(v)
        }
        fn update(a: &mut LinkedListStEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::update(a, item_at)
        }
        fn inject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEphS<T> {
            // first-update wins
            let mut out = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::subseq_copy(
                a,
                0,
                <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a),
            );
            let mut applied: HashSet<N> = HashSet::new();
            for i in 0..<LinkedListStEphS<Pair<N, T>> as LinkedListStEphTrait<Pair<N, T>>>::length(updates) {
                let Pair(idx, val) =
                    <LinkedListStEphS<Pair<N, T>> as LinkedListStEphTrait<Pair<N, T>>>::nth(updates, i).clone();
                if !applied.contains(&idx) {
                    let _ = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::set(&mut out, idx, val);
                    applied.insert(idx);
                }
            }
            out
        }
        fn ninject(a: &LinkedListStEphS<T>, updates: &LinkedListStEphS<Pair<N, T>>) -> LinkedListStEphS<T> {
            // last-update wins
            let mut out = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::subseq_copy(
                a,
                0,
                <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a),
            );
            for i in 0..<LinkedListStEphS<Pair<N, T>> as LinkedListStEphTrait<Pair<N, T>>>::length(updates) {
                let Pair(idx, val) =
                    <LinkedListStEphS<Pair<N, T>> as LinkedListStEphTrait<Pair<N, T>>>::nth(updates, i).clone();
                let _ = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::set(&mut out, idx, val);
            }
            out
        }
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a) {
                acc = f(&acc, <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i));
            }
            acc
        }
        fn iteratePrefixes<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStEphS<A>, A) {
            let mut acc = x.clone();
            let n = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a);
            let mut v: Vec<A> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(acc.clone());
                acc = f(&acc, <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i));
            }
            (LinkedListStEphS::from_vec(v), acc)
        }
        fn reduce(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a);
            if n == 0 {
                return id;
            }
            if n == 1 {
                return <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, 0).clone();
            }
            let m = n / 2;
            let left = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::subseq_copy(a, 0, m);
            let right = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::subseq_copy(a, m, n - m);
            let l = <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }
        fn scan(a: &LinkedListStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStEphS<T>, T) {
            let n = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a);
            if n == 0 {
                return (<LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::tabulate(|_| id.clone(), 0), id);
            }
            let mut prefixes: Vec<T> = Vec::with_capacity(n);
            let mut acc = id.clone();
            for i in 0..n {
                prefixes.push(acc.clone());
                acc = f(&acc, <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i));
            }
            (
                LinkedListStEphS::from_vec(prefixes),
                <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::reduce(a, f, id),
            )
        }
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..<LinkedListStEphS<LinkedListStEphS<T>> as LinkedListStEphTrait<LinkedListStEphS<T>>>::length(ss)
            {
                let inner =
                    <LinkedListStEphS<LinkedListStEphS<T>> as LinkedListStEphTrait<LinkedListStEphS<T>>>::nth(ss, i);
                for j in 0..<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(inner) {
                    v.push(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(inner, j).clone());
                }
            }
            LinkedListStEphS::from_vec(v)
        }
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStEphS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> LinkedListStEphS<Pair<A, LinkedListStEphS<Bv>>> {
            let mut groups: Vec<Pair<A, Vec<Bv>>> = Vec::new();
            for i in 0..<LinkedListStEphS<Pair<A, Bv>> as LinkedListStEphTrait<Pair<A, Bv>>>::length(a) {
                let Pair(k, v) =
                    <LinkedListStEphS<Pair<A, Bv>> as LinkedListStEphTrait<Pair<A, Bv>>>::nth(a, i).clone();
                if let Some(Pair(_, gv)) = groups.iter_mut().find(|Pair(gk, _)| cmp(&k, gk) == O::Equal) {
                    gv.push(v);
                } else {
                    groups.push(Pair(k, vec![v]));
                }
            }
            let pairs: Vec<Pair<A, LinkedListStEphS<Bv>>> =
                groups.into_iter().map(|Pair(k, vs)| Pair(k, LinkedListStEphS::from_vec(vs))).collect();
            LinkedListStEphS::from_vec(pairs)
        }
    }
}
