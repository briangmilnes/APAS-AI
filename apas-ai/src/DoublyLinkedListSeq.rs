//! Doubly-linked-list-backed sequence datatype, named DoublyLinkedListS, mirroring `ArraySeq` APIs.
//!
//! Abstract:
//! - Defines `DoublyLinkedListS<T>` as a variable-length sequence backed by `std::collections::LinkedList<T>`.
//! - Exposes `DoublyLinkedListSeq<T>` with core ops: `new`, `length`, `nth`, `empty`, `set`, `singleton`,
//!   `isEmpty`, `isSingleton`, `subseq_copy`.
//! - Helper: `update` (chainable).
//! - Costs differ from arrays: `nth`/`set` are Θ(index) due to linked traversal.

pub use crate::Types::{N, B, O};

use std::collections::LinkedList;

/// Fixed-length sequence abstraction over a doubly linked list. Elements live in heap-allocated nodes.
pub struct DoublyLinkedListS<T> {
    pub data: LinkedList<T>,
}

/// Generic sequence trait for linked-list-backed sequences.
pub trait DoublyLinkedListSeq<T> {
    /// Create a new sequence of length `length` with each element initialized to `init_value`. <br/>
    /// chatgpt-5-hard: Work Θ(length), Span Θ(length).
    fn new(length: N, init_value: T) -> DoublyLinkedListS<T>
    where
        T: Clone;

    /// Return the number of elements in the sequence. <br/>
    /// chatgpt-5-hard: Work Θ(1), Span Θ(1).
    fn length(&self) -> N;

    /// Return a reference to the element at `index`. Panics if out of bounds. <br/>
    /// chatgpt-5-hard: Work Θ(index), Span Θ(index).
    fn nth(&self, index: N) -> &T;

    /// Construct the empty sequence. <br/>
    /// chatgpt-5-hard: Work Θ(1), Span Θ(1).
    fn empty() -> DoublyLinkedListS<T>;

    /// Set the element at `index` to `item` in place. <br/>
    /// chatgpt-5-hard: Work Θ(index), Span Θ(index).
    fn set(&mut self, index: N, item: T) -> Result<&mut DoublyLinkedListS<T>, &'static str>;

    /// Construct a singleton sequence containing `item`. <br/>
    /// chatgpt-5-hard: Work Θ(1), Span Θ(1).
    fn singleton(item: T) -> DoublyLinkedListS<T>;

    /// B::True iff the sequence has length zero. <br/>
    /// chatgpt-5-hard: Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B;

    /// B::True iff the sequence has length one. <br/>
    /// chatgpt-5-hard: Work Θ(1), Span Θ(1).
    fn isSingleton(&self) -> B;

    /// Return the subsequence starting at `start` of length `length` (cloning elements). <br/>
    /// chatgpt-5-hard: Work Θ(1 + index + length), Span Θ(1 + index).
    fn subseq_copy(&self, start: N, length: N) -> DoublyLinkedListS<T>
    where
        T: Clone + Eq;
}

impl<T> DoublyLinkedListS<T> {
    /// Update `self[index]` to `item` in place if in bounds, and return `self` for chaining. <br/>
    /// Work: Θ(index), Span: Θ(1).
    pub fn update(&mut self, (index, item): (N, T)) -> &mut DoublyLinkedListS<T> {
        if let Some(slot) = self.data.iter_mut().nth(index) { *slot = item; }
        self
    }
}

impl<T: Eq> PartialEq for DoublyLinkedListS<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length() != other.length() { return false; }
        let mut it1 = self.data.iter();
        let mut it2 = other.data.iter();
        loop {
            match (it1.next(), it2.next()) {
                (None, None) => return true,
                (Some(a), Some(b)) => if a != b { return false; },
                _ => return false,
            }
        }
    }
}

impl<T: Eq> Eq for DoublyLinkedListS<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for DoublyLinkedListS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl<T> DoublyLinkedListSeq<T> for DoublyLinkedListS<T> {
    /// ChatGPT-5-hard: Work Θ(length), Span Θ(length).
    fn new(length: N, init_value: T) -> DoublyLinkedListS<T>
    where
        T: Clone,
    {
        let mut ll = LinkedList::new();
        for _ in 0..length { ll.push_back(init_value.clone()); }
        DoublyLinkedListS { data: ll }
    }

    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn length(&self) -> N { self.data.len() }

    /// ChatGPT-5-hard: Work Θ(index), Span Θ(index).
    fn nth(&self, index: N) -> &T { self.data.iter().nth(index).expect("index out of bounds") }

    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn empty() -> DoublyLinkedListS<T> { DoublyLinkedListS { data: LinkedList::new() } }

    /// ChatGPT-5-hard: Work Θ(index), Span Θ(index).
    fn set(&mut self, index: N, item: T) -> Result<&mut DoublyLinkedListS<T>, &'static str> {
        match self.data.iter_mut().nth(index) {
            Some(slot) => { *slot = item; Ok(self) }
            None => Err("Index out of bounds"),
        }
    }

    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn singleton(item: T) -> DoublyLinkedListS<T> { let mut ll = LinkedList::new(); ll.push_back(item); DoublyLinkedListS { data: ll } }

    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }

    /// ChatGPT-5-hard: Work Θ(1), Span Θ(1).
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }

    /// ChatGPT-5-hard: Work Θ(1 + start + length), Span Θ(1 + start).
    fn subseq_copy(&self, start: N, length: N) -> DoublyLinkedListS<T>
    where
        T: Clone + Eq,
    {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s { return <DoublyLinkedListS<T> as crate::DoublyLinkedListSeq::DoublyLinkedListSeq<T>>::empty(); }
        let mut out = LinkedList::new();
        for x in self.data.iter().skip(s).take(e - s) { out.push_back(x.clone()); }
        DoublyLinkedListS { data: out }
    }
}


