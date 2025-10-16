//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Double Hashing Flat Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses double hashing for open addressing collision resolution.

pub mod DoubleHashFlatHashTableStEph {

    use crate::Chap47clean::FlatHashTable::FlatHashTable::*;
    use crate::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Types::Types::*;

    /// Double Hashing Flat Hash Table implementation.
    /// Probe sequence: h_i(k) = (h(k) + i·hh(k)) mod m
    /// Uses two hash functions to avoid both primary and secondary clustering.
    pub struct DoubleHashFlatHashTableStEph;

    impl DoubleHashFlatHashTableStEph {
        /// Compute second hash value for double hashing.
        /// APAS: hh(k) must be relatively prime to m.
        /// Strategy: Always return an odd number (works for power-of-2 sizes),
        /// and for prime sizes, ensure < m and non-zero.
        pub fn second_hash<Key: StT>(key: &Key, table_size: N) -> N {
            // APAS: hh(k) must be non-zero and coprime to m
            // Use key bytes directly instead of memory address

            if table_size <= 2 {
                return 1;
            }

            // Compute hash from key bytes using FNV-1a algorithm
            let key_ptr = key as *const Key as *const u8;
            let key_size = std::mem::size_of::<Key>();
            let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis

            unsafe {
                for i in 0..key_size {
                    let byte = *key_ptr.add(i);
                    hash ^= byte as u64;
                    hash = hash.wrapping_mul(0x100000001b3); // FNV prime
                }
            }

            // For prime table sizes, use: step = 1 + (hash % (m-1))
            // This ensures 1 <= step < m and step != 0
            let base = (table_size - 1) as u64;
            let mut step = ((hash % base) + 1) as N;

            // Ensure odd for power-of-2 coprimality
            if step % 2 == 0 && step < table_size - 1 {
                step += 1;
            }

            step
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> ParaHashTableStEphTrait<Key, Value, FlatEntry<Key, Value>, Metrics>
        for DoubleHashFlatHashTableStEph
    {
        fn insert(table: &mut HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: Key, value: Value) {
            let slot = Self::find_slot(table, &key);
            match &table.table[slot] {
                | FlatEntry::Occupied(k, _) if k == &key => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                }
                | FlatEntry::Empty | FlatEntry::Deleted => {
                    table.table[slot] = FlatEntry::Occupied(key, value);
                    table.num_elements += 1;
                }
                | _ => {
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
                    | FlatEntry::Occupied(k, v) if k == key => return Some(v.clone()),
                    | FlatEntry::Empty => return None,
                    | FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
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
                    | FlatEntry::Occupied(k, _) if k == key => {
                        table.table[slot] = FlatEntry::Deleted;
                        table.num_elements -= 1;
                        return true;
                    }
                    | FlatEntry::Empty => return false,
                    | FlatEntry::Deleted | FlatEntry::Occupied(_, _) => {
                        attempt += 1;
                    }
                }
            }
            false
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> FlatHashTable<Key, Value, FlatEntry<Key, Value>, Metrics>
        for DoubleHashFlatHashTableStEph
    {
        fn probe(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key, attempt: N) -> N {
            let hash1 = (table.hash_fn)(key);
            let step = Self::second_hash(key, table.current_size);

            // Double hashing: (h1(key) + i·h2(key)) mod size
            (hash1 + (attempt * step)) % table.current_size
        }

        fn find_slot(table: &HashTable<Key, Value, FlatEntry<Key, Value>, Metrics>, key: &Key) -> N {
            let mut attempt = 0;
            while attempt < table.current_size {
                let slot = Self::probe(table, key, attempt);
                match &table.table[slot] {
                    | FlatEntry::Empty | FlatEntry::Deleted => return slot,
                    | FlatEntry::Occupied(k, _) if k == key => return slot,
                    | _ => attempt += 1,
                }
            }
            Self::probe(table, key, 0)
        }
    }
}
