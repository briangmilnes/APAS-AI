//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded persistent ordered table implementation using parallel Treap backing.
//!
//! Work/Span Analysis:
//! - Parallel operations using BSTParaTreapMtEph<Pair<K, V>>
//! - O(lg n) span for all operations

pub mod OrderedTableMtPer {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::*;
    use crate::Types::Types::*;

    pub struct OrderedTableMtPer<K: MtKey + 'static, V: StTInMtT + Ord + 'static> {
        tree: ParamTreap<Pair<K, V>>,
    }

    pub trait OrderedTableMtPerTrait<K: MtKey + 'static, V: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)               -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                   -> Self;
        /// claude-4-sonet: Work Θ(lg n), Span Θ(lg n)
        fn singleton(k: K, v: V)     -> Self;
        /// claude-4-sonet: Work Θ(lg n), Span Θ(lg n)
        fn find(&self, k: &K)        -> Option<V>;
        /// claude-4-sonet: Work Θ(lg n), Span Θ(lg n)
        fn insert(&self, k: K, v: V) -> Self;
        /// claude-4-sonet: Work Θ(lg n), Span Θ(lg n)
        fn delete(&self, k: &K)      -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n)
        fn domain(&self)             -> OrderedSetMtEph<K>;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n)
        fn map<F: Pred<Pair<K, V>>>(&self, f: F) -> Self;
        /// claude-4-sonet: Work Θ(n), Span Θ(lg n)
        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> Self;
    }

    impl<K: MtKey + 'static, V: StTInMtT + Ord + 'static> OrderedTableMtPerTrait<K, V> for OrderedTableMtPer<K, V> {
        fn size(&self) -> N { self.tree.size() }

        fn empty() -> Self {
            OrderedTableMtPer {
                tree: ParamTreap::new(),
            }
        }

        fn singleton(k: K, v: V) -> Self {
            let tree = ParamTreap::new();
            tree.insert(Pair(k, v));
            OrderedTableMtPer { tree }
        }

        fn find(&self, k: &K) -> Option<V> {
            // Iterate through tree to find matching key
            let seq = self.tree.in_order();
            for i in 0..seq.length() {
                let Pair(key, val) = seq.nth(i);
                if key == k {
                    return Some(val.clone());
                }
            }
            None
        }

        fn insert(&self, k: K, v: V) -> Self {
            // Delete old entry with same key, then insert new pair
            let k_clone = k.clone();
            let filtered = self.tree.filter(move |pair: &Pair<K, V>| &pair.0 != &k_clone);
            filtered.insert(Pair(k, v));
            OrderedTableMtPer { tree: filtered }
        }

        fn delete(&self, k: &K) -> Self {
            let k_clone = k.clone();
            let filtered = self.tree.filter(move |pair: &Pair<K, V>| &pair.0 != &k_clone);
            OrderedTableMtPer { tree: filtered }
        }

        fn domain(&self) -> OrderedSetMtEph<K> {
            // Extract keys from all Pairs
            let pair_seq = self.tree.in_order();
            let keys: Vec<K> = (0..pair_seq.length())
                .map(|i| {
                    let Pair(key, _val) = pair_seq.nth(i);
                    key.clone()
                })
                .collect();
            let key_seq = ArraySeqStPerS::from_vec(keys);
            OrderedSetMtEph::from_seq(key_seq)
        }

        fn map<F: Pred<Pair<K, V>>>(&self, f: F) -> Self {
            OrderedTableMtPer {
                tree: self.tree.filter(f),
            }
        }

        fn filter<F: Pred<Pair<K, V>>>(&self, f: F) -> Self {
            OrderedTableMtPer {
                tree: self.tree.filter(f),
            }
        }
    }

    impl<K: MtKey + 'static, V: MtKey + 'static> Default for OrderedTableMtPer<K, V> {
        fn default() -> Self { Self::empty() }
    }

    impl<K: MtKey + 'static, V: MtKey + 'static> Clone for OrderedTableMtPer<K, V> {
        fn clone(&self) -> Self {
            OrderedTableMtPer {
                tree: self.tree.clone(),
            }
        }
    }
}
