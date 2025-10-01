//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Top-Down Dynamic Programming - Ephemeral Single-Threaded Implementation
//!
//! This module implements the top-down (memoization) approach to dynamic programming
//! using HashMap with in-place mutations for efficient subproblem caching.

pub mod TopDownDPStEph {

    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};

    /// Ephemeral single-threaded top-down DP structure (struct product record type)
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct TopDownDPStEphS {
        /// Input sequence S
        seq_s: ArraySeqStEphS<char>,
        /// Input sequence T  
        seq_t: ArraySeqStEphS<char>,
        /// Memoization table for subproblem results
        memo_table: HashMap<(usize, usize), usize>,
    }

    impl TopDownDPStEphS {
        /// Claude Work: O(1) - constant time initialization
        /// Claude Span: O(1) - constant time initialization
        pub fn new(s: ArraySeqStEphS<char>, t: ArraySeqStEphS<char>) -> Self {
            TopDownDPStEphS {
                seq_s: s,
                seq_t: t,
                memo_table: HashMap::new(),
            }
        }

        /// Compute minimum edit distance using top-down memoization
        /// Claude Work: O(|S|*|T|) where |S|=source length, |T|=target length
        /// Claude Span: O(|S|+|T|) - sequential recursive calls with memoization
        pub fn med_memoized(&mut self) -> usize {
            let s_len = self.seq_s.length();
            let t_len = self.seq_t.length();

            self.med_recursive(s_len, t_len)
        }

        /// Recursive helper with memoization
        /// Claude Work: O(1) per call with memoization, O(|S|*|T|) total
        /// Claude Span: O(|S|+|T|) - depth of recursion
        fn med_recursive(&mut self, i: usize, j: usize) -> usize {
            // Check memo table first
            if let Some(&cached_result) = self.memo_table.get(&(i, j)) {
                return cached_result;
            }

            // Base cases
            let result = match (i, j) {
                | (0, j) => j, // Insert all remaining characters from T
                | (i, 0) => i, // Delete all remaining characters from S
                | (i, j) => {
                    let s_char = *self.seq_s.nth(i - 1);
                    let t_char = *self.seq_t.nth(j - 1);

                    if s_char == t_char {
                        // Characters match: no edit needed
                        self.med_recursive(i - 1, j - 1)
                    } else {
                        // Characters don't match: try insert, delete, or substitute
                        let insert_cost = 1 + self.med_recursive(i, j - 1);
                        let delete_cost = 1 + self.med_recursive(i - 1, j);
                        let substitute_cost = 1 + self.med_recursive(i - 1, j - 1);

                        insert_cost.min(delete_cost).min(substitute_cost)
                    }
                }
            };

            // Store result in memo table
            self.memo_table.insert((i, j), result);
            result
        }

        /// Get current memoization table size
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn memo_size(&self) -> usize { self.memo_table.len() }

        /// Check if subproblem is memoized
        /// Claude Work: O(1) - constant time lookup
        /// Claude Span: O(1) - constant time lookup
        pub fn is_memoized(&self, i: usize, j: usize) -> bool { self.memo_table.contains_key(&(i, j)) }

        /// Get memoized result if available
        /// Claude Work: O(1) - constant time lookup
        /// Claude Span: O(1) - constant time lookup
        pub fn get_memoized(&self, i: usize, j: usize) -> Option<usize> { self.memo_table.get(&(i, j)).copied() }

        /// Insert memoized result
        /// Claude Work: O(1) - constant time insertion
        /// Claude Span: O(1) - constant time insertion
        pub fn insert_memo(&mut self, i: usize, j: usize, value: usize) { self.memo_table.insert((i, j), value); }

        /// Get the length of sequence S
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn s_length(&self) -> usize { self.seq_s.length() }

        /// Get the length of sequence T
        /// Claude Work: O(1) - constant time access
        /// Claude Span: O(1) - constant time access
        pub fn t_length(&self) -> usize { self.seq_t.length() }

        /// Check if sequences are empty
        /// Claude Work: O(1) - constant time check
        /// Claude Span: O(1) - constant time check
        pub fn is_empty(&self) -> bool { self.seq_s.length() == 0usize && self.seq_t.length() == 0usize }

        /// Clear memoization table
        /// Claude Work: O(1) - constant time clear
        /// Claude Span: O(1) - constant time clear
        pub fn clear_memo(&mut self) { self.memo_table.clear(); }

        /// Mutably update sequence S
        /// Claude Work: O(1) - constant time update
        /// Claude Span: O(1) - constant time update
        pub fn set_s(&mut self, s: ArraySeqStEphS<char>) {
            self.seq_s = s;
            self.clear_memo(); // Clear memo when sequences change
        }

        /// Mutably update sequence T
        /// Claude Work: O(1) - constant time update
        /// Claude Span: O(1) - constant time update
        pub fn set_t(&mut self, t: ArraySeqStEphS<char>) {
            self.seq_t = t;
            self.clear_memo(); // Clear memo when sequences change
        }
    }

    impl Default for TopDownDPStEphS {
        fn default() -> Self {
            let empty_s = ArraySeqStEphS::new(0, ' ');
            let empty_t = ArraySeqStEphS::new(0, ' ');
            Self::new(empty_s, empty_t)
        }
    }

    impl Display for TopDownDPStEphS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "TopDownDPStEph(s_len={}, t_len={}, memo_size={})",
                self.s_length(),
                self.t_length(),
                self.memo_size()
            )
        }
    }
}
