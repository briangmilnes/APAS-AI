//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices.

pub mod InsertionSortSt {

    pub trait InsertionSortStTrait<T: Ord + Clone> {
        /// APAS: Work O(n²), Span O(n log n)
        /// claude-4-sonet: Work Θ(n²), Span Θ(n²), Parallelism Θ(1)
        /// BUG: claude-4-sonet's span analysis differs from APAS (APAS assumes O(log n) span insert operation)
        ///
        /// APAS is underspecified with:
        /// Example 3.2 (Cost of Insertion Sort). Considering insertion sort example, suppose that we
        /// are given a cost specification for insert: for a sequence of length n the cost of insert
        /// should be O(n) work and O(log n) span. We can then determine the overall asymptotic
        /// cost of sort using our composition rules described in Section (Work, Span, and Parallel
        /// Time). Since the code uses insert sequentially and since there are n inserts, the algorithm
        /// insSort has n × O(n) = O(n²) work and n × O(log n) = O(n log n) span.
        fn insSort(&self, slice: &mut [T]);
    }

    impl<T: Ord + Clone> InsertionSortStTrait<T> for T {
        fn insSort(&self, slice: &mut [T]) {
            for i in 1..slice.len() {
                let key = slice[i].clone();
                let mut j = i;
                while j > 0 && slice[j - 1] > key {
                    slice[j] = slice[j - 1].clone();
                    j -= 1;
                }
                slice[j] = key;
            }
        }
    }
}
