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

    struct Inner<T: StT> {
        data: Mutex<Box<[T]>>,
    }

    impl<T: StT> Inner<T> {
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }

        fn len(&self) -> N {
            let guard = self.data.lock().unwrap();
            guard.len()
        }
    }

    /// Shared slice view over the mutex-protected backing buffer.
    pub struct ArraySeqMtEphSliceS<T: StT> {
        inner: Arc<Inner<T>>,
        range: Range<N>,
    }

    /// Sequence trait for the slice-backed MT ephemeral array.
    pub trait ArraySeqMtEphSliceTrait<T: StT> {
        fn new(length: N, init_value: T) -> Self;
        fn length(&self) -> N;
        fn nth_cloned(&self, index: N) -> T;
        fn empty() -> Self;
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
        fn singleton(item: T) -> Self;
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;
        fn subseq_copy(&self, start: N, length: N) -> Self;
        fn slice(&self, start: N, length: N) -> Self;
    }

    impl<T: StT> ArraySeqMtEphSliceS<T> {
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
        pub fn with_exclusive<F, R>(&self, f: F) -> R
        where
            F: FnOnce(&mut [T]) -> R,
        {
            let mut guard = self.inner.data.lock().unwrap();
            let start = self.range.start;
            let end = self.range.end;
            f(&mut guard[start..end])
        }

        fn len(&self) -> N { self.range.end - self.range.start }

        fn clamp_subrange(&self, start: N, length: N) -> Range<N> {
            let local_len = self.len();
            let clamped_start = start.min(local_len);
            let clamped_end = clamped_start.saturating_add(length).min(local_len);
            let base = self.range.start;
            (base + clamped_start)..(base + clamped_end)
        }
    }

    impl<T: StT> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
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

        fn empty() -> Self { ArraySeqMtEphSliceS::from_vec(Vec::new()) }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
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

        fn singleton(item: T) -> Self { ArraySeqMtEphSliceS::from_vec(vec![item]) }

        fn isEmpty(&self) -> B {
            if self.len() == 0 {
                B::True
            } else {
                B::False
            }
        }

        fn isSingleton(&self) -> B {
            if self.len() == 1 {
                B::True
            } else {
                B::False
            }
        }

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
    }

    impl<T: StT> Clone for ArraySeqMtEphSliceS<T> {
        fn clone(&self) -> Self {
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: self.range.clone(),
            }
        }
    }

    impl<T: StT> PartialEq for ArraySeqMtEphSliceS<T> {
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

    impl<T: StT> Eq for ArraySeqMtEphSliceS<T> {}

    impl<T: StT> Debug for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            f.debug_list()
                .entries(guard[self.range.start..self.range.end].iter())
                .finish()
        }
    }

    impl<T: StT> Display for ArraySeqMtEphSliceS<T> {
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

    fn repeat_vec<T: StT>(length: N, init: T) -> Vec<T> {
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(init.clone());
        }
        data
    }

    #[macro_export]
    macro_rules! ArraySeqMtEphSliceSLit {
        () => { $crate::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _ArraySeqMtEphSliceSLit_type_checks() {
        let _ = ArraySeqMtEphSliceSLit![1];
        let _ = ArraySeqMtEphSliceSLit![0; 2];
        let _: ArraySeqMtEphSliceS<i32> = ArraySeqMtEphSliceSLit![];
    }
}
