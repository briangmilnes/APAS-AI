//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Double Hashing Strategy

pub mod DoubleHashing {
    use crate::Chap47::FlatHashTable::FlatHashTable::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Types::Types::*;

    /// Double hashing: h_i(x) = (h1(x) + i * h2(x)) mod m
    #[derive(Clone, Debug)]
    pub struct DoubleHashingStrategy<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> {
        hash1: H1,
        hash2: H2,
        _phantom: std::marker::PhantomData<K>,
    }

    impl<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> DoubleHashingStrategy<K, H1, H2> {
        pub fn new(hash1: H1, hash2: H2) -> Self {
            DoubleHashingStrategy {
                hash1,
                hash2,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<K: StT, H1: HashFunClone<K>, H2: HashFunClone<K>> ProbeSequence<K> for DoubleHashingStrategy<K, H1, H2> {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let h1 = self.hash1.hash(key, table_size);
            let h2 = self.hash2.hash(key, table_size);

            // Ensure h2 is not 0 (would cause infinite loop)
            let h2_safe = if h2 == 0 { 1 } else { h2 };

            (h1 + probe_index * h2_safe) % table_size
        }

        fn strategy_name(&self) -> String {
            format!(
                "DoubleHashing({}, {})",
                self.hash1.description(),
                self.hash2.description()
            )
        }
    }

    /// Type alias for double hashing hash table
    pub type DoubleHashingHashTable<K, V, H1, H2> = FlatHashTable<K, V, DoubleHashingStrategy<K, H1, H2>>;

    /// Factory functions
    pub struct DoubleHashingFactory;

    impl DoubleHashingFactory {
        pub fn create_string_table<V: StT>(
            initial_size: N,
        ) -> DoubleHashingHashTable<String, V, StringPositionHashFunction, PolynomialHashFunction> {
            let hash1 = StringPositionHashFunction;
            let hash2 = PolynomialHashFunction::new(31); // Common polynomial base
            let probe_strategy = DoubleHashingStrategy::new(hash1, hash2);
            FlatHashTable::create_table(probe_strategy, initial_size)
        }

        pub fn create_integer_table<V: StT>(
            initial_size: N,
            seed1: u64,
            seed2: u64,
        ) -> DoubleHashingHashTable<i32, V, UniversalIntegerHashFunction, UniversalIntegerHashFunction> {
            let hash_family = UniversalIntegerHashFamily::new();
            let hash1 = hash_family.generate(seed1);
            let hash2 = hash_family.generate(seed2);
            let probe_strategy = DoubleHashingStrategy::new(hash1, hash2);
            FlatHashTable::create_table(probe_strategy, initial_size)
        }
    }
}
