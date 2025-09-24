//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.2: implement fetch-and-add using compare-and-swap.

pub mod Exercise12_2 {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub trait FetchAddCasTrait {
        fn fetch_add_cas(&self, delta: usize) -> usize;
    }

    impl FetchAddCasTrait for AtomicUsize {
        fn fetch_add_cas(&self, delta: usize) -> usize {
            let mut current = self.load(Ordering::Relaxed);
            loop {
                let next = current.wrapping_add(delta);
                match self.compare_exchange_weak(current, next, Ordering::AcqRel, Ordering::Acquire) {
                    Ok(previous) => return previous,
                    Err(observed) => current = observed,
                }
            }
        }
    }

    pub fn fetch_add_cas(target: &AtomicUsize, delta: usize) -> usize {
        target.fetch_add_cas(delta)
    }

    pub fn efficiency_note() -> &'static str {
        "Hardware fetch_add completes in one atomic operation; the CAS loop may repeat under contention, so it cannot outperform native fetch_add."
    }
}
