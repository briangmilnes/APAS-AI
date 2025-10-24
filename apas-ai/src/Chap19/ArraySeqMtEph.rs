//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph {

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphRedefinableTrait, ArraySeqMtEphS as S};
    use crate::Types::Types::*;

    pub type ArraySeqMtEphS<T> = S<T>;

    // Re-export BaseTrait so Chap19::* brings in base methods
    pub use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphBaseTrait;

    pub trait ArraySeqMtEphTrait<T: StTInMtT>: ArraySeqMtEphBaseTrait<T> {
        // Re-declare methods from RedefinableTrait so they're available when importing Chap19::*
        // These delegate to Chap18 implementations unless redefined below
        fn empty()                                               -> Self;
        fn singleton(item: T)                                    -> Self;
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>)  -> Self;
        fn filter<F: PredMt<T>>(a: &ArraySeqMtEphS<T>, pred: &F) -> Self;
        fn isEmpty(&self)                                        -> B;
        fn isSingleton(&self)                                    -> B;
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A;
        
        // Chap19-specific methods
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T>;
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>)    -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate<F: PredMt<T>>(f: &F, x: &T)                            -> Self;
        
        // Chap19 redefines these with parallel implementations
        /// APAS Algorithm 19.9: Work Θ(|a|), Span Θ(log|a|)
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
        where
            T: Send + 'static;
        /// APAS Algorithm 19.15: Work Θ(Σ|ss[i]|), Span Θ(log|ss|)
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> Self;
        /// APAS Algorithm 19.16: Work Θ(|a| + |updates|), Span Θ(log|updates|)
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> Self;
        /// APAS Algorithm 19.17: Work Θ(|a| + |updates|), Span Θ(log|updates|)
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> Self;
    }

    impl<T: StTInMtT + 'static> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        // Delegate non-redefined methods to Chap18
        fn empty() -> Self {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::empty()
        }
        
        fn singleton(item: T) -> Self {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::singleton(item)
        }
        
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::tabulate(f, n)
        }
        
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static,
        {
            <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::map(a, f)
        }
        
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> Self {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::append(a, b)
        }
        
        fn filter<F: PredMt<T>>(a: &ArraySeqMtEphS<T>, pred: &F) -> Self {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::filter(a, pred)
        }
        
        fn isEmpty(&self) -> B {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::isEmpty(self)
        }
        
        fn isSingleton(&self) -> B {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::isSingleton(self)
        }
        
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A {
            <Self as ArraySeqMtEphRedefinableTrait<T>>::iterate(a, f, x)
        }
        
        // Chap19-specific implementations
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N) -> Option<T> {
            let len_a = a.length();
            if index < len_a {
                return Some(a.nth_cloned(index));
            }
            let offset = index - len_a;
            let len_b = b.length();
            if offset < len_b {
                Some(b.nth_cloned(offset))
            } else {
                None
            }
        }

        // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> ArraySeqMtEphS<T> {
            <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::tabulate(
                &|i| <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::select(a, b, i).unwrap(),
                a.length() + b.length(),
            )
        }

        fn deflate<F: PredMt<T>>(f: &F, x: &T) -> ArraySeqMtEphS<T> {
            if f(x) {
                <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::singleton(x.clone())
            } else {
                <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::empty()
            }
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
        where
            T: Send + 'static,
        {
            // Algorithm 19.9: parallel reduce using divide-and-conquer
            use crate::ParaPair;
            
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth_cloned(0);
            }

            // Divide-and-conquer with ParaPair!
            let mid = a.length() / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, a.length() - mid);
            let f_left = f.clone();
            let f_right = f.clone();
            let id_clone = id.clone();

            let Pair(left_result, right_result) = ParaPair!(
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&left, f_left, id_clone),
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::reduce(&right, f_right, id)
            );
            
            f(&left_result, &right_result)
        }

        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.15: parallel flatten using divide-and-conquer
            use crate::ParaPair;
            
            if ss.length() == 0 {
                return <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::empty();
            }
            if ss.length() == 1 {
                return ss.nth_cloned(0);
            }

            // Divide-and-conquer with ParaPair!
            let mid = ss.length() / 2;
            let left = ss.subseq_copy(0, mid);
            let right = ss.subseq_copy(mid, ss.length() - mid);

            let Pair(left_result, right_result) = ParaPair!(
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&left),
                move || <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&right)
            );

            // Append the two flattened results
            <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::append(&left_result, &right_result)
        }

        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.16: parallel inject with leftmost-wins atomic writes
            use std::sync::{Arc, Mutex};
            
            if updates.length() == 0 {
                return a.clone();
            }

            // Create shared atomic array: (value, update_index)
            let values: Arc<Vec<Mutex<(T, N)>>> = Arc::new(
                (0..a.length())
                    .map(|i| Mutex::new((a.nth_cloned(i), a.length())))
                    .collect()
            );

            // Apply all updates in parallel - leftmost wins
            let updates_vec: Vec<Pair<N, T>> = (0..updates.length())
                .map(|i| updates.nth_cloned(i))
                .collect();
            
            std::thread::scope(|s| {
                for (k, Pair(idx, val)) in updates_vec.into_iter().enumerate() {
                    let vals = Arc::clone(&values);
                    s.spawn(move || {
                        if idx < vals.len() {
                            let mut slot = vals[idx].lock().unwrap();
                            if k < slot.1 {
                                *slot = (val, k);
                            }
                        }
                    });
                }
            });

            // Extract final values (Arc is automatically unwrapped after scope)
            let result: Vec<T> = values
                .iter()
                .map(|m| m.lock().unwrap().0.clone())
                .collect();
            ArraySeqMtEphS::from_vec(result)
        }

        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.17: parallel ninject with rightmost-wins atomic writes
            use std::sync::{Arc, Mutex};
            
            if updates.length() == 0 {
                return a.clone();
            }

            // Create shared atomic array: (value, update_index)
            let values: Arc<Vec<Mutex<(T, N)>>> = Arc::new(
                (0..a.length())
                    .map(|i| Mutex::new((a.nth_cloned(i), 0)))
                    .collect()
            );

            // Apply all updates in parallel - rightmost wins
            let updates_vec: Vec<Pair<N, T>> = (0..updates.length())
                .map(|i| updates.nth_cloned(i))
                .collect();
            
            std::thread::scope(|s| {
                for (k, Pair(idx, val)) in updates_vec.into_iter().enumerate() {
                    let vals = Arc::clone(&values);
                    s.spawn(move || {
                        if idx < vals.len() {
                            let mut slot = vals[idx].lock().unwrap();
                            if k >= slot.1 {
                                *slot = (val, k);
                            }
                        }
                    });
                }
            });

            // Extract final values (Arc is automatically unwrapped after scope)
            let result: Vec<T> = values
                .iter()
                .map(|m| m.lock().unwrap().0.clone())
                .collect();
            ArraySeqMtEphS::from_vec(result)
        }
    }
}
