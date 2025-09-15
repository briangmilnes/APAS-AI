//! Chapter 18 algorithms for LinkedListEph (ephemeral).

pub mod LinkedListEphChap18 {
use crate::LinkedListEph::LinkedListEph::*;
use crate::Types::Types::*;
use std::collections::HashSet;

pub trait LinkedListEphChap18Trait<T: StT> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;
    fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;
    fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;
    fn filter(a: &LinkedListEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListEphS<T>;
    fn update(a: &mut LinkedListEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListEphS<T>;
    fn inject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T>;
    fn ninject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T>;
    fn iterate< A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn iteratePrefixes< A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListEphS<A>, A);
    fn reduce(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
    fn scan(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListEphS<T>, T);
    fn flatten(ss: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;
    fn collect<A: StT, Bv: StT>(a: &LinkedListEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) -> LinkedListEphS<Pair<A, LinkedListEphS<Bv>>>;
}

impl<T: StT> LinkedListEphChap18Trait<T> for LinkedListEphS<T> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> {
        let mut v: Vec<T> = Vec::with_capacity(n);
        for i in 0..n { v.push(f(i)); }
        LinkedListEphS::from_vec(v)
    }
    fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> {
        let mut v: Vec<U> = Vec::with_capacity(<LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a));
        for i in 0..<LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a) { v.push(f(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i))); }
        LinkedListEphS::from_vec(v)
    }
    fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> {
        let na = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a);
        let nb = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(b);
        let mut v: Vec<T> = Vec::with_capacity(na + nb);
        for i in 0..na { v.push(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i).clone()); }
        for j in 0..nb { v.push(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(b, j).clone()); }
        LinkedListEphS::from_vec(v)
    }
    fn filter(a: &LinkedListEphS<T>, pred: impl Fn(&T) -> B) -> LinkedListEphS<T> {
        let mut v: Vec<T> = Vec::new();
        for i in 0..<LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a) {
            let x = <LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i);
            if pred(x) == B::True { v.push(x.clone()); }
        }
        LinkedListEphS::from_vec(v)
    }
    fn update(a: &mut LinkedListEphS<T>, item_at: Pair<N, T>) -> &mut LinkedListEphS<T> { <LinkedListEphS<T> as LinkedListEphTrait<T>>::update(a, item_at) }
    fn inject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T> {
        // first-update wins
        let mut out = <LinkedListEphS<T> as LinkedListEphTrait<T>>::subseq_copy(
            a,
            0,
            <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a),
        );
        let mut applied: HashSet<N> = HashSet::new();
        for i in 0..<LinkedListEphS<Pair<N, T>> as LinkedListEphTrait<Pair<N, T>>>::length(updates) {
            let Pair(idx, val) = <LinkedListEphS<Pair<N, T>> as LinkedListEphTrait<Pair<N, T>>>::nth(updates, i).clone();
            if !applied.contains(&idx) {
                let _ = <LinkedListEphS<T> as LinkedListEphTrait<T>>::set(&mut out, idx, val);
                applied.insert(idx);
            }
        }
        out
    }
    fn ninject(a: &LinkedListEphS<T>, updates: &LinkedListEphS<Pair<N, T>>) -> LinkedListEphS<T> {
        // last-update wins
        let mut out = <LinkedListEphS<T> as LinkedListEphTrait<T>>::subseq_copy(
            a,
            0,
            <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a),
        );
        for i in 0..<LinkedListEphS<Pair<N, T>> as LinkedListEphTrait<Pair<N, T>>>::length(updates) {
            let Pair(idx, val) = <LinkedListEphS<Pair<N, T>> as LinkedListEphTrait<Pair<N, T>>>::nth(updates, i).clone();
            let _ = <LinkedListEphS<T> as LinkedListEphTrait<T>>::set(&mut out, idx, val);
        }
        out
    }
    fn iterate< A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        let mut acc = x; for i in 0..<LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a) { acc = f(&acc, <LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i)); } acc
    }
    fn iteratePrefixes<A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (LinkedListEphS<A>, A) {
        let mut acc = x.clone(); let n = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a);
        let mut v: Vec<A> = Vec::with_capacity(n);
        for i in 0..n { v.push(acc.clone()); acc = f(&acc, <LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i)); }
        (LinkedListEphS::from_vec(v), acc)
    }
    fn reduce(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
        let n = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a);
        if n == 0 { return id; }
        if n == 1 { return <LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, 0).clone(); }
        let m = n / 2;
        let left = <LinkedListEphS<T> as LinkedListEphTrait<T>>::subseq_copy(a, 0, m);
        let right = <LinkedListEphS<T> as LinkedListEphTrait<T>>::subseq_copy(a, m, n - m);
        let l = <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::reduce(&left, f, id.clone());
        let r = <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::reduce(&right, f, id);
        f(&l, &r)
    }
    fn scan(a: &LinkedListEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (LinkedListEphS<T>, T) {
        let n = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a);
        if n == 0 { return (<LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::tabulate(|_| id.clone(), 0), id); }
        let mut prefixes: Vec<T> = Vec::with_capacity(n);
        let mut acc = id.clone();
        for i in 0..n { prefixes.push(acc.clone()); acc = f(&acc, <LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i)); }
        (LinkedListEphS::from_vec(prefixes), <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::reduce(a, f, id))
    }
    fn flatten(ss: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> {
        let mut v: Vec<T> = Vec::new();
        for i in 0..<LinkedListEphS<LinkedListEphS<T>> as LinkedListEphTrait<LinkedListEphS<T>>>::length(ss) {
            let inner = <LinkedListEphS<LinkedListEphS<T>> as LinkedListEphTrait<LinkedListEphS<T>>>::nth(ss, i);
            for j in 0..<LinkedListEphS<T> as LinkedListEphTrait<T>>::length(inner) {
                v.push(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(inner, j).clone());
            }
        }
        LinkedListEphS::from_vec(v)
    }
    fn collect<A: StT, Bv: StT>(a: &LinkedListEphS<Pair<A, Bv>>, cmp: impl Fn(&A, &A) -> O) -> LinkedListEphS<Pair<A, LinkedListEphS<Bv>>> {
        let mut groups: Vec<Pair<A, Vec<Bv>>> = Vec::new();
        for i in 0..<LinkedListEphS<Pair<A, Bv>> as LinkedListEphTrait<Pair<A, Bv>>>::length(a) {
            let Pair(k, v) = <LinkedListEphS<Pair<A, Bv>> as LinkedListEphTrait<Pair<A, Bv>>>::nth(a, i).clone();
            if let Some(Pair(_, gv)) = groups.iter_mut().find(|Pair(gk, _)| cmp(&k, gk) == O::Equal) { gv.push(v); } else { groups.push(Pair(k, vec![v])); }
        }
        let pairs: Vec<Pair<A, LinkedListEphS<Bv>>> = groups.into_iter().map(|Pair(k, vs)| Pair(k, LinkedListEphS::from_vec(vs))).collect();
        LinkedListEphS::from_vec(pairs)
    }
}

}

pub use LinkedListEphChap18::LinkedListEphChap18Trait;


