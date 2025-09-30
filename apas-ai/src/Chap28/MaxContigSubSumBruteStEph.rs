//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Brute Force (Chapter 28, Algorithm 28.8).

pub mod MaxContigSubSumBruteStEph {
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Types::Types::*;

    /// Helper function to compare Option<i32> values, treating None as negative infinity.
    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            | (None, None) => None,
            | (None, Some(_)) => b,
            | (Some(_), None) => a,
            | (Some(x), Some(y)) => Some(x.max(y)),
        }
    }

    /// Trait for brute force maximum contiguous subsequence sum.
    pub trait MaxContigSubSumBruteTrait {
        /// Compute maximum contiguous subsequence sum using brute force.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n³), Span Θ(log n)
        /// claude-4-sonnet: Work Θ(n³), Span Θ(log n), Parallelism Θ(n³/log n)
        fn max_contig_sub_sum_brute(a: &ArraySeqStEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumBruteTrait for ArraySeqStEphS<i32> {
        /// Brute force algorithm for maximum contiguous subsequence sum.
        /// Enumerates all O(n²) contiguous subsequences, computes sum for each O(n),
        /// and selects the maximum.
        /// APAS: Work Θ(n³), Span Θ(log n)
        /// claude-4-sonnet: Work Θ(n³), Span Θ(log n), Parallelism Θ(n³/log n)
        fn max_contig_sub_sum_brute(a: &ArraySeqStEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base case: empty sequence returns None (-∞)
            if n == 0 {
                return None;
            }

            // Generate all contiguous subsequences and their sums
            // For each starting position i and ending position j (i ≤ j)
            let mut max_sum = None;

            for i in 0..n {
                for j in i..n {
                    // Compute sum of subsequence a[i..=j]
                    let mut sum = 0;
                    for k in i..=j {
                        sum += a.nth(k);
                    }
                    max_sum = max_with_neginf(max_sum, Some(sum));
                }
            }

            max_sum
        }
    }
}
