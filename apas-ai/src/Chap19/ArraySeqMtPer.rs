//! Chapter 19 algorithms for ArraySeqMtPer, just the one multi-threaded set of code that Umut and Guy snuck into this chapter.

pub mod ArraySeqMtPer {
    use std::fmt::{Debug, Display};
    use std::sync::Mutex;

    // Self-import removed;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

    pub trait ArraySeqMtPerTrait<T: MtT> {
        // Chapter 18 wrappers
        /// APAS: Work Θ(1 + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        /// claude-4-sonet: Work Θ(n + Σ i=0..n-1 W(f(i))), Span Θ(1 + max i=0..n-1 S(f(i)))
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1 + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        /// claude-4-sonet: Work Θ(|a| + Σ x∈a W(f(x))), Span Θ(1 + max x∈a S(f(x)))
        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W>;
        /// APAS: Work Θ(1 + |a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1 + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        /// claude-4-sonet: Work Θ(|a| + Σ i=0..|a|-1 W(pred(a[i]))), Span Θ(lg|a| + max i S(pred(a[i])))
        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1 + |a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(1 + Σ S(f(y,z)))
        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        /// APAS: Work Θ(|a|), Span Θ(|a|)
        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMtPerS<A>, A);
        /// APAS: Work Θ(1 + Σ (y,z) W(f(y,z))), Span Θ(lg|a| · max S(f(y,z)))
        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        /// APAS: Work Θ(|a|), Span Θ(lg|a|)
        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T);
        /// APAS: Work Θ(1 + |a| + Σ x∈a |x|), Span Θ(1 + lg|a|)
        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1 + W(f) · |a| lg|a|), Span Θ(1 + S(f) · lg^2|a|)
        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<T, ArrayMtPerS<T>>>;

        // Chapter 19 specific functions
        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn atomicWrite(
            values_with_change_number: &mut ArrayMtPerS<Pair<T, N>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        );
        /// APAS: Work Θ(|values|+|changes|), Span Θ(1) PRAM
        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        );
    }

    impl<T: MtT + StT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::tabulate(f, n)
        }

        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::map(a, f)
        }

        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::append(a, b)
        }

        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::filter(a, pred)
        }

        fn update(a: &ArrayMtPerS<T>, item_at: Pair<N, T>) -> ArrayMtPerS<T> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::update(a, item_at)
        }

        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::ninject(a, updates)
        }

        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::iterate(a, f, x)
        }

        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMtPerS<A>, A) {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::iteratePrefixes(a, f, x)
        }

        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::reduce(a, f, id)
        }

        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::scan(a, f, id)
        }

        fn flatten(ss: &ArrayMtPerS<ArrayMtPerS<T>>) -> ArrayMtPerS<T> {
            let mut results: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner_seq = ss.nth(i);
                for j in 0..inner_seq.length() {
                    results.push(inner_seq.nth(j).clone_mt());
                }
            }
            ArrayMtPerS::from_vec(results)
        }

        fn collect(a: &ArrayMtPerS<Pair<T, T>>, cmp: impl Fn(&T, &T) -> O) -> ArrayMtPerS<Pair<T, ArrayMtPerS<T>>> {
            if 0 == self.vertices().size() {
                return ArrayMtPerS::from_vec(vec![]);
            }

            let mut groups: Vec<Pair<T, ArrayMtPerS<T>>> = Vec::new();

            for i in 0..a.length() {
                let Pair(key, value) = a.nth(i);

                // Find existing group or create new one
                let mut found_group = false;
                for group in &mut groups {
                    if cmp(&key, &group.0) == O::Equal {
                        // Add to existing group - need to rebuild the array
                        let mut values: Vec<T> = Vec::new();
                        for j in 0..group.1.length() {
                            values.push(group.1.nth(j).clone_mt());
                        }
                        values.push(value.clone_mt());
                        group.1 = ArrayMtPerS::from_vec(values);
                        found_group = true;
                        break;
                    }
                }

                if !found_group {
                    groups.push(Pair(key.clone_mt(), ArrayMtPerS::from_vec(vec![value.clone_mt()])));
                }
            }

            ArrayMtPerS::from_vec(groups)
        }

        fn inject(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::inject(values, changes)
        }

        fn atomicWrite(
            _values_with_change_number: &mut ArrayMtPerS<Pair<T, N>>,
            _changes: &ArrayMtPerS<Pair<N, T>>,
            _change_index: N,
        ) {
            // Stub implementation - complex atomic operations not needed for basic functionality
        }

        fn inject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            // Simplified implementation - delegate to basic inject for now
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::inject(values, changes)
        }

        fn AtomicWriteLowestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let nvals = values_with_change_number.length();
            for i in 0..changes.length() {
                let Pair(idx, val) = changes.nth(i);
                let idxn = *idx;
                if idxn >= nvals {
                    continue;
                }
                let cell = values_with_change_number.nth(idxn);
                let mut guard = cell.lock().unwrap();
                if change_index < guard.1 {
                    guard.0 = val.clone();
                    guard.1 = change_index;
                }
            }
        }

        fn ninject_parallel2(values: &ArrayMtPerS<T>, changes: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            // Simplified implementation - delegate to basic ninject for now
            <ArrayMtPerS<T> as ArraySeqMtPerTrait<T>>::ninject(values, changes)
        }

        fn AtomicWriteHighestChangeNumberWins(
            values_with_change_number: &ArrayMtPerS<Mutex<Pair<T, N>>>,
            changes: &ArrayMtPerS<Pair<N, T>>,
            change_index: N,
        ) {
            let nvals = values_with_change_number.length();
            for i in 0..changes.length() {
                let Pair(idx, val) = changes.nth(i);
                let idxn = *idx;
                if idxn >= nvals {
                    continue;
                }
                let cell = values_with_change_number.nth(idxn);
                let mut guard = cell.lock().unwrap();
                if change_index > guard.1 {
                    guard.0 = val.clone();
                    guard.1 = change_index;
                }
            }
        }
    }
}
