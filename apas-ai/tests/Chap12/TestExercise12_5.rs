//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::collections::HashSet;
use std::sync::{Arc, mpsc};
use std::thread;

use apas_ai::Chap12::Exercise12_5::Exercise12_5::*;
use apas_ai::Types::Types::*;

#[test]
fn push_pop_lifo_single_thread() {
    let stack = ConcurrentStackMt::<usize>::new();
    for value in 0usize..4 {
        stack.push(value);
    }

    for expected in (0usize..4).rev() {
        assert_eq!(stack.pop(), Some(expected));
    }
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn pop_on_empty_returns_none() {
    let stack = ConcurrentStackMt::<usize>::new();
    assert!(stack.pop().is_none());
}

#[test]
fn multi_thread_push_collects_all_items() {
    let stack = Arc::<ConcurrentStackMt<usize>>::new(ConcurrentStackMt::new());
    let threads = 4;
    let per_thread = 1_000;

    let mut handles = Vec::new();
    for t in 0..threads {
        let stack_clone = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            let base = t * per_thread;
            for offset in 0..per_thread {
                stack_clone.push(base + offset);
            }
        }))
    }

    for handle in handles {
        handle.join().expect("multi_thread_push: worker panicked");
    }

    let mut values = stack.drain();
    assert_eq!(values.len(), threads * per_thread);
    values.sort_unstable();
    let expected = (0..threads * per_thread).collect::<Vec<_>>();
    assert_eq!(values, expected);
}

#[test]
fn multi_thread_pop_consumes_all_elements() {
    let stack = Arc::<ConcurrentStackMt<usize>>::new(ConcurrentStackMt::new());
    let threads = 4;
    let per_thread = 800;
    for value in 0..threads * per_thread {
        stack.push(value);
    }

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..threads {
        let stack_clone = Arc::clone(&stack);
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            let mut items = Vec::new();
            while let Some(value) = stack_clone.pop() {
                items.push(value);
            }
            tx_clone.send(items).expect("send popped items");
        }));
    }
    drop(tx);

    for handle in handles {
        handle.join().expect("multi_thread_pop: worker panicked");
    }

    let mut combined = Vec::new();
    for mut items in rx {
        combined.append(&mut items);
    }
    assert_eq!(combined.len(), threads * per_thread);

    let unique = combined.iter().copied().collect::<HashSet<_>>();
    assert_eq!(unique.len(), combined.len());

    let expected = (0..threads * per_thread).collect::<HashSet<_>>();
    assert_eq!(unique, expected);
    assert!(stack.pop().is_none());
}

#[test]
fn test_default_trait() {
    let stack: ConcurrentStackMt<i32> = Default::default();
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_drain_comprehensive() {
    let stack = ConcurrentStackMt::new();
    for i in 0..10 {
        stack.push(i);
    }

    let drained = stack.drain();
    assert_eq!(drained.len(), 10);
    assert!(stack.is_empty());

    // Stack should be usable after drain
    stack.push(99);
    assert!(!stack.is_empty());
    assert_eq!(stack.pop(), Some(99));
}

#[test]
fn test_multiple_push_pop_cycles() {
    let stack = ConcurrentStackMt::new();

    // Cycle 1
    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));

    // Cycle 2
    stack.push(3);
    stack.push(4);
    stack.push(5);
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));

    assert!(stack.is_empty());
}

#[test]
fn test_is_empty_states() {
    let stack = ConcurrentStackMt::new();
    assert!(stack.is_empty());

    stack.push(1);
    assert!(!stack.is_empty());

    stack.push(2);
    assert!(!stack.is_empty());

    stack.pop();
    assert!(!stack.is_empty());

    stack.pop();
    assert!(stack.is_empty());
}

#[test]
fn test_large_sequential_operations() {
    let stack = ConcurrentStackMt::new();
    let count = 10_000;

    for i in 0..count {
        stack.push(i);
    }

    for expected in (0..count).rev() {
        assert_eq!(stack.pop(), Some(expected));
    }

    assert!(stack.is_empty());
}
