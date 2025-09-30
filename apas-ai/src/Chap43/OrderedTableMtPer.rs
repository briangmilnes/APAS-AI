//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent ordered table implementation.
//!
//! Work/Span Analysis:
//! - Sequential operations (Rc backing prevents true parallelism)
//! - All operations delegate to OrderedTableStPer

pub mod OrderedTableMtPer {
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::Types::Types::*;
    use std::sync::Arc;

    pub struct OrderedTableMtPer<K: StTInMtT + Ord, V: StTInMtT> {
        inner: Arc<OrderedTableStPer<K, V>>,
    }

    pub trait OrderedTableMtPerTrait<K: StTInMtT + Ord, V: StTInMtT> {
        fn size(&self) -> N;
        fn empty() -> Self;
        fn singleton(k: K, v: V) -> Self;
        fn find(&self, k: &K) -> Option<V>;
        fn insert(&self, k: K, v: V) -> Self;
        fn delete(&self, k: &K) -> Self;
        fn domain(&self) -> ArraySetStEph<K>;
        fn map<F: Fn(&V) -> V>(&self, f: F) -> Self;
        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self;
    }

    impl<K: StTInMtT + Ord, V: StTInMtT> OrderedTableMtPerTrait<K, V> for OrderedTableMtPer<K, V> {
        fn size(&self) -> N {
            self.inner.size()
        }

        fn empty() -> Self {
            OrderedTableMtPer {
                inner: Arc::new(OrderedTableStPer::empty()),
            }
        }

        fn singleton(k: K, v: V) -> Self {
            OrderedTableMtPer {
                inner: Arc::new(OrderedTableStPer::singleton(k, v)),
            }
        }

        fn find(&self, k: &K) -> Option<V> {
            self.inner.find(k)
        }

        fn insert(&self, k: K, v: V) -> Self {
            OrderedTableMtPer {
                inner: Arc::new((*self.inner).insert(k, v)),
            }
        }

        fn delete(&self, k: &K) -> Self {
            OrderedTableMtPer {
                inner: Arc::new((*self.inner).delete(k)),
            }
        }

        fn domain(&self) -> ArraySetStEph<K> {
            self.inner.domain()
        }

        fn map<F: Fn(&V) -> V>(&self, f: F) -> Self {
            OrderedTableMtPer {
                inner: Arc::new((*self.inner).map(f)),
            }
        }

        fn filter<F: Fn(&K, &V) -> B>(&self, f: F) -> Self {
            OrderedTableMtPer {
                inner: Arc::new((*self.inner).filter(f)),
            }
        }
    }

    impl<K: StTInMtT + Ord, V: StTInMtT> Default for OrderedTableMtPer<K, V> {
        fn default() -> Self {
            Self::empty()
        }
    }

    impl<K: StTInMtT + Ord, V: StTInMtT> Clone for OrderedTableMtPer<K, V> {
        fn clone(&self) -> Self {
            OrderedTableMtPer {
                inner: Arc::clone(&self.inner),
            }
        }
    }
}
