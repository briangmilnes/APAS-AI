//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Nested Hash Table implementation - Section 1.1 Parametric Design

pub mod NestedHashTable {

use std::fmt::{Debug, Display, Formatter, Result};

use crate::Types::Types::*;
use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    #[derive(Clone, Debug)]
    pub struct NestedHashTable<K: StT, V: StT> {
        buckets: ArraySeqStPerS<ArraySeqStPerS<Pair<K, V>>>,
        num_elements: N,
        load_factor_manager: LoadFactorManager,
    }

    impl<K: StT, V: StT> NestedHashTable<K, V> {
        /// Claude Work: Θ(m), Span: Θ(m)
        /// Create empty nested hash table with given initial size
        pub fn create_table(initial_size: N) -> Self {
            let size = initial_size.max(8);
            let mut buckets = ArraySeqStPerS::empty();

            for _ in 0..size {
                let empty_chain = ArraySeqStPerS::empty();
                let single_seq = ArraySeqStPerS::singleton(empty_chain);
                buckets = ArraySeqStPerS::append(&buckets, &single_seq);
            }

            NestedHashTable {
                buckets,
                num_elements: 0,
                load_factor_manager: LoadFactorManager::new(0.75, 0.25),
            }
        }

        /// Claude Work: Θ(1 + α) expected, Span: Θ(1 + α) expected
        /// Insert key-value pair using default hash function
        pub fn insert(&self, key: K, value: V) -> Self
        where
            K: std::hash::Hash,
        {
            let hash_fn = DefaultHashFunction;
            let table_size = self.buckets.length();
            let hash_code = hash_fn.hash(&key, table_size);
            let chain = self.buckets.nth(hash_code);

            // Check if key exists and remove old entry
            let mut new_chain = ArraySeqStPerS::empty();
            let mut key_existed = false;

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if pair.0 != key {
                    let single_seq = ArraySeqStPerS::singleton(pair.clone());
                    new_chain = ArraySeqStPerS::append(&new_chain, &single_seq);
                } else {
                    key_existed = true;
                }
            }

            // Add new entry at head
            let new_pair = Pair(key, value);
            let new_head = ArraySeqStPerS::singleton(new_pair);
            new_chain = ArraySeqStPerS::append(&new_head, &new_chain);

            // Update buckets
            let mut new_buckets = ArraySeqStPerS::empty();
            for i in 0..table_size {
                let bucket_to_use = if i == hash_code {
                    new_chain.clone()
                } else {
                    self.buckets.nth(i).clone()
                };
                let single_seq = ArraySeqStPerS::singleton(bucket_to_use);
                new_buckets = ArraySeqStPerS::append(&new_buckets, &single_seq);
            }

            let new_num_elements = if key_existed {
                self.num_elements
            } else {
                self.num_elements + 1
            };

            let mut result = NestedHashTable {
                buckets: new_buckets,
                num_elements: new_num_elements,
                load_factor_manager: self.load_factor_manager.clone(),
            };

            // Check resize
            if result.load_factor_manager.should_grow(result.num_elements, table_size) {
                let new_size = result.load_factor_manager.grow_size(table_size);
                result = result.resize(new_size);
            }

            result
        }

        /// Claude Work: Θ(1 + α) expected, Span: Θ(1 + α) expected
        /// Look up value for given key
        pub fn lookup(&self, key: &K) -> Option<&V>
        where
            K: std::hash::Hash,
        {
            let hash_fn = DefaultHashFunction;
            let table_size = self.buckets.length();
            let hash_code = hash_fn.hash(key, table_size);
            let chain = self.buckets.nth(hash_code);

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if &pair.0 == key {
                    return Some(&pair.1);
                }
            }

            None
        }

        /// Claude Work: Θ(1 + α) expected, Span: Θ(1 + α) expected
        pub fn delete(&self, key: &K) -> (Self, bool)
        where
            K: std::hash::Hash,
        {
            let hash_fn = DefaultHashFunction;
            let table_size = self.buckets.length();
            let hash_code = hash_fn.hash(key, table_size);
            let chain = self.buckets.nth(hash_code);

            let mut new_chain = ArraySeqStPerS::empty();
            let mut was_deleted = false;

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if &pair.0 != key {
                    let single_seq = ArraySeqStPerS::singleton(pair.clone());
                    new_chain = ArraySeqStPerS::append(&new_chain, &single_seq);
                } else {
                    was_deleted = true;
                }
            }

            if !was_deleted {
                return (self.clone(), false);
            }

            let mut new_buckets = ArraySeqStPerS::empty();
            for i in 0..table_size {
                let bucket_to_use = if i == hash_code {
                    new_chain.clone()
                } else {
                    self.buckets.nth(i).clone()
                };
                let single_seq = ArraySeqStPerS::singleton(bucket_to_use);
                new_buckets = ArraySeqStPerS::append(&new_buckets, &single_seq);
            }

            let mut result = NestedHashTable {
                buckets: new_buckets,
                num_elements: self.num_elements - 1,
                load_factor_manager: self.load_factor_manager.clone(),
            };

            // Check resize down
            if result
                .load_factor_manager
                .should_shrink(result.num_elements, table_size)
            {
                let new_size = result.load_factor_manager.shrink_size(table_size);
                result = result.resize(new_size);
            }

            (result, true)
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        pub fn resize(&self, new_size: N) -> Self
        where
            K: std::hash::Hash,
        {
            let actual_new_size = new_size.max(8);
            let mut new_table = Self::create_table(actual_new_size);

            // Rehash all elements
            for i in 0..self.buckets.length() {
                let chain = self.buckets.nth(i);
                for j in 0..chain.length() {
                    let pair = chain.nth(j);
                    new_table = new_table.insert_without_resize(pair.0.clone(), pair.1.clone());
                }
            }

            new_table
        }

        fn insert_without_resize(&self, key: K, value: V) -> Self
        where
            K: std::hash::Hash,
        {
            let hash_fn = DefaultHashFunction;
            let table_size = self.buckets.length();
            let hash_code = hash_fn.hash(&key, table_size);
            let chain = self.buckets.nth(hash_code);

            let mut new_chain = ArraySeqStPerS::empty();
            let mut key_existed = false;

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if pair.0 != key {
                    let single_seq = ArraySeqStPerS::singleton(pair.clone());
                    new_chain = ArraySeqStPerS::append(&new_chain, &single_seq);
                } else {
                    key_existed = true;
                }
            }

            let new_pair = Pair(key, value);
            let new_head = ArraySeqStPerS::singleton(new_pair);
            new_chain = ArraySeqStPerS::append(&new_head, &new_chain);

            let mut new_buckets = ArraySeqStPerS::empty();
            for i in 0..table_size {
                let bucket_to_use = if i == hash_code {
                    new_chain.clone()
                } else {
                    self.buckets.nth(i).clone()
                };
                let single_seq = ArraySeqStPerS::singleton(bucket_to_use);
                new_buckets = ArraySeqStPerS::append(&new_buckets, &single_seq);
            }

            let new_num_elements = if key_existed {
                self.num_elements
            } else {
                self.num_elements + 1
            };

            NestedHashTable {
                buckets: new_buckets,
                num_elements: new_num_elements,
                load_factor_manager: self.load_factor_manager.clone(),
            }
        }

        /// Claude Work: Θ(1), Span: Θ(1)
        pub fn load_and_size(&self) -> (N, N) { (self.num_elements, self.buckets.length()) }

        pub fn size(&self) -> N { self.num_elements }

        pub fn is_empty(&self) -> bool { self.num_elements == 0 }

        pub fn load_factor(&self) -> f64 {
            self.load_factor_manager
                .load_factor(self.num_elements, self.buckets.length())
        }

        pub fn statistics(&self) -> HashTableStats {
            let mut total_collisions = 0;
            let mut max_chain_length = 0;
            let mut non_empty_buckets = 0;
            let mut total_chain_length = 0;

            for i in 0..self.buckets.length() {
                let chain = self.buckets.nth(i);
                let chain_length = chain.length();

                if chain_length > 0 {
                    non_empty_buckets += 1;
                    total_chain_length += chain_length;

                    if chain_length > 1 {
                        total_collisions += chain_length - 1;
                    }

                    max_chain_length = max_chain_length.max(chain_length);
                }
            }

            let avg_chain_length = if non_empty_buckets > 0 {
                total_chain_length as f64 / non_empty_buckets as f64
            } else {
                0.0
            };

            HashTableStats::new(self.num_elements, self.buckets.length()).with_collision_stats(
                total_collisions,
                max_chain_length,
                avg_chain_length,
            )
        }
    }

    impl<K: StT + Display, V: StT + Display> Display for NestedHashTable<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            writeln!(f, "NestedHashTable {{")?;
            writeln!(
                f,
                "  size: {}, load_factor: {:.3}",
                self.num_elements,
                self.load_factor()
            )?;

            for i in 0..self.buckets.length() {
                let chain = self.buckets.nth(i);

                if chain.length() > 0 {
                    write!(f, "  bucket[{}]: ", i)?;
                    for j in 0..chain.length() {
                        let pair = chain.nth(j);
                        if j > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "({} → {})", pair.0, pair.1)?;
                    }
                    writeln!(f)?;
                }
            }

            write!(f, "}}")
        }
    }

    /// Type aliases for common configurations
    pub type StringNestedHashTable<V> = NestedHashTable<String, V>;
    pub type IntegerNestedHashTable<V> = NestedHashTable<i32, V>;

    /// Constructor functions for common nested hash table configurations
    /// APAS: Work Θ(1), Span Θ(1)
    pub fn create_string_nested_hash_table<V: StT>(initial_size: N) -> StringNestedHashTable<V> {
        NestedHashTable::create_table(initial_size)
    }

    /// APAS: Work Θ(1), Span Θ(1)
    pub fn create_integer_nested_hash_table<V: StT>(initial_size: N) -> IntegerNestedHashTable<V> {
        NestedHashTable::create_table(initial_size)
    }

    /// Macro for creating nested hash tables with initial data
    #[macro_export]
    macro_rules! NestedHashTableLit {
        ($size:expr) => {
            NestedHashTable::create_table($size)
        };
        ($size:expr, $(($key:expr, $value:expr)),* $(,)?) => {{
            let mut table = NestedHashTable::create_table($size);
            $(
                table = table.insert($key, $value);
            )*
            table
        }};
    }

    #[allow(dead_code)]
    fn _nested_hash_table_lit_type_checks() {
        let _: StringNestedHashTable<i32> = NestedHashTableLit!(8, ("key1".to_string(), 1), ("key2".to_string(), 2));
    }
}
