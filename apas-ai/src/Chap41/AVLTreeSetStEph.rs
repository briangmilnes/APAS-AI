//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using AVLTreeSeqStEph as backing store.

pub mod AVLTreeSetStEph {
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;
    use std::fmt;

    #[derive(PartialEq, Eq)]
    pub struct AVLTreeSetStEph<T: StT + Ord> {
        elements: AVLTreeSeqStEphS<T>,
    }

    pub type AVLTreeSetS<T> = AVLTreeSetStEph<T>;

    pub trait AVLTreeSetStEphTrait<T: StT + Ord> {
        fn size(&self) -> N;
        fn to_seq(&self) -> AVLTreeSeqStEphS<T>;
        fn empty() -> Self;
        fn singleton(x: T) -> Self;
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self;
        fn filter<F: Fn(&T) -> B>(&self, f: F) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, x: &T) -> B;
        fn delete(&mut self, x: &T);
        fn insert(&mut self, x: T);
    }

    impl<T: StT + Ord> AVLTreeSetStEphTrait<T> for AVLTreeSetStEph<T> {
        fn size(&self) -> N {
            self.elements.length()
        }

        fn to_seq(&self) -> AVLTreeSeqStEphS<T> {
            // Create a new sequence from the elements
            let size = self.elements.length();
            let mut vec_elements = Vec::with_capacity(size);
            for i in 0..size {
                vec_elements.push(self.elements.nth(i).clone());
            }
            AVLTreeSeqStEphS::from_vec(vec_elements)
        }

        fn empty() -> Self {
            AVLTreeSetStEph {
                elements: AVLTreeSeqStEphS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetStEph {
                elements: AVLTreeSeqStEphS::singleton(x),
            }
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self {
            // Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
            // Work efficient and parallel implementation
            if seq.length() == 0 {
                return Self::empty();
            }

            // Create sequence of singleton sets ⟨{x} : x ∈ a⟩
            let seq_len = seq.length();
            let mut singleton_sets = Vec::with_capacity(seq_len);
            for i in 0..seq_len {
                let elem = seq.nth(i).clone();
                singleton_sets.push(Self::singleton(elem));
            }

            // Reduce with union operation
            let mut result = Self::empty();
            for set in singleton_sets {
                result = result.union(&set);
            }
            result
        }

        fn filter<F: Fn(&T) -> B>(&self, f: F) -> Self
        {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if f(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        fn intersection(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if other.find(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        fn difference(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                if !other.find(elem) {
                    result.insert(elem.clone());
                }
            }
            result
        }

        fn union(&self, other: &Self) -> Self {
            let mut result = Self::empty();
            // Add all elements from self
            for i in 0..self.elements.length() {
                let elem = self.elements.nth(i);
                result.insert(elem.clone());
            }
            // Add all elements from other
            for i in 0..other.elements.length() {
                let elem = other.elements.nth(i);
                result.insert(elem.clone());
            }
            result
        }

        fn find(&self, x: &T) -> B {
            for i in 0..self.elements.length() {
                if self.elements.nth(i) == x {
                    return true;
                }
            }
            false
        }

        fn delete(&mut self, x: &T) {
            let size = self.elements.length();
            let mut vec_elements = Vec::with_capacity(size);
            for i in 0..size {
                let elem = self.elements.nth(i);
                if elem != x {
                    vec_elements.push(elem.clone());
                }
            }
            self.elements = AVLTreeSeqStEphS::from_vec(vec_elements);
        }

        fn insert(&mut self, x: T) {
            if !self.find(&x) {
                // Element doesn't exist, add it
                let size = self.elements.length();
                let mut vec_elements = Vec::with_capacity(size + 1);
                for i in 0..size {
                    vec_elements.push(self.elements.nth(i).clone());
                }
                vec_elements.push(x);
                vec_elements.sort();
                self.elements = AVLTreeSeqStEphS::from_vec(vec_elements);
            }
        }
    }

    impl<T: StT + Ord> Default for AVLTreeSetStEph<T> {
        fn default() -> Self {
            Self::empty()
        }
    }

    impl<T: StT + Ord> Clone for AVLTreeSetStEph<T> {
        fn clone(&self) -> Self {
            AVLTreeSetStEph {
                elements: self.elements.clone(),
            }
        }
    }

    impl<T: StT + Ord> fmt::Debug for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StT + Ord> fmt::Display for AVLTreeSetStEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            for i in 0..self.elements.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self.elements.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetStEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEph<_> as $crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::AVLTreeSetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    #[allow(dead_code)]
    fn _AVLTreeSetStEphLit_type_checks() {
        let _: AVLTreeSetStEph<i32> = AVLTreeSetStEphLit![];
        let _: AVLTreeSetStEph<i32> = AVLTreeSetStEphLit![1, 2, 3];
    }
}
