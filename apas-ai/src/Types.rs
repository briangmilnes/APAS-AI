//! Common types used across the crate.
//!

pub mod Types {

    use std::fmt::{Debug, Display};

    /// Type alias for natural numbers to match APAS, N.
    pub type N = usize;

    /// Data Type 18.1 (Boolean) type used by APAS.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub enum B {
        True,
        False,
    }

    /// Data Type 18.1 (Ordering) relationships used by APAS, using Rust's as it matches.
    /// Enumerated values in `std::cmp::Ordering` are named: Less, Equal, Greater.
    pub use std::cmp::Ordering as O;

    impl std::fmt::Display for B {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                B::True => write!(f, "True"),
                B::False => write!(f, "False"),
            }
        }
    }

    // Type bounds shorthands
    // StT: single-threaded friendly elements: Eq + Clone + Display + Debug + Sized
    pub trait StT: Eq + Clone + Display + Debug + Sized {}
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}

    // StTinMtT: St-friendly elements that can be shared across threads (StT + Send + Sync)
    pub trait StTinMtT: StT + Send + Sync {}
    impl<T> StTinMtT for T where T: StT + Send + Sync {}

    // MtT: multi-threaded friendly elements; minimal so it can include Mutex<..>
    // Keep only thread-safety and size requirements.
    pub trait MtT: Sized + Send + Sync {
        type Inner: StT;
        fn clone_mt(&self) -> Self;
        fn new_mt(inner: Self::Inner) -> Self;
    }

    impl<T: StT + Send> MtT for std::sync::Mutex<T> {
        type Inner = T;
        fn clone_mt(&self) -> Self {
            let inner = self.lock().unwrap().clone();
            std::sync::Mutex::new(inner)
        }
        fn new_mt(inner: Self::Inner) -> Self {
            std::sync::Mutex::new(inner)
        }
    }

    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {
        type Inner = Pair<A, B>;
        fn clone_mt(&self) -> Self {
            self.clone()
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    // Ad-hoc implementations for specific primitive types to avoid conflicts
    impl MtT for usize {
        type Inner = usize;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for isize {
        type Inner = isize;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for i32 {
        type Inner = i32;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for u32 {
        type Inner = u32;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for i64 {
        type Inner = i64;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for u64 {
        type Inner = u64;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for bool {
        type Inner = bool;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    impl MtT for char {
        type Inner = char;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    // Special case: ad-hoc implementation for String
    impl MtT for String {
        type Inner = String;
        fn clone_mt(&self) -> Self {
            self.clone()
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    // String slice implementation
    impl<'a> MtT for &'a str {
        type Inner = &'a str;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    // Custom boolean enum implementation
    impl MtT for B {
        type Inner = B;
        fn clone_mt(&self) -> Self {
            *self
        }
        fn new_mt(inner: Self::Inner) -> Self {
            inner
        }
    }

    /// Edge wrapper to enable Display/Debug for pairs (V,V) under baseline bounds.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Edge<V: StT>(pub V, pub V);

    impl<V: StT> std::fmt::Display for Edge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl<V: StT> From<(V, V)> for Edge<V> {
        fn from(t: (V, V)) -> Self {
            Edge(t.0, t.1)
        }
    }

    impl<V: StT> From<Edge<V>> for (V, V) {
        fn from(e: Edge<V>) -> (V, V) {
            (e.0, e.1)
        }
    }

    /// Pair type with proper Display/Debug available when elements support them.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Pair<A, B>(pub A, pub B);

    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl<A, B> From<(A, B)> for Pair<A, B> {
        fn from(t: (A, B)) -> Self {
            Pair(t.0, t.1)
        }
    }

    impl<A, B> From<Pair<A, B>> for (A, B) {
        fn from(p: Pair<A, B>) -> (A, B) {
            (p.0, p.1)
        }
    }

    /// Set equivalence comparison for sequences (order-independent, useful for MT tests)
    /// APAS: Work Θ(n²), Span Θ(1) - simple membership test both ways
    /// claude-4-sonet: Work Θ(n²), Span Θ(1) - simple membership test both ways
    pub fn ArraySeqSetEq<T: PartialEq>(a_len: N, a_nth: impl Fn(N) -> T, b_len: N, b_nth: impl Fn(N) -> T) -> bool {
        if a_len != b_len {
            return false;
        }

        // For each element in sequence A, check if it exists in sequence B
        for i in 0..a_len {
            let a_elem = a_nth(i);
            let mut found = false;
            for j in 0..b_len {
                if a_elem == b_nth(j) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        // For each element in sequence B, check if it exists in sequence A
        for j in 0..b_len {
            let b_elem = b_nth(j);
            let mut found = false;
            for i in 0..a_len {
                if b_elem == a_nth(i) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }

        true
    }
}
