//! Chapter 18 algorithms for ArraySeqMtPer multithreaded, just tabulate needed.

pub mod ArraySeqMtPer{
    use crate::Types::Types::*;
    use crate::Types::Types::*;

    pub trait ArraySeqMtPerTrait<T: MtT> {
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
        /// APAS: Work Θ(1 + |a| + |updates|), Span Θ(lg(degree(updates)))
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(lg(degree(updates)))
        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T>;
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
    }

    impl<T: MtT> ArraySeqMtPerTrait<T> for ArrayMtPerS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArrayMtPerS<T> {
            let data: Vec<T> = (0..n).map(|i| f(i)).collect();
            ArrayMtPerS::from_vec(data)
        }

        fn map<W: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&T) -> W) -> ArrayMtPerS<W> {
            <ArrayMtPerS<W> as ArraySeqMtPerTrait<W>>::tabulate(|i| f(a.nth(i)), a.length())
        }

        fn append(a: &ArrayMtPerS<T>, b: &ArrayMtPerS<T>) -> ArrayMtPerS<T> {
            Self::tabulate(
                |i| {
                    if i < a.length() {
                        a.nth(i).clone_mt()
                    } else {
                        b.nth(i - a.length()).clone_mt()
                    }
                },
                a.length() + b.length(),
            )
        }

        fn filter(a: &ArrayMtPerS<T>, pred: impl Fn(&T) -> B) -> ArrayMtPerS<T> {
            let mut results: Vec<T> = Vec::new();
            for i in 0..a.length() {
                if pred(a.nth(i)) == B::True {
                    results.push(a.nth(i).clone_mt());
                }
            }
            ArrayMtPerS::from_vec(results)
        }

        fn update(a: &ArrayMtPerS<T>, Pair(index, item): Pair<N, T>) -> ArrayMtPerS<T> {
            a.set(index, item).unwrap_or_else(|_| a.clone())
        }

        fn inject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            let mut result = a.clone();
            let mut updated = vec![false; a.length()]; // track which indices have been updated
            for i in 0..updates.length() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() && !updated[*index] {
                    result = result.set(*index, value.clone_mt()).unwrap_or(result);
                    updated[*index] = true; // mark as updated so future updates to this index are ignored
                }
            }
            result
        }

        fn ninject(a: &ArrayMtPerS<T>, updates: &ArrayMtPerS<Pair<N, T>>) -> ArrayMtPerS<T> {
            let mut result = a.clone();
            for i in (0..updates.length()).rev() {
                let Pair(index, value) = updates.nth(i);
                if *index < result.length() {
                    result = result.set(*index, value.clone_mt()).unwrap_or(result);
                }
            }
            result
        }

        fn iterate<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn iteratePrefixes<A: MtT>(a: &ArrayMtPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> (ArrayMtPerS<A>, A) {
            let mut acc = x;
            let mut results: Vec<A> = Vec::new();
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                results.push(acc.clone_mt());
            }
            let final_acc = acc.clone_mt();
            (ArrayMtPerS::from_vec(results), final_acc)
        }

        fn reduce(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            if a.length() == 0usize {
                return id;
            }
            let mut result = a.nth(0).clone_mt();
            for i in 1..a.length() {
                result = f(&result, a.nth(i));
            }
            result
        }

        fn scan(a: &ArrayMtPerS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArrayMtPerS<T>, T) {
            let mut acc = id.clone_mt();
            let mut results: Vec<T> = Vec::new();
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
                results.push(acc.clone_mt());
            }
            (ArrayMtPerS::from_vec(results), acc)
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
            if a.length() == 0usize {
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
    }
}
