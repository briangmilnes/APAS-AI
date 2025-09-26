//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.5: lock-free concurrent stack using compare-and-swap.

pub mod Exercise12_5 {
    use std::mem::ManuallyDrop;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

    use crate::Types::Types::StTInMtT;

    struct Node<T: StTInMtT> {
        value: ManuallyDrop<T>,
        next: *mut Node<T>,
    }

    /// Concurrent stack backed by a Treiber-style CAS list.
    #[derive(Debug)]
    pub struct ConcurrentStackMt<T: StTInMtT> {
        head: AtomicPtr<Node<T>>,
    }

    pub trait ConcurrentStackMtTrait<T: StTInMtT> {
        fn new() -> Self;
        fn push(&self, value: T);
        fn pop(&self) -> Option<T>;
        fn is_empty(&self) -> bool;
    }

    impl<T: StTInMtT> ConcurrentStackMt<T> {
        fn raw_pop(&self) -> Option<*mut Node<T>> {
            loop {
                let head = self.head.load(Ordering::Acquire);
                if head.is_null() {
                    return None;
                }
                let next = unsafe { (*head).next };
                if self
                    .head
                    .compare_exchange_weak(head, next, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    return Some(head);
                }
            }
        }
    }

    impl<T: StTInMtT> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {
        fn new() -> Self {
            ConcurrentStackMt { head: AtomicPtr::new(null_mut()) }
        }

        fn push(&self, value: T) {
            let mut new_node = Box::new(Node { value: ManuallyDrop::new(value), next: null_mut() });
            loop {
                let head = self.head.load(Ordering::Acquire);
                new_node.next = head;
                let node_ptr = Box::into_raw(new_node);
                if self
                    .head
                    .compare_exchange_weak(head, node_ptr, Ordering::AcqRel, Ordering::Acquire)
                    .is_ok()
                {
                    break;
                }
                new_node = unsafe { Box::from_raw(node_ptr) };
            }
        }

        fn pop(&self) -> Option<T> {
            let node_ptr = self.raw_pop()?;
            let boxed = unsafe { Box::from_raw(node_ptr) };
            let value = ManuallyDrop::into_inner(boxed.value);
            Some(value)
        }

        fn is_empty(&self) -> bool {
            self.head.load(Ordering::Acquire).is_null()
        }
    }

    impl<T: StTInMtT> Default for ConcurrentStackMt<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StTInMtT> Drop for ConcurrentStackMt<T> {
        fn drop(&mut self) {
            let mut current = self.head.load(Ordering::Relaxed);
            while !current.is_null() {
                unsafe {
                    let node = Box::from_raw(current);
                    let next = node.next;
                    drop(ManuallyDrop::into_inner(node.value));
                    current = next;
                }
            }
        }
    }

    impl<T: StTInMtT> ConcurrentStackMt<T> {
        /// Pop every element into a Vec for testing/teardown convenience.
        pub fn drain(&self) -> Vec<T> {
            let mut items = Vec::new();
            while let Some(value) = self.pop() {
                items.push(value);
            }
            items
        }
    }
}
