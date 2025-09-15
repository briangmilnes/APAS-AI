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

    // MtT: multi-threaded friendly elements; minimal so it can include Mutex<..>
    // Keep only thread-safety and size requirements.
    pub trait MtT: Clone + Eq + Sized + Send + Sync {}
    impl<T> MtT for T where T: Clone + Eq + Sized + Send + Sync {}

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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    // Note: No explicit MtT impl for Pair; blanket MtT impl covers all Sized+Send+Sync types.
}
