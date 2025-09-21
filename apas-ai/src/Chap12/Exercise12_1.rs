//! Chapter 12 — Exercise 12.1: spin-lock via fetch-and-add tickets.

pub mod Exercise12_1 {
    use crate::Types::Types::*;
    use std::hint::spin_loop;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::thread;

    #[derive(Debug)]
    pub struct SpinLock {
        ticket: AtomicUsize,
        turn: AtomicUsize,
    }

    pub trait SpinLockTrait {
        fn new() -> Self;
        fn lock(&self);
        fn unlock(&self);
    }

    impl SpinLock {
        pub fn new() -> Self {
            SpinLock { ticket: AtomicUsize::new(0), turn: AtomicUsize::new(0) }
        }

        pub fn lock(&self) {
            let my_ticket = self.ticket.fetch_add(1, Ordering::Relaxed);
            while self.turn.load(Ordering::Acquire) != my_ticket {
                spin_loop();
            }
        }

        pub fn unlock(&self) {
            self.turn.fetch_add(1, Ordering::Release);
        }

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
