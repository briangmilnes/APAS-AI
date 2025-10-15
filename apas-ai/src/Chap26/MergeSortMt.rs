//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel merge sort implementation (Chapter 26).
//! Note: Unconditionally parallel - no thresholding per APAS rules.

pub mod MergeSortMt {

    use std::sync::Arc;
    use std::thread;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::ParaPair;
    use crate::Types::Types::*;

    pub trait MergeSortMtTrait<T: StTInMtT + Ord + 'static> {
        /// Merge two sorted sequences in parallel using binary search.
        /// APAS: Work Θ(n), Span Θ(log n)
        /// claude-4-sonet: Work Θ(n), Span Θ(log n), Parallelism Θ(n/log n)
        fn merge_parallel(left: &ArraySeqMtPerS<T>, right: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;

        /// Sort a sequence using parallel merge sort.
        /// APAS: Work Θ(n log n), Span Θ(log² n)
        /// claude-4-sonet: Work Θ(n log n), Span Θ(log² n), Parallelism Θ(n/log n)
        fn merge_sort_parallel(a: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T>;
    }

    impl<T: StTInMtT + Ord + 'static> MergeSortMtTrait<T> for ArraySeqMtPerS<T> {
        fn merge_parallel(left: &ArraySeqMtPerS<T>, right: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {
            let n_left = left.length();
            let n_right = right.length();
            let total = n_left + n_right;

            if total == 0 {
                return ArraySeqMtPerS::empty();
            }
            if n_left == 0 {
                return right.clone();
            }
            if n_right == 0 {
                return left.clone();
            }

            // Base case for small merges
            if total <= 2 {
                if n_left == 1 && n_right == 1 {
                    if left.nth(0) <= right.nth(0) {
                        return ArraySeqMtPerS::append(left, right);
                    } else {
                        return ArraySeqMtPerS::append(right, left);
                    }
                }
                // For total == 1 or other small cases
                return ArraySeqMtPerS::tabulate(
                    &|i| {
                        if i < n_left {
                            left.nth(i).clone()
                        } else {
                            right.nth(i - n_left).clone()
                        }
                    },
                    total,
                );
            }

            // Parallel merge: split the longer sequence
            if n_left >= n_right {
                let mid_left = n_left / 2;
                let pivot = left.nth(mid_left);

                // Binary search in right sequence
                let mut lo = 0;
                let mut hi = n_right;
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if right.nth(mid) < pivot {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                let mid_right = lo;

                // Parallel recursive merge
                let left_left = left.subseq_copy(0, mid_left);
                let left_right = left.subseq_copy(mid_left + 1, n_left - mid_left - 1);
                let right_left = right.subseq_copy(0, mid_right);
                let right_right = right.subseq_copy(mid_right, n_right - mid_right);

                let left_left_arc = Arc::new(left_left);
                let right_left_arc = Arc::new(right_left);
                let left_right_arc = Arc::new(left_right);
                let right_right_arc = Arc::new(right_right);

                let (merged_left, merged_right) = thread::scope(|s| {
                    let ll = left_left_arc.clone();
                    let rl = right_left_arc.clone();
                    let handle_left = s.spawn(move || Self::merge_parallel(&*ll, &*rl));

                    let lr = left_right_arc.clone();
                    let rr = right_right_arc.clone();
                    let merged_right = Self::merge_parallel(&*lr, &*rr);

                    let merged_left = handle_left.join().unwrap();
                    (merged_left, merged_right)
                });

                // Combine: merged_left + [pivot] + merged_right
                let pivot_seq = ArraySeqMtPerS::singleton(pivot.clone());
                let left_with_pivot = ArraySeqMtPerS::append(&merged_left, &pivot_seq);
                ArraySeqMtPerS::append(&left_with_pivot, &merged_right)
            } else {
                // Symmetric case: split right sequence
                let mid_right = n_right / 2;
                let pivot = right.nth(mid_right);

                // Binary search in left sequence
                let mut lo = 0;
                let mut hi = n_left;
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if left.nth(mid) <= pivot {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                let mid_left = lo;

                // Parallel recursive merge
                let left_left = left.subseq_copy(0, mid_left);
                let left_right = left.subseq_copy(mid_left, n_left - mid_left);
                let right_left = right.subseq_copy(0, mid_right);
                let right_right = right.subseq_copy(mid_right + 1, n_right - mid_right - 1);

                let left_left_arc = Arc::new(left_left);
                let right_left_arc = Arc::new(right_left);
                let left_right_arc = Arc::new(left_right);
                let right_right_arc = Arc::new(right_right);

                let (merged_left, merged_right) = thread::scope(|s| {
                    let ll = left_left_arc.clone();
                    let rl = right_left_arc.clone();
                    let handle_left = s.spawn(move || Self::merge_parallel(&*ll, &*rl));

                    let lr = left_right_arc.clone();
                    let rr = right_right_arc.clone();
                    let merged_right = Self::merge_parallel(&*lr, &*rr);

                    let merged_left = handle_left.join().unwrap();
                    (merged_left, merged_right)
                });

                // Combine: merged_left + [pivot] + merged_right
                let pivot_seq = ArraySeqMtPerS::singleton(pivot.clone());
                let left_with_pivot = ArraySeqMtPerS::append(&merged_left, &pivot_seq);
                ArraySeqMtPerS::append(&left_with_pivot, &merged_right)
            }
        }

        fn merge_sort_parallel(a: &ArraySeqMtPerS<T>) -> ArraySeqMtPerS<T> {
            let n = a.length();

            // Base case: sequences of length 0 or 1 are already sorted
            if n <= 1 {
                return a.clone();
            }

            // Divide: split at midpoint
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            // Recur: UNCONDITIONALLY PARALLEL using ParaPair!
            let left_arc = Arc::new(left);
            let right_arc = Arc::new(right);

            let Pair(sorted_left, sorted_right) = ParaPair!(
                {
                    let l = left_arc.clone();
                    move || Self::merge_sort_parallel(&*l)
                },
                {
                    let r = right_arc.clone();
                    move || Self::merge_sort_parallel(&*r)
                }
            );

            // Combine: parallel merge the two sorted halves
            Self::merge_parallel(&sorted_left, &sorted_right)
        }
    }
}
