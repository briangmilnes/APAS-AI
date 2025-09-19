//! Set interface built atop the Splay multi-threaded BST implementation.

pub mod BSTSetSplayMtEph {
    use std::collections::BTreeSet;

    use crate::ArraySeqStPer::ArraySeqStPer::ArrayStPerS;
    use crate::BSTSplayMtEph::BSTSplayMtEph::BSTSplayMtEph;
    use crate::Types::Types::*;

    pub struct BSTSetSplayMtEph<T: StTinMtT + Ord> {
        tree: BSTSplayMtEph<T>,
    }

    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;

    pub trait BSTSetSplayMtEphTrait<T: StTinMtT + Ord>: Sized {
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
        fn iter_in_order(&self) -> ArrayStPerS<T>;
        fn as_tree(&self) -> &BSTSplayMtEph<T>;
    }

    impl<T: StTinMtT + Ord> BSTSetSplayMtEph<T> {
        pub fn empty() -> Self {
            Self { tree: BSTSplayMtEph::new() }
        }

        pub fn singleton(value: T) -> Self {
            let tree = BSTSplayMtEph::new();
            tree.insert(value);
            Self { tree }
        }

        pub fn size(&self) -> N {
            self.tree.size()
        }

        pub fn is_empty(&self) -> B {
            self.tree.is_empty()
        }

        pub fn find(&self, value: &T) -> Option<T> {
            self.tree.find(value)
        }

        pub fn contains(&self, value: &T) -> B {
            self.tree.contains(value)
        }

        pub fn minimum(&self) -> Option<T> {
            self.tree.minimum()
        }

        pub fn maximum(&self) -> Option<T> {
            self.tree.maximum()
        }

        pub fn insert(&mut self, value: T) {
            self.tree.insert(value);
        }

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
                .filter_map(|v| if other_values.contains(v) { Some(v.clone()) } else { None })
                .collect();
            Self::from_sorted_iter(filtered.into_iter())
        }

        pub fn difference(&self, other: &Self) -> Self {
            let other_values: BTreeSet<T> = other.values_vec().into_iter().collect();
            let filtered: Vec<T> = self
                .tree
                .in_order()
                .iter()
                .filter_map(|v| if !other_values.contains(v) { Some(v.clone()) } else { None })
                .collect();
            Self::from_sorted_iter(filtered.into_iter())
        }

        pub fn split(&self, pivot: &T) -> (Self, B, Self) {
            let mut left: Vec<T> = Vec::new();
            let mut right: Vec<T> = Vec::new();
            let mut found = B::False;
            for value in self.tree.in_order().iter() {
                if value < pivot {
                    left.push(value.clone());
                } else if value > pivot {
                    right.push(value.clone());
                } else {
                    found = B::True;
                }
            }
            (Self::from_sorted_iter(left.into_iter()), found, Self::from_sorted_iter(right.into_iter()))
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
            let filtered: Vec<T> =
                self.tree.in_order().iter().filter_map(|v| if predicate(v) { Some(v.clone()) } else { None }).collect();
            Self::from_sorted_iter(filtered.into_iter())
        }

        pub fn reduce<F>(&self, mut op: F, base: T) -> T
        where
            F: FnMut(T, T) -> T,
        {
            self.tree.in_order().iter().fold(base, |acc, value| op(acc, value.clone()))
        }

        pub fn iter_in_order(&self) -> ArrayStPerS<T> {
            self.tree.in_order()
        }

        pub fn as_tree(&self) -> &BSTSplayMtEph<T> {
            &self.tree
        }

        fn values_vec(&self) -> Vec<T> {
            self.tree.in_order().iter().cloned().collect()
        }

        fn rebuild_from_vec(values: Vec<T>) -> BSTSplayMtEph<T> {
            let tree = BSTSplayMtEph::new();
            for value in values {
                tree.insert(value);
            }
            tree
        }

        fn from_sorted_iter<I>(values: I) -> Self
        where
            I: IntoIterator<Item = T>,
        {
            let tree = BSTSplayMtEph::new();
            for value in values {
                tree.insert(value);
            }
            Self { tree }
        }
    }

    impl<T: StTinMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {
        fn empty() -> Self {
            Self::empty()
        }

        fn singleton(value: T) -> Self {
            Self::singleton(value)
        }

        fn size(&self) -> N {
            self.tree.size()
        }

        fn is_empty(&self) -> B {
            self.tree.is_empty()
        }

        fn find(&self, value: &T) -> Option<T> {
            self.tree.find(value)
        }

        fn contains(&self, value: &T) -> B {
            self.tree.contains(value)
        }

        fn minimum(&self) -> Option<T> {
            self.tree.minimum()
        }

        fn maximum(&self) -> Option<T> {
            self.tree.maximum()
        }

        fn insert(&mut self, value: T) {
            self.tree.insert(value);
        }

        fn delete(&mut self, target: &T) {
            BSTSetSplayMtEph::delete(self, target)
        }

        fn union(&self, other: &Self) -> Self {
            BSTSetSplayMtEph::union(self, other)
        }

        fn intersection(&self, other: &Self) -> Self {
            BSTSetSplayMtEph::intersection(self, other)
        }

        fn difference(&self, other: &Self) -> Self {
            BSTSetSplayMtEph::difference(self, other)
        }

        fn split(&self, pivot: &T) -> (Self, B, Self) {
            BSTSetSplayMtEph::split(self, pivot)
        }

        fn join_pair(left: Self, right: Self) -> Self {
            BSTSetSplayMtEph::join_pair(left, right)
        }

        fn join_m(left: Self, pivot: T, right: Self) -> Self {
            BSTSetSplayMtEph::join_m(left, pivot, right)
        }

        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self {
            BSTSetSplayMtEph::filter(self, predicate)
        }

        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T {
            BSTSetSplayMtEph::reduce(self, op, base)
        }

        fn iter_in_order(&self) -> ArrayStPerS<T> {
            self.tree.in_order()
        }

        fn as_tree(&self) -> &BSTSplayMtEph<T> {
            &self.tree
        }
    }
}
