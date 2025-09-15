//! Chapter 18 algorithms for `ArraySeqEph<T>`.

pub mod ArraySeqEphChap18 {
    use crate::ArraySeqEph::ArraySeqEph::*;
    use crate::Types::Types::*;

    pub trait ArraySeqEphChap18Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;
        fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;
        fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;
        fn filter(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T>;
        fn update(a: &mut ArraySeqEphS<T>, item_at: (N, T)) -> &mut ArraySeqEphS<T>;
        fn inject(a: &ArraySeqEphS<T>, updates: &ArraySeqEphS<Pair<N, T>>) -> ArraySeqEphS<T>;
        fn ninject(a: &ArraySeqEphS<T>, updates: &ArraySeqEphS<Pair<N, T>>) -> ArraySeqEphS<T>;
        fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn iteratePrefixes<A: StT>(
            a: &ArraySeqEphS<T>,
            f: impl Fn(&A, &T) -> A,
            x: A,
        ) -> (ArraySeqEphS<A>, A);
        fn reduce(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(
            a: &ArraySeqEphS<T>,
            f: &impl Fn(&T, &T) -> T,
            id: T,
        ) -> (ArraySeqEphS<T>, T);
        fn flatten(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;
        fn collect<A: StT, Bv: StT>(
            a: &ArraySeqEphS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> ArraySeqEphS<Pair<A, ArraySeqEphS<Bv>>>;
    }

    impl<T: StT> ArraySeqEphChap18Trait<T> for ArraySeqEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            ArraySeqEphS::from_vec(v)
        }
        fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {
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
        fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {
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
        fn filter(a: &ArraySeqEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqEphS<T> {
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
        fn update(a: &mut ArraySeqEphS<T>, (index, item): (N, T)) -> &mut ArraySeqEphS<T> {
            a.update((index, item))
        }
        fn inject(
            a: &ArraySeqEphS<T>,
            updates: &ArraySeqEphS<Pair<N, T>>,
        ) -> ArraySeqEphS<T> {
            let mut out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                let _ = out.set(idx, val);
            }
            out
        }
        fn ninject(
            a: &ArraySeqEphS<T>,
            updates: &ArraySeqEphS<Pair<N, T>>,
        ) -> ArraySeqEphS<T> {
            Self::inject(a, updates)
        }
        fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }
        fn iteratePrefixes<A: StT>(
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
        fn reduce(a: &ArraySeqEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let mut acc = id;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }
        fn scan(
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
        fn flatten(ss: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {
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
        fn collect<A: StT, Bv: StT>(
            a: &ArraySeqEphS<Pair<A, Bv>>,
            cmp: impl Fn(&A, &A) -> O,
        ) -> ArraySeqEphS<Pair<A, ArraySeqEphS<Bv>>> {
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
            let out: Vec<Pair<A, ArraySeqEphS<Bv>>> = groups
                .into_iter()
                .map(|Pair(k, gv)| Pair(k, ArraySeqEphS::from_vec(gv)))
                .collect();
            ArraySeqEphS::from_vec(out)
        }
    }
}

pub use ArraySeqEphChap18::ArraySeqEphChap18Trait;
