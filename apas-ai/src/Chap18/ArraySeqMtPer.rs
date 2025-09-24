//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for ArraySeqMtPer multithreaded, just tabulate needed.

pub mod ArraySeqMtPer {
    use std::collections::HashSet;
    use std::thread;

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Box<[T]>` (persistent MT variant).
    #[derive(Debug)]
    pub struct ArraySeqMtPerS<T: StTInMtT> {
        data: Box<[T]>,
    }

    impl<T: StTInMtT> ArraySeqMtPerS<T> {
        pub fn empty() -> Self {
            ArraySeqMtPerS {
                data: Vec::new().into_boxed_slice(),
            }
        }

        pub fn new(length: N, init_value: T) -> Self {
            let mut values: Vec<T> = Vec::with_capacity(length);
            for _ in 0..length {
                values.push(init_value.clone());
            }
            ArraySeqMtPerS::from_vec(values)
        }

        pub fn singleton(item: T) -> Self { ArraySeqMtPerS::from_vec(vec![item]) }

        pub fn from_vec(values: Vec<T>) -> Self {
            ArraySeqMtPerS {
                data: values.into_boxed_slice(),
            }
        }

        pub fn length(&self) -> N { self.data.len() }

        pub fn nth(&self, index: N) -> &T { &self.data[index] }


        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.data.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            let values: Vec<T> = self.data[s..e].iter().cloned().collect();
            ArraySeqMtPerS::from_vec(values)
        }

        pub fn is_empty(&self) -> B { if self.data.is_empty() { B::True } else { B::False } }

        pub fn is_singleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }
    }

    impl<T: StTInMtT> Clone for ArraySeqMtPerS<T> {
        fn clone(&self) -> Self {
            let values: Vec<T> = self.data.iter().cloned().collect();
            ArraySeqMtPerS::from_vec(values)
        }
    }

    impl<T: StTInMtT + PartialEq> PartialEq for ArraySeqMtPerS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.length() != other.length() {
                return false;
            }
            for i in 0..self.length() {
                if self.nth(i) != other.nth(i) {
                    return false;
                }
            }
            true
        }
    }

    impl<T: StTInMtT + Eq> Eq for ArraySeqMtPerS<T> {}

    impl<T: StTInMtT> std::fmt::Display for ArraySeqMtPerS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtPerS[")?;
            for (i, item) in self.data.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    pub trait ArraySeqMtPerTrait<T: StTInMtT> {
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn empty() -> ArraySeqMtPerS<T>;
        fn singleton(item: T) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i S(f(i)))
        /// claude-4-sonet: Work Θ(n + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        /// claude-4-sonet: Work Θ(|a| + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<W: StTInMtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W + Send + Sync) -> ArraySeqMtPerS<W>;
        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        /// claude-4-sonet: Work Θ(|a| + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B + Send + Sync) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|)
        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(1 + |a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates)))
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(lg(degree(updates)))
        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T>;
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;
        /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|)
        fn collect(
            a: &ArraySeqMtPerS<Pair<T, T>>,
            cmp: impl Fn(&T, &T) -> O + Send + Sync,
        ) -> ArraySeqMtPerS<Pair<T, ArraySeqMtPerS<T>>>;
        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(1 + Σ S(f(y,z)))
        fn iterate<A: StTInMtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A + Send + Sync, x: A) -> A;
        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(lg|a| · max S(f(y,z)))
        fn reduce(a: &ArraySeqMtPerS<T>, f: &(impl Fn(&T, &T) -> T + Send + Sync), id: T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan(a: &ArraySeqMtPerS<T>, f: &(impl Fn(&T, &T) -> T + Send + Sync), id: T) -> (ArraySeqMtPerS<T>, T);
        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T>;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        fn iteratePrefixes<A: StTInMtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArraySeqMtPerS<A>, A);
    }

    impl<T: StTInMtT> ArraySeqMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn new(length: N, init_value: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::new(length, init_value) }

        fn length(&self) -> N { ArraySeqMtPerS::length(self) }

        fn nth(&self, index: N) -> &T { ArraySeqMtPerS::nth(self, index) }

        fn empty() -> ArraySeqMtPerS<T> { ArraySeqMtPerS::empty() }

        fn singleton(item: T) -> ArraySeqMtPerS<T> { ArraySeqMtPerS::singleton(item) }

        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqMtPerS<T> {
            let values: Vec<T> = (0..n).map(|i| f(i)).collect();
            ArraySeqMtPerS::from_vec(values)
        }

        fn map<W: StTInMtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&T) -> W + Send + Sync) -> ArraySeqMtPerS<W> {
            let mut values: Vec<W> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn subseq_copy(&self, start: N, length: N) -> ArraySeqMtPerS<T> {
            ArraySeqMtPerS::subseq_copy(self, start, length)
        }

        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone_mt());
            }
            ArraySeqMtPerS::from_vec(values)
        }

        fn filter(a: &ArraySeqMtPerS<T>, pred: impl Fn(&T) -> B + Send + Sync) -> ArraySeqMtPerS<T> {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) == B::True {
                    kept.push(value.clone());
                }
            }
            ArraySeqMtPerS::from_vec(kept)
        }

        fn flatten(ss: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> ArraySeqMtPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            ArraySeqMtPerS::from_vec(values)
        }


        fn inject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T> {
            let mut result = a.clone();
            let mut updated: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() && updated.insert(*index) {
                    result = result.set(*index, value.clone()).unwrap_or(result);
                }
            }
            result
        }

        fn isEmpty(&self) -> B { ArraySeqMtPerS::is_empty(self) }

        fn isSingleton(&self) -> B { ArraySeqMtPerS::is_singleton(self) }

        fn collect(
            a: &ArraySeqMtPerS<Pair<T, T>>,
            cmp: impl Fn(&T, &T) -> O + Send + Sync,
        ) -> ArraySeqMtPerS<Pair<T, ArraySeqMtPerS<T>>> {
            if a.length() == 0 {
                return ArraySeqMtPerS::from_vec(vec![]);
            }
            let mut groups: Vec<Pair<T, ArraySeqMtPerS<T>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(key, value) = a.nth(i);
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(&key, &group.0) == O::Equal {
                        let mut values: Vec<T> = (0..group.1.length()).map(|j| group.1.nth(j).clone()).collect();
                        values.push(value.clone());
                        group.1 = ArraySeqMtPerS::from_vec(values);
                        found_group = true;
                        break;
                    }
                }
                if !found_group {
                    groups.push(Pair(key.clone(), ArraySeqMtPerS::from_vec(vec![value.clone()])));
                }
            }
            ArraySeqMtPerS::from_vec(groups)
        }

        fn iterate<A: StTInMtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A + Send + Sync, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn reduce(a: &ArraySeqMtPerS<T>, f: &(impl Fn(&T, &T) -> T + Send + Sync), id: T) -> T {
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth(0).clone();
            }
            if a.length() < 1000 {
                // Sequential for small arrays
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                let l = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&left, f, id.clone());
                let r = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&right, f, id);
                f(&l, &r)
            } else {
                // Parallel for large arrays
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                
                let id_clone = id.clone();
                let left_handle = thread::spawn(move || {
                    <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&left, f, id_clone)
                });
                let r = <ArraySeqMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(&right, f, id);
                let l = left_handle.join().unwrap();
                f(&l, &r)
            }
        }

        fn scan(a: &ArraySeqMtPerS<T>, f: &(impl Fn(&T, &T) -> T + Send + Sync), id: T) -> (ArraySeqMtPerS<T>, T) {
            let mut acc = id.clone();
            let mut values: Vec<T> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                values.push(acc.clone());
            }
            (ArraySeqMtPerS::from_vec(values), acc)
        }


        fn ninject(a: &ArraySeqMtPerS<T>, updates: &ArraySeqMtPerS<Pair<N, T>>) -> ArraySeqMtPerS<T> {
            let mut result = a.clone();
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() {
                    result = result.set(*index, value.clone()).unwrap_or(result);
                }
            }
            result
        }

        fn iteratePrefixes<A: StTInMtT>(a: &ArraySeqMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArraySeqMtPerS<A>, A) {
            let mut acc = x;
            let mut values: Vec<A> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                values.push(acc.clone());
            }
            let final_acc = acc.clone();
            (ArraySeqMtPerS::from_vec(values), final_acc)
        }
    }

    #[macro_export]
    macro_rules! ArraySeqMtPerSLit {
        () => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::empty() };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::new($n, $x) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqMtPerSLit_type_checks() {
        let _ = ArraySeqMtPerSLit![1];
        let _: crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS<usize> = ArraySeqMtPerSLit![];
    }
}
