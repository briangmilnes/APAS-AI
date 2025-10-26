//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Parallel Persistent (Chapter 35, Algorithm 35.2).
//! Randomized selection algorithm for finding kth order statistic with parallel partition.
//! Work: O(n) expected, Span: O(lg² n) expected.

pub mod OrderStatSelectMtPer {

    use rand::Rng;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;
    pub type T<T> = ArraySeqMtPerS<T>;

    pub trait OrderStatSelectMtPerTrait<T: StTInMtT + Ord> {
        /// claude-4-sonet: Work Θ(n) expected, Θ(n²) worst case; Span Θ(log² n) expected (with parallel filter), Parallelism Θ(n/log² n) expected
        fn select(&self, k: N) -> Option<T>;
    }

    impl<T: StTInMtT + Ord> OrderStatSelectMtPerTrait<T> for ArraySeqMtPerS<T> {
        fn select(&self, k: N) -> Option<T> {
            let n = self.length();
            if k >= n || n == 0 {
                return None;
            }
            if n == 1 {
                return Some(self.nth(0).clone());
            }

            // Algorithm 35.2: Randomized selection with parallel partition
            let pivot_idx = rand::rng().random_range(0..n);
            let pivot = self.nth(pivot_idx).clone();

            // Parallel partition using rayon
            use rayon::prelude::*;
            
            let pivot_left = pivot.clone();
            let pivot_right = pivot.clone();
            
            // Parallel filter for left partition (elements < pivot)
            let left_vec: Vec<T> = (0..n)
                .into_par_iter()
                .filter_map(|i| {
                    let elem = self.nth(i);
                    if elem < &pivot_left {
                        Some(elem.clone())
                    } else {
                        None
                    }
                })
                .collect();
            
            // Parallel filter for right partition (elements > pivot)
            let right_vec: Vec<T> = (0..n)
                .into_par_iter()
                .filter_map(|i| {
                    let elem = self.nth(i);
                    if elem > &pivot_right {
                        Some(elem.clone())
                    } else {
                        None
                    }
                })
                .collect();

            let left_count = left_vec.len();
            let right_count = right_vec.len();

            // Recursive selection based on partition sizes
            if k < left_count {
                let left = ArraySeqMtPerS::from_vec(left_vec);
                left.select(k)
            } else if k < n - right_count {
                Some(pivot)
            } else {
                let right = ArraySeqMtPerS::from_vec(right_vec);
                right.select(k - (n - right_count))
            }
        }
    }
}
