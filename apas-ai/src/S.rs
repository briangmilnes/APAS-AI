//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! This file builds the sequence datatype, named S, from Chapter 17 but we are skipping the
//! fundamental version of it as a mapping and going straight to fixed size arrays.

#![allow(non_snake_case)]

/// Type alias for natural numbers to match APAS, N.
pub type N = usize;

/// Data Type 18.1 (Boolean) type used by APAS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum B { True, False }

/// Data Type 18.1 (Ordering) relationships used by APAS (re-export of `std::cmp::Ordering`).
pub use std::cmp::Ordering as O;

/// Definition 17.1 (Sequence). Fixed-length sequence backed by `Box<[T]>`.
/// The elements are stored on the heap; the `S<T>` value (which holds the boxed-slice handle)
/// lives wherever it is bound (e.g., on the stack).
pub struct S<T> {
    pub data: Box<[T]>,
}

/// Data Type 18.1: Generic sequence trait.
pub trait Sequence<T> {
    /// Create a new sequence of length `length` with each element initialized to `init_value`. <br/>
    /// Work: Θ(length), Span: Θ(1).
    fn new(length: N, init_value: T) -> S<T>
    where
        T: Clone;

    /// Algorithm 19.12 (Function length). Return the number of elements in the sequence. <br/>
    /// Work: Θ(1), Span: Θ(1).
    fn length(&self) -> N;

    /// Algorithm 19.11 (Function nth). Return a reference to the element at `index`. <br/>
    /// Work: Θ(1), Span: Θ(1).
    /// Although we could use the Rust Option type, APAS has out-of-index references panic.
    fn nth(&self, index: N) -> &T;

    /// Definition 18.1 (empty). Construct the empty sequence. <br/>
    /// Work: Θ(1), Span: Θ(1).
    fn empty() -> S<T>;

    /// Set the element at `index` to `item` in place. <br/>
    /// Work: Θ(1), Span: Θ(1).
    fn set(&mut self, index: N, item: T) -> Result<&mut S<T>, &'static str>;

    /// Definition 18.1 (singleton). Construct a singleton sequence containing `item`. <br/>
    /// Work: Θ(1), Span: Θ(1).
    fn singleton(item: T) -> S<T>;

    /// Definition 18.5 (isEmpty). B::True iff the sequence has length zero. <br/>
    /// Work: Θ(1), Span: Θ(1).
    fn isEmpty(&self) -> B;

    /// Definition 18.5 (isSingleton). B::True iff the sequence has length one. <br/>
    /// Work: Θ(1), Span: Θ(1).
    fn isSingleton(&self) -> B;
}

impl<T> S<T> {
    pub fn length(&self) -> N { self.data.len() }

    /// Update `self[index]` to `item` in place if in bounds, and return `self` for chaining. <br/>
    /// Work: Θ(1), Span: Θ(1).
    pub fn update(&mut self, (index, item): (N, T)) -> &mut S<T> {
        if index < self.data.len() { self.data[index] = item; }
        self
    }

    /// Create sequence from a Vec (used by `seq!` and tests). <br/>
    /// Work: Θ(n) worst case (shrink-to-fit moves), Θ(1) best case (rebrand); Span: Θ(1). <br/>
    /// Reason: `Vec<T>` owns a heap buffer; `into_boxed_slice()` reuses it when capacity==len, else shrinks and moves elements.
    pub fn from_vec(data: Vec<T>) -> S<T> { S { data: data.into_boxed_slice() } }
}

impl<T: Eq> PartialEq for S<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length() != other.length() { return false; }
        for i in 0..self.length() {
            if self.nth(i) != other.nth(i) { return false; }
        }
        true
    }
}

impl<T: Eq> Eq for S<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for S<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let it = (0..self.length()).map(|i| self.nth(i));
        f.debug_list().entries(it).finish()
    }
}

impl<T> Sequence<T> for S<T> {
    fn new(length: N, init_value: T) -> S<T>
    where
        T: Clone,
    { crate::seq![init_value; length] }

    fn length(&self) -> N { self.data.len() }
    fn nth(&self, index: N) -> &T { &self.data[index] }
    fn empty() -> S<T> { crate::seq![] }
    fn set(&mut self, index: N, item: T) -> Result<&mut S<T>, &'static str> {
        if index < self.data.len() {
            self.data[index] = item;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }

    fn singleton(item: T) -> S<T> { crate::seq![item] }

    fn isEmpty(&self) -> B { if self.data.len() == 0 { B::True } else { B::False } }

    fn isSingleton(&self) -> B { if self.data.len() == 1 { B::True } else { B::False } }
}
