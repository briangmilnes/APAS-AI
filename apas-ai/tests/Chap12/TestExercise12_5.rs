//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestExercise12_5 {
    use std::collections::HashSet;
    use std::sync::{Arc, mpsc};
    use std::thread;

    use apas_ai::Chap12::Exercise12_5::Exercise12_5::{ConcurrentStackMt, ConcurrentStackMtTrait};
#[test]
fn push_pop_lifo_single_thread() {
    let stack: ConcurrentStackMt<usize> = ConcurrentStackMt::new();
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
    let stack: ConcurrentStackMt<usize> = ConcurrentStackMt::new();
    assert!(stack.pop().is_none());
}

#[test]
fn multi_thread_push_collects_all_items() {
    let stack: Arc<ConcurrentStackMt<usize>> = Arc::new(ConcurrentStackMt::new());
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
    let expected: Vec<_> = (0..threads * per_thread).collect();
    assert_eq!(values, expected);
}

#[test]
fn multi_thread_pop_consumes_all_elements() {
    let stack: Arc<ConcurrentStackMt<usize>> = Arc::new(ConcurrentStackMt::new());
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
            loop {
                match stack_clone.pop() {
                    | Some(value) => items.push(value),
                    | None => break,
                }
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

    let unique: HashSet<_> = combined.iter().copied().collect();
    assert_eq!(unique.len(), combined.len());

    let expected: HashSet<_> = (0..threads * per_thread).collect();
    assert_eq!(unique, expected);
    assert!(stack.pop().is_none());
    }

}
