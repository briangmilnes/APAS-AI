//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.1: spin-lock via fetch-and-add tickets.

pub mod Exercise12_1 {
    use std::hint::spin_loop;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    use crate::Types::Types::*;

    #[derive(Debug)]
    pub struct SpinLock {
        ticket: AtomicUsize,
        turn: AtomicUsize,
    }

    pub trait SpinLockTrait {
        fn new() -> Self;
        /// APAS: Work Θ(1) expected, Θ(n) worst case, Span Θ(1)
        /// claude-4-sonet: Work Θ(1) expected under low contention, Θ(n) worst case with n waiting threads, Span Θ(1) - sequential ticket acquisition
        fn lock(&self);
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - atomic increment releases next thread
        fn unlock(&self);
    }

    impl SpinLock {
        pub fn new() -> Self {
            SpinLock {
                ticket: AtomicUsize::new(0),
                turn: AtomicUsize::new(0),
            }
        }

        /// Acquire lock by taking a ticket and waiting for our turn.
        ///
        /// APAS: Work Θ(1) expected, Θ(n) worst case, Span Θ(1)
        /// claude-4-sonet: Work Θ(1) expected under low contention, Θ(n) worst case with n waiting threads, Span Θ(1) - sequential ticket acquisition
        pub fn lock(&self) {
            let my_ticket = self.ticket.fetch_add(1, Ordering::Relaxed);
            while self.turn.load(Ordering::Acquire) != my_ticket {
                spin_loop();
            }
        }

        /// Release lock by advancing turn counter.
        ///
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - atomic increment releases next thread
        pub fn unlock(&self) { self.turn.fetch_add(1, Ordering::Release); }

        /// Execute action while holding the lock.
        ///
        /// APAS: Work Θ(W_action), Span Θ(S_action)
        /// claude-4-sonet: Work Θ(W_action + 1), Span Θ(S_action + 1), Parallelism Θ(W_action/S_action) - dominated by action complexity
        pub fn with_lock<T>(&self, action: impl FnOnce() -> T) -> T {
            self.lock();
            let result = action();
            self.unlock();
            result
        }
    }

    impl SpinLockTrait for SpinLock {
        fn new() -> Self { SpinLock::new() }

        fn lock(&self) { SpinLock::lock(self) }

        fn unlock(&self) { SpinLock::unlock(self) }
    }

    impl Default for SpinLock {
        fn default() -> Self { SpinLock::new() }
    }

    /// Parallel counter increment using spin-lock for mutual exclusion.
    ///
    /// APAS: Work Θ(t × i), Span Θ(i)
    /// claude-4-sonet: Work Θ(t × i) where t=threads, i=iterations, Span Θ(i) assuming bounded contention, Parallelism Θ(t) - linear speedup under low contention
    pub fn parallel_increment(iterations: N) -> usize {
        let lock = Arc::new(SpinLock::new());
        let shared = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for _ in 0..4 {
            let lock_clone = Arc::clone(&lock);
            let shared_clone = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                for _ in 0..iterations {
                    lock_clone.lock();
                    shared_clone.fetch_add(1, Ordering::Relaxed);
                    lock_clone.unlock();
                }
            }));
        }

        for handle in handles {
            handle.join().expect("parallel_increment: worker panicked");
        }

        shared.load(Ordering::Relaxed)
    }
}
