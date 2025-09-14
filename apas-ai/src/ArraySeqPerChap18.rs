//! Chapter 18 algorithms for ArraySeqPer.

pub mod ArraySeqPerChap18 {
use crate::ArraySeqPer::ArraySeqPer::*;
use crate::Types::Types::*;

pub trait ArraySeqPerChap18Trait {
    /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
    fn map<T, U: Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;

    /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + |a|), Span Θ(1)
    /// gpt-5-hard: Work Θ(|a|), Span Θ(1)
    /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
    fn update<T: Clone + Eq>(a: &ArrayPerS<T>, item_at: (N, T)) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates)))
    fn inject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1)
    fn ninject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(1 + Σ S(f(y,z)))
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;

    /// APAS: Work Θ(|a|), Span Θ(|a|)
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(
        a: &ArrayPerS<T>,
        f: impl Fn(&A, &T) -> A,
        x: A,
    ) -> (ArrayPerS<A>, A);

    /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(lg|a| · max S(f(y,z)))
    fn reduce<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;

    /// APAS: Work Θ(|a|), Span Θ(lg|a|)
    fn scan<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, T);

    /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|)
    fn flatten<T: Clone + Eq>(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;

    /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|)
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(
        a: &ArrayPerS<(A, Bv)>,
        cmp: impl Fn(&A, &A) -> O,
    ) -> ArrayPerS<(A, ArrayPerS<Bv>)>;
}

impl<T2> ArraySeqPerChap18Trait for ArrayPerS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {
        let data: Vec<T> = (0..n).map(|i| f(i)).collect();
        ArrayPerS::from_vec(data)
    }
    fn map<T, U: Clone>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {
        if a.length() == 0 {
            return <ArrayPerS<U> as ArraySeqPerTrait<U>>::empty();
        }
        let first = f(a.nth(0));
        // Fill by cloning into a Vec then boxing
        let mut v: Vec<U> = vec![first; a.length()];
        for i in 0..a.length() {
            v[i] = f(a.nth(i));
        }
        ArrayPerS::from_vec(v)
    }
    fn append<T: Clone + Eq>(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {
        let n = a.length() + b.length();
        if n == 0 {
            return <ArrayPerS<T> as ArraySeqPerTrait<T>>::empty();
        }
        let mut v: Vec<T> = Vec::with_capacity(n);
        for i in 0..a.length() {
            v.push(a.nth(i).clone());
        }
        for j in 0..b.length() {
            v.push(b.nth(j).clone());
        }
        ArrayPerS::from_vec(v)
    }
    fn filter<T: Clone + Eq>(a: &ArrayPerS<T>, pred: impl Fn(&T) -> B) -> ArrayPerS<T> {
        let mut v: Vec<T> = Vec::new();
        for i in 0..a.length() {
            if pred(a.nth(i)) == B::True {
                v.push(a.nth(i).clone());
            }
        }
        ArrayPerS::from_vec(v)
    }
    fn update<T: Clone + Eq>(a: &ArrayPerS<T>, (index, item): (N, T)) -> ArrayPerS<T> {
        match a.set(index, item) {
            Ok(updated) => updated,
            Err(_) => a.clone(),
        }
    }
    fn inject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {
        let mut v: Vec<T> = a.data.to_vec();
        let mut seen = std::collections::HashSet::new();
        for k in 0..updates.length() {
            let (idx, val) = updates.nth(k).clone();
            if (idx as usize) < v.len() && !seen.contains(&idx) {
                v[idx] = val;
                seen.insert(idx);
            }
        }
        ArrayPerS::from_vec(v)
    }
    fn ninject<T: Clone + Eq>(a: &ArrayPerS<T>, updates: &ArrayPerS<(N, T)>) -> ArrayPerS<T> {
        let mut v: Vec<T> = a.data.to_vec();
        for k in 0..updates.length() {
            let (idx, val) = updates.nth(k).clone();
            if (idx as usize) < v.len() {
                v[idx] = val;
            }
        }
        ArrayPerS::from_vec(v)
    }
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x;
        for i in 0..a.length() {
            acc = f(&acc, a.nth(i));
        }
        acc
    }
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(
        a: &ArrayPerS<T>,
        f: impl Fn(&A, &T) -> A,
        x: A,
    ) -> (ArrayPerS<A>, A) {
        let mut acc = x.clone();
        let mut v: Vec<A> = Vec::with_capacity(a.length());
        for i in 0..a.length() {
            v.push(acc.clone());
            acc = f(&acc, a.nth(i));
        }
        (ArrayPerS::from_vec(v), acc)
    }
    fn reduce<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        fn rec<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = s.len();
            if n == 0 {
                return id;
            }
            if n == 1 {
                return s[0].clone();
            }
            let m = n / 2;
            let l = rec(&s[..m], f, id.clone());
            let r = rec(&s[m..], f, id);
            f(&l, &r)
        }
        rec(&a.data[..], f, id)
    }
    fn scan<T: Clone + Eq>(a: &ArrayPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayPerS<T>, T) {
        fn rec<T: Clone + Eq>(s: &[T], f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = s.len();
            if n == 0 {
                return id;
            }
            if n == 1 {
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
            prefixes.push(rec(&a.data[..i], f, id.clone()));
        }
        let total = rec(&a.data[..], f, id);
        (ArrayPerS::from_vec(prefixes), total)
    }
    fn flatten<T: Clone + Eq>(ss: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {
        let mut v: Vec<T> = Vec::new();
        for i in 0..ss.length() {
            let inner = ss.nth(i);
            for j in 0..inner.length() {
                v.push(inner.nth(j).clone());
            }
        }
        ArrayPerS::from_vec(v)
    }
    fn collect<A: Clone + Eq, Bv: Clone + Eq>(
        a: &ArrayPerS<(A, Bv)>,
        cmp: impl Fn(&A, &A) -> O,
    ) -> ArrayPerS<(A, ArrayPerS<Bv>)> {
        let mut groups: Vec<(A, Vec<Bv>)> = Vec::new();
        for i in 0..a.length() {
            let (k, v) = a.nth(i).clone();
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
        let pairs: Vec<(A, ArrayPerS<Bv>)> = groups
            .into_iter()
            .map(|(k, vs)| (k, ArrayPerS::from_vec(vs)))
            .collect();
        ArrayPerS::from_vec(pairs)
    }
}

}

pub use ArraySeqPerChap18::ArraySeqPerChap18Trait;
