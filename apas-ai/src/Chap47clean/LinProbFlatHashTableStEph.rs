//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Linear Probing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses linear probing for open addressing collision resolution.

pub mod LinProbFlatHashTableStEph {

    use crate::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Chap47clean::FlatHashTable::FlatHashTable::*;
    use crate::Types::Types::*;

    /// Linear Probing Flat Hash Table implementation.
    pub struct LinProbFlatHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> 
        ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics> 
        for LinProbFlatHashTableStEph
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
                    // This shouldn't happen if find_slot works correctly
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
            }
        }

        fn lookup(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    FlatEntry::Occupied(k, v) if k == key => return Some(v.clone()),
                    FlatEntry::Empty => return None, // Stop at Empty - key not in table
                    FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        // Continue probing past Deleted or non-matching Occupied
                        attempt += 1;
                    }
                }
            }
            None
        }

        fn delete(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> B {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    FlatEntry::Occupied(k, _) if k == key => {
                        // Mark as Deleted (tombstone) to maintain probe chain integrity
                        table.table[slot] = FlatEntry::Deleted;
                        table.num_elements -= 1;
                        return true;
                    }
                    FlatEntry::Empty => return false, // Key not found, stop at Empty
                    FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        // Continue probing
                        attempt += 1;
                    }
                }
            }
            false
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> 
        FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics> 
        for LinProbFlatHashTableStEph
    {
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key, attempt: N) -> N {
            let hash_val = (table.hash_fn)(key);
            
            // Linear probing: (hash(key) + attempt) mod size
            (hash_val + attempt) % table.current_size
        }

        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> N {
            // Find first Empty or Deleted slot (can reuse Deleted slots for insertion)
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    FlatEntry::Empty | FlatEntry::Deleted => return slot,
                    FlatEntry::Occupied(k, _) if k == key => return slot, // Update existing key
                    _ => attempt += 1,
                }
            }
            // Table full - return first slot as fallback (shouldn't happen with proper load factor)
            Self::probe(table, key, 0)
        }
    }
}

