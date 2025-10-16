//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Quadratic Probing Strategy

pub mod QuadraticProbing {

    use std::marker::PhantomData;

use crate::Types::Types::*;
use crate::Chap47::FlatHashTable::FlatHashTable::*;
use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    #[derive(Clone, Debug)]
    pub struct QuadraticProbingStrategy<K: StT, H: HashFunClone<K>> {
        base_hash: H,
        c1: N,
        c2: N,
        _phantom: PhantomData<K>,
    }

    impl<K: StT, H: HashFunClone<K>> QuadraticProbingStrategy<K, H> {
        pub fn new(hash_fn: H, c1: N, c2: N) -> Self {
            QuadraticProbingStrategy {
                base_hash: hash_fn,
                c1,
                c2,
                _phantom: PhantomData,
            }
        }

        /// Standard quadratic probing with c1=1, c2=1
        pub fn standard(hash_fn: H) -> Self { Self::new(hash_fn, 1, 1) }
    }

    impl<K: StT, H: HashFunClone<K>> ProbeSequence<K> for QuadraticProbingStrategy<K, H> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let base_hash = self.base_hash.hash(key, table_size);
            let quadratic_offset = self.c1 * probe_index + self.c2 * probe_index * probe_index;
            (base_hash + quadratic_offset) % table_size
        }

        fn strategy_name(&self) -> String {
            format!(
                "QuadraticProbing({}, c1={}, c2={})",
                self.base_hash.description(),
                self.c1,
                self.c2
            )
        }
    }

    /// Type alias for quadratic probing hash table
    pub type QuadraticProbingHashTable<K, V, H> = FlatHashTable<K, V, QuadraticProbingStrategy<K, H>>;

    /// Constructor functions for quadratic probing hash tables
    /// APAS: Work Θ(1), Span Θ(1)
    pub fn create_quadratic_probing_string_table<V: StT>(
        initial_size: N,
    ) -> QuadraticProbingHashTable<String, V, StringPositionHashFunction> {
        let probe_strategy = QuadraticProbingStrategy::standard(StringPositionHashFunction);
        FlatHashTable::create_table(probe_strategy, initial_size)
    }

    /// APAS: Work Θ(1), Span Θ(1)
    pub fn create_quadratic_probing_integer_table<V: StT>(
        initial_size: N,
        seed: u64,
    ) -> QuadraticProbingHashTable<i32, V, UniversalIntegerHashFunction> {
        let hash_family = UniversalIntegerHashFamily::new();
        let hash_fn = hash_family.generate(seed);
        let probe_strategy = QuadraticProbingStrategy::standard(hash_fn);
        FlatHashTable::create_table(probe_strategy, initial_size)
    }
}
