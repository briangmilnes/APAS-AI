//! Chapter 19 algorithms for ArraySeqMtPer.

pub mod ArraySeqStPerChap19 {
    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
    use crate::Types::Types::*;

    pub trait ArraySeqStPerChap19Trait<T: StT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(1 + max i S(f(a[i])))
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1 + Σ W(f)), Span Θ(1 + Σ S(f))
        /// gpt-5-hard: Work Θ(|a|^2), Span Θ(|a|)
        /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T);
        /// APAS: Work Θ(1 + |a| + Σ |x|), Span Θ(1 + lg|a|)
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(lg|a|)
        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(lg|a|)
        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;
    }

    impl<T: StT> ArraySeqStPerChap19Trait<T> for ArrayStPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {
            <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::tabulate(f, n)
        }

        fn map<U: StT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {
            <ArrayStPerS<U> as ArraySeqStPerChap18Trait<U>>::tabulate(|i| f(a.nth(i)), a.length())
        }
        fn select<'a>(a: &'a ArrayStPerS<T>, b: &'a ArrayStPerS<T>, i: N) -> Option<&'a T> {
            if i < a.length() {
                Some(a.nth(i))
            } else {
                let j = i - a.length();
                if j < b.length() {
                    Some(b.nth(j))
                } else {
                    None
                }
            }
        }

        fn append(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {
            <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(
                |i| Self::select(a, b, i).unwrap().clone(),
                a.length() + b.length(),
            )
        }

        fn append2(a: &ArrayStPerS<T>, b: &ArrayStPerS<T>) -> ArrayStPerS<T> {
            <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(
                |i| Self::select(a, b, i).unwrap().clone(),
                a.length() + b.length(),
            )
        }

        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayStPerS<T> {
            let keep = f(x) == true;
            <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(|_| x.clone(), if keep { 1 } else { 0 })
        }
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {
            let mapped: ArrayStPerS<ArrayStPerS<T>> = <ArrayStPerS<ArrayStPerS<T>> as ArraySeqStPerChap18Trait<
                ArrayStPerS<T>,
            >>::tabulate(
                |i| Self::deflate(|elt| f(elt), a.nth(i)), a.length()
            );
            <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::flatten(&mapped)
        }

        fn iterate<A: StT>(a: &ArrayStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let n = a.length();
            if n == 0 {
                x
            } else if n == 1 {
                f(&x, a.nth(0))
            } else {
                let first = f(&x, a.nth(0));
                let rest = ArrayStPerS::from_vec(a.subseq(1, n - 1).to_vec());
                <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::iterate(&rest, f, first)
            }
        }
        fn reduce(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            let n = a.length();
            if n == 0 {
                return id;
            }
            if n == 1 {
                return a.nth(0).clone();
            }
            let m = n / 2;
            let left = ArrayStPerS::from_vec(a.subseq(0, m).to_vec());
            let right = ArrayStPerS::from_vec(a.subseq(m, n - m).to_vec());
            let l = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::reduce(&left, &f, id.clone());
            let r = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::reduce(&right, &f, id);
            f(&l, &r)
        }
        fn scan(a: &ArrayStPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayStPerS<T>, T) {
            let n = a.length();
            if n == 0 {
                (
                    <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::tabulate(|_| id.clone(), 0),
                    id,
                )
            } else if n == 1 {
                (
                    <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::tabulate(|_| id.clone(), 1),
                    a.nth(0).clone(),
                )
            } else {
                let half = (n + 1) / 2;
                let pairwise = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::tabulate(
                    |i| {
                        let l = a.nth(2 * i);
                        let r_i = 2 * i + 1;
                        if r_i < n {
                            let r = a.nth(r_i);
                            f(l, r)
                        } else {
                            l.clone()
                        }
                    },
                    half,
                );
                let (reductions, total) = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::scan(&pairwise, &f, id);
                let prefixes = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::tabulate(
                    |i| {
                        if i % 2 == 0 {
                            reductions.nth(i / 2).clone()
                        } else {
                            let prev = a.nth(i - 1);
                            let base = reductions.nth(i / 2);
                            f(base, prev)
                        }
                    },
                    n,
                );
                (prefixes, total)
            }
        }
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T> {
            <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::flatten(s)
        }

        fn inject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {
            <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::inject(a, updates)
        }

        fn ninject(a: &ArrayStPerS<T>, updates: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {
            <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::ninject(a, updates)
        }
    }
}
