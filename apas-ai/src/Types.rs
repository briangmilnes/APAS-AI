//! Common types used across the crate.
//!
//! Abstract:
//! - Defines shared aliases and enums used throughout the APAS library.
//! - `N` (usize) for naturals, `B` (Boolean), and re-exported `O` (Ordering).

pub mod Types {
    //! Common types used across the crate.
    //!
    //! Abstract:
    //! - Defines shared aliases and enums used throughout the APAS library.
    //! - `N` (usize) for naturals, `B` (Boolean), and re-exported `O` (Ordering).
    
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

    /// Edge wrapper to enable Display/Debug for pairs (V,V) under baseline bounds.
    #[derive(Clone, PartialEq, Eq, Hash, Debug)]
    pub struct Edge<V: Clone + Sized + std::fmt::Display + std::fmt::Debug>(pub V, pub V);

    impl<V: Clone + Sized + std::fmt::Display + std::fmt::Debug> std::fmt::Display for Edge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    impl<V: Clone + Sized + std::fmt::Display + std::fmt::Debug> From<(V, V)> for Edge<V> {
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }
    }

    impl<V: Clone + Sized + std::fmt::Display + std::fmt::Debug> From<Edge<V>> for (V, V) {
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }
    }
}
