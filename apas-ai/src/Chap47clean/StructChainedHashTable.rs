//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Struct Chained Hash Table - Sequential Ephemeral (Chapter 47).
//! Uses custom linked list struct for separate chaining collision resolution.

pub mod StructChainedHashTable {

    use crate::Chap47clean::ParaHashTableStEph::ParaHashTableStEph::*;
    use crate::Chap47clean::ChainedHashTable::ChainedHashTable::*;
    use crate::Types::Types::*;

    /// Custom linked list node.
    #[derive(Clone, Debug, PartialEq)]
    pub struct Node<Key, Value> {
        pub key: Key,
        pub value: Value,
        pub next: Option<Box<Node<Key, Value>>>,
    }

    /// Custom linked list for chained hash table.
    #[derive(Clone, Debug, PartialEq)]
    pub struct ChainList<Key, Value> {
        pub head: Option<Box<Node<Key, Value>>>,
    }

    impl<Key, Value> ChainList<Key, Value> {
        pub fn new() -> Self { ChainList { head: None } }
    }

    impl<Key: PartialEq + Clone, Value: Clone> EntryTrait<Key, Value> for ChainList<Key, Value> {
        fn new() -> Self { ChainList::new() }

        fn insert(&mut self, key: Key, value: Value) {
            // Search for existing key and update
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.key == key {
                    node.value = value;
                    return;
                }
                current = &mut node.next;
            }
            // Key not found, insert at head
            let new_node = Box::new(Node {
                key,
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        fn lookup(&self, key: &Key) -> Option<Value> {
            let mut current = &self.head;
            while let Some(node) = current {
                if &node.key == key {
                    return Some(node.value.clone());
                }
                current = &node.next;
            }
            None
        }

        fn delete(&mut self, key: &Key) -> B {
            let mut current = &mut self.head;
            loop {
                match current {
                    None => return false,
                    Some(node) if &node.key == key => {
                        *current = node.next.take();
                        return true;
                    }
                    Some(node) => {
                        current = &mut node.next;
                    }
                }
            }
        }
    }

    /// Struct Chained Hash Table implementation.
    pub struct StructChainedHashTableStEph;

    impl<Key: StT, Value: StT, Metrics: Default> 
        ParaHashTableStEphTrait<Key, Value, ChainList<Key, Value>, Metrics> 
        for StructChainedHashTableStEph
    {
        fn insert(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: Key, value: Value) {
            Self::insert_chained(table, key, value);
        }

        fn lookup(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: &Key) -> Option<Value> {
            Self::lookup_chained(table, key)
        }

        fn delete(table: &mut HashTable<Key, Value, ChainList<Key, Value>, Metrics>, key: &Key) -> B {
            Self::delete_chained(table, key)
        }
    }

    impl<Key: StT, Value: StT, Metrics: Default> 
        ChainedHashTable<Key, Value, ChainList<Key, Value>, Metrics> 
        for StructChainedHashTableStEph
    {
        fn hash_index(table: &HashTable<Key, Value, ChainList<Key, Value>, Metrics>, _key: &Key) -> N {
            // Simple modulo hash - implementers can provide better hash function
            let hash_val = 0; // Placeholder: would use actual hash function
            hash_val % table.current_size
        }
    }
}

