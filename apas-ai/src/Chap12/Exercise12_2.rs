//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.2: implement fetch-and-add using compare-and-swap.

pub mod Exercise12_2 {

use std::sync::atomic::{AtomicUsize, Ordering};

    pub trait FetchAddCasTrait {
        /// APAS: Work Θ(1) expected, Θ(n) worst case, Span Θ(1)
        /// claude-4-sonet: Work Θ(1) expected under low contention, Θ(n) worst case with n concurrent operations, Span Θ(1) - CAS retry loop
        fn fetch_add_cas(&self, delta: usize) -> usize;
    }

    impl FetchAddCasTrait for AtomicUsize {
        /// Implement fetch_add using compare-and-swap retry loop.
        ///
        /// APAS: Work Θ(1) expected, Θ(n) worst case, Span Θ(1)
        /// claude-4-sonet: Work Θ(1) expected under low contention, Θ(n) worst case with n concurrent operations, Span Θ(1), Parallelism Θ(1) - CAS retry loop
        ///
        /// Note: Hardware fetch_add is single atomic op; CAS loop may retry under contention, so cannot outperform native.
        fn fetch_add_cas(&self, delta: usize) -> usize {
            let mut current = self.load(Ordering::Relaxed);
            loop {
                let next = current.wrapping_add(delta);
                match self.compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire) {
                    | Ok(previous) => return previous,
                    | Err(observed) => current = observed,
                }
            }
        }
    }

    /// Convenience function wrapping the trait method.
    ///
    /// APAS: Work Θ(1) expected, Θ(n) worst case, Span Θ(1)
    /// claude-4-sonet: Work Θ(1) expected, Θ(n) worst case, Span Θ(1), Parallelism Θ(1)
    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> usize { target.fetch_add_cas(delta) }

    pub fn efficiency_note() -> &'static str {
        "Hardware fetch_add completes in one atomic operation; the CAS loop may repeat under contention, so it cannot outperform native fetch_add."
    }
}
