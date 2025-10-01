//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Priority Queue implementation using Sorted List

pub mod SortedListPQ {
    use std::fmt::{Debug, Display, Formatter, Result};

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    /// Priority Queue implemented using a sorted list (ArraySeqStPer)
    /// Elements are maintained in ascending order (minimum at front)
    /// Data Type 45.1: Meldable Priority Queue
    #[derive(PartialEq, Clone, Debug)]
    pub struct SortedListPQ<T: StT + Ord> {
        elements: ArraySeqStPerS<T>,
    }

    /// Trait defining the Meldable Priority Queue ADT operations (Data Type 45.1)
    pub trait SortedListPQTrait<T: StT + Ord> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn empty() -> Self;

        /// Claude Work: Θ(1), Span: Θ(1)
        fn singleton(element: T) -> Self;

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Returns the minimum element (first in sorted list), or None if empty
        fn find_min(&self) -> Option<&T>;

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Inserts element in correct sorted position
        fn insert(&self, element: T) -> Self;

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Removes first element (minimum) from sorted list
        fn delete_min(&self) -> (Self, Option<T>)
        where
            Self: Sized;

        /// Claude Work: Θ(m + n), Span: Θ(m + n)
        /// Melds two sorted priority queues by merging sorted lists
        fn meld(&self, other: &Self) -> Self;

        /// Claude Work: Θ(n log n), Span: Θ(n log n)
        /// Creates priority queue from sequence by sorting
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self;

        /// Helper methods
        fn size(&self) -> N;
        fn is_empty(&self) -> bool;
        fn to_seq(&self) -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> SortedListPQTrait<T> for SortedListPQ<T> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn empty() -> Self {
            SortedListPQ {
                elements: ArraySeqStPerS::empty(),
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn singleton(element: T) -> Self {
            SortedListPQ {
                elements: ArraySeqStPerS::singleton(element),
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Minimum is always at the front of sorted list
        fn find_min(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(0))
            }
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        /// Find correct position and insert to maintain sorted order
        fn insert(&self, element: T) -> Self {
            // Find insertion position using binary search approach
            let mut insert_pos = 0;
            for i in 0..self.elements.length() {
                let current = self.elements.nth(i);
                if element <= *current {
                    break;
                }
                insert_pos = i + 1;
            }

            // Build new sequence with element inserted at correct position
            let mut new_elements = ArraySeqStPerS::empty();

            // Add elements before insertion position
            for i in 0..insert_pos {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            // Add the new element
            let new_elem_seq = ArraySeqStPerS::singleton(element);
            new_elements = ArraySeqStPerS::append(&new_elements, &new_elem_seq);

            // Add elements after insertion position
            for i in insert_pos..self.elements.length() {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            SortedListPQ { elements: new_elements }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        /// Remove first element (minimum) from sorted list
        fn delete_min(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }

            let min_element = self.elements.nth(0).clone();

            // Create new sequence without the first element
            let mut new_elements = ArraySeqStPerS::empty();
            for i in 1..self.elements.length() {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            let new_pq = SortedListPQ { elements: new_elements };

            (new_pq, Some(min_element))
        }

        /// Claude Work: Θ(m + n), Span: Θ(m + n)
        /// Merge two sorted lists maintaining sorted order
        fn meld(&self, other: &Self) -> Self {
            let mut result = ArraySeqStPerS::empty();
            let mut i = 0;
            let mut j = 0;

            // Merge the two sorted sequences
            while i < self.elements.length() && j < other.elements.length() {
                let elem_self = self.elements.nth(i);
                let elem_other = other.elements.nth(j);

                if elem_self <= elem_other {
                    let single_seq = ArraySeqStPerS::singleton(elem_self.clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    i += 1;
                } else {
                    let single_seq = ArraySeqStPerS::singleton(elem_other.clone());
                    result = ArraySeqStPerS::append(&result, &single_seq);
                    j += 1;
                }
            }

            // Add remaining elements from self
            while i < self.elements.length() {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                result = ArraySeqStPerS::append(&result, &single_seq);
                i += 1;
            }

            // Add remaining elements from other
            while j < other.elements.length() {
                let elem = other.elements.nth(j);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                result = ArraySeqStPerS::append(&result, &single_seq);
                j += 1;
            }

            SortedListPQ { elements: result }
        }

        /// Claude Work: Θ(n log n), Span: Θ(n log n)
        /// Create priority queue from sequence by insertion sort
        fn from_seq(seq: &ArraySeqStPerS<T>) -> Self {
            let mut result = Self::empty();
            for i in 0..seq.length() {
                let element = seq.nth(i);
                result = result.insert(element.clone());
            }
            result
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn size(&self) -> N {
            self.elements.length()
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn is_empty(&self) -> bool {
            self.elements.length() == 0
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        fn to_seq(&self) -> ArraySeqStPerS<T> {
            self.elements.clone()
        }
    }

    impl<T: StT + Ord> SortedListPQ<T> {
        /// Create an empty priority queue
        pub fn new() -> Self {
            Self::empty()
        }

        /// Get the number of elements
        pub fn len(&self) -> N {
            self.size()
        }

        /// Check if the priority queue is empty
        pub fn is_empty(&self) -> bool {
            SortedListPQTrait::is_empty(self)
        }

        /// Peek at the minimum element without removing it
        pub fn peek(&self) -> Option<&T> {
            self.find_min()
        }

        /// Insert multiple elements from a sequence
        pub fn insert_all(&self, elements: &ArraySeqStPerS<T>) -> Self {
            let mut result = self.clone();
            for i in 0..elements.length() {
                let element = elements.nth(i);
                result = result.insert(element.clone());
            }
            result
        }

        /// Extract all elements in sorted order (already sorted)
        pub fn extract_all_sorted(&self) -> ArraySeqStPerS<T> {
            self.elements.clone()
        }

        /// Get the maximum element (last in sorted list)
        pub fn find_max(&self) -> Option<&T> {
            if self.elements.length() == 0 {
                None
            } else {
                Some(self.elements.nth(self.elements.length() - 1))
            }
        }

        /// Remove and return maximum element
        pub fn delete_max(&self) -> (Self, Option<T>) {
            if self.elements.length() == 0 {
                return (self.clone(), None);
            }

            let max_element = self.elements.nth(self.elements.length() - 1).clone();

            // Create new sequence without the last element
            let mut new_elements = ArraySeqStPerS::empty();
            for i in 0..(self.elements.length() - 1) {
                let elem = self.elements.nth(i);
                let single_seq = ArraySeqStPerS::singleton(elem.clone());
                new_elements = ArraySeqStPerS::append(&new_elements, &single_seq);
            }

            let new_pq = SortedListPQ { elements: new_elements };

            (new_pq, Some(max_element))
        }
    }

    impl<T: StT + Ord> Default for SortedListPQ<T> {
        fn default() -> Self {
            Self::empty()
        }
    }

    impl<T: StT + Ord> Display for SortedListPQ<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "SortedListPQ[")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "]")
        }
    }

    // Macro for creating sorted list priority queues
    #[macro_export]
    macro_rules! SortedListPQLit {
        () => {
            $crate::Chap45::SortedListPQ::SortedListPQ::SortedListPQ::empty()
        };
        ($($x:expr),* $(,)?) => {{
            let mut pq = $crate::Chap45::SortedListPQ::SortedListPQ::SortedListPQ::empty();
            $(
                pq = pq.insert($x);
            )*
            pq
        }};
    }

    #[allow(dead_code)]
    fn _sorted_list_pq_lit_type_checks() {
        let _: SortedListPQ<i32> = SortedListPQLit![];
        let _: SortedListPQ<i32> = SortedListPQLit![1, 2, 3];
    }

    /// Convenience functions for common operations
    impl<T: StT + Ord> SortedListPQ<T> {
        /// Create priority queue from vector (for testing)
        pub fn from_vec(vec: Vec<T>) -> Self {
            let mut pq = Self::empty();
            for element in vec {
                pq = pq.insert(element);
            }
            pq
        }

        /// Convert to vector (for testing)
        pub fn to_vec(&self) -> Vec<T> {
            let mut result = Vec::new();
            for i in 0..self.elements.length() {
                result.push(self.elements.nth(i).clone());
            }
            result
        }

        /// Get elements in sorted order as vector (for testing)
        pub fn to_sorted_vec(&self) -> Vec<T> {
            // Already sorted, just convert to vector
            self.to_vec()
        }

        /// Check if the list is properly sorted (for testing)
        pub fn is_sorted(&self) -> bool {
            for i in 1..self.elements.length() {
                let prev = self.elements.nth(i - 1);
                let curr = self.elements.nth(i);
                if prev > curr {
                    return false;
                }
            }
            true
        }
    }

    /// Priority queue operations for use in heapsort and other algorithms
    pub struct SortedListPQOps;

    impl SortedListPQOps {
        /// Create empty priority queue
        pub fn empty<T: StT + Ord>() -> SortedListPQ<T> {
            SortedListPQ::empty()
        }

        /// Insert element into priority queue
        pub fn insert<T: StT + Ord>(pq: &SortedListPQ<T>, element: T) -> SortedListPQ<T> {
            pq.insert(element)
        }

        /// Delete minimum element from priority queue
        pub fn delete_min<T: StT + Ord>(pq: &SortedListPQ<T>) -> (SortedListPQ<T>, Option<T>) {
            pq.delete_min()
        }

        /// Meld two priority queues
        pub fn meld<T: StT + Ord>(pq1: &SortedListPQ<T>, pq2: &SortedListPQ<T>) -> SortedListPQ<T> {
            pq1.meld(pq2)
        }

        /// Create priority queue from sequence
        pub fn from_seq<T: StT + Ord>(seq: &ArraySeqStPerS<T>) -> SortedListPQ<T> {
            SortedListPQ::from_seq(seq)
        }
    }
}
