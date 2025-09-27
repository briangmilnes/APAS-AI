//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEphChap19 {
    use crate::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::ArraySeqMtEphChap18::ArraySeqMtEphChap18::*;
    use crate::Types::Types::*;

    pub trait ArraySeqMtEphChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U>;
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T>;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T>;
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T>;
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqMtEphS<T>, T);
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T>;
    }

    impl<T: StT> ArraySeqMtEphChap19Trait<T> for ArraySeqMtEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtEphS<T> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqMtEphS<U> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphChap18Trait<T>>::map(a, f)
        }
        fn select<'a>(a: &'a ArraySeqMtEphS<T>, b: &'a ArraySeqMtEphS<T>, i: N) -> Option<T> {
            if i < a.length() {
                Some(a.nth_cloned(i))
            } else {
                let off = i - a.length();
                if off < b.length() {
                    Some(b.nth_cloned(off))
                } else {
                    None
                }
            }
        }
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphChap18Trait<T>>::append(a, b)
        }
        fn append2(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqMtEphS<T> {
            if f(x) == true {
                ArraySeqMtEphS::from_vec(vec![x.clone()])
            } else {
                ArraySeqMtEphS::from_vec(Vec::new())
            }
        }
        fn filter(a: &ArraySeqMtEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqMtEphS<T> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphChap18Trait<T>>::filter(a, f)
        }
        fn iterate<A: StT>(a: &ArraySeqMtEphS<T>, f: impl Fn(&A, &T) -> A, mut x: A) -> A {
            for i in 0..a.length() {
                x = f(&x, &a.nth_cloned(i));
            }
            x
        }
        fn reduce(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> T {
            for i in 0..a.length() {
                id = f(&id, &a.nth_cloned(i));
            }
            id
        }
        fn scan(a: &ArraySeqMtEphS<T>, f: &impl Fn(&T, &T) -> T, mut id: T) -> (ArraySeqMtEphS<T>, T) {
            let n = a.length();
            let mut out = if n == 0 {
                ArraySeqMtEphS::from_vec(Vec::new())
            } else {
                ArraySeqMtEphS::from_vec(vec![id.clone(); n])
            };
            for i in 0..n {
                let _ = out.set(i, id.clone());
                id = f(&id, &a.nth_cloned(i));
            }
            (out, id)
        }
        fn flatten(s: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {
            let mut total: N = 0;
            let n_outer = s.length();
            for i in 0..n_outer {
                total += s.nth_cloned(i).length();
            }
            if total == 0 {
                return ArraySeqMtEphS::from_vec(Vec::new());
            }
            // Initialize with first available value to build fixed-length vector
            let mut init: Option<T> = None;
            for i in 0..n_outer {
                let inner = s.nth_cloned(i);
                if inner.length() > 0 {
                    init = Some(inner.nth_cloned(0));
                    break;
                }
            }
            let initv = init.expect("total > 0 implies some inner non-empty");
            let mut out = ArraySeqMtEphS::from_vec(vec![initv.clone(); total]);
            let mut w: N = 0;
            for i in 0..n_outer {
                let inner = s.nth_cloned(i);
                for j in 0..inner.length() {
                    let _ = out.set(w, inner.nth_cloned(j));
                    w += 1;
                }
            }
            out
        }
    }
}
