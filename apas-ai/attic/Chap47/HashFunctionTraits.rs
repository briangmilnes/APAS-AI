//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Hash Function Traits and Utilities

pub mod HashFunctionTraits {

    use std::collections::hash_map::DefaultHasher;
    use std::fmt::Formatter;
    use std::fmt::{Debug, Display};
    use std::hash::{Hash, Hasher};
    use std::marker::PhantomData;
    use std::time::Duration;

    use crate::Types::Types::*;
    pub trait HashFunction<K> {
        /// claude-4-sonet: Work Θ(|key|), Span Θ(|key|), Parallelism Θ(1)
        /// Maps a key to a hash code in range [0, table_size)
        fn hash(&self, key: &K, table_size: N) -> N;

        /// Get a description of this hash function
        fn description(&self)                  -> String;
    }

    /// Trait for key equality testing
    /// Required by Definition 47.1 for hash table operations
    pub trait KeyEquality<K> {
        /// claude-4-sonet: Work Θ(|key|), Span Θ(|key|), Parallelism Θ(1)
        /// Tests if two keys are equal
        fn eq(&self, a: &K, b: &K) -> bool;
    }

    /// Universal hash function family generator
    /// Provides theoretical guarantees for collision analysis
    pub trait UniversalHashFamily<K> {
        type HashFn: HashFunction<K>;

        /// Generate a random hash function from the universal family
        fn generate(&self, seed: u64) -> Self::HashFn;

        /// Get the family description
        fn family_description(&self)  -> String;
    }

    /// Simple hash function using Rust's built-in hasher
    #[derive(Clone, Debug)]
    pub struct DefaultHashFunction;

    impl<K: Hash> HashFunction<K> for DefaultHashFunction {
        fn hash(&self, key: &K, table_size: N) -> N {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            (hasher.finish() as N) % table_size
        }

        fn description(&self) -> String { "DefaultHashFunction (Rust built-in)".to_string() }
    }

    /// String hash function from Example 47.1 in textbook
    /// h(x) = (Σ pos(x[i])) mod m
    #[derive(Clone, Debug)]
    pub struct StringPositionHashFunction;

    impl HashFunction<String> for StringPositionHashFunction {
        /// Claude Work: Θ(|string|), Span: Θ(|string|)
        /// Example 47.1: Sum character positions modulo table size
        fn hash(&self, key: &String, table_size: N) -> N {
            let mut sum = 0;
            for ch in key.chars() {
                // Convert character to position (a=0, b=1, ..., z=25)
                if ch.is_ascii_lowercase() {
                    sum += (ch as u8 - b'a') as N;
                } else if ch.is_ascii_uppercase() {
                    sum += (ch as u8 - b'A') as N;
                }
            }
            sum % table_size
        }

        fn description(&self) -> String { "StringPositionHashFunction (Example 47.1)".to_string() }
    }

    /// Polynomial rolling hash function for strings
    /// h(x) = (Σ x[i] * base^i) mod m
    #[derive(Clone, Debug)]
    pub struct PolynomialHashFunction {
        base: N,
    }

    impl PolynomialHashFunction {
        pub fn new(base: N) -> Self { PolynomialHashFunction { base } }
    }

    impl HashFunction<String> for PolynomialHashFunction {
        /// Claude Work: Θ(|string|), Span: Θ(|string|)
        fn hash(&self, key: &String, table_size: N) -> N {
            let mut hash_value = 0;
            let mut power = 1;

            for ch in key.chars() {
                hash_value = (hash_value + (ch as N) * power) % table_size;
                power = (power * self.base) % table_size;
            }

            hash_value
        }

        fn description(&self) -> String { format!("PolynomialHashFunction (base={})", self.base) }
    }

    /// Universal hash function for integers
    /// h_a,b(x) = ((ax + b) mod p) mod m
    /// where p is a large prime and a, b are random
    #[derive(Clone, Debug)]
    pub struct UniversalIntegerHashFunction {
        a: u64,
        b: u64,
        p: u64, // Large prime
    }

    impl UniversalIntegerHashFunction {
        pub fn new(a: u64, b: u64) -> Self {
            const LARGE_PRIME: u64 = 1000000007; // 10^9 + 7
            UniversalIntegerHashFunction { a, b, p: LARGE_PRIME }
        }
    }

    impl HashFunction<i32> for UniversalIntegerHashFunction {
        /// Claude Work: Θ(1), Span: Θ(1)
        fn hash(&self, key: &i32, table_size: N) -> N {
            let x = *key as u64;
            let hash_value = ((self.a * x + self.b) % self.p) as N;
            hash_value % table_size
        }

        fn description(&self) -> String {
            format!(
                "UniversalIntegerHashFunction (a={}, b={}, p={})",
                self.a, self.b, self.p
            )
        }
    }

    /// Default key equality using Rust's PartialEq
    #[derive(Clone, Debug)]
    pub struct DefaultKeyEquality;

    impl<K: PartialEq> KeyEquality<K> for DefaultKeyEquality {
        fn eq(&self, a: &K, b: &K) -> bool { a == b }
    }

    /// Case-insensitive string equality
    #[derive(Clone, Debug)]
    pub struct CaseInsensitiveStringEquality;

    impl KeyEquality<String> for CaseInsensitiveStringEquality {
        fn eq(&self, a: &String, b: &String) -> bool { a.to_lowercase() == b.to_lowercase() }
    }

    /// Universal hash family for integers
    pub struct UniversalIntegerHashFamily {
        p: u64,
    }


    impl UniversalIntegerHashFamily {
        pub fn new() -> Self {
            const LARGE_PRIME: u64 = 1000000007;
            UniversalIntegerHashFamily { p: LARGE_PRIME }
        }
    }

    impl UniversalHashFamily<i32> for UniversalIntegerHashFamily {
        type HashFn = UniversalIntegerHashFunction;

        fn generate(&self, seed: u64) -> Self::HashFn {
            // Use seed to generate pseudo-random a and b
            let a = (seed * 1103515245 + 12345) % (self.p - 1) + 1; // Ensure a != 0
            let b = (a * 1103515245 + 12345) % self.p;
            UniversalIntegerHashFunction::new(a, b)
        }

        fn family_description(&self) -> String { format!("UniversalIntegerHashFamily (p={})", self.p) }
    }

    /// Hash function combiner for creating probe sequences
    /// Used in flat hash tables for open addressing
    pub struct ProbeSequenceGenerator<K, H1: HashFunction<K>, H2: HashFunction<K>> {
        hash1: H1,
        hash2: H2,
        _phantom: PhantomData<K>,
    }

    impl<K, H1: HashFunction<K>, H2: HashFunction<K>> ProbeSequenceGenerator<K, H1, H2> {
        pub fn new(hash1: H1, hash2: H2) -> Self {
            ProbeSequenceGenerator {
                hash1,
                hash2,
                _phantom: PhantomData,
            }
        }

        /// Generate the i-th hash value in the probe sequence
        /// For double hashing: h_i(x) = (h1(x) + i * h2(x)) mod m
        pub fn probe_hash(&self, key: &K, probe_index: N, table_size: N) -> N {
            let h1 = self.hash1.hash(key, table_size);
            let h2 = self.hash2.hash(key, table_size);
            (h1 + probe_index * h2) % table_size
        }
    }

    /// Load factor calculation and management
    /// Definition 47.2: α = n/m
    #[derive(Clone, Debug)]
    pub struct LoadFactorManager {
        max_load_factor: f64,
        min_load_factor: f64,
    }

    impl LoadFactorManager {
        pub fn new(max_load: f64, min_load: f64) -> Self {
            LoadFactorManager {
                max_load_factor: max_load,
                min_load_factor: min_load,
            }
        }

        /// Calculate current load factor
        pub fn load_factor(&self, num_elements: N, table_size: N) -> f64 {
            if table_size == 0 {
                0.0
            } else {
                num_elements as f64 / table_size as f64
            }
        }

        /// Check if table should be resized up
        pub fn should_grow(&self, num_elements: N, table_size: N) -> bool {
            self.load_factor(num_elements, table_size) > self.max_load_factor
        }

        /// Check if table should be resized down
        pub fn should_shrink(&self, num_elements: N, table_size: N) -> bool {
            table_size > 8 && // Don't shrink below minimum size
            self.load_factor(num_elements, table_size) < self.min_load_factor
        }

        /// Calculate new table size for growth
        pub fn grow_size(&self, current_size: N) -> N { current_size * 2 }

        /// Calculate new table size for shrinkage
        pub fn shrink_size(&self, current_size: N) -> N { (current_size / 2).max(8) }
    }

    /// Hash table statistics for analysis
    #[derive(Debug, Clone)]
    pub struct HashTableStats {
        pub num_elements: N,
        pub table_size: N,
        pub load_factor: f64,
        pub num_collisions: N,
        pub max_chain_length: N,
        pub avg_chain_length: f64,
    }

    impl HashTableStats {
        pub fn new(num_elements: N, table_size: N) -> Self {
            let load_factor = if table_size == 0 {
                0.0
            } else {
                num_elements as f64 / table_size as f64
            };
            HashTableStats {
                num_elements,
                table_size,
                load_factor,
                num_collisions: 0,
                max_chain_length: 0,
                avg_chain_length: 0.0,
            }
        }

        pub fn with_collision_stats(mut self, collisions: N, max_chain: N, avg_chain: f64) -> Self {
            self.num_collisions = collisions;
            self.max_chain_length = max_chain;
            self.avg_chain_length = avg_chain;
            self
        }
    }


    /// Utility functions for hash table implementation
    pub struct HashTableUtils;

    impl HashTableUtils {
        /// Find next prime number >= n (for table sizing)
        pub fn next_prime(n: N) -> N {
            if n <= 2 {
                return 2;
            }
            let mut candidate = if n % 2 == 0 { n + 1 } else { n };

            while !Self::is_prime(candidate) {
                candidate += 2;
            }
            candidate
        }

        /// Check if number is prime
        pub fn is_prime(n: N) -> bool {
            if n < 2 {
                return false;
            }
            if n == 2 {
                return true;
            }
            if n % 2 == 0 {
                return false;
            }

            let sqrt_n = (n as f64).sqrt() as N;
            for i in (3..=sqrt_n).step_by(2) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }

        /// Generate good table sizes (powers of 2 or primes)
        pub fn good_table_size(desired_size: N, use_prime: bool) -> N {
            if use_prime {
                Self::next_prime(desired_size)
            } else {
                // Next power of 2
                if desired_size <= 1 {
                    return 2;
                }
                let mut size = 1;
                while size < desired_size {
                    size *= 2;
                }
                size
            }
        }
    }

    /// Macro for creating hash function implementations
    #[macro_export]
    macro_rules! impl_hash_function {
        ($name:ident, $key_type:ty, $hash_expr:expr, $desc:expr) => {
            #[derive(Clone, Debug)]
            pub struct $name;

            impl HashFunction<$key_type> for $name {
                fn hash(&self, key: &$key_type, table_size: N) -> N {
                    let hash_value = $hash_expr(key);
                    hash_value % table_size
                }

                fn description(&self) -> String { $desc.to_string() }
            }
        };
    }

    // Example usage of the macro
    impl_hash_function!(
        SimpleIntegerHash,
        i32,
        |key: &i32| (*key as N).wrapping_mul(2654435761), // Knuth's multiplicative hash
        "SimpleIntegerHash (Knuth multiplicative)"
    );

    /// Test utilities for hash functions
    pub struct HashFunctionTester;

    impl HashFunctionTester {
        /// Test hash function distribution
        pub fn test_distribution<K, H: HashFunction<K>>(hash_fn: &H, keys: &[K], table_size: N) -> HashTableStats {
            let mut bucket_counts = vec![0; table_size];
            let mut max_count = 0;
            let mut total_collisions = 0;

            for key in keys {
                let hash_value = hash_fn.hash(key, table_size);
                bucket_counts[hash_value] += 1;
                if bucket_counts[hash_value] > 1 {
                    total_collisions += 1;
                }
                max_count = max_count.max(bucket_counts[hash_value]);
            }

            let avg_count = keys.len() as f64 / table_size as f64;

            HashTableStats::new(keys.len(), table_size).with_collision_stats(total_collisions, max_count, avg_count)
        }

        /// Measure hash function performance
        pub fn benchmark_hash_function<K, H: HashFunction<K>>(hash_fn: &H, keys: &[K], table_size: N) -> Duration {
            let start = std::time::Instant::now();
            for key in keys {
                let _ = hash_fn.hash(key, table_size);
            }
            start.elapsed()
        }
    }

    impl Default for UniversalIntegerHashFamily {
        fn default() -> Self { Self::new() }
    }
    impl Display for HashTableStats {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "HashTableStats {{ elements: {}, size: {}, load_factor: {:.3}, collisions: {}, max_chain: {}, avg_chain: {:.2} }}",
                self.num_elements,
                self.table_size,
                self.load_factor,
                self.num_collisions,
                self.max_chain_length,
                self.avg_chain_length
            )
        }
    }

}
