//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Parallel Divide and Conquer (Chapter 28, Algorithm 28.17).
//!
//! Historical Note: Based on the divide-and-conquer algorithm first designed by Michael Shamos
//! of Carnegie Mellon University CS in 1977. This parallel version uses ParaPair! for
//! unconditional parallelism with 32MB stack per thread.

pub mod MaxContigSubSumDivConMtEph {

    use std::sync::Arc;

    use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap27::ScanContractMtEph::ScanContractMtEph::ScanContractMtEphTrait;
    use crate::ParaPair;
    use crate::Types::Types::*;

    fn max_with_neginf(a: Option<i32>, b: Option<i32>) -> Option<i32> {
        match (a, b) {
            | (None, None) => None,
            | (None, Some(_)) => b,
            | (Some(_), None) => a,
            | (Some(x), Some(y)) => Some(x.max(y)),
        }
    }

    /// Helper to find max suffix sum using inclusive prefix sums.
    fn max_suffix_sum(a: &ArraySeqMtEphS<i32>) -> i32 {
        if a.length() == 0 {
            return i32::MIN / 2; // treat as -∞
        }

        let mut min_prefix = 0;
        let mut running_sum = 0;
        for i in 0..a.length() {
            running_sum += a.nth_cloned(i);
            min_prefix = min_prefix.min(running_sum);
        }
        let total = running_sum;

        total - min_prefix
    }

    /// Helper to find max prefix sum using inclusive prefix sums.
    fn max_prefix_sum(a: &ArraySeqMtEphS<i32>) -> i32 {
        if a.length() == 0 {
            return i32::MIN / 2; // treat as -∞
        }

        // Note: Start with first element (not empty prefix) since empty sequence is not allowed
        let mut max_val = a.nth_cloned(0);
        let mut running_sum = a.nth_cloned(0);
        for i in 1..a.length() {
            running_sum += a.nth_cloned(i);
            max_val = max_val.max(running_sum);
        }

        max_val
    }

    /// Trait for parallel divide-and-conquer maximum contiguous subsequence sum.
    pub trait MaxContigSubSumDivConMtTrait {
        /// Compute maximum contiguous subsequence sum using parallel divide-and-conquer.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n log n), Span Θ(log² n)
        /// claude-4-sonet: Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log n)
        /// claude-4-sonnet: Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_divcon_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumDivConMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_divcon_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base cases
            if n == 0 {
                return None; // -∞
            }
            if n == 1 {
                return Some(a.nth_cloned(0));
            }

            // Divide: split at midpoint
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n);

            // Conquer: parallel recursive solve using ParaPair! with 32MB stack
            let left_clone = left.clone();
            let right_clone = right.clone();

            let result_pair = ParaPair!(move || Self::max_contig_sub_sum_divcon_mt(&left_clone), move || {
                Self::max_contig_sub_sum_divcon_mt(&right_clone)
            });
            let max_left = result_pair.0;
            let max_right = result_pair.1;

            // Combine: handle subsequence spanning the cut (parallel suffix/prefix computation)
            let left_for_suffix = left.clone();
            let right_for_prefix = right.clone();

            let crossing_pair = ParaPair!(move || max_suffix_sum(&left_for_suffix), move || max_prefix_sum(
                &right_for_prefix
            ));
            let max_suffix_left = crossing_pair.0;
            let max_prefix_right = crossing_pair.1;

            let max_crossing = max_suffix_left + max_prefix_right;

            // Return maximum of the three cases
            let result = max_with_neginf(max_left, max_right);
            max_with_neginf(result, Some(max_crossing))
        }
    }
}
