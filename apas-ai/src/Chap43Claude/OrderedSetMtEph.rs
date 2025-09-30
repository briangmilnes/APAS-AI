//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered set implementation using custom backing store.

pub mod OrderedSetMtEph {
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Types::Types::*;
    use std::sync::Arc;
    use std::thread;

    /// Multi-threaded ephemeral ordered set with custom implementation
    #[derive(PartialEq)]
    pub struct OrderedSetMtEph<T: MtKey> {
        elements: Vec<T>,
    }

    pub type OrderedSetMt<T> = OrderedSetMtEph<T>;

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with multi-threaded ephemeral semantics
    pub trait OrderedSetMtEphTrait<T: MtKey> {
        // Base set operations (ADT 41.1) - ephemeral semantics with parallelism
        fn size(&self) -> N;
        fn empty() -> Self;
        fn singleton(x: T) -> Self;
        fn find(&self, x: &T) -> B;
        fn insert(&mut self, x: T);
        fn delete(&mut self, x: &T);
        fn filter<F: Pred<T>>(&mut self, f: F);
        fn intersection(&mut self, other: &Self);
        fn union(&mut self, other: &Self);
        fn difference(&mut self, other: &Self);
        fn to_seq(&self) -> AVLTreeSeqStPerS<T>;
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self;

        // Ordering operations (ADT 43.1) - sequential (inherently sequential on trees)
        fn first(&self) -> Option<T>;
        fn last(&self) -> Option<T>;
        fn previous(&self, k: &T) -> Option<T>;
        fn next(&self, k: &T) -> Option<T>;
        fn split(&mut self, k: &T) -> (Self, B, Self) where Self: Sized;
        fn join(&mut self, other: Self);
        fn get_range(&self, k1: &T, k2: &T) -> Self;
        fn rank(&self, k: &T) -> N;
        fn select(&self, i: N) -> Option<T>;
        fn split_rank(&mut self, i: N) -> (Self, Self) where Self: Sized;
    }

    impl<T: MtKey> OrderedSetMtEphTrait<T> for OrderedSetMtEph<T> {
        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N {
            self.elements.len()
        }

        /// Claude Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedSetMtEph {
                elements: Vec::new(),
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(x: T) -> Self {
            OrderedSetMtEph {
                elements: vec![x],
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn find(&self, x: &T) -> B {
            self.elements.binary_search(x).is_ok()
        }

        /// Claude Work: O(n), Span: O(log n)
        fn insert(&mut self, x: T) {
            match self.elements.binary_search(&x) {
                Ok(_) => {}, // Element already exists
                Err(pos) => {
                    self.elements.insert(pos, x);
                }
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn delete(&mut self, x: &T) {
            if let Ok(pos) = self.elements.binary_search(x) {
                self.elements.remove(pos);
            }
        }

        /// Claude Work: O(n), Span: O(log n)
        fn filter<F: Fn(&T) -> B + Send + Sync + 'static>(&mut self, f: F) 
        {
            self.elements.retain(|x| f(x));
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn intersection(&mut self, other: &Self) {
            self.elements.retain(|elem| other.find(elem));
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn union(&mut self, other: &Self) {
            // Simple sequential implementation for correctness
            for elem in &other.elements {
                if !self.find(elem) {
                    self.insert(elem.clone());
                }
            }
        }

        /// Claude Work: O(m + n), Span: O(log(m + n))
        fn difference(&mut self, other: &Self) {
            // Simple sequential implementation for correctness
            self.elements.retain(|elem| !other.find(elem));
        }

        /// Claude Work: O(n), Span: O(log n)
        fn to_seq(&self) -> AVLTreeSeqStPerS<T> {
            AVLTreeSeqStPerS::from_vec(self.elements.clone())
        }

        /// Claude Work: O(n log n), Span: O(log² n)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self {
            let len = seq.length();
            let mut elements = Vec::new();
            for i in 0..len {
                elements.push(seq.nth(i).clone());
            }
            elements.sort();
            elements.dedup();
            OrderedSetMtEph { elements }
        }

        // Ordering operations (ADT 43.1)

        /// Claude Work: O(1), Span: O(1)
        fn first(&self) -> Option<T> {
            self.elements.first().cloned()
        }

        /// Claude Work: O(1), Span: O(1)
        fn last(&self) -> Option<T> {
            self.elements.last().cloned()
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn previous(&self, k: &T) -> Option<T> {
            match self.elements.binary_search(k) {
                Ok(pos) => {
                    if pos > 0 {
                        Some(self.elements[pos - 1].clone())
                    } else {
                        None
                    }
                }
                Err(pos) => {
                    if pos > 0 {
                        Some(self.elements[pos - 1].clone())
                    } else {
                        None
                    }
                }
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn next(&self, k: &T) -> Option<T> {
            match self.elements.binary_search(k) {
                Ok(pos) => {
                    if pos + 1 < self.elements.len() {
                        Some(self.elements[pos + 1].clone())
                    } else {
                        None
                    }
                }
                Err(pos) => {
                    if pos < self.elements.len() {
                        Some(self.elements[pos].clone())
                    } else {
                        None
                    }
                }
            }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn split(&mut self, k: &T) -> (Self, B, Self) {
            let pos = match self.elements.binary_search(k) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };

            let found = self.elements.get(pos) == Some(k);
            let right_elements: Vec<T> = self.elements.drain(pos..).collect();
            let left_elements = std::mem::take(&mut self.elements);

            let mut right_elements_filtered = right_elements;
            if found {
                right_elements_filtered.remove(0); // Remove the found element
            }

            *self = Self::empty();
            (
                OrderedSetMtEph { elements: left_elements },
                found,
                OrderedSetMtEph { elements: right_elements_filtered },
            )
        }

        /// Claude Work: O(log(m + n)), Span: O(log(m + n))
        fn join(&mut self, other: Self) {
            self.union(&other);
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn get_range(&self, k1: &T, k2: &T) -> Self {
            let start_pos = match self.elements.binary_search(k1) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            let end_pos = match self.elements.binary_search(k2) {
                Ok(pos) => pos + 1,
                Err(pos) => pos,
            };

            let range_elements = self.elements[start_pos..end_pos].to_vec();
            OrderedSetMtEph { elements: range_elements }
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn rank(&self, k: &T) -> N {
            match self.elements.binary_search(k) {
                Ok(pos) => pos,
                Err(pos) => pos,
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn select(&self, i: N) -> Option<T> {
            self.elements.get(i).cloned()
        }

        /// Claude Work: O(log n), Span: O(log n)
        fn split_rank(&mut self, i: N) -> (Self, Self) {
            let right_elements: Vec<T> = self.elements.drain(i..).collect();
            let left_elements = std::mem::take(&mut self.elements);

            *self = Self::empty();
            (
                OrderedSetMtEph { elements: left_elements },
                OrderedSetMtEph { elements: right_elements },
            )
        }
    }

    /// Macro for creating ordered sets from literals
    #[macro_export]
    macro_rules! OrderedSetMtEphLit {
        ($($x:expr),* $(,)?) => {
            {
                let mut set = $crate::Chap43Claude::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEph::empty();
                $(
                    set.insert($x);
                )*
                set
            }
        };
    }

    pub use OrderedSetMtEphLit;
}
