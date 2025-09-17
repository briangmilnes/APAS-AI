//! Chapter 18 algorithms for LinkedListStPer.

pub mod LinkedListStPerChap18 {
    use crate::LinkedListStPer::LinkedListStPer::*;
    use crate::Types::Types::*;

    pub trait LinkedListStPerChap18Trait<T: StT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i S(f(i)))
        /// claude-4-sonet: Work Θ(n + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        /// claude-4-sonet: Work Θ(|a| + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;

        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        /// claude-4-sonet: Work Θ(|a| + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + |a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates)))
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1)
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(1 + Σ S(f(y,z)))
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

        /// APAS: Work Θ(|a|), Span Θ(|a|)
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStPerS<A>, A);

        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(lg|a| · max S(f(y,z)))
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<T>, T);

        /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|)
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;

        /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|)
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStPerS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> LinkedListStPerS<Pair<A, LinkedListStPerS<Bv>>>;
    }

    impl<T: StT> LinkedListStPerChap18Trait<T> for LinkedListStPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            LinkedListStPerS::from_vec(v)
        }

        fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {
            let mut v: Vec<U> = Vec::with_capacity(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a));
            for i in 0..<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a) {
                v.push(f(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i)));
            }
            LinkedListStPerS::from_vec(v)
        }
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {
            let na = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a);
            let nb = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(b);
            let mut v: Vec<T> = Vec::with_capacity(na + nb);
            for i in 0..na {
                v.push(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i).clone());
            }
            for j in 0..nb {
                v.push(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(b, j).clone());
            }
            LinkedListStPerS::from_vec(v)
        }
        fn filter(a: &LinkedListStPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListStPerS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a) {
                let x = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i);
                if pred(x) == B::True {
                    v.push(x.clone());
                }
            }
            LinkedListStPerS::from_vec(v)
        }
        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T> {
            let mut v: Vec<T> = Vec::with_capacity(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a));
            for i in 0..<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a) {
                let cur = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i).clone();
                v.push(if i == index { item.clone() } else { cur });
            }
            LinkedListStPerS::from_vec(v)
        }
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T> {
            // first-update wins
            let n = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a);
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                let mut replaced: Option<T> = None;
                for j in 0..<LinkedListStPerS<Pair<N, T>> as LinkedListStPerTrait<Pair<N, T>>>::length(updates) {
                    let Pair(idx, val) =
                        <LinkedListStPerS<Pair<N, T>> as LinkedListStPerTrait<Pair<N, T>>>::nth(updates, j).clone();
                    if idx == i {
                        replaced = Some(val);
                        break;
                    }
                }
                v.push(replaced.unwrap_or_else(|| <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i).clone()));
            }
            LinkedListStPerS::from_vec(v)
        }
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T> {
            // last-update wins
            let n = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a);
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                let mut replaced: Option<T> = None;
                for j in 0..<LinkedListStPerS<Pair<N, T>> as LinkedListStPerTrait<Pair<N, T>>>::length(updates) {
                    let Pair(idx, val) =
                        <LinkedListStPerS<Pair<N, T>> as LinkedListStPerTrait<Pair<N, T>>>::nth(updates, j).clone();
                    if idx == i {
                        replaced = Some(val);
                    }
                }
                v.push(replaced.unwrap_or_else(|| <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i).clone()));
            }
            LinkedListStPerS::from_vec(v)
        }
        fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a) {
                acc = f(&acc, <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i));
            }
            acc
        }
        fn iteratePrefixes<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListStPerS<A>, A) {
            let mut acc = x.clone();
            let mut v: Vec<A> = Vec::with_capacity(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a));
            for i in 0..<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a) {
                v.push(acc.clone());
                acc = f(&acc, <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i));
            }
            (LinkedListStPerS::from_vec(v), acc)
        }
        fn reduce(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a);
            if n == 0 {
                return id;
            }
            if n == 1 {
                return <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, 0).clone();
            }
            let m = n / 2;
            let left = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::subseq_copy(a, 0, m);
            let right = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::subseq_copy(a, m, n - m);
            let l = <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }
        fn scan(a: &LinkedListStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListStPerS<T>, T) {
            let n = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a);
            if n == 0 {
                return (
                    <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::tabulate(|_| id.clone(), 0),
                    id,
                );
            }
            let mut prefixes: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                let prefix = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::subseq_copy(a, 0, i);
                let red = <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::reduce(&prefix, f, id.clone());
                prefixes.push(red);
            }
            let total = <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::reduce(a, f, id);
            (LinkedListStPerS::from_vec(prefixes), total)
        }
        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..<LinkedListStPerS<LinkedListStPerS<T>> as LinkedListStPerTrait<LinkedListStPerS<T>>>::length(ss)
            {
                let inner =
                    <LinkedListStPerS<LinkedListStPerS<T>> as LinkedListStPerTrait<LinkedListStPerS<T>>>::nth(ss, i);
                for j in 0..<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(inner) {
                    v.push(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(inner, j).clone());
                }
            }
            LinkedListStPerS::from_vec(v)
        }
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStPerS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> LinkedListStPerS<Pair<A, LinkedListStPerS<Bv>>> {
            let mut groups: Vec<(A, Vec<Bv>)> = Vec::new();
            for i in 0..<LinkedListStPerS<Pair<A, Bv>> as LinkedListStPerTrait<Pair<A, Bv>>>::length(a) {
                let Pair(k, v) =
                    <LinkedListStPerS<Pair<A, Bv>> as LinkedListStPerTrait<Pair<A, Bv>>>::nth(a, i).clone();
                let mut found = false;
                for (gk, gv) in &mut groups {
                    if cmp(&k, gk) == O::Equal {
                        gv.push(v.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    groups.push((k, vec![v]));
                }
            }
            let pairs: Vec<Pair<A, LinkedListStPerS<Bv>>> = groups
                .into_iter()
                .map(|(k, vs)| Pair(k, LinkedListStPerS::from_vec(vs)))
                .collect();
            LinkedListStPerS::from_vec(pairs)
        }
    }
}
