//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral set implementation using ArraySeqStEph as backing store.

pub mod ArraySetStEph {
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ArraySetStEph<T: StT + Ord> {
        elements: ArraySeqStEphS<T>,
    }

    pub type ArraySetS<T> = ArraySetStEph<T>;

    pub trait ArraySetStEphTrait<T: StT + Ord> {
        fn size(&self) -> N;
        fn to_seq(&self) -> ArraySeqStEphS<T>;
        fn empty() -> Self;
        fn singleton(x: T) -> Self;
        fn from_seq(seq: ArraySeqStEphS<T>) -> Self;
        fn filter<F: Fn(&T) -> B>(&self, f: F) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, x: &T) -> B;
        fn delete(&mut self, x: &T);
        fn insert(&mut self, x: T);
    }

    impl<T: StT + Ord> ArraySetStEphTrait<T> for ArraySetStEph<T> {
        fn size(&self) -> N {
            self.elements.length()
        }

        fn to_seq(&self) -> ArraySeqStEphS<T> {
            self.elements.clone()
        }

        fn empty() -> Self {
            ArraySetStEph {
                elements: ArraySeqStEphS::empty(),
            }
        }

        fn singleton(x: T) -> Self {
            ArraySetStEph {
                elements: ArraySeqStEphS::singleton(x),
            }
        }

        fn from_seq(seq: ArraySeqStEphS<T>) -> Self {
            // Example 41.3: fromSeq a = Seq.reduce Set.union ∅ ⟨{x} : x ∈ a⟩
            // Work efficient and parallel implementation
            if seq.length() == 0 {
                return Self::empty();
            }

            // Reduce with union operation directly from seq
            let mut result = Self::empty();
            for i in 0..seq.length() {
                let elem = seq.nth(i).clone();
                let singleton_set = Self::singleton(elem);
                result = result.union(&singleton_set);
            }
            result
        }

        fn filter<F: Fn(&T) -> B>(&self, f: F) -> Self
        {
            let filtered = ArraySeqStEphS::filter(&self.elements, &f);
            ArraySetStEph { elements: filtered }
        }

        fn intersection(&self, other: &Self) -> Self {
            let filtered = ArraySeqStEphS::filter(&self.elements, &|elem| other.find(elem));
            ArraySetStEph { elements: filtered }
        }

        fn difference(&self, other: &Self) -> Self {
            let filtered = ArraySeqStEphS::filter(&self.elements, &|elem| !other.find(elem));
            ArraySetStEph { elements: filtered }
        }

        fn union(&self, other: &Self) -> Self {
            let self_len = self.elements.length();
            let other_len = other.elements.length();
            let mut result = Vec::with_capacity(self_len + other_len);

            // Add all elements from self
            for i in 0..self_len {
                result.push(self.elements.nth(i).clone());
            }

            // Add elements from other that are not in self
            for i in 0..other_len {
                let elem = other.elements.nth(i);
                if !self.find(elem) {
                    result.push(elem.clone());
                }
            }

            result.sort();
            ArraySetStEph {
                elements: ArraySeqStEphS::from_vec(result),
            }
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
            let filtered = ArraySeqStEphS::filter(&self.elements, &|elem| elem != x);
            self.elements = filtered;
        }

        fn insert(&mut self, x: T) {
            if !self.find(&x) {
                // Element doesn't exist, add it
                let new_len = self.elements.length() + 1;
                let mut temp_vec = Vec::with_capacity(new_len);
                for i in 0..self.elements.length() {
                    temp_vec.push(self.elements.nth(i).clone());
                }
                temp_vec.push(x);
                temp_vec.sort();
                self.elements = ArraySeqStEphS::from_vec(temp_vec);
            }
        }
    }

    impl<T: StT + Ord> Default for ArraySetStEph<T> {
        fn default() -> Self {
            Self::empty()
        }
    }

    #[macro_export]
    macro_rules! ArraySetStEphLit {
        () => {
            < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEph<_> as $crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    #[allow(dead_code)]
    fn _ArraySetStEphLit_type_checks() {
        let _: ArraySetStEph<i32> = ArraySetStEphLit![];
        let _: ArraySetStEph<i32> = ArraySetStEphLit![1, 2, 3];
    }
}
