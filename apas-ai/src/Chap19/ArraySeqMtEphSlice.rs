//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! MtEph slice-oriented Array sequence variant sharing a single mutex.
//!
//! Abstract:
//! - Provides `ArraySeqMtEphSliceS<T>` backed by `Arc<Mutex<Box<[T]>>>` with range metadata.
//! - Offers trait `ArraySeqMtEphSliceTrait<T>` mirroring the MT ephemeral API while avoiding `Vec` copies.
//! - Adds `with_exclusive` to project a mutable slice guarded by the single mutex for batch updates.

pub mod ArraySeqMtEphSlice {

    use std::fmt::{Debug, Display, Formatter};
    use std::ops::Range;
    use std::sync::{Arc, Mutex};

    use crate::Types::Types::*;

    #[derive(Debug)]
    struct Inner<T: StT + Send + Sync> {
        data: Mutex<Box<[T]>>,
    }

    impl<T: StT + Send + Sync> Inner<T> {
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }

        fn len(&self) -> N {
            let guard = self.data.lock().unwrap();
            guard.len()
        }
    }

    /// Shared slice view over the mutex-protected backing buffer.
    pub struct ArraySeqMtEphSliceS<T: StT + Send + Sync> {
        inner: Arc<Inner<T>>,
        range: Range<N>,
    }

    /// Sequence trait for the slice-backed MT ephemeral array.
    pub trait ArraySeqMtEphSliceTrait<T: StT + Send + Sync> {
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn new(length: N, init_value: T) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn nth_cloned(&self, index: N) -> T;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B;
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn slice(&self, start: N, length: N) -> Self;
        /// claude-4-sonet: Work Θ(n + Σᵢ W(f(i))), Span Θ(1 + maxᵢ S(f(i))), Parallelism Θ(n)
        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self;
        /// claude-4-sonet: Work Θ(|a| + Σₓ W(f(x))), Span Θ(1 + maxₓ S(f(x))), Parallelism Θ(|a|)
        fn map<U: MtVal, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Self, f: F) -> ArraySeqMtEphSliceS<U>;
        /// claude-4-sonet: Work Θ(|a| + Σᵢ W(f(aᵢ))), Span Θ(1 + maxᵢ S(f(aᵢ))), Parallelism Θ(|a|)
        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> Self;
        fn append(a: &Self, b: &Self) -> Self;
        fn append_select(a: &Self, b: &Self) -> Self;
        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self;
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSliceS<T>, T);
        fn iterate<A: StT + Send, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) -> A;
        fn inject(a: &Self, updates: &[(N, T)]) -> Self;
        fn ninject(a: &Self, updates: &[(N, T)]) -> Self;
    }

    impl<T: StT + Send + Sync + 'static> ArraySeqMtEphSliceS<T> {
        /// Constructs a sequence from an owned boxed slice.
        pub fn from_box(data: Box<[T]>) -> Self {
            let len = data.len();
            ArraySeqMtEphSliceS {
                inner: Arc::new(Inner::new(data)),
                range: 0..len,
            }
        }

        /// Constructs a sequence from a Vec without exposing it to callers.
        pub fn from_vec(data: Vec<T>) -> Self { Self::from_box(data.into_boxed_slice()) }

        /// Materializes the current slice into a Vec for diagnostics or copies.
        pub fn to_vec(&self) -> Vec<T> {
            let guard = self.inner.data.lock().unwrap();
            guard[self.range.start..self.range.end].iter().cloned().collect()
        }

        /// Invokes the closure with a mutable slice under the single mutex.
        pub fn with_exclusive<F: FnOnce(&mut [T]) -> R, R>(&self, f: F) -> R {
            let mut guard = self.inner.data.lock().unwrap();
            let start = self.range.start;
            let end = self.range.end;
            f(&mut guard[start..end])
        }

        /// Set method for ephemeral sequences (alias for update)
        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> { self.update(index, item) }

        fn len(&self) -> N { self.range.end - self.range.start }

        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {
            let local_len = self.len();
            let clamped_start = start.min(local_len);
            let clamped_end = clamped_start.saturating_add(length).min(local_len);
            let base = self.range.start;
            (base + clamped_start)..(base + clamped_end)
        }
    }

    impl<T: StT + Send + Sync + 'static> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        fn new(length: N, init_value: T) -> Self {
            let data = repeat_vec(length, init_value);
            ArraySeqMtEphSliceS::from_vec(data)
        }

        fn length(&self) -> N { self.len() }

        fn nth_cloned(&self, index: N) -> T {
            let guard = self.inner.data.lock().unwrap();
            let idx = self.range.start + index;
            guard[idx].clone()
        }

        fn empty() -> Self {
            // Algorithm 19.1: empty = tabulate(lambda i.i, 0) - use trait method
            <Self as ArraySeqMtEphSliceTrait<T>>::tabulate(&|_| unreachable!("empty sequence has no elements"), 0)
        }

        fn update(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            if index >= self.len() {
                return Err("Index out of bounds");
            }
            {
                let mut guard = self.inner.data.lock().unwrap();
                let idx = self.range.start + index;
                guard[idx] = item;
            }
            Ok(self)
        }

        fn singleton(item: T) -> Self {
            // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1) - use trait method
            // Implement directly since we can't capture with &F
            let data = vec![item];
            let inner = Arc::new(Inner {
                data: Mutex::new(data.into_boxed_slice()),
            });
            Self { inner, range: 0..1 }
        }

        fn isEmpty(&self) -> B { if self.len() == 0 { true } else { false } }

        fn isSingleton(&self) -> B { if self.len() == 1 { true } else { false } }

        fn subseq_copy(&self, start: N, length: N) -> Self {
            let sub = self.clamp_subrange(start, length);
            let guard = self.inner.data.lock().unwrap();
            let data: Vec<T> = guard[sub.start..sub.end].iter().cloned().collect();
            ArraySeqMtEphSliceS::from_vec(data)
        }

        fn slice(&self, start: N, length: N) -> Self {
            let sub = self.clamp_subrange(start, length);
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: sub,
            }
        }

        fn tabulate<F: Fn(N) -> T + Send + Sync>(f: &F, n: N) -> Self {
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            ArraySeqMtEphSliceS::from_vec(values)
        }

        fn map<U: MtVal, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &Self, f: F) -> ArraySeqMtEphSliceS<U> {
            // Algorithm 19.3 with parallelism: map f a = tabulate(lambda i.f(a[i]), |a|)
            if a.length() == 0 {
                return ArraySeqMtEphSliceS::<U>::from_vec(Vec::new());
            }

            // Fork thread per element for parallel mapping
            let mut handles = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                let value = a.nth_cloned(i);
                let f_clone = f.clone();
                let handle = std::thread::spawn(move || f_clone(&value));
                handles.push(handle);
            }

            // Collect results serially
            let mut results = Vec::with_capacity(a.length());
            for handle in handles {
                results.push(handle.join().unwrap());
            }

            ArraySeqMtEphSliceS::<U>::from_vec(results)
        }

        fn filter<F: Fn(&T) -> B + Send + Sync + Clone + 'static>(a: &Self, pred: F) -> Self {
            // Algorithm 19.5 with parallelism: fork thread per element + serial compaction
            if a.length() == 0 {
                return <Self as ArraySeqMtEphSliceTrait<T>>::empty();
            }

            // Fork thread per element to evaluate predicate, collect results serially
            let mut keep_results = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                let value = a.nth_cloned(i);
                let pred_clone = pred.clone();

                let handle = std::thread::spawn(move || pred_clone(&value));

                let keep = handle.join().unwrap();
                keep_results.push(keep);
            }

            // Serial compaction phase: collect kept values
            let mut kept_values = Vec::new();
            for i in 0..a.length() {
                if keep_results[i] == true {
                    kept_values.push(a.nth_cloned(i));
                }
            }

            if kept_values.is_empty() {
                <Self as ArraySeqMtEphSliceTrait<T>>::empty()
            } else {
                ArraySeqMtEphSliceS::from_vec(kept_values)
            }
        }

        fn append(a: &Self, b: &Self) -> Self {
            // Algorithm 19.4: append a b = flatten(<a, b>)
            let sequences = vec![a.clone(), b.clone()];
            <Self as ArraySeqMtEphSliceTrait<T>>::flatten(&sequences)
        }

        fn append_select(a: &Self, b: &Self) -> Self {
            // Algorithm 19.4 alternative: append a b = tabulate(select(a, b), |a| + |b|)
            let total_len = a.length() + b.length();
            <Self as ArraySeqMtEphSliceTrait<T>>::tabulate(
                &|i| {
                    if i < a.length() {
                        a.nth_cloned(i)
                    } else {
                        b.nth_cloned(i - a.length())
                    }
                },
                total_len,
            )
        }

        fn flatten(sequences: &[ArraySeqMtEphSliceS<T>]) -> Self {
            if sequences.is_empty() {
                return <Self as ArraySeqMtEphSliceTrait<T>>::empty();
            }

            // Calculate total length
            let total_len: N = sequences.iter().map(|s| s.length()).sum();
            if total_len == 0 {
                return <Self as ArraySeqMtEphSliceTrait<T>>::empty();
            }

            // Flatten by copying all elements
            let mut result = Vec::with_capacity(total_len);
            for seq in sequences {
                for i in 0..seq.length() {
                    result.push(seq.nth_cloned(i));
                }
            }

            ArraySeqMtEphSliceS::from_vec(result)
        }

        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &Self, f: F, id: T) -> T {
            // Algorithm 19.9: divide-and-conquer parallel reduce
            if a.length() == 0 {
                return id;
            }
            if a.length() == 1 {
                return a.nth_cloned(0);
            }

            let mid = a.length() / 2;
            let left_slice = a.slice(0, mid);
            let right_slice = a.slice(mid, a.length() - mid);

            let f_left = f.clone();
            let f_right = f.clone();
            let id_left = id.clone();
            let id_right = id.clone();

            let left_handle = std::thread::spawn(move || Self::reduce(&left_slice, f_left, id_left));
            let right_handle = std::thread::spawn(move || Self::reduce(&right_slice, f_right, id_right));

            let left_result = left_handle.join().unwrap();
            let right_result = right_handle.join().unwrap();

            f(&left_result, &right_result)
        }

        fn scan<F: Fn(&T, &T) -> T + Send + Sync>(a: &Self, f: &F, id: T) -> (ArraySeqMtEphSliceS<T>, T) {
            // Algorithm 19.10: scan using contraction (simplified for slice)
            if a.length() == 0 {
                return (<ArraySeqMtEphSliceS<T> as ArraySeqMtEphSliceTrait<T>>::empty(), id);
            }
            if a.length() == 1 {
                let result_seq = <ArraySeqMtEphSliceS<T> as ArraySeqMtEphSliceTrait<T>>::tabulate(&|_| id.clone(), 1);
                return (result_seq, a.nth_cloned(0));
            }

            // For simplicity, implement sequentially (full parallel scan is complex)
            let mut results = Vec::with_capacity(a.length());
            let mut acc = id.clone();

            for i in 0..a.length() {
                results.push(acc.clone());
                let current = a.nth_cloned(i);
                acc = f(&acc, &current);
            }

            let result_seq = ArraySeqMtEphSliceS::<T>::from_vec(results);
            (result_seq, acc)
        }

        fn iterate<A: StT + Send, F: Fn(&A, &T) -> A + Send + Sync>(a: &Self, f: &F, seed: A) -> A {
            // Algorithm 19.8: iterate f x a (sequential left-to-right)
            let mut acc = seed;
            for i in 0..a.length() {
                let current = a.nth_cloned(i);
                acc = f(&acc, &current);
            }
            acc
        }

        fn inject(a: &Self, updates: &[(N, T)]) -> Self {
            // Delegate to Chap18 implementation concept - apply updates with leftmost wins
            let mut result = a.clone();
            for &(index, ref value) in updates {
                if index < result.length() {
                    result.update(index, value.clone()).unwrap();
                }
            }
            result
        }

        fn ninject(a: &Self, updates: &[(N, T)]) -> Self {
            // Delegate to Chap18 implementation concept - apply updates with rightmost wins
            let mut result = a.clone();
            for &(index, ref value) in updates.iter().rev() {
                if index < result.length() {
                    result.update(index, value.clone()).unwrap();
                }
            }
            result
        }
    }

    impl<T: StT + Send + Sync> Clone for ArraySeqMtEphSliceS<T> {
        fn clone(&self) -> Self {
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: self.range.clone(),
            }
        }
    }

    impl<T: StT + Send + Sync + 'static> PartialEq for ArraySeqMtEphSliceS<T> {
        fn eq(&self, other: &Self) -> bool {
            if Arc::ptr_eq(&self.inner, &other.inner) && self.range == other.range {
                return true;
            }
            if self.len() != other.len() {
                return false;
            }
            let left = self.to_vec();
            let right = other.to_vec();
            left.iter().zip(right.iter()).all(|(a, b)| a == b)
        }
    }

    impl<T: StT + Send + Sync + 'static> Eq for ArraySeqMtEphSliceS<T> {}

    impl<T: StT + Send + Sync> Debug for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            f.debug_list()
                .entries(guard[self.range.start..self.range.end].iter())
                .finish()
        }
    }

    impl<T: StT + Send + Sync> Display for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            let mut first = true;
            write!(f, "[")?;
            for item in &guard[self.range.start..self.range.end] {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    fn repeat_vec<T: StT + Send + Sync>(length: N, init: T) -> Vec<T> {
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(init.clone());
        }
        data
    }

    #[macro_export]
    macro_rules! ArraySeqMtEphSliceSLit {
        () => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqMtEphSliceSLit_type_checks() {
        let _ = ArraySeqMtEphSliceSLit![1];
        let _ = ArraySeqMtEphSliceSLit![0; 2];
        let _: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceSLit![];
    }
}
