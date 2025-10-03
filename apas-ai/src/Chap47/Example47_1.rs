//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Table Example 47.1 - String Hash Function

pub mod Example47_1 {

    use crate::Types::Types::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;

    pub trait Example47_1Trait {
        /// Example 47.1: String hash function demonstration
        /// APAS: Work Θ(1), Span Θ(1)
        fn example_47_1_hash_function() -> StringPositionHashFunction;

        /// Example 47.1: Demonstrate hash function on sample strings
        /// APAS: Work Θ(n), Span Θ(1)
        fn example_47_1_demonstration() -> Vec<(String, N)>;
    }

    /// Example 47.1: String hash function demonstration
    pub fn example_47_1_hash_function() -> StringPositionHashFunction { StringPositionHashFunction }

    /// Example 47.1: Demonstrate hash function on sample strings
    pub fn example_47_1_demonstration() -> Vec<(String, N)> {
        let hash_fn = example_47_1_hash_function();
        let table_size = 5;

        let test_strings = vec![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "ee".to_string(),
            "ff".to_string(),
            "gg".to_string(),
            "hh".to_string(),
            "ii".to_string(),
            "jj".to_string(),
        ];

        test_strings
            .into_iter()
            .map(|s| {
                let hash_value = hash_fn.hash(&s, table_size);
                (s, hash_value)
            })
            .collect()
    }
}