//! Singly-linked-list-backed sequence datatype, named SinglyLinkedListS, mirroring `ArraySeq` APIs.
//!
//! Abstract:
//! - Defines `SinglyLinkedListS<T>` as a variable-length sequence backed by `std::collections::LinkedList<T>`.
//! - Exposes `SinglyLinkedListSeq<T>` with core ops: `new`, `length`, `nth`, `empty`, `set`, `singleton`,
//!   `isEmpty`, `isSingleton`, `subseq_copy`.
//! - Helper: `update` (chainable).
//! - Costs differ from arrays: `nth`/`set` are Θ(index) due to linked traversal.

pub use crate::Types::{N, B, O};

use std::collections::LinkedList;

/// Sequence abstraction over a linked list. Elements live in heap-allocated nodes.
pub struct SinglyLinkedListS<T> {
    pub data: LinkedList<T>,
}

/// Generic sequence trait for linked-list-backed sequences.
pub trait SinglyLinkedListSeq<T> {
    /// Construct a new list of length `length` with each element `init_value`. <br/>
    /// gpt-5-hard: Work Θ(length), Span Θ(length).
    fn new(length: N, init_value: T) -> SinglyLinkedListS<T>
    where
        T: Clone;
    /// APAS 20.7 (length a): Work Θ(1), Span Θ(1).
    fn length(&self) -> N;
    /// APAS 20.7 (nth a i): Work Θ(i), Span Θ(i).
    fn nth(&self, index: N) -> &T;
    /// APAS 20.7 (empty): Work Θ(1), Span Θ(1).
    fn empty() -> SinglyLinkedListS<T>;
    /// APAS 20.7 (update a (i,x)): Work Θ(1+|a|), Span Θ(1+|a|). <br/>
    /// gpt-5-hard: Work Θ(i), Span Θ(i).
    /// BUG: APAS and gpt-5 algorithmic analysis differs.
    fn set(&mut self, index: N, item: T) -> Result<&mut SinglyLinkedListS<T>, &'static str>;
    /// APAS 20.7 (singleton x): Work Θ(1), Span Θ(1).
    fn singleton(item: T) -> SinglyLinkedListS<T>;
    /// APAS 20.7 (isEmpty x): Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B;
    /// APAS 20.7 (isSingleton x): Work Θ(1), Span Θ(1).
    fn isSingleton(&self) -> B;
    /// APAS 20.7 (subseq a (i,j)): Work Θ(1+i), Span Θ(1+i). <br/>
    /// gpt-5-hard: Work Θ(1 + i + length), Span Θ(1 + i).
    /// BUG: APAS and gpt-5 algorithmic analysis differs.
    fn subseq_copy(&self, start: N, length: N) -> SinglyLinkedListS<T>
    where
        T: Clone + Eq;
}

impl<T> SinglyLinkedListS<T> {
    pub fn update(&mut self, (index, item): (N, T)) -> &mut SinglyLinkedListS<T> {
        if let Some(slot) = self.data.iter_mut().nth(index) { *slot = item; }
        self
    }
}

impl<T: Eq> PartialEq for SinglyLinkedListS<T> {
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

impl<T: Eq> Eq for SinglyLinkedListS<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for SinglyLinkedListS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl<T> SinglyLinkedListSeq<T> for SinglyLinkedListS<T> {
    /// gpt-5-hard: Work Θ(length), Span Θ(length).
    fn new(length: N, init_value: T) -> SinglyLinkedListS<T>
    where
        T: Clone,
    {
        let mut ll = LinkedList::new();
        for _ in 0..length { ll.push_back(init_value.clone()); }
        SinglyLinkedListS { data: ll }
    }
    /// APAS 20.7 (length a): Work Θ(1), Span Θ(1).
    fn length(&self) -> N { self.data.len() }
    /// APAS 20.7 (nth a i): Work Θ(i), Span Θ(i).
    fn nth(&self, index: N) -> &T { self.data.iter().nth(index).expect("index out of bounds") }
    /// APAS 20.7 (empty): Work Θ(1), Span Θ(1).
    fn empty() -> SinglyLinkedListS<T> { SinglyLinkedListS { data: LinkedList::new() } }
    /// gpt-5-hard: Work Θ(i), Span Θ(i).
    /// BUG: APAS and gpt-5 algorithmic analysis differs.
    fn set(&mut self, index: N, item: T) -> Result<&mut SinglyLinkedListS<T>, &'static str> {
        match self.data.iter_mut().nth(index) { Some(slot) => { *slot = item; Ok(self) }, None => Err("Index out of bounds") }
    }
    /// APAS 20.7 (singleton x): Work Θ(1), Span Θ(1).
    fn singleton(item: T) -> SinglyLinkedListS<T> { let mut ll = LinkedList::new(); ll.push_back(item); SinglyLinkedListS { data: ll } }
    /// APAS 20.7 (isEmpty x): Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }
    /// APAS 20.7 (isSingleton x): Work Θ(1), Span Θ(1).
    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }
    /// gpt-5-hard: Work Θ(1 + start + length), Span Θ(1 + start).
    /// BUG: APAS and gpt-5 algorithmic analysis differs.
    fn subseq_copy(&self, start: N, length: N) -> SinglyLinkedListS<T>
    where
        T: Clone + Eq,
    {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s { return <SinglyLinkedListS<T> as crate::SinglyLinkedListSeq::SinglyLinkedListSeq<T>>::empty(); }
        let mut out = LinkedList::new();
        for x in self.data.iter().skip(s).take(e - s) { out.push_back(x.clone()); }
        SinglyLinkedListS { data: out }
    }
}


