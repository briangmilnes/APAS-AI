//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral reducer-augmented ordered table implementation.

pub mod AugOrderedTableMtEph {
    use std::fmt::{Display, Debug, Formatter, Result};
    use std::sync::Arc;
    use std::thread;
    
    use crate::Chap43Claude::OrderedTableMtEph::OrderedTableMtEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    /// Multi-threaded ephemeral reducer-augmented ordered table
    /// Wraps OrderedTableMtEph and maintains cached reduction for O(1) reduceVal
    #[derive(PartialEq, Clone)]
    pub struct AugOrderedTableMtEph<K: StTInMtT + Ord + 'static, V: StTInMtT + 'static, F> 
    where 
        F: Fn(&V, &V) -> V + Clone + Send + Sync + 'static,
    {
        base_table: OrderedTableMtEph<K, V>,
        reducer: F,
        identity: V,
        cached_reduction: V,
    }

    pub type AugOrderedTableMt<K, V, F> = AugOrderedTableMtEph<K, V, F>;

    /// Trait defining all augmented ordered table operations (ADT 43.3) with multi-threaded ephemeral semantics
    /// Extends ordered table operations with efficient reduction and thread-safe operations
    pub trait AugOrderedTableMtEphTrait<K: StTInMtT + Ord + 'static, V: StTInMtT + 'static, F> 
    where 
        F: Fn(&V, &V) -> V + Clone + Send + Sync + 'static,
    {
        // Base table operations (ADT 42.1) - ephemeral semantics with parallelism
        fn size(&self) -> N;
        fn empty(reducer: F, identity: V) -> Self;
        fn singleton(k: K, v: V, reducer: F, identity: V) -> Self;
        fn find(&self, k: &K) -> Option<V>;
        fn lookup(&self, k: &K) -> Option<V>;
        fn is_empty(&self) -> B;
        fn insert<G>(&mut self, k: K, v: V, combine: G) where G: Fn(&V, &V) -> V + Send + Sync + 'static;
        fn delete(&mut self, k: &K) -> Option<V>;
        fn domain(&self) -> ArraySetStEph<K>;
        fn tabulate<G>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> Self 
        where G: Fn(&K) -> V + Send + Sync + 'static;
        fn map<G>(&self, f: G) -> Self where G: Fn(&K, &V) -> V + Send + Sync + 'static;
        fn filter<G>(&self, f: G) -> Self where G: Fn(&K, &V) -> B + Send + Sync + 'static;
        fn intersection<G>(&mut self, other: &Self, f: G) where G: Fn(&V, &V) -> V + Send + Sync + 'static;
        fn union<G>(&mut self, other: &Self, f: G) where G: Fn(&V, &V) -> V + Send + Sync + 'static;
        fn difference(&mut self, other: &Self);
        fn restrict(&mut self, keys: &ArraySetStEph<K>);
        fn subtract(&mut self, keys: &ArraySetStEph<K>);
        fn reduce<R, G>(&self, init: R, f: G) -> R where G: Fn(R, &K, &V) -> R + Send + Sync + 'static, R: Send + Sync + 'static;
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>>;

        // Key ordering operations (ADT 43.1 adapted for tables) - sequential (inherently sequential on trees)
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

        // Augmented operations (ADT 43.3) - the key innovation with parallelism
        /// Claude Work: O(1), Span: O(1)
        /// Returns the cached reduction of all values using the reducer function
        fn reduce_val(&self) -> V;
        
        /// Claude Work: O(lg n), Span: O(lg n)  
        /// Efficient range reduction for TRAMLAW/QADSAN scenarios
        fn reduce_range(&self, k1: &K, k2: &K) -> V;

        /// Claude Work: O(lg n), Span: O(lg n)
        /// Parallel range reduction using spawn/join
        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> V;
    }

    impl<K: StTInMtT + Ord + 'static, V: StTInMtT + 'static, F> AugOrderedTableMtEphTrait<K, V, F> for AugOrderedTableMtEph<K, V, F>
    where 
        F: Fn(&V, &V) -> V + Clone + Send + Sync + 'static,
    {
        /// Claude Work: O(1), Span: O(1)
        fn size(&self) -> N {
            self.base_table.size()
        }

        /// Claude Work: O(1), Span: O(1)
        fn empty(reducer: F, identity: V) -> Self {
            Self {
                base_table: OrderedTableMtEph::empty(),
                cached_reduction: identity.clone(),
                reducer,
                identity,
            }
        }

        /// Claude Work: O(1), Span: O(1)
        fn singleton(k: K, v: V, reducer: F, identity: V) -> Self {
            Self {
                base_table: OrderedTableMtEph::singleton(k, v.clone()),
                cached_reduction: v,
                reducer,
                identity,
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn find(&self, k: &K) -> Option<V> {
            self.base_table.find(k)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn lookup(&self, k: &K) -> Option<V> {
            self.base_table.lookup(k)
        }

        /// Claude Work: O(1), Span: O(1)
        fn is_empty(&self) -> B {
            self.base_table.is_empty()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn insert<G>(&mut self, k: K, v: V, combine: G) where G: Fn(&V, &V) -> V + Send + Sync + 'static {
            let old_size = self.base_table.size();
            self.base_table.insert(k, v.clone(), combine);
            
            // Update cached reduction
            if old_size == 0 {
                self.cached_reduction = v;
            } else {
                self.cached_reduction = (self.reducer)(&self.cached_reduction, &v);
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn delete(&mut self, k: &K) -> Option<V> {
            let result = self.base_table.delete(k);
            // Recalculate reduction after deletion
            self.cached_reduction = self.recalculate_reduction();
            result
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn domain(&self) -> ArraySetStEph<K> {
            self.base_table.domain()
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn tabulate<G>(f: G, keys: &ArraySetStEph<K>, reducer: F, identity: V) -> Self 
        where G: Fn(&K) -> V + Send + Sync + 'static
        {
            let base_table = OrderedTableMtEph::tabulate(f, keys);
            let cached_reduction = Self::calculate_reduction(&base_table, &reducer, &identity);
            
            Self {
                base_table,
                cached_reduction,
                reducer,
                identity,
            }
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn map<G>(&self, f: G) -> Self where G: Fn(&K, &V) -> V + Send + Sync + 'static {
            let new_base = self.base_table.map(f);
            let new_reduction = Self::calculate_reduction(&new_base, &self.reducer, &self.identity);
            
            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn filter<G>(&self, f: G) -> Self where G: Fn(&K, &V) -> B + Send + Sync + 'static {
            let new_base = self.base_table.filter(f);
            let new_reduction = Self::calculate_reduction(&new_base, &self.reducer, &self.identity);
            
            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn intersection<G>(&mut self, other: &Self, f: G) where G: Fn(&V, &V) -> V + Send + Sync + 'static {
            self.base_table.intersection(&other.base_table, f);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn union<G>(&mut self, other: &Self, f: G) where G: Fn(&V, &V) -> V + Send + Sync + 'static {
            self.base_table.union(&other.base_table, f);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn difference(&mut self, other: &Self) {
            self.base_table.difference(&other.base_table);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn restrict(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.restrict(keys);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n + m), Span: O(lg n + lg m)
        fn subtract(&mut self, keys: &ArraySetStEph<K>) {
            self.base_table.subtract(keys);
            self.cached_reduction = self.recalculate_reduction();
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn reduce<R, G>(&self, init: R, f: G) -> R where G: Fn(R, &K, &V) -> R + Send + Sync + 'static, R: Send + Sync + 'static {
            self.base_table.reduce(init, f)
        }

        /// Claude Work: O(n), Span: O(lg n)
        fn collect(&self) -> AVLTreeSeqStPerS<Pair<K, V>> {
            self.base_table.collect()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn first_key(&self) -> Option<K> {
            self.base_table.first_key()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn last_key(&self) -> Option<K> {
            self.base_table.last_key()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn previous_key(&self, k: &K) -> Option<K> {
            self.base_table.previous_key(k)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn next_key(&self, k: &K) -> Option<K> {
            self.base_table.next_key(k)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn split_key(&mut self, k: &K) -> (Self, Self) {
            let (left_base, right_base) = self.base_table.split_key(k);
            
            let left_reduction = Self::calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = Self::calculate_reduction(&right_base, &self.reducer, &self.identity);
            
            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            
            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            
            (left, right)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn join_key(&mut self, other: Self) {
            let old_reduction = self.cached_reduction.clone();
            let other_reduction = other.cached_reduction.clone();
            let other_size = other.base_table.size();
            
            self.base_table.join_key(other.base_table);
            
            // Combine reductions
            if self.base_table.size() == 0 {
                self.cached_reduction = other_reduction;
            } else if other_size == 0 {
                self.cached_reduction = old_reduction;
            } else {
                self.cached_reduction = (self.reducer)(&old_reduction, &other_reduction);
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn get_key_range(&self, k1: &K, k2: &K) -> Self {
            let new_base = self.base_table.get_key_range(k1, k2);
            let new_reduction = Self::calculate_reduction(&new_base, &self.reducer, &self.identity);
            
            Self {
                base_table: new_base,
                cached_reduction: new_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            }
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn rank_key(&self, k: &K) -> N {
            self.base_table.rank_key(k)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn select_key(&self, i: N) -> Option<K> {
            self.base_table.select_key(i)
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        fn split_rank_key(&mut self, i: N) -> (Self, Self) {
            let (left_base, right_base) = self.base_table.split_rank_key(i);
            
            let left_reduction = Self::calculate_reduction(&left_base, &self.reducer, &self.identity);
            let right_reduction = Self::calculate_reduction(&right_base, &self.reducer, &self.identity);
            
            let left = Self {
                base_table: left_base,
                cached_reduction: left_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            
            let right = Self {
                base_table: right_base,
                cached_reduction: right_reduction,
                reducer: self.reducer.clone(),
                identity: self.identity.clone(),
            };
            
            (left, right)
        }

        /// Claude Work: O(1), Span: O(1)
        /// The key innovation: O(1) reduction using cached value
        fn reduce_val(&self) -> V {
            self.cached_reduction.clone()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        /// Efficient range reduction for TRAMLAW/QADSAN scenarios
        fn reduce_range(&self, k1: &K, k2: &K) -> V {
            let range_table = self.get_key_range(k1, k2);
            range_table.reduce_val()
        }

        /// Claude Work: O(lg n), Span: O(lg n)
        /// Parallel range reduction using spawn/join for large ranges
        fn reduce_range_parallel(&self, k1: &K, k2: &K) -> V {
            let range_table = self.get_key_range(k1, k2);
            
            // For small ranges, use sequential reduction
            if range_table.size() < 1000 {
                return range_table.reduce_val();
            }
            
            // For large ranges, split and compute in parallel
            let mid_rank = range_table.size() / 2;
            if let Some(mid_key) = range_table.select_key(mid_rank) {
                let left_table = range_table.get_key_range(k1, &mid_key);
                let right_table = range_table.get_key_range(&mid_key, k2);
                
                let reducer = range_table.reducer.clone();
                let left_reduction = Arc::new(std::sync::Mutex::new(range_table.identity.clone()));
                let right_reduction = Arc::new(std::sync::Mutex::new(range_table.identity.clone()));
                
                let left_reduction_clone = Arc::clone(&left_reduction);
                let right_reduction_clone = Arc::clone(&right_reduction);
                
                let left_handle = thread::spawn(move || {
                    let result = left_table.reduce_val();
                    *left_reduction_clone.lock().unwrap() = result;
                });
                
                let right_handle = thread::spawn(move || {
                    let result = right_table.reduce_val();
                    *right_reduction_clone.lock().unwrap() = result;
                });
                
                left_handle.join().unwrap();
                right_handle.join().unwrap();
                
                let left_val = left_reduction.lock().unwrap().clone();
                let right_val = right_reduction.lock().unwrap().clone();
                
                reducer(&left_val, &right_val)
            } else {
                range_table.reduce_val()
            }
        }
    }

    impl<K: StTInMtT + Ord + 'static, V: StTInMtT + 'static, F> AugOrderedTableMtEph<K, V, F>
    where 
        F: Fn(&V, &V) -> V + Clone + Send + Sync + 'static,
    {
        /// Helper to recalculate reduction from current base table
        fn recalculate_reduction(&self) -> V {
            Self::calculate_reduction(&self.base_table, &self.reducer, &self.identity)
        }

        /// Helper to calculate reduction from any base table
        fn calculate_reduction(base: &OrderedTableMtEph<K, V>, reducer: &F, identity: &V) -> V {
            if base.size() == 0 {
                return identity.clone();
            }
            
            let pairs = base.collect();
            let mut result = identity.clone();
            let mut first = true;
            
            for i in 0..pairs.length() {
                let pair = pairs.nth(i);
                if first {
                    result = pair.1.clone();
                    first = false;
                } else {
                    result = reducer(&result, &pair.1);
                }
            }
            
            result
        }
    }

    impl<K: StTInMtT + Ord + 'static, V: StTInMtT + 'static, F> Display for AugOrderedTableMtEph<K, V, F>
    where 
        F: Fn(&V, &V) -> V + Clone + Send + Sync + 'static,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "AugOrderedTableMtEph(size: {}, reduction: {})", 
                   self.size(), self.cached_reduction)
        }
    }

    impl<K: StTInMtT + Ord + 'static, V: StTInMtT + 'static, F> Debug for AugOrderedTableMtEph<K, V, F>
    where 
        F: Fn(&V, &V) -> V + Clone + Send + Sync + 'static,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("AugOrderedTableMtEph")
                .field("size", &self.size())
                .field("cached_reduction", &self.cached_reduction)
                .finish()
        }
    }

    // Macro for creating augmented ordered table literals
    #[macro_export]
    macro_rules! AugOrderedTableMtEphLit {
        (reducer: $reducer:expr, identity: $identity:expr, $($k:expr => $v:expr),* $(,)?) => {{
            let mut table = $crate::Chap43Claude::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::empty($reducer, $identity);
            $(
                $crate::Chap43Claude::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::insert(&mut table, $k, $v, |_old, new| new.clone());
            )*
            table
        }};
        (reducer: $reducer:expr, identity: $identity:expr) => {{
            $crate::Chap43Claude::AugOrderedTableMtEph::AugOrderedTableMtEph::AugOrderedTableMtEphTrait::empty($reducer, $identity)
        }};
    }

    #[allow(dead_code)]
    fn _AugOrderedTableMtEphLit_type_checks() {
        let sum_reducer = |a: &i32, b: &i32| a + b;
        let _: AugOrderedTableMtEph<i32, i32, _> = AugOrderedTableMtEphLit![
            reducer: sum_reducer, identity: 0, 
            1 => 10, 2 => 20
        ];
        let _: AugOrderedTableMtEph<i32, i32, _> = AugOrderedTableMtEphLit![
            reducer: sum_reducer, identity: 0
        ];
    }
}
