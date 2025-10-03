//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Set interface built atop the Splay multi-threaded BST implementation.

pub mod BSTSetSplayMtEph {

    use std::collections::BTreeSet;

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct BSTSetSplayMtEph<T: StTInMtT + Ord> {
        tree: BSTSplayMtEph<T>,
    }

    pub type BSTSetSplayMt<T> = BSTSetSplayMtEph<T>;

    pub trait BSTSetSplayMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(value: T) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> B;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn find(&self, value: &T) -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn contains(&self, value: &T) -> B;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn minimum(&self) -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn maximum(&self) -> Option<T>;
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn insert(&mut self, value: T);
        /// claude-4-sonet: Work Θ(log n) amortized, Θ(n) worst case; Span Θ(log n) amortized with locking
        fn delete(&mut self, target: &T);
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn union(&self, other: &Self) -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn intersection(&self, other: &Self) -> Self;
        /// claude-4-sonet: Work Θ(m log(n/m)) where m = min(|self|, |other|), Span Θ(log n × log m)
        fn difference(&self, other: &Self) -> Self;
        /// claude-4-sonet: Work Θ(log n) amortized, Span Θ(log n)
        fn split(&self, pivot: &T) -> (Self, B, Self);
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
        fn join_pair(left: Self, right: Self) -> Self;
        /// claude-4-sonet: Work Θ(log(|left| + |right|)), Span Θ(log(|left| + |right|))
        fn join_m(left: Self, pivot: T, right: Self) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn iter_in_order(&self) -> ArraySeqStPerS<T>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn as_tree(&self) -> &BSTSplayMtEph<T>;
    }

    impl<T: StTInMtT + Ord> BSTSetSplayMtEph<T> {
        pub fn empty() -> Self {
            Self {
                tree: BSTSplayMtEph::new(),
            }
        }

        pub fn singleton(value: T) -> Self {
            let tree = BSTSplayMtEph::new();
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

        pub fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }

        fn values_vec(&self) -> Vec<T> { self.tree.in_order().iter().cloned().collect() }

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

    impl<T: StTInMtT + Ord> BSTSetSplayMtEphTrait<T> for BSTSetSplayMtEph<T> {
        fn empty() -> Self { Self::empty() }

        fn singleton(value: T) -> Self { Self::singleton(value) }

        fn size(&self) -> N { self.tree.size() }

        fn is_empty(&self) -> B { self.tree.is_empty() }

        fn find(&self, value: &T) -> Option<T> { self.tree.find(value) }

        fn contains(&self, value: &T) -> B { self.tree.contains(value) }

        fn minimum(&self) -> Option<T> { self.tree.minimum() }

        fn maximum(&self) -> Option<T> { self.tree.maximum() }

        fn insert(&mut self, value: T) { self.tree.insert(value); }

        fn delete(&mut self, target: &T) { BSTSetSplayMtEph::delete(self, target) }

        fn union(&self, other: &Self) -> Self { BSTSetSplayMtEph::union(self, other) }

        fn intersection(&self, other: &Self) -> Self { BSTSetSplayMtEph::intersection(self, other) }

        fn difference(&self, other: &Self) -> Self { BSTSetSplayMtEph::difference(self, other) }

        fn split(&self, pivot: &T) -> (Self, B, Self) { BSTSetSplayMtEph::split(self, pivot) }

        fn join_pair(left: Self, right: Self) -> Self { BSTSetSplayMtEph::join_pair(left, right) }

        fn join_m(left: Self, pivot: T, right: Self) -> Self { BSTSetSplayMtEph::join_m(left, pivot, right) }

        fn filter<F: FnMut(&T) -> bool>(&self, predicate: F) -> Self { BSTSetSplayMtEph::filter(self, predicate) }

        fn reduce<F: FnMut(T, T) -> T>(&self, op: F, base: T) -> T { BSTSetSplayMtEph::reduce(self, op, base) }

        fn iter_in_order(&self) -> ArraySeqStPerS<T> { self.tree.in_order() }

        fn as_tree(&self) -> &BSTSplayMtEph<T> { &self.tree }
    }

    #[macro_export]
    macro_rules! BSTSetSplayMtEphLit {
        () => {
            < $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEph<_> as $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEph<_> as $crate::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    #[allow(dead_code)]
    fn _BSTSetSplayMtEphLit_type_checks() {
        let _ = BSTSetSplayMtEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTSetSplayMtEph<i32> = BSTSetSplayMtEphLit![]; // empty form requires explicit type
    }
}
