//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential merge sort implementation (Chapter 26).

pub mod MergeSortSt {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS, ArraySeqStPerTrait};
    use crate::Types::Types::*;

    /// Sequential merge sort trait.
    pub trait MergeSortStTrait<T: StT + Ord> {
        /// Merge two sorted sequences into one sorted sequence.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn merge(left: &ArraySeqStPerS<T>, right: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;

        /// Sort a sequence using merge sort.
        /// APAS: Work Θ(n log n), Span Θ(n log n)
        /// claude-4-sonet: Work Θ(n log n), Span Θ(n log n), Parallelism Θ(1)
        fn merge_sort(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> MergeSortStTrait<T> for ArraySeqStPerS<T> {
        fn merge(left: &ArraySeqStPerS<T>, right: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let n_left = left.length();
            let n_right = right.length();
            let total = n_left + n_right;

            if total == 0 {
                return ArraySeqStPerS::empty();
            }
            if n_left == 0 {
                return right.clone();
            }
            if n_right == 0 {
                return left.clone();
            }

            // Build result using tabulate (no Vec!)
            ArraySeqStPerS::tabulate(
                &|i| {
                    // Determine position in left and right sequences
                    let mut l_idx = 0;
                    let mut r_idx = 0;
                    let mut count = 0;

                    // Simulate the merge to find element at position i
                    while count < i {
                        if l_idx < n_left && r_idx < n_right {
                            if left.nth(l_idx) <= right.nth(r_idx) {
                                l_idx += 1;
                            } else {
                                r_idx += 1;
                            }
                        } else if l_idx < n_left {
                            l_idx += 1;
                        } else {
                            r_idx += 1;
                        }
                        count += 1;
                    }

                    // Get the element at position i
                    if l_idx < n_left && r_idx < n_right {
                        if left.nth(l_idx) <= right.nth(r_idx) {
                            left.nth(l_idx).clone()
                        } else {
                            right.nth(r_idx).clone()
                        }
                    } else if l_idx < n_left {
                        left.nth(l_idx).clone()
                    } else {
                        right.nth(r_idx).clone()
                    }
                },
                total,
            )
        }

        fn merge_sort(a: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let n = a.length();

            // Base case: sequences of length 0 or 1 are already sorted
            if n <= 1 {
                return a.clone();
            }

            // Divide: split at midpoint
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            // Recur: sequential recursive calls
            let sorted_left = Self::merge_sort(&left);
            let sorted_right = Self::merge_sort(&right);

            // Combine: merge the two sorted halves
            Self::merge(&sorted_left, &sorted_right)
        }
    }
}
