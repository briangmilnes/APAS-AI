//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for `ArraySeqMtEph<T>` (ephemeral, MT).

pub mod ArraySeqMtEph {

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphRedefinableTrait;
    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS as S;
    use crate::Types::Types::*;

    pub type ArraySeqMtEphS<T> = S<T>;

    // Re-export BaseTrait so Chap19::* brings in base methods
    pub use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphBaseTrait;

    pub trait ArraySeqMtEphTrait<T: StTInMtT>: ArraySeqMtEphBaseTrait<T> {
        // Re-declare methods from RedefinableTrait so they're available when importing Chap19::*
        // These delegate to Chap18 implementations unless redefined below
        fn empty()                                                              -> Self;
        fn singleton(item: T)                                                   -> Self;
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T>;
        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static;
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>)                 -> Self;
        fn filter<F: PredMt<T>>(a: &ArraySeqMtEphS<T>, pred: &F)                -> Self;
        fn update(a: &ArraySeqMtEphS<T>, item_at: (N, T))                       -> Self;
        fn isEmpty(&self)                                                       -> B;
        fn isSingleton(&self)                                                   -> B;
        fn iterate<A: StT, F: Fn(&A, &T) -> A + Send + Sync>(a: &ArraySeqMtEphS<T>, f: &F, x: A) -> A;

        // Chap19-specific methods
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>, index: N)       -> Option<T>;
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append_select(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>)          -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate<F: PredMt<T>>(f: &F, x: &T)                                  -> Self;

        // Chap19 redefines these with parallel implementations
        /// APAS Algorithm 19.9: Work Θ(|a|), Span Θ(log|a|)
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: F, id: T) -> T
        where
            T: Send + 'static;
        /// APAS Algorithm 19.10: Work Θ(|a|), Span Θ(log²|a|)
        fn scan<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T)
        where
            T: Send + 'static;
        /// APAS Algorithm 19.15: Work Θ(Σ|ss[i]|), Span Θ(log|ss|)
        fn flatten(ss: &ArraySeqMtEphS<ArraySeqMtEphS<T>>)                      -> Self;
        /// APAS Algorithm 19.16: Work Θ(|a| + |updates|), Span Θ(log|updates|)
        fn inject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>)  -> Self;
        /// APAS Algorithm 19.17: Work Θ(|a| + |updates|), Span Θ(log|updates|)
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> Self;
    }

    impl<T: StTInMtT + 'static> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        // Algorithm 19.1: empty = tabulate(lambda i.i, 0)
        fn empty() -> Self {
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(&|_| unreachable!("empty sequence"), 0)
        }

        // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
        fn singleton(item: T) -> Self {
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(&|_| item.clone(), 1)
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> ArraySeqMtEphS<T> {
            // Algorithm 19.14: "allocate a fresh array of n elements, evaluate f at each position i 
            // and write the result into position i of the array"
            // "the function f can be evaluated at each element independently in parallel"
            // Span: O(max W_f) - all f evaluations happen simultaneously
            // Work: O(Σ W_f(i)) - sum of work to evaluate f at each position

            if n == 0 {
                return <ArraySeqMtEphS<T> as ArraySeqMtEphRedefinableTrait<T>>::empty();
            }

            // Allocate sequence with f(0) as initial value for all positions
            // Element 0 already correct, we'll overwrite the rest
            let seq = ArraySeqMtEphS::new(n, f(0));

            // Fork a thread per remaining element (skip i=0, already correct)
            std::thread::scope(|s| {
                for i in 1..n {
                    let seq_ref = &seq; // Borrow seq for this iteration
                    s.spawn(move || {
                        // Evaluate f at position i (OUTSIDE lock - this is expensive)
                        let value = f(i);
                        
                        // Write result to position i (INSIDE lock - quick)
                        // set takes &self (shared ref), Mutex provides thread-safety
                        seq_ref.set(i, value).unwrap();
                    });
                }
            }); // All threads join here - all writes complete

            seq
        }

        fn map<U: StTInMtT + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> ArraySeqMtEphS<U>
        where
            T: Send + 'static,
        {
            // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
            <ArraySeqMtEphS<U> as ArraySeqMtEphTrait<U>>::tabulate(&|i| f(&a.nth_cloned(i)), a.length())
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> Self {
            // Algorithm 19.4: append a b = flatten ⟨a, b⟩
            let sequences = <ArraySeqMtEphS<ArraySeqMtEphS<T>> as ArraySeqMtEphTrait<ArraySeqMtEphS<T>>>::tabulate(
                &|i| if i == 0 { a.clone() } else { b.clone() },
                2
            );
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&sequences)
        }

        fn filter<F: PredMt<T>>(a: &ArraySeqMtEphS<T>, pred: &F) -> Self {
            // Algorithm 19.5: filter f a = flatten (map (deflate f) a)
            // Map each element to a sequence (empty or singleton), then flatten
            // Using tabulate instead of map to avoid lifetime issues with pred
            let deflated = <ArraySeqMtEphS<ArraySeqMtEphS<T>> as ArraySeqMtEphTrait<ArraySeqMtEphS<T>>>::tabulate(
                &|i| {
                    let x = a.nth_cloned(i);
                    <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::deflate(pred, &x)
                },
                a.length()
            );
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::flatten(&deflated)
        }

        fn update(a: &ArraySeqMtEphS<T>, (i, x): (N, T)) -> Self {
            // Algorithm 19.6: update a (i, x) = tabulate (lambda j. if i = j then x else a[j]) |a|
            let n = a.length();
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|j| if j == i { x.clone() } else { a.nth_cloned(j) },
                n
            )
        }

        // Algorithm 19.7: isEmpty a = |a| = 0
        fn isEmpty(&self) -> B { self.length() == 0 }

        // Algorithm 19.7: isSingleton a = |a| = 1
        fn isSingleton(&self) -> B { self.length() == 1 }

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

        fn scan<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (Self, T)
        where
            T: Send + 'static,
        {
            // Algorithm 19.10: parallel scan using contraction
            use crate::ParaPair;

            let n = a.length();
            if n == 0 {
                return (<ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::empty(), id);
            }
            if n == 1 {
                let item = a.nth_cloned(0);
                let result = f(&id, &item);
                let seq = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::singleton(result.clone());
                return (seq, result);
            }

            // Contract: combine pairs
            let contracted = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|i| {
                    if 2 * i + 1 < n {
                        f(&a.nth_cloned(2 * i), &a.nth_cloned(2 * i + 1))
                    } else {
                        a.nth_cloned(2 * i)
                    }
                },
                (n + 1) / 2
            );

            // Recursive scan on contracted
            let (scanned_contracted, total) = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::scan(&contracted, f, id.clone());

            // Expand: reconstruct full scan
            let expanded = <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|i| {
                    if i % 2 == 0 {
                        scanned_contracted.nth_cloned(i / 2)
                    } else {
                        f(&scanned_contracted.nth_cloned(i / 2), &a.nth_cloned(i))
                    }
                },
                n
            );

            (expanded, total)
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
            use std::sync::Arc;
            use std::sync::Mutex;

            // Helper: APAS Algorithm 19.16 atomicWrite - leftmost wins
            fn atomic_write_leftmost<T: StTInMtT>(
                aa: &Arc<Box<[Mutex<(T, N)>]>>,
                idx: N,
                val: T,
                k: N,
            ) {
                // atomicWrite aa b k =
                //   atomically do:
                //     (j, v) ← b[k]      // j=idx, v=val from caller
                //     (w, i) ← aa[j]     // read current (value, index) at position idx
                //     if k < i then      // leftmost wins: update only if this update is earlier
                //       aa[j] ← (v, k)   // write new value and update index
                if idx < aa.len() {
                    let mut slot = aa[idx].lock().unwrap();
                    if k < slot.1 {
                        *slot = (val, k);
                    }
                }
            }

            if updates.length() == 0 {
                return a.clone();
            }

            // Algorithm 19.16: Create aa from a where aa[i] = (a[i], |a|)
            let n = a.length();
            
            // Create copied array aa with per-element Mutexes for atomic writes (benign effect)
            // Initialize aa[i] = (a[i], |a|) by reading from sequence a
            let mut aa_uninit = Box::new_uninit_slice(n);
            for i in 0..n {
                aa_uninit[i].write(Mutex::new((a.nth_cloned(i), n)));
            }
            let aa = Arc::new(unsafe { aa_uninit.assume_init() });

            // Inject all updates in parallel using atomicWrite - leftmost wins
            std::thread::scope(|s| {
                for k in 0..updates.length() {
                    let Pair(idx, val) = updates.nth_cloned(k);
                    let aa_ref = Arc::clone(&aa);
                    s.spawn(move || atomic_write_leftmost(&aa_ref, idx, val, k));
                }
            });

            // Extract result: create array with just the value component using tabulate
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|i| aa[i].lock().unwrap().0.clone(),
                n
            )
        }

        fn ninject(a: &ArraySeqMtEphS<T>, updates: &ArraySeqMtEphS<Pair<N, T>>) -> ArraySeqMtEphS<T> {
            // Algorithm 19.17: parallel ninject with rightmost-wins atomic writes
            use std::sync::Arc;
            use std::sync::Mutex;

            // Helper: APAS Algorithm 19.17 atomicWrite - rightmost wins
            fn atomic_write_rightmost<T: StTInMtT>(
                aa: &Arc<Box<[Mutex<(T, N)>]>>,
                idx: N,
                val: T,
                k: N,
            ) {
                // atomicWrite aa b k =
                //   atomically do:
                //     (j, v) ← b[k]      // j=idx, v=val from caller
                //     (w, i) ← aa[j]     // read current (value, index) at position idx
                //     if k >= i then     // rightmost wins: update if this update is later or equal
                //       aa[j] ← (v, k)   // write new value and update index
                if idx < aa.len() {
                    let mut slot = aa[idx].lock().unwrap();
                    if k >= slot.1 {
                        *slot = (val, k);
                    }
                }
            }

            if updates.length() == 0 {
                return a.clone();
            }

            // Algorithm 19.17: Create aa from a where aa[i] = (a[i], 0)
            let n = a.length();
            
            // Create copied array aa with per-element Mutexes for atomic writes (benign effect)
            // Initialize aa[i] = (a[i], 0) by reading from sequence a
            let mut aa_uninit = Box::new_uninit_slice(n);
            for i in 0..n {
                aa_uninit[i].write(Mutex::new((a.nth_cloned(i), 0)));
            }
            let aa = Arc::new(unsafe { aa_uninit.assume_init() });

            // Inject all updates in parallel using atomicWrite - rightmost wins
            std::thread::scope(|s| {
                for k in 0..updates.length() {
                    let Pair(idx, val) = updates.nth_cloned(k);
                    let aa_ref = Arc::clone(&aa);
                    s.spawn(move || atomic_write_rightmost(&aa_ref, idx, val, k));
                }
            });

            // Extract result: create array with just the value component using tabulate
            <ArraySeqMtEphS<T> as ArraySeqMtEphTrait<T>>::tabulate(
                &|i| aa[i].lock().unwrap().0.clone(),
                n
            )
        }
    }
}
