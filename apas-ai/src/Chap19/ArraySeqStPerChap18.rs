//! Chapter 18 algorithms for ArraySeqStPer.

pub mod ArraySeqStPerChap18 {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    pub trait ArraySeqStPerChap18Trait<T: StT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        /// claude-4-sonet: Work Θ(n + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        /// claude-4-sonet: Work Θ(|a| + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;

        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        /// claude-4-sonet: Work Θ(|a| + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + |a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn update(a: &ArrayStPerS<T>, item_at: Pair<N, T>) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates)))
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(lg(degree(updates)))
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(1 + Σ S(f(y,z)))
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

        /// APAS: Work Θ(|a|), Span Θ(|a|)
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayStPerS<A>, A);

        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(lg|a| · max S(f(y,z)))
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);

        /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|)
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;

        /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|)
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<T, ArrayStPerS<T>>>;
    }

    impl<T: StT> ArraySeqStPerChap18Trait<T> for ArrayStPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {
            let data: Vec<T> = (0..n).map(|i| f(i)).collect();
            ArrayStPerS::from_vec(data)
        }
        fn map<U: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {
            if a.length() == 0usize {
                return <ArrayStPerS<U> as ArraySeqStPerTrait<U>>::empty();
            }
            let first = f(a.nth(0));
            // Fill by cloning into a Vec then boxing
            let mut v: Vec<U> = vec![first; a.length()];
            for i in 0..a.length() {
                v[i] = f(a.nth(i));
            }
            ArrayStPerS::from_vec(v)
        }
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {
            let n = a.length() + b.length();
            if n == 0usize {
                return <ArrayStPerS<T> as ArraySeqStPerTrait<T>>::empty();
            }
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..a.length() {
                v.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                v.push(b.nth(j).clone());
            }
            ArrayStPerS::from_vec(v)
        }
        fn filter(a: &ArrayStPerS<T>, pred: impl Fn(&T) -> B) -> ArrayStPerS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..a.length() {
                if pred(a.nth(i)) == B::True {
                    v.push(a.nth(i).clone());
                }
            }
            ArrayStPerS::from_vec(v)
        }
        fn update(a: &ArrayStPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayStPerS<T> {
            match a.set(index, item) {
                | Ok(updated) => updated,
                | Err(_) => a.clone(),
            }
        }
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {
            let mut v: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            let mut seen = std::collections::HashSet::new();
            for k in 0..updates.length() {
                let Pair(idx, val) = updates.nth(k).clone();
                if (idx as usize) < v.len() && !seen.contains(&idx) {
                    v[idx] = val;
                    seen.insert(idx);
                }
            }
            ArrayStPerS::from_vec(v)
        }
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {
            let mut v: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            for k in 0..updates.length() {
                let Pair(idx, val) = updates.nth(k).clone();
                if (idx as usize) < v.len() {
                    v[idx] = val;
                }
            }
            ArrayStPerS::from_vec(v)
        }
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }
        fn iteratePrefixes<A: StT + Clone>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayStPerS<A>, A) {
            let mut acc = x.clone();
            let mut v: Vec<A> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                v.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (ArrayStPerS::from_vec(v), acc)
        }
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {
                let n = s.len();
                if n == 0usize {
                    return id;
                }
                if n == 1usize {
                    return s[0].clone();
                }
                let m = n / 2;
                let l = rec(&s[..m], f, id.clone());
                let r = rec(&s[m..], f, id);
                f(&l, &r)
            }
            rec(a.subseq(0, a.length()), f, id)
        }
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {
            fn rec<T: StT>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {
                let n = s.len();
                if n == 0usize {
                    return id;
                }
                if n == 1usize {
                    return s[0].clone();
                }
                let m = n / 2;
                let l = rec(&s[..m], f, id.clone());
                let r = rec(&s[m..], f, id);
                f(&l, &r)
            }
            let n = a.length();
            let mut prefixes: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                prefixes.push(rec(a.subseq(0, i), f, id.clone()));
            }
            let total = rec(a.subseq(0, n), f, id);
            (ArrayStPerS::from_vec(prefixes), total)
        }
        fn flatten(ss: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {
            let mut v: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    v.push(inner.nth(j).clone());
                }
            }
            ArrayStPerS::from_vec(v)
        }
        fn collect(a: &ArrayStPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayStPerS<Pair<T, ArrayStPerS<T>>> {
            let mut groups: Vec<Pair<T, Vec<T>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(k, v) = a.nth(i).clone();
                let mut found = false;
                for Pair(gk, gv) in &mut groups {
                    if cmp(&k, &gk) == O::Equal {
                        gv.push(v.clone());
                        found = true;
                        break;
                    }
                }
                if !found {
                    groups.push(Pair(k.clone(), vec![v.clone()]));
                }
            }
            let pairs: Vec<Pair<T, ArrayStPerS<T>>> = groups
                .into_iter()
                .map(|Pair(k, vs)| Pair(k, ArrayStPerS::from_vec(vs)))
                .collect();
            ArrayStPerS::from_vec(pairs)
        }
    }
}
