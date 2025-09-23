//! Chapter 18 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph{
    use crate::Types::Types::*;
    use crate::Types::Types::*;

    pub trait ArraySeqMtEphTrait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;
        fn update(a: &mut ArraySeqMtEphS<T>, item_at: (N, T)) -> &mut ArraySeqMtEphS<T>;
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T>;
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T>;
    }

    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {
            let mut v: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                v.push(f(i));
            }
            ArraySeqMtEphS::from_vec(v)
        }
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {
            let n = a.length();
            if n.clone() == 0 {
                return ArraySeqMtEphS::from_vec(Vec::new());
            }
            let first = f(&a.nth_cloned(0));
            let mut out = ArraySeqMtEphS::from_vec(vec![first.clone(); n]);
            let _ = out.set(0, first);
            for i in 1..n {
                let _ = out.set(i, f(&a.nth_cloned(i)));
            }
            out
        }
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            let na = a.length();
            let nb = b.length();
            let n = na + nb;
            if n.clone() == 0 {
                return ArraySeqMtEphS::from_vec(Vec::new());
            }
            let init = if na > 0usize { a.nth_cloned(0) } else { b.nth_cloned(0) };
            let mut out = ArraySeqMtEphS::from_vec(vec![init.clone(); n]);
            for i in 0..na {
                let _ = out.set(i, a.nth_cloned(i));
            }
            for j in 0..nb {
                let _ = out.set(na + j, b.nth_cloned(j));
            }
            out
        }
        fn filter(a: &ArraySeqMtEphS<T>, pred: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {
            let n = a.length();
            let mut kept: Vec<T> = Vec::new();
            kept.reserve(n);
            for i in 0..n {
                let x = a.nth_cloned(i);
                if pred(&x) == B::True {
                    kept.push(x);
                }
            }
            ArraySeqMtEphS::from_vec(kept)
        }
        fn update(a: &mut ArraySeqMtEphS<T>, (index, item): (N, T)) -> &mut ArraySeqMtEphS<T> {
            let _ = a.set(index, item);
            a
        }
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            let mut out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth_cloned(i);
                let _ = out.set(idx, val);
            }
            out
        }
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            Self::inject(a, updates)
        }
    }
}
