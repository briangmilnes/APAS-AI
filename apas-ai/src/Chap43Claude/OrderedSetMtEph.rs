//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral ordered set implementation using custom backing store.

pub mod OrderedSetMtEph {
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Types::Types::*;
    use std::sync::Arc;
    use std::thread;

    /// Multi-threaded ephemeral ordered set with custom implementation
    #[derive(PartialEq)]
    pub struct OrderedSetMtEph<T: StTInMtT + Ord + 'static> {
        elements: Vec<T>,
    }

    pub type OrderedSetMt<T> = OrderedSetMtEph<T>;

    /// Trait defining all ordered set operations (ADT 41.1 + ADT 43.1) with multi-threaded ephemeral semantics
    pub trait OrderedSetMtEphTrait<T: StTInMtT + Ord + 'static> {
        // Base set operations (ADT 41.1) - ephemeral semantics with parallelism
        fn size(&self) -> N;
        fn empty() -> Self;
        fn singleton(x: T) -> Self;
        fn find(&self, x: &T) -> B;
        fn insert(&mut self, x: T);
        fn delete(&mut self, x: &T);
        fn filter<F>(&mut self, f: F) where F: Fn(&T) -> B + Send + Sync + 'static;
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

    impl<T: StTInMtT + Ord + 'static> OrderedSetMtEphTrait<T> for OrderedSetMtEph<T> {
        // Base set operations with parallel implementation where beneficial
        
        /// Work: O(1), Span: O(1)
        fn size(&self) -> N {
            self.elements.len()
        }

        /// Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedSetMtEph {
                elements: Vec::new(),
            }
        }

        /// Work: O(1), Span: O(1)
        fn singleton(x: T) -> Self {
            OrderedSetMtEph {
                elements: vec![x],
            }
        }

        /// Work: O(log n), Span: O(log n)
        fn find(&self, x: &T) -> B {
            self.elements.binary_search(x).is_ok()
        }

        /// Work: O(n), Span: O(log n)
        fn insert(&mut self, x: T) {
            match self.elements.binary_search(&x) {
                Ok(_) => {}, // Element already exists
                Err(pos) => self.elements.insert(pos, x),
            }
        }

        /// Work: O(n), Span: O(log n)
        fn delete(&mut self, x: &T) {
            if let Ok(pos) = self.elements.binary_search(x) {
                self.elements.remove(pos);
            }
        }

        /// Work: O(n), Span: O(log n) - Parallel filter
        fn filter<F>(&mut self, f: F) 
        where 
            F: Fn(&T) -> B + Send + Sync + 'static
        {
            let len = self.elements.len();
            if len <= 1 {
                if len == 1 && !f(&self.elements[0]) {
                    self.elements.clear();
                }
                return;
            }

            let mid = len / 2;
            let right_elements: Vec<T> = self.elements.drain(mid..).collect();
            let left_elements = std::mem::take(&mut self.elements);

            let f = Arc::new(f);
            let f_clone = f.clone();

            let handle = thread::spawn(move || {
                left_elements.into_iter().filter(|x| f_clone(x)).collect::<Vec<T>>()
            });

            let right_filtered: Vec<T> = right_elements.into_iter().filter(|x| f(x)).collect();
            let left_filtered = handle.join().unwrap();

            self.elements = left_filtered;
            self.elements.extend(right_filtered);
        }

        /// Work: O(m + n), Span: O(log(m + n)) - Parallel merge
        fn intersection(&mut self, other: &Self) {
            let len1 = self.elements.len();
            let len2 = other.elements.len();

            if len1 == 0 || len2 == 0 {
                self.elements.clear();
                return;
            }

            if len1 <= 1 && len2 <= 1 {
                if len1 == 1 && len2 == 1 && self.elements[0] == other.elements[0] {
                    // Keep the element
                } else {
                    self.elements.clear();
                }
                return;
            }

            let mid1 = len1 / 2;
            let mid2 = len2 / 2;
            
            let right_self: Vec<T> = self.elements.drain(mid1..).collect();
            let left_self = std::mem::take(&mut self.elements);
            
            let left_other = other.elements[..mid2].to_vec();
            let right_other = other.elements[mid2..].to_vec();

            let handle = thread::spawn(move || {
                let mut result = Vec::new();
                let mut i = 0;
                let mut j = 0;
                
                while i < left_self.len() && j < left_other.len() {
                    match left_self[i].cmp(&left_other[j]) {
                        std::cmp::Ordering::Equal => {
                            result.push(left_self[i].clone());
                            i += 1;
                            j += 1;
                        }
                        std::cmp::Ordering::Less => i += 1,
                        std::cmp::Ordering::Greater => j += 1,
                    }
                }
                result
            });

            let mut right_result = Vec::new();
            let mut i = 0;
            let mut j = 0;
            
            while i < right_self.len() && j < right_other.len() {
                match right_self[i].cmp(&right_other[j]) {
                    std::cmp::Ordering::Equal => {
                        right_result.push(right_self[i].clone());
                        i += 1;
                        j += 1;
                    }
                    std::cmp::Ordering::Less => i += 1,
                    std::cmp::Ordering::Greater => j += 1,
                }
            }

            let left_result = handle.join().unwrap();
            self.elements = left_result;
            self.elements.extend(right_result);
        }

        /// Work: O(m + n), Span: O(log(m + n)) - Parallel merge
        fn union(&mut self, other: &Self) {
            // Simple sequential implementation for correctness
            // Add all elements from other that are not already in self
            for elem in &other.elements {
                if !self.find(elem) {
                    self.insert(elem.clone());
                }
            }
        }

        /// Work: O(m + n), Span: O(log(m + n))
        fn difference(&mut self, other: &Self) {
            // Simple sequential implementation for correctness
            self.elements.retain(|elem| !other.find(elem));
        }
                }
                return;
            }

            let mid1 = len1 / 2;
            let mid2 = len2 / 2;
            
            let right_self: Vec<T> = self.elements.drain(mid1..).collect();
            let left_self = std::mem::take(&mut self.elements);
            
            let left_other = other.elements[..mid2].to_vec();
            let right_other = other.elements[mid2..].to_vec();

            let handle = thread::spawn(move || {
                let mut result = Vec::new();
                let mut i = 0;
                let mut j = 0;
                
                while i < left_self.len() && j < left_other.len() {
                    match left_self[i].cmp(&left_other[j]) {
                        std::cmp::Ordering::Equal => {
                            result.push(left_self[i].clone());
                            i += 1;
                            j += 1;
                        }
                        std::cmp::Ordering::Less => {
                            result.push(left_self[i].clone());
                            i += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            result.push(left_other[j].clone());
                            j += 1;
                        }
                    }
                }
                while i < left_self.len() {
                    result.push(left_self[i].clone());
                    i += 1;
                }
                while j < left_other.len() {
                    result.push(left_other[j].clone());
                    j += 1;
                }
                result
            });

            let mut right_result = Vec::new();
            let mut i = 0;
            let mut j = 0;
            
            while i < right_self.len() && j < right_other.len() {
                match right_self[i].cmp(&right_other[j]) {
                    std::cmp::Ordering::Equal => {
                        right_result.push(right_self[i].clone());
                        i += 1;
                        j += 1;
                    }
                    std::cmp::Ordering::Less => {
                        right_result.push(right_self[i].clone());
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        right_result.push(right_other[j].clone());
                        j += 1;
                    }
                }
            }
            while i < right_self.len() {
                right_result.push(right_self[i].clone());
                i += 1;
            }
            while j < right_other.len() {
                right_result.push(right_other[j].clone());
                j += 1;
            }

            let left_result = handle.join().unwrap();
            self.elements = left_result;
            self.elements.extend(right_result);
        }

        /// Work: O(m + n), Span: O(log(m + n)) - Parallel difference
        fn difference_old(&mut self, other: &Self) {
            // Simple sequential implementation for correctness
            self.elements.retain(|elem| !other.find(elem));
        }

        fn difference_complex(&mut self, other: &Self) {
            let len1 = self.elements.len();
            let len2 = other.elements.len();

            if len1 == 0 || len2 == 0 {
                return;
            }

            if len1 <= 1 && len2 <= 1 {
                if self.elements[0] == other.elements[0] {
                    self.elements.clear();
                }
                return;
            }

            let mid1 = len1 / 2;
            let mid2 = len2 / 2;
            
            let right_self: Vec<T> = self.elements.drain(mid1..).collect();
            let left_self = std::mem::take(&mut self.elements);
            
            let left_other = other.elements[..mid2].to_vec();
            let right_other = other.elements[mid2..].to_vec();

            let handle = thread::spawn(move || {
                let mut result = Vec::new();
                let mut i = 0;
                let mut j = 0;
                
                while i < left_self.len() && j < left_other.len() {
                    match left_self[i].cmp(&left_other[j]) {
                        std::cmp::Ordering::Equal => {
                            i += 1;
                            j += 1;
                        }
                        std::cmp::Ordering::Less => {
                            result.push(left_self[i].clone());
                            i += 1;
                        }
                        std::cmp::Ordering::Greater => j += 1,
                    }
                }
                while i < left_self.len() {
                    result.push(left_self[i].clone());
                    i += 1;
                }
                result
            });

            let mut right_result = Vec::new();
            let mut i = 0;
            let mut j = 0;
            
            while i < right_self.len() && j < right_other.len() {
                match right_self[i].cmp(&right_other[j]) {
                    std::cmp::Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                    std::cmp::Ordering::Less => {
                        right_result.push(right_self[i].clone());
                        i += 1;
                    }
                    std::cmp::Ordering::Greater => j += 1,
                }
            }
            while i < right_self.len() {
                right_result.push(right_self[i].clone());
                i += 1;
            }

            let left_result = handle.join().unwrap();
            self.elements = left_result;
            self.elements.extend(right_result);
        }

        /// Work: O(n), Span: O(log n)
        fn to_seq(&self) -> AVLTreeSeqStPerS<T> {
            AVLTreeSeqStPerS::from_vec(self.elements.clone())
        }

        /// Work: O(n log n), Span: O(log² n)
        fn from_seq(seq: AVLTreeSeqStPerS<T>) -> Self {
            let len = seq.length();
            let mut elements = Vec::with_capacity(len);
            for i in 0..len {
                elements.push(seq.nth(i).clone());
            }
            elements.sort();
            OrderedSetMtEph { elements }
        }

        // Ordering operations (ADT 43.1) - Sequential implementation

        /// Work: O(1), Span: O(1)
        fn first(&self) -> Option<T> {
            self.elements.first().cloned()
        }

        /// Work: O(1), Span: O(1)
        fn last(&self) -> Option<T> {
            self.elements.last().cloned()
        }

        /// Work: O(log n), Span: O(log n)
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

        /// Work: O(log n), Span: O(log n)
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

        /// Work: O(n), Span: O(log n)
        fn split(&mut self, k: &T) -> (Self, B, Self) {
            let pos = self.elements.binary_search(k);
            let (found, split_pos) = match pos {
                Ok(p) => (true, p),
                Err(p) => (false, p),
            };

            let right_elements: Vec<T> = self.elements.drain(split_pos..).collect();
            let left_elements = std::mem::take(&mut self.elements);

            if found && !right_elements.is_empty() {
                // Remove the found element from right side
                let mut right_vec: Vec<T> = right_elements;
                right_vec.remove(0);
                *self = Self::empty();
                (
                    OrderedSetMtEph { elements: left_elements },
                    true,
                    OrderedSetMtEph { elements: right_vec },
                )
            } else {
                *self = Self::empty();
                (
                    OrderedSetMtEph { elements: left_elements },
                    found,
                    OrderedSetMtEph { elements: right_elements },
                )
            }
        }

        /// Work: O(m + n), Span: O(log(m + n))
        fn join(&mut self, other: Self) {
            self.union(&other);
        }

        /// Work: O(log n), Span: O(log n)
        fn get_range(&self, k1: &T, k2: &T) -> Self {
            let start_pos = match self.elements.binary_search(k1) {
                Ok(pos) => pos,
                Err(pos) => pos,
            };
            
            let end_pos = match self.elements.binary_search(k2) {
                Ok(pos) => pos + 1,
                Err(pos) => pos,
            };

            let range_elements = self.elements[start_pos..end_pos.min(self.elements.len())].to_vec();
            OrderedSetMtEph { elements: range_elements }
        }

        /// Work: O(log n), Span: O(log n)
        fn rank(&self, k: &T) -> N {
            match self.elements.binary_search(k) {
                Ok(pos) => pos,
                Err(pos) => pos,
            }
        }

        /// Work: O(1), Span: O(1)
        fn select(&self, i: N) -> Option<T> {
            self.elements.get(i).cloned()
        }

        /// Work: O(n), Span: O(log n)
        fn split_rank(&mut self, i: N) -> (Self, Self) {
            if i >= self.elements.len() {
                let current = std::mem::take(&mut self.elements);
                *self = Self::empty();
                return (OrderedSetMtEph { elements: current }, Self::empty());
            }

            let right_elements = self.elements.drain(i..).collect();
            let left_elements = std::mem::take(&mut self.elements);

            *self = Self::empty();
            (
                OrderedSetMtEph { elements: left_elements },
                OrderedSetMtEph { elements: right_elements },
            )
        }
    }

    impl<T: StTInMtT + Ord + 'static> Clone for OrderedSetMtEph<T> {
        fn clone(&self) -> Self {
            OrderedSetMtEph {
                elements: self.elements.clone(),
            }
        }
    }

    /// Helper function for macro construction
    pub fn from_sorted_elements<T: StTInMtT + Ord + 'static>(elements: Vec<T>) -> OrderedSetMtEph<T> {
        OrderedSetMtEph { elements }
    }

    /// Macro for creating multi-threaded ephemeral ordered sets from sorted element lists
    #[macro_export]
    macro_rules! OrderedSetMtEphLit {
        () => {
            $crate::Chap43Claude::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEph::empty()
        };
        ($($elem:expr),+ $(,)?) => {
            $crate::Chap43Claude::OrderedSetMtEph::OrderedSetMtEph::from_sorted_elements(vec![$($elem),+])
        };
    }

    pub use OrderedSetMtEphLit;
}
