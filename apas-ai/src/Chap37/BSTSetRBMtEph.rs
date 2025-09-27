//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the Red-Black multi-threaded BST implementation.

pub mod BSTSetRBMtEph {
    use std::collections::BTreeSet;

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTRBMtEph::BSTRBMtEph::{BSTRBMtEph, BSTRBMtEphTrait};
    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct BSTSetRBMtEph<T: StTInMtT + Ord> {
        tree: BSTRBMtEph<T>,
    }

    pub type BSTSetRBMt<T> = BSTSetRBMtEph<T>;

    pub trait BSTSetRBMtEphTrait<T: StTInMtT + Ord>: Sized {
        fn empty() -> Self;
        fn singleton(value: T) -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn find(&self, value: &T) -> Option<T>;
        fn contains(&self, value: &T) -> B;
        fn minimum(&self) -> Option<T>;
        fn maximum(&self) -> Option<T>;
        fn insert(&mut self, value: T);
        fn delete(&mut self, target: &T);
        fn union(&self, other: &Self) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn split(&self, pivot: &T) -> (Self, B, Self);
        fn join_pair(left: Self, right: Self) -> Self;
        fn join_m(left: Self, pivot: T, right: Self) -> Self;
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;
        fn as_tree(&self) -> &BSTRBMtEph<T>;
    }

    impl<T: StTInMtT + Ord> BSTSetRBMtEph<T> {
        pub fn empty() -> Self {
            Self {
                tree: BSTRBMtEph::new(),
            }
        }

        pub fn singleton(value: T) -> Self {
            let tree = BSTRBMtEph::new();
            tree.insert(value);
            Self { tree }
        }

        pub fn size(&self) -> N { self.tree.size() }

        pub fn is_empty(&self) -> B { self.tree.is_empty() }

        pub fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        pub fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        pub fn minimum(&self) -> Option<T> { self.tree.minimum() }

        pub fn maximum(&self) -> Option<T> { self.tree.maximum() }

        pub fn insert(&mut self, value: T) { self.tree.insert(value); }

        pub fn delete(&mut self, target: &T) {
            let mut values = self.values_vec();
            if let Some(pos) = values.iter().position(|x| x == target) {
                values.remove(pos);
                self.tree = Self::rebuild_from_vec(values);
            }
        }

        pub fn union(&self, other: &Self) -> Self {
            let mut merged: BTreeSet<T> = self.values_vec().into_iter().collect();
            for value in other.values_vec() {
                merged.insert(value);
            }
            Self::from_sorted_iter(merged.into_iter())
        }

        pub fn intersection(&self, other: &Self) -> Self {
            let other_values: BTreeSet<T> = other.values_vec().into_iter().collect();
            let filtered: Vec<T> = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| {
                    if other_values.contains(v) {
                        Some(v.clone())
                    } else {
                        None
                    }
                })
                .collect();
            Self::from_sorted_iter(filtered.into_iter())
        }

        pub fn difference(&self, other: &Self) -> Self {
            let other_values: BTreeSet<T> = other.values_vec().into_iter().collect();
            let filtered: Vec<T> = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| {
                    if !other_values.contains(v) {
                        Some(v.clone())
                    } else {
                        None
                    }
                })
                .collect();
            Self::from_sorted_iter(filtered.into_iter())
        }

        pub fn split(&self, pivot: &T) -> (Self, B, Self) {
            let mut left: Vec<T> = Vec::new();
            let mut right: Vec<T> = Vec::new();
            let mut found = false;
            for value in self.tree.in_order().iter() {
                if value < pivot {
                    left.push(value.clone());
                } else if value > pivot {
                    right.push(value.clone());
                } else {
                    found = true;
                }
            }
            (
                Self::from_sorted_iter(left.into_iter()),
                found,
                Self::from_sorted_iter(right.into_iter()),
            )
        }

        pub fn join_pair(left: Self, right: Self) -> Self {
            let mut combined: BTreeSet<T> = left.values_vec().into_iter().collect();
            for value in right.values_vec() {
                combined.insert(value);
            }
            Self::from_sorted_iter(combined.into_iter())
        }

        pub fn join_m(left: Self, pivot: T, right: Self) -> Self {
            let mut combined: BTreeSet<T> = left.values_vec().into_iter().collect();
            combined.insert(pivot);
            for value in right.values_vec() {
                combined.insert(value);
            }
            Self::from_sorted_iter(combined.into_iter())
        }

        pub fn filter<F>(&self, mut predicate: F) -> Self
        where
            F: FnMut(&T) -> bool,
        {
            let filtered: Vec<T> = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if predicate(v) { Some(v.clone()) } else { None })
                .collect();
            Self::from_sorted_iter(filtered.into_iter())
        }

        pub fn reduce<F>(&self, mut op: F, base: T) -> T
        where
            F: FnMut(T, T) -> T,
        {
            self.tree
                .in_order()
                .iter()
                .fold(base, |acc, value| op(acc, value.clone()))
        }

        pub fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        pub fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }

        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }

        fn rebuild_from_vec(values: Vec<T>) -> BSTRBMtEph<T> {
            let tree = BSTRBMtEph::new();
            for value in values {
                tree.insert(value);
            }
            tree
        }

        fn from_sorted_iter<I>(values: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let tree = BSTRBMtEph::new();
            for value in values {
                tree.insert(value);
            }
            Self { tree }
        }
    }

    impl<T: StTInMtT + Ord> BSTSetRBMtEphTrait<T> for BSTSetRBMtEph<T> {
        fn empty() -> Self { Self::empty() }

        fn singleton(value: T) -> Self { Self::singleton(value) }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        fn insert(&mut self, value: T) { self.tree.insert(value); }

        fn delete(&mut self, target: &T) { BSTSetRBMtEph::delete(self, target) }

        fn union(&self, other: &Self) -> Self { BSTSetRBMtEph::union(self, other) }

        fn intersection(&self, other: &Self) -> Self { BSTSetRBMtEph::intersection(self, other) }

        fn difference(&self, other: &Self) -> Self { BSTSetRBMtEph::difference(self, other) }

        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetRBMtEph::split(self, pivot) }

        fn join_pair(left: Self, right: Self) -> Self { BSTSetRBMtEph::join_pair(left, right) }

        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetRBMtEph::join_m(left, pivot, right) }

        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetRBMtEph::filter(self, predicate) }

        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetRBMtEph::reduce(self, op, base) }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTRBMtEph<T> { &self.tree }
    }

    #[macro_export]
    macro_rules! BSTSetRBMtEphLit {
        () => {
            < $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEph<_> as $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEph<_> as $crate::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    #[allow(dead_code)]
    fn _BSTSetRBMtEphLit_type_checks() {
        let _ = BSTSetRBMtEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTSetRBMtEph<i32> = BSTSetRBMtEphLit![]; // empty form requires explicit type
    }
}
