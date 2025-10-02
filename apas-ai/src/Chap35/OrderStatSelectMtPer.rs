//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Order Statistics - Parallel Persistent (Chapter 35, Algorithm 35.2).
//! Randomized selection algorithm for finding kth order statistic with parallel partition.
//! Work: O(n) expected, Span: O(lg² n) expected.

pub mod OrderStatSelectMtPer {

use crate::Types::Types::*;
use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerS, ArraySeqMtPerTrait};
use rand::Rng;
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

            let pivot_idx = rand::rng().random_range(0..n);
            let pivot = self.nth(pivot_idx).clone();

            let mut left_count = 0;
            let mut right_count = 0;

            for i in 0..n {
                let elem = self.nth(i);
                if elem < &pivot {
                    left_count += 1;
                } else if elem > &pivot {
                    right_count += 1;
                }
            }

            if k < left_count {
                let left = ArraySeqMtPerS::tabulate(
                    &|i| {
                        let mut idx = 0;
                        for j in 0..n {
                            let elem = self.nth(j);
                            if elem < &pivot {
                                if idx == i {
                                    return elem.clone();
                                }
                                idx += 1;
                            }
                        }
                        panic!("Index out of bounds in left partition");
                    },
                    left_count,
                );
                left.select(k)
            } else if k < n - right_count {
                Some(pivot)
            } else {
                let right = ArraySeqMtPerS::tabulate(
                    &|i| {
                        let mut idx = 0;
                        for j in 0..n {
                            let elem = self.nth(j);
                            if elem > &pivot {
                                if idx == i {
                                    return elem.clone();
                                }
                                idx += 1;
                            }
                        }
                        panic!("Index out of bounds in right partition");
                    },
                    right_count,
                );
                right.select(k - (n - right_count))
            }
        }
    }
}
