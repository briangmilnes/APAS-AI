//! Chapter 19 algorithms for ArraySeqStPer.

pub mod ArraySeqStPerChap19 {
    use std::sync::Mutex;

    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::ArraySeqStPerChap18::ArraySeqStPerChap18Trait;
    use crate::Types::Types::*;

    pub trait ArraySeqStPerChap19Trait<T: MtT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: MtT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U>;
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
        fn reduce<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> (ArrayStPerS<T>, T)
        where
            F: Fn(&T, &T) -> T;
        /// APAS: Work Θ(1 + |a| + Σ |x|), Span Θ(1 + lg|a|)
        fn flatten(s: &ArrayStPerS<ArrayStPerS<T>>) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1 + |a| + |changes|), Span Θ(lg(degree))
        /// gpt-5-hard: Work Θ(1 + |a| + |changes|), Span Θ(1)
        /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
        fn inject(values: &ArrayStPerS<T>, changes: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn atomicWrite(
            values_with_change_number: &mut ArrayStPerS<Pair<T, N>>,
            changes: &ArrayStPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn inject_parallel2(values: &ArrayStPerS<T>, changes: &ArrayStPerS<Pair<N, T>>)
            -> ArrayStPerS<T>;
        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayStPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayStPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn ninject_parallel2(
            values: &ArrayStPerS<T>,
            changes: &ArrayStPerS<Pair<N, T>>,
        ) -> ArrayStPerS<T>;
        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayStPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayStPerS<Pair<N, T>>,
            change_index: N,
        );
    }

    impl<T: MtT> ArraySeqStPerChap19Trait<T> for ArrayStPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayStPerS<T> {
            let v: Vec<T> = (0..n).map(|i| f(i)).collect();
            ArrayStPerS::from_vec(v)
        }
        fn map<U: MtT>(a: &ArrayStPerS<T>, f: impl Fn(&T) -> U) -> ArrayStPerS<U> {
            <ArrayStPerS<U> as ArraySeqStPerChap19Trait<T>>::tabulate(|i| f(a.nth(i)), a.length())
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
            let keep = f(x) == B::True;
            <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::tabulate(
                |_| x.clone(),
                if keep { 1 } else { 0 },
            )
        }
        fn filter(a: &ArrayStPerS<T>, f: impl Fn(&T) -> B) -> ArrayStPerS<T> {
            let mapped: ArrayStPerS<ArrayStPerS<T>> =
                <ArrayStPerS<ArrayStPerS<T>> as ArraySeqStPerChap19Trait<T>>::tabulate(
                    |i| Self::deflate(|elt| f(elt), a.nth(i)),
                    a.length(),
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
                <ArrayStPerS<T> as crate::ArraySeqStPerChap19::ArraySeqStPerChap19Trait<T>>::iterate(
                    &rest, f, first,
                )
            }
        }
        fn reduce<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T,
        {
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
            let l = <ArrayStPerS<T> as crate::ArraySeqStPerChap19::ArraySeqStPerChap19Trait<T>>::reduce(
                &left,
                f,
                id.clone(),
            );
            let r = <ArrayStPerS<T> as crate::ArraySeqStPerChap19::ArraySeqStPerChap19Trait<T>>::reduce(
                &right, f, id,
            );
            f(&l, &r)
        }
        fn scan<F>(a: &ArrayStPerS<T>, f: &F, id: T) -> (ArrayStPerS<T>, T)
        where
            F: Fn(&T, &T) -> T,
        {
            let n = a.length();
            if n == 0 {
                (
                    <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(|_| id.clone(), 0),
                    id,
                )
            } else if n == 1 {
                (
                    <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(|_| id.clone(), 1),
                    a.nth(0).clone(),
                )
            } else {
                let half = (n + 1) / 2;
                let pairwise = <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(
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
                let (reductions, total) =
                    <ArrayStPerS<T> as crate::ArraySeqStPerChap19::ArraySeqStPerChap19Trait<T>>::scan(
                        &pairwise, f, id,
                    );
                let prefixes = <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(
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
        fn inject(values: &ArrayStPerS<T>, changes: &ArrayStPerS<Pair<N, T>>) -> ArrayStPerS<T> {
            let n = values.length();
            let mut with_num: ArrayStPerS<Pair<T, N>> =
                <ArrayStPerS<Pair<T, N>> as ArraySeqStPerChap19Trait<T>>::tabulate(
                    |i| Pair(values.nth(i).clone(), n),
                    n,
                );
            for ci in 0..changes.length() {
                Self::atomicWrite(&mut with_num, changes, ci);
            }
            <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(|i| with_num.nth(i).0.clone(), n)
        }
        fn atomicWrite(
            values_with_change_number: &mut ArrayStPerS<Pair<T, N>>,
            changes: &ArrayStPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let Pair(loc, val) = changes.nth(change_index).clone();
            if loc < values_with_change_number.length() {
                let Pair(_, num) = values_with_change_number.nth(loc).clone();
                if change_index < num {
                    let _ = *values_with_change_number = values_with_change_number
                        .set(loc, Pair(val, change_index))
                        .unwrap();
                }
            }
        }
        fn inject_parallel2(
            values: &ArrayStPerS<T>,
            changes: &ArrayStPerS<Pair<N, T>>,
        ) -> ArrayStPerS<T> {
            let n = values.length();
            let with_num: ArrayStPerS<Mutex<Pair<T, N>>> =
                <ArrayStPerS<Mutex<Pair<T, N>>> as ArraySeqStPerChap19Trait<T>>::tabulate(
                    |i| Mutex::new(Pair(values.nth(i).clone(), n)),
                    n,
                );
            std::thread::scope(|scope| {
                for ci in 0..changes.length() {
                    let vr = &with_num;
                    let cr = changes;
                    scope.spawn(move || {
                        Self::AtomicWriteLowestChangeNumberWins(vr, cr, ci);
                    });
                }
            });
            <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(
                |i| {
                    let g = with_num.nth(i).lock().unwrap();
                    g.0.clone()
                },
                n,
            )
        }
        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayStPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayStPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let Pair(loc, val) = changes.nth(change_index).clone();
            if loc >= values_with_change_number.length() {
                return;
            }
            let mut g = values_with_change_number.nth(loc).lock().unwrap();
            let Pair(ref mut cur, ref mut num) = *g;
            if change_index < *num {
                *cur = val;
                *num = change_index;
            }
        }
        fn ninject_parallel2(
            values: &ArrayStPerS<T>,
            changes: &ArrayStPerS<Pair<N, T>>,
        ) -> ArrayStPerS<T> {
            let n = values.length();
            let with_num: ArrayStPerS<Mutex<Pair<T, N>>> =
                <ArrayStPerS<Mutex<Pair<T, N>>> as ArraySeqStPerChap19Trait<T>>::tabulate(
                    |i| Mutex::new(Pair(values.nth(i).clone(), 0)),
                    n,
                );
            std::thread::scope(|scope| {
                for ci in 0..changes.length() {
                    let vr = &with_num;
                    let cr = changes;
                    scope.spawn(move || {
                        Self::AtomicWriteHighestChangeNumberWins(vr, cr, ci);
                    });
                }
            });
            <ArrayStPerS<T> as ArraySeqStPerChap19Trait<T>>::tabulate(
                |i| {
                    let g = with_num.nth(i).lock().unwrap();
                    g.0.clone()
                },
                n,
            )
        }
        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayStPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayStPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let Pair(loc, val) = changes.nth(change_index).clone();
            if loc >= values_with_change_number.length() {
                return;
            }
            let mut g = values_with_change_number.nth(loc).lock().unwrap();
            let Pair(ref mut cur, ref mut num) = *g;
            if change_index >= *num {
                *cur = val;
                *num = change_index;
            }
        }
    }
}

pub use ArraySeqStPerChap19::ArraySeqStPerChap19Trait;
