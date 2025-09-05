//! Chapter 18 algorithms for `ArraySEph<T>`.

use crate::ArraySeqEph::{ArraySeqEphS, ArraySeqEphTrait};
use crate::Types::{B, N, O};

pub trait ArraySeqEphChap18Trait {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T>;
    fn update<T: Clone + Eq>(a: &mut ArraySeqEphS<T>, item_at: (N, T)) -> &mut ArraySeqEphS<T>;
    fn inject<T: Clone + Eq>(
        a: &ArraySeqEphS<T>,
        updates: &ArraySeqEphS<(N, T)>,
    ) -> ArraySeqEphS<T>;
    fn ninject<T: Clone + Eq>(
        a: &ArraySeqEphS<T>,
        updates: &ArraySeqEphS<(N, T)>,
    ) -> ArraySeqEphS<T>;
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(
        a: &ArraySeqEphS<T>,
        f: impl Fn(&A, &T) -> A,
        x: A,
    ) -> (ArraySeqEphS<A>, A);
    fn reduce<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
    fn scan<T: Clone + Eq>(
        a: &ArraySeqEphS<T>,
        f: &impl Fn(&T, &T) -> T,
        id: T,
    ) -> (ArraySeqEphS<T>, T);
    fn flatten<T: Clone + Eq>(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;
    fn collect<A: Clone + Eq, Bv: Clone>(
        a: &ArraySeqEphS<(A, Bv)>,
        cmp: impl Fn(&A, &A) -> O,
    ) -> ArraySeqEphS<(A, ArraySeqEphS<Bv>)>;
}

impl<T2> ArraySeqEphChap18Trait for ArraySeqEphS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {
        let mut v: Vec<T> = Vec::with_capacity(n);
        for i in 0..n {
            v.push(f(i));
        }
        ArraySeqEphS::from_vec(v)
    }
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {
        let n = a.length();
        if n == 0 {
            return <ArraySeqEphS<U> as ArraySeqEphTrait<U>>::empty();
        }
        let first = f(a.nth(0)).clone();
        let mut out = <ArraySeqEphS<U> as ArraySeqEphTrait<U>>::new(n, first.clone());
        let _ = out.set(0, first);
        for i in 1..n {
            let _ = out.set(i, f(a.nth(i)));
        }
        out
    }
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {
        let na = a.length();
        let nb = b.length();
        let n = na + nb;
        if n == 0 {
            return <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::empty();
        }
        let init = if na > 0 {
            a.nth(0).clone()
        } else {
            b.nth(0).clone()
        };
        let mut out = <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::new(n, init.clone());
        for i in 0..na {
            let _ = out.set(i, a.nth(i).clone());
        }
        for j in 0..nb {
            let _ = out.set(na + j, b.nth(j).clone());
        }
        out
    }
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T> {
        let n = a.length();
        let mut kept: Vec<T> = Vec::new();
        kept.reserve(n);
        for i in 0..n {
            let x = a.nth(i);
            if pred(x) == B::True {
                kept.push(x.clone());
            }
        }
        ArraySeqEphS::from_vec(kept)
    }
    fn update<T: Clone + Eq>(
        a: &mut ArraySeqEphS<T>,
        (index, item): (N, T),
    ) -> &mut ArraySeqEphS<T> {
        a.update((index, item))
    }
    fn inject<T: Clone + Eq>(
        a: &ArraySeqEphS<T>,
        updates: &ArraySeqEphS<(N, T)>,
    ) -> ArraySeqEphS<T> {
        let mut out = a.clone();
        for i in 0..updates.length() {
            let (idx, val) = updates.nth(i);
            let _ = out.set(*idx, val.clone());
        }
        out
    }
    fn ninject<T: Clone + Eq>(
        a: &ArraySeqEphS<T>,
        updates: &ArraySeqEphS<(N, T)>,
    ) -> ArraySeqEphS<T> {
        Self::inject(a, updates)
    }
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x;
        for i in 0..a.length() {
            acc = f(&acc, a.nth(i));
        }
        acc
    }
    fn iteratePrefixes<T: Clone + Eq, A: Clone>(
        a: &ArraySeqEphS<T>,
        f: impl Fn(&A, &T) -> A,
        x: A,
    ) -> (ArraySeqEphS<A>, A) {
        let n = a.length();
        let mut acc = x;
        if n == 0 {
            return (<ArraySeqEphS<A> as ArraySeqEphTrait<A>>::empty(), acc);
        }
        let mut out = <ArraySeqEphS<A> as ArraySeqEphTrait<A>>::new(n, acc.clone());
        for i in 0..n {
            let _ = out.set(i, acc.clone());
            acc = f(&acc, a.nth(i));
        }
        (out, acc)
    }
    fn reduce<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        let mut acc = id;
        for i in 0..a.length() {
            acc = f(&acc, a.nth(i));
        }
        acc
    }
    fn scan<T: Clone + Eq>(
        a: &ArraySeqEphS<T>,
        f: &impl Fn(&T, &T) -> T,
        id: T,
    ) -> (ArraySeqEphS<T>, T) {
        let n = a.length();
        let mut acc = id;
        let mut out = if n == 0 {
            <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::empty()
        } else {
            <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::new(n, acc.clone())
        };
        for i in 0..n {
            let _ = out.set(i, acc.clone());
            acc = f(&acc, a.nth(i));
        }
        (out, acc)
    }
    fn flatten<T: Clone + Eq>(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {
        let mut total: N = 0;
        for i in 0..ss.length() {
            total += ss.nth(i).length();
        }
        if total == 0 {
            return <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::empty();
        }
        let mut init: Option<T> = None;
        for i in 0..ss.length() {
            let inner = ss.nth(i);
            if inner.length() > 0 {
                init = Some(inner.nth(0).clone());
                break;
            }
        }
        let initv = init.expect("total > 0 implies some inner non-empty");
        let mut out = <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::new(total, initv.clone());
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
    fn collect<A: Clone + Eq, Bv: Clone>(
        a: &ArraySeqEphS<(A, Bv)>,
        cmp: impl Fn(&A, &A) -> O,
    ) -> ArraySeqEphS<(A, ArraySeqEphS<Bv>)> {
        let mut groups: Vec<(A, Vec<Bv>)> = Vec::new();
        for i in 0..a.length() {
            let (k, v) = a.nth(i);
            let mut found = false;
            for (gk, gv) in groups.iter_mut() {
                if cmp(k, gk) == O::Equal {
                    gv.push(v.clone());
                    found = true;
                    break;
                }
            }
            if !found {
                groups.push((k.clone(), vec![v.clone()]));
            }
        }
        let out: Vec<(A, ArraySeqEphS<Bv>)> = groups
            .into_iter()
            .map(|(k, gv)| (k, ArraySeqEphS::from_vec(gv)))
            .collect();
        ArraySeqEphS::from_vec(out)
    }
}
