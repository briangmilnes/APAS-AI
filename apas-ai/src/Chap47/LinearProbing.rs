//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Linear Probing Strategy

pub mod LinearProbing {
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Types::Types::*;

    /// Linear probing: h_i(x) = (h(x) + i) mod m
    #[derive(Clone, Debug)]
    pub struct LinearProbingStrategy<K: StT, H: HashFunClone<K>> {
        base_hash: H,
        _phantom: std::marker::PhantomData<K>,
    }

    impl<K: StT, H: HashFunClone<K>> LinearProbingStrategy<K, H> {
        pub fn new(hash_fn: H) -> Self {
            LinearProbingStrategy {
                base_hash: hash_fn,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<K: StT, H: HashFunClone<K>> ProbeSequence<K> for LinearProbingStrategy<K, H> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let base_hash = self.base_hash.hash(key, table_size);
            (base_hash + probe_index) % table_size
        }
        
        fn strategy_name(&self) -> String {
            format!("LinearProbing({})", self.base_hash.description())
        }
    }

    /// Type alias for linear probing hash table
    pub type LinearProbingHashTable<K, V, H> = FlatHashTable<K, V, LinearProbingStrategy<K, H>>;

    /// Factory functions
    pub struct LinearProbingFactory;

    impl LinearProbingFactory {
        pub fn create_string_table<V: StT>(initial_size: N) -> LinearProbingHashTable<String, V, StringPositionHashFunction> {
            let probe_strategy = LinearProbingStrategy::new(StringPositionHashFunction);
            FlatHashTable::create_table(probe_strategy, initial_size)
        }
        
        pub fn create_integer_table<V: StT>(initial_size: N, seed: u64) -> LinearProbingHashTable<i32, V, UniversalIntegerHashFunction> {
            let hash_family = UniversalIntegerHashFamily::new();
            let hash_fn = hash_family.generate(seed);
            let probe_strategy = LinearProbingStrategy::new(hash_fn);
            FlatHashTable::create_table(probe_strategy, initial_size)
        }
    }
}
