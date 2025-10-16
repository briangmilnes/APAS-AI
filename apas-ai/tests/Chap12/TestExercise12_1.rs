//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use apas_ai::Chap12::Exercise12_1::Exercise12_1::*;

#[test]
fn spin_lock_excludes_parallel_threads() {
    let lock = Arc::new(SpinLock::new());
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let lock_clone = Arc::clone(&lock);
        let counter_clone = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..128 {
                lock_clone.lock();
                let current = counter_clone.load(Ordering::Relaxed);
                counter_clone.store(current + 1, Ordering::Relaxed);
                lock_clone.unlock();
            }
        }));
    }

    for handle in handles {
        handle
            .join()
            .expect("spin_lock_excludes_parallel_threads: worker panicked");
    }

    assert_eq!(counter.load(Ordering::Relaxed), 4 * 128);
}

#[test]
fn spin_lock_with_lock_helper_executes_body() {
    let lock = SpinLock::new();
    let flag = AtomicBool::new(false);
    lock.with_lock(|| flag.store(true, Ordering::Relaxed));
    assert!(flag.load(Ordering::Relaxed));
}

#[test]
fn parallel_increment_counts_all_iterations() {
    assert_eq!(parallel_increment(1_000), 4_000);
}

#[test]
fn spin_lock_is_non_reentrant() {
    let lock = Arc::new(SpinLock::new());
    let attempting = Arc::new(AtomicBool::new(false));
    let acquired = Arc::new(AtomicBool::new(false));

    lock.lock();
    let lock_clone = Arc::clone(&lock);
    let attempting_clone = Arc::clone(&attempting);
    let acquired_clone = Arc::clone(&acquired);
    let handle = thread::spawn(move || {
        attempting_clone.store(true, Ordering::Relaxed);
        lock_clone.lock();
        acquired_clone.store(true, Ordering::Relaxed);
        lock_clone.unlock();
    });

    while !attempting.load(Ordering::Relaxed) {
        thread::yield_now();
    }
    thread::sleep(Duration::from_millis(5));
    assert!(!acquired.load(Ordering::Relaxed));

    lock.unlock();
    handle.join().expect("spin_lock_is_non_reentrant: worker panicked");
    assert!(acquired.load(Ordering::Relaxed));
}

#[test]
fn test_default_trait() {
    let lock: SpinLock = Default::default();
    lock.lock();
    lock.unlock();
}

#[test]
fn test_with_lock_returns_value() {
    let lock = SpinLock::new();
    let result = lock.with_lock(|| 42);
    assert_eq!(result, 42);

    let result2 = lock.with_lock(|| "hello");
    assert_eq!(result2, "hello");
}

#[test]
fn test_multiple_lock_unlock_cycles() {
    let lock = SpinLock::new();

    for i in 0..10 {
        lock.lock();
        // Critical section
        let _ = i;
        lock.unlock();
    }
}

#[test]
fn test_with_lock_multiple_times() {
    let lock = SpinLock::new();
    let counter = AtomicUsize::new(0);

    for _ in 0..5 {
        lock.with_lock(|| {
            counter.fetch_add(1, Ordering::Relaxed);
        });
    }

    assert_eq!(counter.load(Ordering::Relaxed), 5);
}
