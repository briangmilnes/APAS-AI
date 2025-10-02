//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Maximum Contiguous Subsequence Sum - Parallel Optimal (Chapter 28, Algorithm 28.16).

pub mod MaxContigSubSumOptMtEph {

use std::sync::Arc;

use crate::Types::Types::*;
use crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
use crate::Chap27::ScanContractMtEph::ScanContractMtEph::ScanContractMtEphTrait;
use crate::ParaPair;
    pub trait MaxContigSubSumOptMtTrait {
        /// Compute maximum contiguous subsequence sum using parallel optimal scan-based algorithm.
        /// Returns None for empty sequence (representing -∞).
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        /// claude-4-sonnet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32>;
    }

    impl MaxContigSubSumOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let n = a.length();

            // Base case: empty sequence returns None (-∞)
            if n == 0 {
                return None;
            }

            // Compute all prefix sums manually (inclusive)
            let mut all_prefixes_vec = Vec::with_capacity(n + 1);
            all_prefixes_vec.push(0); // empty prefix
            let mut running_sum = 0;
            for i in 0..n {
                running_sum += a.nth_cloned(i);
                all_prefixes_vec.push(running_sum);
            }
            let all_prefixes = ArraySeqMtEphS::from_vec(all_prefixes_vec);

            // Compute minimum prefix up to each position (inclusive)
            let mut min_prefixes_vec = Vec::with_capacity(n + 1);
            let mut running_min = i32::MAX;
            for i in 0..=n {
                running_min = running_min.min(all_prefixes.nth_cloned(i));
                min_prefixes_vec.push(running_min);
            }
            let min_prefixes = ArraySeqMtEphS::from_vec(min_prefixes_vec);

            // Compute maximum over all ending positions
            let mut max_sum = None;
            for i in 1..=n {
                let ending_max = all_prefixes.nth_cloned(i) - min_prefixes.nth_cloned(i - 1);
                max_sum = match max_sum {
                    | None => Some(ending_max),
                    | Some(current_max) => Some(current_max.max(ending_max)),
                };
            }

            max_sum
        }
    }
}
