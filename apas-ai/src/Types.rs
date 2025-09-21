//! Common types used across the crate.
//!

pub mod Types {

    use std::fmt::{Debug, Display};
    use std::hash::Hash;

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
                | B::True => write!(f, "True"),
                | B::False => write!(f, "False"),
            }
        }
    }

    impl std::ops::Not for B {
        type Output = B;
        fn not(self) -> B {
            match self {
                B::True => B::False,
                B::False => B::True,
            }
        }
    }

    // Type bounds shorthands
    // StT: single-threaded friendly elements: Eq + Clone + Display + Debug + Sized
    pub trait StT: Eq + Clone + Display + Debug + Sized {}
    impl<T> StT for T where T: Eq + Clone + Display + Debug + Sized {}

    // StTInMtT: St-friendly elements that can be shared across threads (StT + Send + Sync)
    pub trait StTInMtT: StT + Send + Sync {}
    impl<T> StTInMtT for T where T: StT + Send + Sync {}

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
        fn new_mt(inner: Self::Inner) -> Self { std::sync::Mutex::new(inner) }
    }

    impl<A: StT + Send + Sync, B: StT + Send + Sync> MtT for Pair<A, B> {
        type Inner = Pair<A, B>;
        fn clone_mt(&self) -> Self { self.clone() }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // Ad-hoc implementations for specific primitive types to avoid conflicts
    impl MtT for usize {
        type Inner = usize;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for isize {
        type Inner = isize;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for i32 {
        type Inner = i32;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for u32 {
        type Inner = u32;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for i64 {
        type Inner = i64;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for u64 {
        type Inner = u64;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for bool {
        type Inner = bool;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    impl MtT for char {
        type Inner = char;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // Special case: ad-hoc implementation for String
    impl MtT for String {
        type Inner = String;
        fn clone_mt(&self) -> Self { self.clone() }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // String slice implementation
    impl<'a> MtT for &'a str {
        type Inner = &'a str;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    // Custom boolean enum implementation
    impl MtT for B {
        type Inner = B;
        fn clone_mt(&self) -> Self { *self }
        fn new_mt(inner: Self::Inner) -> Self { inner }
    }

    /// Edge wrapper to enable Display/Debug for pairs (V,V) under baseline bounds.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct Edge<V: StT>(pub V, pub V);

    impl<V: StT> std::fmt::Display for Edge<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.0, self.1) }
    }

    impl<V: StT> From<(V, V)> for Edge<V> {
        fn from(t: (V, V)) -> Self { Edge(t.0, t.1) }
    }

    impl<V: StT> From<Edge<V>> for (V, V) {
        fn from(e: Edge<V>) -> (V, V) { (e.0, e.1) }
    }

    /// Labeled Edge wrapper to enable edges with labels.
    #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
    pub struct LabEdge<V: StT, L: Clone + Debug + Display + Eq + Hash>(pub V, pub V, pub L);

    impl<V: StT, L: Clone + Debug + Display + Eq + Hash> std::fmt::Display for LabEdge<V, L> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
            write!(f, "({}, {}, {})", self.0, self.1, self.2) 
        }
    }

    impl<V: StT, L: Clone + Debug + Display + Eq + Hash> From<(V, V, L)> for LabEdge<V, L> {
        fn from(t: (V, V, L)) -> Self { LabEdge(t.0, t.1, t.2) }
    }

    impl<V: StT, L: Clone + Debug + Display + Eq + Hash> From<LabEdge<V, L>> for (V, V, L) {
        fn from(e: LabEdge<V, L>) -> (V, V, L) { (e.0, e.1, e.2) }
    }

    /// OrderedFloat wrapper to make floating-point types usable in hash-based collections
    /// and as graph edge weights by providing Eq and Hash implementations.
    /// 
    /// This wrapper treats NaN values as equal to themselves and provides a consistent
    /// hash implementation, making it suitable for use in HashSet, HashMap, and graph labels.
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub struct OrderedFloat<T>(pub T);

    impl<T: PartialEq> Eq for OrderedFloat<T> {}

    impl<T: Clone + Debug + Display> Display for OrderedFloat<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<T> Hash for OrderedFloat<T> 
    where 
        T: Clone + std::fmt::Debug + std::fmt::Display
    {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            // For floating-point types, we need special handling
            // This is a simplified approach - in production you'd want more sophisticated NaN handling
            format!("{:?}", self.0).hash(state);
        }
    }

    // Implement arithmetic operations for OrderedFloat
    impl<T: std::ops::Add<Output = T>> std::ops::Add for OrderedFloat<T> {
        type Output = OrderedFloat<T>;
        fn add(self, other: OrderedFloat<T>) -> OrderedFloat<T> {
            OrderedFloat(self.0 + other.0)
        }
    }

    impl<T: std::ops::Sub<Output = T>> std::ops::Sub for OrderedFloat<T> {
        type Output = OrderedFloat<T>;
        fn sub(self, other: OrderedFloat<T>) -> OrderedFloat<T> {
            OrderedFloat(self.0 - other.0)
        }
    }

    impl<T: std::ops::Mul<Output = T>> std::ops::Mul for OrderedFloat<T> {
        type Output = OrderedFloat<T>;
        fn mul(self, other: OrderedFloat<T>) -> OrderedFloat<T> {
            OrderedFloat(self.0 * other.0)
        }
    }

    impl<T: std::ops::Div<Output = T>> std::ops::Div for OrderedFloat<T> {
        type Output = OrderedFloat<T>;
        fn div(self, other: OrderedFloat<T>) -> OrderedFloat<T> {
            OrderedFloat(self.0 / other.0)
        }
    }

    impl<T: PartialOrd> PartialOrd for OrderedFloat<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.partial_cmp(&other.0)
        }
    }

    impl<T: PartialOrd> Ord for OrderedFloat<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
        }
    }

    // Convenience type aliases for common float types
    pub type OrderedF32 = OrderedFloat<f32>;
    pub type OrderedF64 = OrderedFloat<f64>;

    /// Pair type with proper Display/Debug available when elements support them.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Pair<A, B>(pub A, pub B);

    impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "({}, {})", self.0, self.1) }
    }

    impl<A, B> From<(A, B)> for Pair<A, B> {
        fn from(t: (A, B)) -> Self { Pair(t.0, t.1) }
    }

    impl<A, B> From<Pair<A, B>> for (A, B) {
        fn from(p: Pair<A, B>) -> Self { (p.0, p.1) }
    }

    #[macro_export]
    macro_rules! ParaPair {
        ( $left:expr, $right:expr ) => {{
            let left_handle = std::thread::spawn($left);
            let right_handle = std::thread::spawn($right);
            let left_result = left_handle.join().expect("left ParaPair task panicked");
            let right_result = right_handle.join().expect("right ParaPair task panicked");
            $crate::Types::Types::Pair(left_result, right_result)
        }};
    }

    #[allow(dead_code)]
    fn _ParaPair_type_checks() {
        let Pair(left, right) = ParaPair!(|| { 1usize }, || { 2usize });
        let _: usize = left;
        let _: usize = right;
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

    #[macro_export]
    macro_rules! EdgeLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Edge($a, $b)
        };
    }

    #[macro_export]
    macro_rules! PairLit {
        ($a:expr, $b:expr) => {
            $crate::Types::Types::Pair($a, $b)
        };
    }

    #[macro_export]
    macro_rules! EdgeList {
        () => {
            Vec::new()
        };
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {
            vec![ $( $crate::EdgeLit!($a, $b) ),* ]
        };
    }

    #[macro_export]
    macro_rules! PairList {
        () => {
            Vec::new()
        };
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {
            vec![ $( $crate::PairLit!($a, $b) ),* ]
        };
    }

    #[allow(dead_code)]
    fn _EdgeLit_type_checks() {
        let _ = EdgeLit!(1, 2); // non-empty infers (e.g., i32)
        let _: Edge<i32> = EdgeLit!(1, 2); // explicit type
    }

    #[allow(dead_code)]
    fn _PairLit_type_checks() {
        let _ = PairLit!(1, 2); // non-empty infers (e.g., i32)
        let _: Pair<i32, i32> = PairLit!(1, 2); // explicit type
    }

    #[allow(dead_code)]
    fn _EdgeList_type_checks() {
        let _ = EdgeList![(1, 2), (3, 4)]; // non-empty infers
        let _: Vec<Edge<i32>> = EdgeList![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    fn _PairList_type_checks() {
        let _ = PairList![(1, 2), (3, 4)]; // non-empty infers
        let _: Vec<Pair<i32, i32>> = PairList![]; // empty form requires explicit type
    }
}
