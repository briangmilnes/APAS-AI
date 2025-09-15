//! Chapter 18 algorithms for LinkedListPer.

pub mod LinkedListPerChap18 {
    use crate::LinkedListPer::LinkedListPer::*;
    use crate::Types::Types::*;

    pub trait LinkedListPerChap18Trait<T: StT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;

        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        fn filter(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + |a|), Span Θ(1)
        /// gpt-5-hard: Work Θ(|a|), Span Θ(1)
        /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
        fn update(a: &LinkedListPerS<T>, item_at: Pair<N, T>) -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates)))
        fn inject(a: &LinkedListPerS<T>, updates: &LinkedListPerS<Pair<N, T>>,)
         -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1)
        fn ninject(
            a: &LinkedListPerS<T>,
            updates: &LinkedListPerS<Pair<N, T>>,
        ) -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(1 + Σ S(f(y,z)))
        fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

        /// APAS: Work Θ(|a|), Span Θ(|a|)
        fn iteratePrefixes<A: StT>(
            a: &LinkedListPerS<T>,
            f: impl Fn(&A, &T) -> A,
            x: A,
        ) -> (LinkedListPerS<A>, A);

        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(lg|a| · max S(f(y,z)))
        fn reduce(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListPerS<T>, T);

        /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|)
        fn flatten(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;

        /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|)
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListPerS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> LinkedListPerS<Pair<A, LinkedListPerS<Bv>>>;
    }

    impl<T: StT> LinkedListPerChap18Trait<T> for LinkedListPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            LinkedListPerS::from_vec(v)
        }

        fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {
            let mut v: Vec<U> =
                Vec::with_capacity(<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a));
            for i in 0..<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a) {
                v.push(f(<LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i)));
            }
            LinkedListPerS::from_vec(v)
        }
        fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {
            let na = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a);
            let nb = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(b);
            let mut v: Vec<T> = Vec::with_capacity(na + nb);
            for i in 0..na {
                v.push(<LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i).clone());
            }
            for j in 0..nb {
                v.push(<LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(b, j).clone());
            }
            LinkedListPerS::from_vec(v)
        }
        fn filter(a: &LinkedListPerS<T>, pred: impl Fn(&T) -> B) -> LinkedListPerS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a) {
                let x = <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i);
                if pred(x) == B::True {
                    v.push(x.clone());
                }
            }
            LinkedListPerS::from_vec(v)
        }
        fn update(a: &LinkedListPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListPerS<T> {
            let mut v: Vec<T> =
                Vec::with_capacity(<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a));
            for i in 0..<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a) {
                let cur = <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i).clone();
                v.push(if i == index { item.clone() } else { cur });
            }
            LinkedListPerS::from_vec(v)
        }
        fn inject(a: &LinkedListPerS<T>, updates: &LinkedListPerS<Pair<N, T>>,) 
         -> LinkedListPerS<T> {
            // first-update wins
            let n = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a);
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                let mut replaced: Option<T> = None;
                for j in 0..<LinkedListPerS<Pair<N, T>> as LinkedListPerTrait<Pair<N, T>>>::length(
                    updates,
                ) {
                    let Pair(idx, val) = <LinkedListPerS<Pair<N, T>> as LinkedListPerTrait<
                        Pair<N, T>,
                    >>::nth(updates, j)
                    .clone();
                    if idx == i {
                        replaced = Some(val);
                        break;
                    }
                }
                v.push(replaced.unwrap_or_else(|| {
                    <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i).clone()
                }));
            }
            LinkedListPerS::from_vec(v)
        }
        fn ninject(
            a: &LinkedListPerS<T>,
            updates: &LinkedListPerS<Pair<N, T>>,
        ) -> LinkedListPerS<T> {
            // last-update wins
            let n = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a);
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                let mut replaced: Option<T> = None;
                for j in 0..<LinkedListPerS<Pair<N, T>> as LinkedListPerTrait<Pair<N, T>>>::length(
                    updates,
                ) {
                    let Pair(idx, val) = <LinkedListPerS<Pair<N, T>> as LinkedListPerTrait<
                        Pair<N, T>,
                    >>::nth(updates, j)
                    .clone();
                    if idx == i {
                        replaced = Some(val);
                    }
                }
                v.push(replaced.unwrap_or_else(|| {
                    <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i).clone()
                }));
            }
            LinkedListPerS::from_vec(v)
        }
        fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a) {
                acc = f(
                    &acc,
                    <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i),
                );
            }
            acc
        }
        fn iteratePrefixes<A: StT>(
            a: &LinkedListPerS<T>,
            f: impl Fn(&A, &T) -> A,
            x: A,
        ) -> (LinkedListPerS<A>, A) {
            let mut acc = x.clone();
            let mut v: Vec<A> =
                Vec::with_capacity(<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a));
            for i in 0..<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a) {
                v.push(acc.clone());
                acc = f(
                    &acc,
                    <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i),
                );
            }
            (LinkedListPerS::from_vec(v), acc)
        }
        fn reduce(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a);
            if n == 0 {
                return id;
            }
            if n == 1 {
                return <LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, 0).clone();
            }
            let m = n / 2;
            let left = <LinkedListPerS<T> as LinkedListPerTrait<T>>::subseq_copy(a, 0, m);
            let right = <LinkedListPerS<T> as LinkedListPerTrait<T>>::subseq_copy(a, m, n - m);
            let l =
                <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }
        fn scan(a: &LinkedListPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListPerS<T>, T) {
            let n = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a);
            if n == 0 {
                return (
                    <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::tabulate(|_| id.clone(), 0),
                    id,
                );
            }
            let mut prefixes: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                let prefix = <LinkedListPerS<T> as LinkedListPerTrait<T>>::subseq_copy(a, 0, i);
                let red = <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::reduce(
                    &prefix,
                    f,
                    id.clone(),
                );
                prefixes.push(red);
            }
            let total = <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::reduce(a, f, id);
            (LinkedListPerS::from_vec(prefixes), total)
        }
        fn flatten(ss: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..<LinkedListPerS<LinkedListPerS<T>> as LinkedListPerTrait<
                LinkedListPerS<T>,
            >>::length(ss)
            {
                let inner = <LinkedListPerS<LinkedListPerS<T>> as LinkedListPerTrait<
                    LinkedListPerS<T>,
                >>::nth(ss, i);
                for j in 0..<LinkedListPerS<T> as LinkedListPerTrait<T>>::length(inner) {
                    v.push(<LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(inner, j).clone());
                }
            }
            LinkedListPerS::from_vec(v)
        }
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListPerS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> LinkedListPerS<Pair<A, LinkedListPerS<Bv>>> {
            let mut groups: Vec<(A, Vec<Bv>)> = Vec::new();
            for i in 0..<LinkedListPerS<Pair<A, Bv>> as LinkedListPerTrait<Pair<A, Bv>>>::length(a)
            {
                let Pair(k, v) =
                    <LinkedListPerS<Pair<A, Bv>> as LinkedListPerTrait<Pair<A, Bv>>>::nth(a, i)
                        .clone();
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
            let pairs: Vec<Pair<A, LinkedListPerS<Bv>>> = groups
                .into_iter()
                .map(|(k, vs)| Pair(k, LinkedListPerS::from_vec(vs)))
                .collect();
            LinkedListPerS::from_vec(pairs)
        }
    }
}

pub use LinkedListPerChap18::LinkedListPerChap18Trait;
