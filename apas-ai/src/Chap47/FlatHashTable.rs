//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Flat Hash Table - Data Structure 47.6 Parametric Implementation

pub mod FlatHashTable {
    use std::fmt::{Display, Debug};
    
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Types::Types::*;

    /// Entry types for flat hash table from Data Structure 47.6
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum Entry<K, V> 
    where
        K: StT,
        V: StT,
    {
        Empty,
        Dead,
        Live(K, V),
    }

    impl<K, V> Display for Entry<K, V>
    where
        K: StT,
        V: StT,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Entry::Empty => write!(f, "Empty"),
                Entry::Dead => write!(f, "Dead"),
                Entry::Live(key, value) => write!(f, "Live({}, {})", key, value),
            }
        }
    }

    /// Trait for probe sequence generation
    pub trait ProbeSequence<K> 
    where
        K: StT,
    {
        /// Generate i-th hash value in probe sequence
        fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N;
        
        /// Get description of probing strategy
        fn strategy_name(&self) -> String;
    }

    /// Data Structure 47.6: Parametric Flat Hash Table
    #[derive(Clone, Debug)]
    pub struct FlatHashTable<K, V, P> 
    where
        K: StT,
        V: StT,
        P: ProbeSequence<K>,
    {
        table: ArraySeqStPerS<Entry<K, V>>,
        probe_sequence: P,
        num_elements: N,
        num_deleted: N,
        load_factor_manager: LoadFactorManager,
    }

    impl<K, V, P> FlatHashTable<K, V, P>
    where
        K: StT,
        V: StT,
        P: ProbeSequence<K> + Clone,
    {
        /// Claude Work: Θ(m), Span: Θ(m)
        pub fn create_table(probe_seq: P, initial_size: N) -> Self {
            let size = initial_size.max(8);
            let mut table = ArraySeqStPerS::empty();
            
            for _ in 0..size {
                let empty_entry = Entry::Empty;
                let single_seq = ArraySeqStPerS::singleton(empty_entry);
                table = ArraySeqStPerS::append(&table, &single_seq);
            }
            
            FlatHashTable {
                table,
                probe_sequence: probe_seq,
                num_elements: 0,
                num_deleted: 0,
                load_factor_manager: LoadFactorManager::new(0.5, 0.125), // Lower load factor for open addressing
            }
        }

        /// Claude Work: 1/(1-α) expected, Span: 1/(1-α) expected
        /// Algorithm from Data Structure 47.6
        pub fn insert(&self, key: K, value: V) -> Self {
            let table_size = self.table.length();
            let mut probe_index = 0;
            
            // Find insertion position
            let mut insertion_pos = None;
            while probe_index < table_size {
                let hash_pos = self.probe_sequence.probe_hash(&key, probe_index, table_size);
                let entry = self.table.nth(hash_pos);
                
                match entry {
                    Entry::Empty => {
                        insertion_pos = Some(hash_pos);
                        break;
                    },
                    Entry::Dead => {
                        if insertion_pos.is_none() {
                            insertion_pos = Some(hash_pos);
                        }
                    },
                    Entry::Live(existing_key, _) => {
                        if existing_key == &key {
                            insertion_pos = Some(hash_pos);
                            break;
                        }
                    }
                }
                probe_index += 1;
            }
            
            let pos = insertion_pos.expect("Table should have space for insertion");
            
            // Check if key already exists
            let key_existed = matches!(self.table.nth(pos), Entry::Live(existing_key, _) if existing_key == &key);
            let was_dead = matches!(self.table.nth(pos), Entry::Dead);
            
            // Create new table with updated entry
            let mut new_table = ArraySeqStPerS::empty();
            for i in 0..table_size {
                let entry_to_use = if i == pos {
                    Entry::Live(key.clone(), value.clone())
                } else {
                    self.table.nth(i).clone()
                };
                let single_seq = ArraySeqStPerS::singleton(entry_to_use);
                new_table = ArraySeqStPerS::append(&new_table, &single_seq);
            }
            
            let new_num_elements = if key_existed {
                self.num_elements
            } else {
                self.num_elements + 1
            };
            
            let new_num_deleted = if was_dead {
                self.num_deleted - 1
            } else {
                self.num_deleted
            };
            
            let mut result = FlatHashTable {
                table: new_table,
                probe_sequence: self.probe_sequence.clone(),
                num_elements: new_num_elements,
                num_deleted: new_num_deleted,
                load_factor_manager: self.load_factor_manager.clone(),
            };
            
            // Check resize
            let total_occupied = result.num_elements + result.num_deleted;
            if result.load_factor_manager.should_grow(total_occupied, table_size) {
                let new_size = result.load_factor_manager.grow_size(table_size);
                result = result.resize(new_size);
            }
            
            result
        }

        /// Claude Work: 1/(1-α) expected for unsuccessful, (1/α)ln(1/(1-α)) expected for successful
        /// Algorithm from Data Structure 47.6
        pub fn lookup(&self, key: &K) -> Option<&V> {
            let table_size = self.table.length();
            let mut probe_index = 0;
            
            while probe_index < table_size {
                let hash_pos = self.probe_sequence.probe_hash(key, probe_index, table_size);
                let entry = self.table.nth(hash_pos);
                
                match entry {
                    Entry::Empty => return None,
                    Entry::Dead => {
                        // Continue probing
                    },
                    Entry::Live(existing_key, value) => {
                        if existing_key == key {
                            return Some(value);
                        }
                    }
                }
                probe_index += 1;
            }
            
            None
        }

        /// Claude Work: Similar to lookup, Span: Similar to lookup
        /// Algorithm from Data Structure 47.6
        pub fn delete(&self, key: &K) -> (Self, bool) {
            let table_size = self.table.length();
            let mut probe_index = 0;
            
            // Find key position
            while probe_index < table_size {
                let hash_pos = self.probe_sequence.probe_hash(key, probe_index, table_size);
                let entry = self.table.nth(hash_pos);
                
                match entry {
                    Entry::Empty => return (self.clone(), false),
                    Entry::Dead => {
                        // Continue probing
                    },
                    Entry::Live(existing_key, _) => {
                        if existing_key == key {
                            // Mark as dead
                            let mut new_table = ArraySeqStPerS::empty();
                            for i in 0..table_size {
                                let entry_to_use = if i == hash_pos {
                                    Entry::Dead
                                } else {
                                    self.table.nth(i).clone()
                                };
                                let single_seq = ArraySeqStPerS::singleton(entry_to_use);
                                new_table = ArraySeqStPerS::append(&new_table, &single_seq);
                            }
                            
                            let mut result = FlatHashTable {
                                table: new_table,
                                probe_sequence: self.probe_sequence.clone(),
                                num_elements: self.num_elements - 1,
                                num_deleted: self.num_deleted + 1,
                                load_factor_manager: self.load_factor_manager.clone(),
                            };
                            
                            // Check resize down
                            if result.load_factor_manager.should_shrink(result.num_elements, table_size) {
                                let new_size = result.load_factor_manager.shrink_size(table_size);
                                result = result.resize(new_size);
                            }
                            
                            return (result, true);
                        }
                    }
                }
                probe_index += 1;
            }
            
            (self.clone(), false)
        }

        /// Claude Work: Θ(n), Span: Θ(n)
        pub fn resize(&self, new_size: N) -> Self {
            let actual_new_size = new_size.max(8);
            let mut new_table = Self::create_table(
                self.probe_sequence.clone(),
                actual_new_size
            );
            
            // Rehash all live entries
            for i in 0..self.table.length() {
                let entry = self.table.nth(i);
                if let Entry::Live(key, value) = entry {
                    new_table = new_table.insert_without_resize(key.clone(), value.clone());
                }
            }
            
            new_table
        }

        fn insert_without_resize(&self, key: K, value: V) -> Self {
            let table_size = self.table.length();
            let mut probe_index = 0;
            
            let mut insertion_pos = None;
            while probe_index < table_size {
                let hash_pos = self.probe_sequence.probe_hash(&key, probe_index, table_size);
                let entry = self.table.nth(hash_pos);
                
                match entry {
                    Entry::Empty => {
                        insertion_pos = Some(hash_pos);
                        break;
                    },
                    Entry::Dead => {
                        if insertion_pos.is_none() {
                            insertion_pos = Some(hash_pos);
                        }
                    },
                    Entry::Live(existing_key, _) => {
                        if existing_key == &key {
                            insertion_pos = Some(hash_pos);
                            break;
                        }
                    }
                }
                probe_index += 1;
            }
            
            let pos = insertion_pos.expect("Table should have space");
            let key_existed = matches!(self.table.nth(pos), Entry::Live(existing_key, _) if existing_key == &key);
            let was_dead = matches!(self.table.nth(pos), Entry::Dead);
            
            let mut new_table = ArraySeqStPerS::empty();
            for i in 0..table_size {
                let entry_to_use = if i == pos {
                    Entry::Live(key.clone(), value.clone())
                } else {
                    self.table.nth(i).clone()
                };
                let single_seq = ArraySeqStPerS::singleton(entry_to_use);
                new_table = ArraySeqStPerS::append(&new_table, &single_seq);
            }
            
            let new_num_elements = if key_existed {
                self.num_elements
            } else {
                self.num_elements + 1
            };
            
            let new_num_deleted = if was_dead {
                self.num_deleted - 1
            } else {
                self.num_deleted
            };
            
            FlatHashTable {
                table: new_table,
                probe_sequence: self.probe_sequence.clone(),
                num_elements: new_num_elements,
                num_deleted: new_num_deleted,
                load_factor_manager: self.load_factor_manager.clone(),
            }
        }

        pub fn load_and_size(&self) -> (N, N) {
            (self.num_elements, self.table.length())
        }

        pub fn statistics(&self) -> HashTableStats {
            HashTableStats::new(self.num_elements, self.table.length())
                .with_collision_stats(self.num_deleted, 0, 0.0) // Simplified for flat tables
        }

        pub fn probe_statistics(&self) -> (N, N, f64) {
            // (live_entries, dead_entries, load_factor)
            let load_factor = (self.num_elements + self.num_deleted) as f64 / self.table.length() as f64;
            (self.num_elements, self.num_deleted, load_factor)
        }
    }

    impl<K, V, P> Display for FlatHashTable<K, V, P>
    where
        K: StT + Display,
        V: StT + Display,
        P: ProbeSequence<K> + Clone,
    {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "FlatHashTable ({}) {{", self.probe_sequence.strategy_name())?;
            writeln!(f, "  elements: {}, size: {}, deleted: {}", 
                     self.num_elements, self.table.length(), self.num_deleted)?;
            
            for i in 0..self.table.length() {
                let entry = self.table.nth(i);
                match entry {
                    Entry::Empty => writeln!(f, "  [{}]: Empty", i)?,
                    Entry::Dead => writeln!(f, "  [{}]: Dead", i)?,
                    Entry::Live(key, value) => writeln!(f, "  [{}]: {} → {}", i, key, value)?,
                }
            }
            
            write!(f, "}}")
        }
    }
}
