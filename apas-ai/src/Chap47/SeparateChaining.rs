//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Separate Chaining implementation - Definition 47.3

pub mod SeparateChaining {

use std::fmt::{Debug, Display};

use crate::Types::Types::*;
use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    #[derive(Clone, Debug)]
    pub struct SeparateChainingHashTable<K, V, H, E>
    where
        K: StT,
        V: StT,
        H: HashFunction<K> + Clone,
        E: KeyEquality<K> + Clone,
    {
        buckets: ArraySeqStPerS<ArraySeqStPerS<Pair<K, V>>>,
        hash_function: H,
        key_equality: E,
        num_elements: N,
        load_factor_manager: LoadFactorManager,
    }

    impl<K, V, H, E> SeparateChainingHashTable<K, V, H, E>
    where
        K: StT,
        V: StT,
        H: HashFunction<K> + Clone,
        E: KeyEquality<K> + Clone,
    {
        /// Claude Work: Θ(m), Span: Θ(m)
        pub fn create_table(eq_fn: E, hash_fn: H, initial_size: N) -> Self {
            let size = initial_size.max(8);
            let mut buckets = ArraySeqStPerS::empty();

            for _ in 0..size {
                let empty_chain = ArraySeqStPerS::empty();
                let single_seq = ArraySeqStPerS::singleton(empty_chain);
                buckets = ArraySeqStPerS::append(&buckets, &single_seq);
            }

            SeparateChainingHashTable {
                buckets,
                hash_function: hash_fn,
                key_equality: eq_fn,
                num_elements: 0,
                load_factor_manager: LoadFactorManager::new(0.75, 0.25),
            }
        }

        /// Claude Work: Θ(1) expected, Span: Θ(1) expected
        /// Insert at head of chain - constant time
        pub fn insert(&self, key: K, value: V) -> Self {
            let table_size = self.buckets.length();
            let hash_code = self.hash_function.hash(&key, table_size);
            let chain = self.buckets.nth(hash_code);

            // Check if key exists and remove old entry
            let mut new_chain = ArraySeqStPerS::empty();
            let mut key_existed = false;

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if !self.key_equality.eq(&pair.0, &key) {
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

            let mut result = SeparateChainingHashTable {
                buckets: new_buckets,
                hash_function: self.hash_function.clone(),
                key_equality: self.key_equality.clone(),
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
        /// Linear search through chain
        pub fn lookup(&self, key: &K) -> Option<&V> {
            let table_size = self.buckets.length();
            let hash_code = self.hash_function.hash(key, table_size);
            let chain = self.buckets.nth(hash_code);

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if self.key_equality.eq(&pair.0, key) {
                    return Some(&pair.1);
                }
            }

            None
        }

        /// Claude Work: Θ(1 + α) expected, Span: Θ(1 + α) expected
        pub fn delete(&self, key: &K) -> (Self, bool) {
            let table_size = self.buckets.length();
            let hash_code = self.hash_function.hash(key, table_size);
            let chain = self.buckets.nth(hash_code);

            let mut new_chain = ArraySeqStPerS::empty();
            let mut was_deleted = false;

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if !self.key_equality.eq(&pair.0, key) {
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

            let mut result = SeparateChainingHashTable {
                buckets: new_buckets,
                hash_function: self.hash_function.clone(),
                key_equality: self.key_equality.clone(),
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
        pub fn resize(&self, new_size: N) -> Self {
            let actual_new_size = new_size.max(8);
            let mut new_table =
                Self::create_table(self.key_equality.clone(), self.hash_function.clone(), actual_new_size);

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

        fn insert_without_resize(&self, key: K, value: V) -> Self {
            let table_size = self.buckets.length();
            let hash_code = self.hash_function.hash(&key, table_size);
            let chain = self.buckets.nth(hash_code);

            let mut new_chain = ArraySeqStPerS::empty();
            let mut key_existed = false;

            for i in 0..chain.length() {
                let pair = chain.nth(i);
                if !self.key_equality.eq(&pair.0, &key) {
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

            SeparateChainingHashTable {
                buckets: new_buckets,
                hash_function: self.hash_function.clone(),
                key_equality: self.key_equality.clone(),
                num_elements: new_num_elements,
                load_factor_manager: self.load_factor_manager.clone(),
            }
        }

        pub fn load_and_size(&self) -> (N, N) { (self.num_elements, self.buckets.length()) }

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

    /// Type aliases for common configurations
    pub type StringSeparateChaining<V> =
        SeparateChainingHashTable<String, V, StringPositionHashFunction, DefaultKeyEquality>;

    pub fn create_example_47_2_table() -> StringSeparateChaining<String> {
        let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 5);

        // Example 47.2 data
        table = table.insert("aa".to_string(), "a".to_string());
        table = table.insert("bb".to_string(), "b".to_string());
        table = table.insert("cc".to_string(), "c".to_string());
        table = table.insert("dd".to_string(), "d".to_string());
        table = table.insert("ee".to_string(), "e".to_string());
        table = table.insert("ff".to_string(), "f".to_string());
        table = table.insert("gg".to_string(), "g".to_string());
        table = table.insert("hh".to_string(), "h".to_string());
        table = table.insert("ii".to_string(), "i".to_string());
        table = table.insert("jj".to_string(), "j".to_string());

        table
    }
}
