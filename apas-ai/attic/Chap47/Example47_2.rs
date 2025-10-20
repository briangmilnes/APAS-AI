//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Example 47.2 - Separate Chaining Hash Table Demonstration

pub mod Example47_2 {

    use crate::ArraySeqStPerSLit;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap47::HashFunctionTraits::HashFunctionTraits::*;
    use crate::Chap47::SeparateChaining::SeparateChaining::*;
    use crate::Types::Types::*;

    /// Trait for Example 47.2 operations
    pub trait Example47_2Trait {
        /// Example 47.2: Create separate chaining hash table with sample data
        /// APAS: Work Θ(n), Span Θ(n) where n is number of insertions
        fn example_47_2_separate_chaining() -> StringSeparateChaining<String>;

        /// Create the specific table used in Example 47.2
        /// APAS: Work Θ(n), Span Θ(n) where n is number of insertions
        fn create_example_47_2_table()      -> StringSeparateChaining<String>;

        /// Run comprehensive demonstration of Example 47.2
        /// APAS: Work Θ(n), Span Θ(n) where n is number of elements
        fn run_example_47_2()               -> String;

        /// Analyze the hash distribution in Example 47.2
        /// APAS: Work Θ(n), Span Θ(n) where n is number of elements
        fn analyze_hash_distribution()      -> String;
    }

    /// Example 47.2: Create separate chaining hash table with sample data
    /// Returns a hash table populated with the data from textbook Example 47.2
    /// APAS: Work Θ(n), Span Θ(n) where n is number of insertions
    pub fn example_47_2_separate_chaining() -> StringSeparateChaining<String> { create_example_47_2_table() }

    /// Create the specific table used in Example 47.2
    /// Populates a separate chaining hash table with strings "aa" through "jj"
    /// APAS: Work Θ(n), Span Θ(n) where n is number of insertions
    pub fn create_example_47_2_table() -> StringSeparateChaining<String> {
        let mut table = SeparateChainingHashTable::create_table(DefaultKeyEquality, StringPositionHashFunction, 5);

        // Example 47.2 data - strings that demonstrate hash collisions
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

    /// Run comprehensive demonstration of Example 47.2
    /// Shows the separate chaining hash table structure and statistics
    /// APAS: Work Θ(n), Span Θ(n) where n is number of elements
    pub fn run_example_47_2() -> String {
        let mut output = String::new();

        output.push_str("=== Example 47.2: Separate Chaining Hash Table ===\n\n");
        output.push_str("Hash Function: h(x) = (Σ pos(x[i])) mod m\n");
        output.push_str("Table Size: 5\n");
        output.push_str("Collision Resolution: Separate Chaining\n\n");

        let table = example_47_2_separate_chaining();
        let stats = table.statistics();

        output.push_str("Table Contents:\n");
        output.push_str("Keys: aa, bb, cc, dd, ee, ff, gg, hh, ii, jj\n");
        output.push_str("Values: a, b, c, d, e, f, g, h, i, j\n\n");

        output.push_str("Hash Table Statistics:\n");
        output.push_str(&format!("  {}\n\n", stats));

        output.push_str(&analyze_hash_distribution());

        output.push_str("\nAnalysis:\n");
        output.push_str("- Separate chaining handles collisions by maintaining linked lists\n");
        output.push_str("- Each bucket can store multiple key-value pairs\n");
        output.push_str("- Load factor α = n/m = 10/5 = 2.0 (high but manageable)\n");
        output.push_str("- Expected chain length is α = 2.0\n");
        output.push_str("- Demonstrates collision resolution without probe sequences\n");

        output
    }

    /// Analyze the hash distribution in Example 47.2
    /// Shows which keys hash to which buckets
    /// APAS: Work Θ(n), Span Θ(n) where n is number of elements
    pub fn analyze_hash_distribution() -> String {
        let mut output = String::new();
        let hash_fn = StringPositionHashFunction;
        let table_size = 5;

        output.push_str("Hash Distribution Analysis:\n");

        let test_keys = ArraySeqStPerSLit![
            "aa".to_string(),
            "bb".to_string(),
            "cc".to_string(),
            "dd".to_string(),
            "ee".to_string(),
            "ff".to_string(),
            "gg".to_string(),
            "hh".to_string(),
            "ii".to_string(),
            "jj".to_string()
        ];

        // Group keys by hash value
        let mut buckets: ArraySeqStPerS<ArraySeqStPerS<String>> =
            ArraySeqStPerS::tabulate(table_size, |_| ArraySeqStPerS::empty());

        for i in 0..test_keys.length() {
            let key = test_keys.nth(i);
            let hash_value = hash_fn.hash(&key, table_size);
            let current_bucket = buckets.nth(hash_value);
            let updated_bucket = current_bucket.append(&ArraySeqStPerS::singleton(key));
            buckets = buckets.update(hash_value, updated_bucket);
        }

        for bucket_idx in 0..buckets.length() {
            let bucket = buckets.nth(bucket_idx);
            output.push_str(&format!("  Bucket {}: ", bucket_idx));

            if bucket.length() == 0 {
                output.push_str("(empty)\n");
            } else {
                for j in 0..bucket.length() {
                    if j > 0 {
                        output.push_str(" -> ");
                    }
                    output.push_str(&format!("'{}'", bucket.nth(j)));
                }
                output.push_str(&format!(" ({} items)\n", bucket.length()));
            }
        }

        output
    }
}
