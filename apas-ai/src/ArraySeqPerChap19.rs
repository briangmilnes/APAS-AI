//! Chapter 19 algorithms for ArraySeqPer.

pub mod ArraySeqPerChap19 {
    use std::sync::Mutex;

    use crate::ArraySeqPer::ArraySeqPer::*;
    use crate::ArraySeqPerChap18::ArraySeqPerChap18Trait;
    use crate::Types::Types::*;

    pub trait ArraySeqPerChap19Trait<T: MtT> {
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<U: MtT>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn select<'a>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        fn append2(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(f(a[i]))), Span Θ(1 + max i S(f(a[i])))
        fn filter(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T>;
        /// APAS: Work Θ(1 + Σ W(f)), Span Θ(1 + Σ S(f))
        /// gpt-5-hard: Work Θ(|a|^2), Span Θ(|a|)
        /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
        fn iterate<A: StT>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn reduce<F>(a: &ArrayPerS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan<F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T)
        where
            F: Fn(&T, &T) -> T;
        /// APAS: Work Θ(1 + |a| + Σ |x|), Span Θ(1 + lg|a|)
        fn flatten(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T>;
        /// APAS: Work Θ(1 + |a| + |changes|), Span Θ(lg(degree))
        /// gpt-5-hard: Work Θ(1 + |a| + |changes|), Span Θ(1)
        /// BUG: APAS and gpt-5-hard algorithmic analyses differ.
        fn inject(values: &ArrayPerS<T>, changes: &ArrayPerS<Pair<N, T>>) -> ArrayPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn atomicWrite(
            values_with_change_number: &mut ArrayPerS<Pair<T, N>>,
            changes: &ArrayPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn inject_parallel2(values: &ArrayPerS<T>, changes: &ArrayPerS<Pair<N, T>>)
            -> ArrayPerS<T>;
        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn ninject_parallel2(
            values: &ArrayPerS<T>,
            changes: &ArrayPerS<Pair<N, T>>,
        ) -> ArrayPerS<T>;
        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayPerS<Pair<N, T>>,
            change_index: N,
        );
    }

    impl<T: MtT> ArraySeqPerChap19Trait<T> for ArrayPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayPerS<T> {
            let v: Vec<T> = (0..n).map(|i| f(i)).collect();
            ArrayPerS::from_vec(v)
        }
        fn map<U: MtT>(a: &ArrayPerS<T>, f: impl Fn(&T) -> U) -> ArrayPerS<U> {
            <ArrayPerS<U> as ArraySeqPerChap19Trait<T>>::tabulate(|i| f(a.nth(i)), a.length())
        }
        fn select<'a>(a: &'a ArrayPerS<T>, b: &'a ArrayPerS<T>, i: N) -> Option<&'a T> {
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
        fn append(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {
            <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(
                |i| Self::select(a, b, i).unwrap().clone(),
                a.length() + b.length(),
            )
        }
        fn append2(a: &ArrayPerS<T>, b: &ArrayPerS<T>) -> ArrayPerS<T> {
            <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(
                |i| Self::select(a, b, i).unwrap().clone(),
                a.length() + b.length(),
            )
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArrayPerS<T> {
            let keep = f(x) == B::True;
            <ArrayPerS<T> as ArraySeqPerChap18Trait<T>>::tabulate(
                |_| x.clone(),
                if keep { 1 } else { 0 },
            )
        }
        fn filter(a: &ArrayPerS<T>, f: impl Fn(&T) -> B) -> ArrayPerS<T> {
            let mapped: ArrayPerS<ArrayPerS<T>> =
                <ArrayPerS<ArrayPerS<T>> as ArraySeqPerChap19Trait<T>>::tabulate(
                    |i| Self::deflate(|elt| f(elt), a.nth(i)),
                    a.length(),
                );
            <ArrayPerS<T> as ArraySeqPerChap18Trait<T>>::flatten(&mapped)
        }
        fn iterate<A: StT>(a: &ArrayPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let n = a.length();
            if n == 0 {
                x
            } else if n == 1 {
                f(&x, a.nth(0))
            } else {
                let first = f(&x, a.nth(0));
                let rest = ArrayPerS::from_vec(a.subseq(1, n - 1).to_vec());
                <ArrayPerS<T> as crate::ArraySeqPerChap19::ArraySeqPerChap19Trait<T>>::iterate(
                    &rest, f, first,
                )
            }
        }
        fn reduce<F>(a: &ArrayPerS<T>, f: &F, id: T) -> T
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
            let left = ArrayPerS::from_vec(a.subseq(0, m).to_vec());
            let right = ArrayPerS::from_vec(a.subseq(m, n - m).to_vec());
            let l = <ArrayPerS<T> as crate::ArraySeqPerChap19::ArraySeqPerChap19Trait<T>>::reduce(
                &left,
                f,
                id.clone(),
            );
            let r = <ArrayPerS<T> as crate::ArraySeqPerChap19::ArraySeqPerChap19Trait<T>>::reduce(
                &right, f, id,
            );
            f(&l, &r)
        }
        fn scan<F>(a: &ArrayPerS<T>, f: &F, id: T) -> (ArrayPerS<T>, T)
        where
            F: Fn(&T, &T) -> T,
        {
            let n = a.length();
            if n == 0 {
                (
                    <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(|_| id.clone(), 0),
                    id,
                )
            } else if n == 1 {
                (
                    <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(|_| id.clone(), 1),
                    a.nth(0).clone(),
                )
            } else {
                let half = (n + 1) / 2;
                let pairwise = <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(
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
                    <ArrayPerS<T> as crate::ArraySeqPerChap19::ArraySeqPerChap19Trait<T>>::scan(
                        &pairwise, f, id,
                    );
                let prefixes = <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(
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
        fn flatten(s: &ArrayPerS<ArrayPerS<T>>) -> ArrayPerS<T> {
            <ArrayPerS<T> as ArraySeqPerChap18Trait<T>>::flatten(s)
        }
        fn inject(values: &ArrayPerS<T>, changes: &ArrayPerS<Pair<N, T>>) -> ArrayPerS<T> {
            let n = values.length();
            let mut with_num: ArrayPerS<Pair<T, N>> =
                <ArrayPerS<Pair<T, N>> as ArraySeqPerChap19Trait<T>>::tabulate(
                    |i| Pair(values.nth(i).clone(), n),
                    n,
                );
            for ci in 0..changes.length() {
                Self::atomicWrite(&mut with_num, changes, ci);
            }
            <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(|i| with_num.nth(i).0.clone(), n)
        }
        fn atomicWrite(
            values_with_change_number: &mut ArrayPerS<Pair<T, N>>,
            changes: &ArrayPerS<Pair<N, T>>,
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
            values: &ArrayPerS<T>,
            changes: &ArrayPerS<Pair<N, T>>,
        ) -> ArrayPerS<T> {
            let n = values.length();
            let with_num: ArrayPerS<Mutex<Pair<T, N>>> =
                <ArrayPerS<Mutex<Pair<T, N>>> as ArraySeqPerChap19Trait<T>>::tabulate(
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
            <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(
                |i| {
                    let g = with_num.nth(i).lock().unwrap();
                    g.0.clone()
                },
                n,
            )
        }
        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayPerS<Pair<N, T>>,
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
            values: &ArrayPerS<T>,
            changes: &ArrayPerS<Pair<N, T>>,
        ) -> ArrayPerS<T> {
            let n = values.length();
            let with_num: ArrayPerS<Mutex<Pair<T, N>>> =
                <ArrayPerS<Mutex<Pair<T, N>>> as ArraySeqPerChap19Trait<T>>::tabulate(
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
            <ArrayPerS<T> as ArraySeqPerChap19Trait<T>>::tabulate(
                |i| {
                    let g = with_num.nth(i).lock().unwrap();
                    g.0.clone()
                },
                n,
            )
        }
        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayPerS<Pair<N, T>>,
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

pub use ArraySeqPerChap19::ArraySeqPerChap19Trait;
