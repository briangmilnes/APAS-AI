//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Single-threaded ephemeral ordered table implementation extending TableStEph.

pub mod OrderedTableStEph {
    use crate::Chap42Claude::TableStEph::TableStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    /// Single-threaded ephemeral ordered table backed by TableStEph
    #[derive(PartialEq)]
    pub struct OrderedTableStEph<K: StT + Ord, V: StT> {
        base_table: TableStEph<K, V>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1 for keys) with ephemeral semantics
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT> {
        // Base table operations (ADT 42.1) - ephemeral semantics
        fn size(&self) -> N;
        fn empty() -> Self;
        fn singleton(k: K, v: V) -> Self;
        fn find(&self, k: &K) -> Option<V>;
        fn lookup(&self, k: &K) -> Option<V>; // Alias for find
        fn is_empty(&self) -> B;
        fn insert<F>(&mut self, k: K, v: V, combine: F) where F: Fn(&V, &V) -> V;
        fn delete(&mut self, k: &K) -> Option<V>;
        fn domain(&self) -> ArraySetStEph<K>;
        fn tabulate<F>(f: F, keys: &ArraySetStEph<K>) -> Self where F: Fn(&K) -> V;
        fn map<F>(&self, f: F) -> Self where F: Fn(&K, &V) -> V;
        fn filter<F>(&self, f: F) -> Self where F: Fn(&K, &V) -> B;
        fn reduce<R, F>(&self, init: R, f: F) -> R where F: Fn(R, &K, &V) -> R;
        fn intersection<F>(&mut self, other: &Self, f: F) where F: Fn(&V, &V) -> V;
        fn union<F>(&mut self, other: &Self, f: F) where F: Fn(&V, &V) -> V;
        fn difference(&mut self, other: &Self);
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        fn subtract(&mut self, keys: &ArraySetStEph<K>);
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>>;

        // Key ordering operations (ADT 43.1 adapted for tables)
        fn first_key(&self) -> Option<K>;
        fn last_key(&self) -> Option<K>;
        fn previous_key(&self, k: &K) -> Option<K>;
        fn next_key(&self, k: &K) -> Option<K>;
        fn split_key(&mut self, k: &K) -> (Self, Self) where Self: Sized;
        fn join_key(&mut self, other: Self);
        fn get_key_range(&self, k1: &K, k2: &K) -> Self;
        fn rank_key(&self, k: &K) -> N;
        fn select_key(&self, i: N) -> Option<K>;
        fn split_rank_key(&mut self, i: N) -> (Self, Self) where Self: Sized;
    }

    impl<K: StT + Ord, V: StT> OrderedTableStEph<K, V> {
        /// APAS: Work Θ(1), Span Θ(1)
        pub fn new() -> Self {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }
    }

    impl<K: StT + Ord, V: StT> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        // Base table operations - delegate to backing store with ephemeral semantics
        
        /// Work: O(1), Span: O(1)
        fn size(&self) -> N {
            self.base_table.size()
        }

        /// Work: O(1), Span: O(1)
        fn empty() -> Self {
            OrderedTableStEph {
                base_table: TableStEph::empty(),
            }
        }

        /// Work: O(1), Span: O(1)
        fn singleton(k: K, v: V) -> Self {
            OrderedTableStEph {
                base_table: TableStEph::singleton(k, v),
            }
        }

        /// Work: O(log n), Span: O(log n)
        fn find(&self, k: &K) -> Option<V> {
            self.base_table.find(k)
        }

        /// Work: O(log n), Span: O(log n)
        fn lookup(&self, k: &K) -> Option<V> {
            self.find(k)
        }

        /// Work: O(1), Span: O(1)
        fn is_empty(&self) -> B {
            self.size() == 0
        }

        /// Work: O(log n), Span: O(log n)
        fn insert<F>(&mut self, k: K, v: V, combine: F) where F: Fn(&V, &V) -> V {
            self.base_table.insert(k, v, combine);
        }

        /// Work: O(log n), Span: O(log n)
        fn delete(&mut self, k: &K) -> Option<V> {
            let old_value = self.find(k);
            self.base_table.delete(k);
            old_value
        }

        /// Work: O(n), Span: O(log n)
        fn domain(&self) -> ArraySetStEph<K> {
            self.base_table.domain()
        }

        /// Work: O(n log n), Span: O(log² n)
        fn tabulate<F>(f: F, keys: &ArraySetStEph<K>) -> Self 
        where 
            F: Fn(&K) -> V 
        {
            OrderedTableStEph {
                base_table: TableStEph::tabulate(f, keys),
            }
        }

        /// Work: O(n), Span: O(log n)
        fn map<F>(&self, f: F) -> Self
        where 
            F: Fn(&K, &V) -> V 
        {
            let mut result = OrderedTableStEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                let new_value = f(&pair.0, &pair.1);
                result.base_table.insert(pair.0.clone(), new_value, |_old, new| new.clone());
            }
            result
        }

        /// Work: O(n), Span: O(log n)
        fn filter<F>(&self, f: F) -> Self
        where 
            F: Fn(&K, &V) -> B 
        {
            let mut result = OrderedTableStEph::empty();
            let entries = self.collect();
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                if f(&pair.0, &pair.1) {
                    result.base_table.insert(pair.0.clone(), pair.1.clone(), |_old, new| new.clone());
                }
            }
            result
        }

        /// Work: O(n), Span: O(log n)
        fn reduce<R, F>(&self, init: R, f: F) -> R 
        where 
            F: Fn(R, &K, &V) -> R 
        {
            let entries = self.collect();
            let mut result = init;
            for i in 0..entries.length() {
                let pair = entries.nth(i);
                result = f(result, &pair.0, &pair.1);
            }
            result
        }

        /// Work: O(m + n), Span: O(log(m + n))
        fn intersection<F>(&mut self, other: &Self, f: F) 
        where 
            F: Fn(&V, &V) -> V 
        {
            self.base_table.intersection(&other.base_table, f);
        }

        /// Work: O(m + n), Span: O(log(m + n))
        fn union<F>(&mut self, other: &Self, f: F) 
        where 
            F: Fn(&V, &V) -> V 
        {
            self.base_table.union(&other.base_table, f);
        }

        /// Work: O(m + n), Span: O(log(m + n))
        fn difference(&mut self, other: &Self) {
            self.base_table.difference(&other.base_table);
        }

        /// Work: O(n), Span: O(log n)
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.restrict(keys);
        }

        /// Work: O(n), Span: O(log n)
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.subtract(keys);
        }

        /// Work: O(n), Span: O(log n)
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>> {
            let array_seq = self.base_table.collect();
            // Convert ArraySeqStEphS to AVLTreeSeqStPerS
            let len = array_seq.length();
            let mut elements = Vec::with_capacity(len);
            for i in 0..len {
                elements.push(array_seq.nth(i).clone());
            }
            AVLTreeSeqStPerS::from_vec(elements)
        }

        // Key ordering operations (ADT 43.1 adapted for tables)

        /// Work: O(log n), Span: O(log n)
        fn first_key(&self) -> Option<K> {
            let entries = self.collect();
            if entries.length() == 0 {
                None
            } else {
                Some(entries.nth(0).0.clone())
            }
        }

        /// Work: O(log n), Span: O(log n)
        fn last_key(&self) -> Option<K> {
            let entries = self.collect();
            let size = entries.length();
            if size == 0 {
                None
            } else {
                Some(entries.nth(size - 1).0.clone())
            }
        }

        /// Work: O(log n), Span: O(log n)
        fn previous_key(&self, k: &K) -> Option<K> {
            let entries = self.collect();
            let size = entries.length();
            
            for i in (0..size).rev() {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    return Some(pair.0.clone());
                }
            }
            None
        }

        /// Work: O(log n), Span: O(log n)
        fn next_key(&self, k: &K) -> Option<K> {
            let entries = self.collect();
            let size = entries.length();
            
            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 > k {
                    return Some(pair.0.clone());
                }
            }
            None
        }

        /// Work: O(log n), Span: O(log n)
        fn split_key(&mut self, k: &K) -> (Self, Self) {
            let entries = self.collect();
            let size = entries.length();
            let mut left_entries = Vec::with_capacity(size);
            let mut right_entries = Vec::with_capacity(size);
            let mut _found_value: Option<V> = None;

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    left_entries.push(pair.clone());
                } else {
                    // Keys >= k go to the right side
                    right_entries.push(pair.clone());
                }
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            // Clear current table (ephemeral behavior)
            *self = Self::empty();

            (
                from_sorted_entries(left_seq),
                from_sorted_entries(right_seq),
            )
        }

        /// Work: O(log(m + n)), Span: O(log(m + n))
        fn join_key(&mut self, other: Self) {
            self.union(&other, |v1, _v2| v1.clone());
        }

        /// Work: O(log n), Span: O(log n)
        fn get_key_range(&self, k1: &K, k2: &K) -> Self {
            let entries = self.collect();
            let size = entries.length();
            let mut range_entries = Vec::with_capacity(size);

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 >= k1 && &pair.0 <= k2 {
                    range_entries.push(pair.clone());
                }
            }

            let range_seq = AVLTreeSeqStPerS::from_vec(range_entries);
            from_sorted_entries(range_seq)
        }

        /// Work: O(log n), Span: O(log n)
        fn rank_key(&self, k: &K) -> N {
            let entries = self.collect();
            let size = entries.length();
            let mut count = 0;

            for i in 0..size {
                let pair = entries.nth(i);
                if &pair.0 < k {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        }

        /// Work: O(log n), Span: O(log n)
        fn select_key(&self, i: N) -> Option<K> {
            let entries = self.collect();
            if i >= entries.length() {
                None
            } else {
                Some(entries.nth(i).0.clone())
            }
        }

        /// Work: O(log n), Span: O(log n)
        fn split_rank_key(&mut self, i: N) -> (Self, Self) {
            let entries = self.collect();
            let size = entries.length();
            
            if i >= size {
                let current = self.clone();
                *self = Self::empty();
                return (current, Self::empty());
            }

            let mut left_entries = Vec::with_capacity(i);
            let mut right_entries = Vec::with_capacity(size - i);

            for j in 0..i {
                left_entries.push(entries.nth(j).clone());
            }
            for j in i..size {
                right_entries.push(entries.nth(j).clone());
            }

            let left_seq = AVLTreeSeqStPerS::from_vec(left_entries);
            let right_seq = AVLTreeSeqStPerS::from_vec(right_entries);

            // Clear current table (ephemeral behavior)
            *self = Self::empty();

            (
                from_sorted_entries(left_seq),
                from_sorted_entries(right_seq),
            )
        }
    }

    impl<K: StT + Ord, V: StT> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> Self {
            OrderedTableStEph {
                base_table: self.base_table.clone(),
            }
        }
    }

    /// Helper function for macro construction
    pub fn from_sorted_entries<K: StT + Ord, V: StT>(entries: AVLTreeSeqStPerS<Pair<K, V>>) -> OrderedTableStEph<K, V> {
        // Convert persistent sequence to Vec for TableStEph helper
        let len = entries.length();
        let mut elements = Vec::with_capacity(len);
        for i in 0..len {
            elements.push(entries.nth(i).clone());
        }
        OrderedTableStEph {
            base_table: crate::Chap42Claude::TableStEph::TableStEph::from_sorted_entries(elements),
        }
    }

    /// Macro for creating ephemeral ordered tables from sorted key-value pairs
    #[macro_export]
    macro_rules! OrderedTableStEphLit {
        () => {
            $crate::Chap43Claude::OrderedTableStEph::OrderedTableStEph::OrderedTableStEph::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43Claude::OrderedTableStEph::OrderedTableStEph::from_sorted_entries(seq)
        }};
    }

    pub use OrderedTableStEphLit;
}
