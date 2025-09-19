//! MtEph (ephemeral, mutable) Array sequence variant backed by a Mutex.
//!
//! Abstract:
//! - Defines `ArraySeqMtEphS<T>` backed by `Mutex<Box<[T]>>` for thread-safe, in-place updates.
//! - Provides trait `ArraySeqMtEphTrait<T>` mirroring the ST ephemeral API with cloned reads.
//! - All reads return owned values to avoid leaking references across the mutex guard lifetime.

pub mod ArraySeqMtEph {
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};
    use std::slice::Iter;
    use std::sync::Mutex;

    use crate::Types::Types::*;

    /// Fixed-length sequence backed by `Mutex<Box<[T]>>` (ephemeral/mutable MT variant).
    pub struct ArraySeqMtEphS<T: StT> {
        data: Mutex<Box<[T]>>,
    }

    /// Sequence trait for `ArraySeqMtEphS<T>`.
    pub trait ArraySeqMtEphTrait<T: StT> {
        /// APAS: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// Returns a cloned element to avoid returning references tied to a lock guard.
        /// APAS: Work Θ(1), Span Θ(1)
        fn nth_cloned(&self, index: N) -> T;
        /// APAS: Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
        /// APAS: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B;
        /// APAS: Work Θ(length), Span Θ(1)
        fn subseq_copy(&self, start: N, length: N) -> Self;
    }

    impl<T: StT> ArraySeqMtEphS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        pub fn from_vec(v: Vec<T>) -> Self {
            ArraySeqMtEphS {
                data: Mutex::new(v.into_boxed_slice()),
            }
        }

        /// APAS: Work Θ(1), Span Θ(1)
        pub fn length(&self) -> N {
            let guard = self.data.lock().unwrap();
            guard.len()
        }

        /// APAS: Work Θ(1), Span Θ(1)
        pub fn nth_cloned(&self, index: N) -> T {
            let guard = self.data.lock().unwrap();
            guard[index].clone()
        }

        /// Convenience iterator snapshot (clones out to a Vec and iterates it).
        /// APAS: Work Θ(|a|), Span Θ(1)
        pub fn iter_snapshot(&self) -> Iter<'_, T>
        where
            T: 'static,
        {
            // Materialize a snapshot Vec and leak it for a short-lived iter; used only in Debug/Display.
            // This is kept private to avoid lifetime leaks in public API.
            let v = self.to_vec();
            let b = v.into_boxed_slice();
            let static_ref: &'static [T] = Box::leak(b);
            static_ref.iter()
        }

        /// APAS: Work Θ(|a|), Span Θ(1)
        pub fn to_vec(&self) -> Vec<T> {
            let guard = self.data.lock().unwrap();
            guard.iter().cloned().collect()
        }

        /// APAS: Work Θ(length), Span Θ(1)
        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            let guard = self.data.lock().unwrap();
            let n = guard.len();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            let v: Vec<T> = guard[s..e].iter().cloned().collect();
            ArraySeqMtEphS::from_vec(v)
        }
    }

    impl<T: StT> Clone for ArraySeqMtEphS<T> {
        fn clone(&self) -> Self { ArraySeqMtEphS::from_vec(self.to_vec()) }
    }

    impl<T: StT> PartialEq for ArraySeqMtEphS<T> {
        fn eq(&self, other: &Self) -> bool {
            let a = self.to_vec();
            let b = other.to_vec();
            a == b
        }
    }
    impl<T: StT> Eq for ArraySeqMtEphS<T> {}

    impl<T: StT> Debug for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let v = self.to_vec();
            f.debug_list().entries(v.iter()).finish()
        }
    }

    impl<T: StT> Display for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let v = self.to_vec();
            write!(f, "[")?;
            for (i, x) in v.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", x)?;
            }
            write!(f, "]")
        }
    }

    impl<T: StT> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn new(length: N, init_value: T) -> Self { ArraySeqMtEphS::from_vec(vec![init_value; length]) }
        fn length(&self) -> N { self.length() }
        fn nth_cloned(&self, index: N) -> T { self.nth_cloned(index) }
        fn empty() -> Self { ArraySeqMtEphS::from_vec(Vec::new()) }
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            {
                let mut guard = self.data.lock().unwrap();
                if index < guard.len() {
                    guard[index] = item;
                } else {
                    return Err("Index out of bounds");
                }
            }
            Ok(self)
        }
        fn singleton(item: T) -> Self { ArraySeqMtEphS::from_vec(vec![item]) }
        fn isEmpty(&self) -> B {
            if self.length() == 0 {
                B::True
            } else {
                B::False
            }
        }
        fn isSingleton(&self) -> B {
            if self.length() == 1 {
                B::True
            } else {
                B::False
            }
        }
        fn subseq_copy(&self, start: N, length: N) -> Self { self.subseq_copy(start, length) }
    }

    #[macro_export]
    macro_rules! ArraySeqMtEph {
        () => { $crate::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$($x),*]) };
    }
}
