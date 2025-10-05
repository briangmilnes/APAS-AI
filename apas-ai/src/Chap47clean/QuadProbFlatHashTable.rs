//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Quadratic Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses quadratic probing for open addressing collision resolution.

pub mod QuadProbFlatHashTable {

    use crate::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Chap47clean::FlatHashTable::FlatHashTable::*;
    use crate::Types::Types::*;

    /// Quadratic Probing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i²) mod m
    pub struct QuadProbFlatHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> 
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics> 
        for QuadProbFlatHashTableStEph
    {
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: Key, value: Value) {
            let slot = Self::find_slot(table, &key);
            match &table.table[slot] {
                FlatEntry::Occupied(k, _) if k == &key => {
                    // Update existing
                    table.table[slot] = FlatEntry::Occupied(key, value);
                }
                FlatEntry::Empty | FlatEntry::Deleted => {
                    // Insert new
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
                _ => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
            }
        }

        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            let mut attempt = 0;
            let max_attempts = (table.current_size + 1) / 2; // APAS Lemma 47.1: first ⌈m/2⌉ probes are distinct
            while attempt < max_attempts {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    FlatEntry::Occupied(k, v) if k == key => return Some(v.clone()),
                    FlatEntry::Empty => return None,
                    FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        attempt += 1;
                    }
                }
            }
            None
        }

        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> B {
            let mut attempt = 0;
            let max_attempts = (table.current_size + 1) / 2; // APAS Lemma 47.1: first ⌈m/2⌉ probes are distinct
            while attempt < max_attempts {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    FlatEntry::Occupied(k, _) if k == key => {
                        table.table[slot] = FlatEntry::Deleted;
                        table.num_elements -= 1;
                        return true;
                    }
                    FlatEntry::Empty => return false,
                    FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        attempt += 1;
                    }
                }
            }
            false
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> 
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics> 
        for QuadProbFlatHashTableStEph
    {
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key, attempt: N) -> N {
            let hash_val = (table.hash_fn)(key);
            
            // Quadratic probing: (hash(key) + i²) mod size
            (hash_val + (attempt * attempt)) % table.current_size
        }

        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> N {
            let mut attempt = 0;
            let max_attempts = (table.current_size + 1) / 2; // APAS Lemma 47.1: first ⌈m/2⌉ probes are distinct
            while attempt < max_attempts {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    FlatEntry::Empty | FlatEntry::Deleted => return slot,
                    FlatEntry::Occupied(k, _) if k == key => return slot,
                    _ => attempt += 1,
                }
            }
            Self::probe(table, key, 0)
        }
    }
}
