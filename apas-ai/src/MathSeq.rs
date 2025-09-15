//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Mathematical sequence backed by a growable vector. Dense domain 0..len-1.
//!
//! Abstract: Definition 17.1 (Sequence) — runtime-sized, dense-domain sequence (0..n-1),
//! using rust vector which is dense.

pub mod MathSeq {
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use crate::Types::Types::*;

/// Mathematical sequence with dense domain, backed by `Vec<T>`.
pub struct MathSeqS<T: StT> {
    data: Vec<T>,
}

impl<T: StT> PartialEq for MathSeqS<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: StT> Eq for MathSeqS<T> {}

impl<T: StT> std::fmt::Debug for MathSeqS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl<T: StT> std::fmt::Display for MathSeqS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        for x in &self.data {
            if !first { write!(f, ", ")?; } else { first = false; }
            write!(f, "{}", x)?;
        }
        write!(f, "]")
    }
}

impl<T: StT> MathSeqS<T> {
    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.data.iter_mut() }

    pub fn empty() -> Self { Self { data: Vec::new() } }
    pub fn singleton(item: T) -> Self { Self { data: vec![item] } }
    pub fn from_vec(data: Vec<T>) -> Self { Self { data } }
    pub fn with_len(length: N, init_value: T) -> Self { Self { data: vec![init_value; length] } }
}

impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.data.iter() }
}

impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }
}

impl<T: StT> IntoIterator for MathSeqS<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
}

/// Core API for `MathSeqS<T>`.
pub trait MathSeqTrait<T: StT> {
    /// Construct a new sequence of length `length` initialized with `init_value`.
    /// Work: Θ(length)
    /// Span: Θ(1)
    fn new(length: N, init_value: T) -> Self;

    /// Construct the empty sequence.
    /// Work: Θ(1)
    /// Span: Θ(1)
    fn empty() -> Self;

    /// Construct a singleton sequence containing `item`.
    /// Work: Θ(1)
    /// Span: Θ(1)
    fn singleton(item: T) -> Self;

    /// Return the number of elements.
    /// Work Θ(1), Span Θ(1).
    fn length(&self) -> N;

    /// Reference to element at `index` (panics if out of bounds).
    /// Work Θ(1), Span Θ(1).
    fn nth(&self, index: N) -> &T;

    /// Set element at `index` (Err if out of bounds).
    /// Work Θ(1), Span Θ(1).
    fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str>;

    /// Append at end (grow by 1).
    /// Work amortized Θ(1), worst case Θ(2·length(&self)); Span amortized Θ(1), worst case Θ(2·length(&self)).
    /// Vec typically doubles capacity on growth; one growth copies existing elements once, then frees the old buffer.
    fn add_last(&mut self, value: T) -> &mut Self;

    /// Remove last if any (shrink by 1).
    /// Work Θ(1), Span Θ(1).
    fn delete_last(&mut self) -> Option<T>;

    /// View of [start, start+length); if beyond end, it stops at the end.
    /// Work Θ(1), Span Θ(1).
    fn subseq(&self, start: N, length: N) -> &[T];

    /// Owning subsequence starting at `start` with the given `length` (clones elements).
    /// Work: Θ(length)
    /// Span: Θ(1)
    fn subseq_copy(&self, start: N, length: N) -> Self;

    /// True iff the sequence has length zero.
    /// Work: Θ(1)
    /// Span: Θ(1)
    fn isEmpty(&self) -> B;

    /// True iff the sequence has length one.
    /// Work: Θ(1)
    /// Span: Θ(1)
    fn isSingleton(&self) -> B;

    /// Domain indices 0..len-1.
    /// Work Θ(length(&self)), Span Θ(1).
    fn domain(&self) -> Vec<N>;

    /// Range where duplicates will not be added (preserves the ordering of the elements).
    /// Work Θ(length(&self)), Span Θ(1).
    fn range(&self) -> Vec<T>
    where
        T: Hash;

    /// Multiset (count, value) ordered by the first occurrence of value in the range.
    /// Work Θ(length(&self)), Span Θ(1).
    fn multiset_range(&self) -> Vec<(N, T)>
    where
        T: Hash;
}

impl<T: StT> MathSeqTrait<T> for MathSeqS<T> {
    /// Work: Θ(length), Span: Θ(1).
    fn new(length: N, init_value: T) -> Self { MathSeqS { data: vec![init_value; length] } }

    /// Work: Θ(1), Span: Θ(1).
    fn empty() -> Self { MathSeqS { data: Vec::new() } }

    /// Work: Θ(1), Span: Θ(1).
    fn singleton(item: T) -> Self { MathSeqS { data: vec![item] } }

    /// Work: Θ(1), Span: Θ(1).
    fn length(&self) -> N {
        self.data.len()
    }

    /// Work/Span: Θ(1), Θ(1).
    fn nth(&self, index: N) -> &T {
        &self.data[index]
    }

    /// Work/Span: Θ(1), Θ(1).
    fn set(&mut self, index: N, value: T) -> Result<&mut Self, &'static str> {
        if index < self.data.len() {
            self.data[index] = value;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }

    /// Work/Span: amortized Θ(1), Θ(1); worst‑case Θ(2·length(&self)), Θ(2·length(&self)).
    /// Vec typically doubles capacity when it out grows Vec's allocated storage. Rust will set
    /// this automatically or you can set a total size.
    fn add_last(&mut self, value: T) -> &mut Self {
        self.data.push(value);
        self
    }

    /// Work/Span: Θ(1), Θ(1).
    fn delete_last(&mut self) -> Option<T> {
        self.data.pop()
    }

    /// Work/Span: Θ(1), Θ(1).
    fn subseq(&self, start: N, length: N) -> &[T] {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        &self.data[s..e]
    }

    /// Work: Θ(length), Span: Θ(1).
    fn subseq_copy(&self, start: N, length: N) -> Self {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s {
            return MathSeqS { data: Vec::new() };
        }
        MathSeqS {
            data: self.data[s..e].to_vec(),
        }
    }

    /// Work: Θ(1), Span: Θ(1).
    fn isEmpty(&self) -> B {
        if self.data.is_empty() {
            B::True
        } else {
            B::False
        }
    }

    /// Work: Θ(1), Span: Θ(1).
    fn isSingleton(&self) -> B {
        if self.data.len() == 1 {
            B::True
        } else {
            B::False
        }
    }

    /// Work/Span: Θ(length(&self)), Θ(1).
    fn domain(&self) -> Vec<N> {
        (0..self.data.len()).collect()
    }

    /// Work/Span: Θ(length(&self)), Θ(1).
    fn range(&self) -> Vec<T>
    where
        T: Hash,
    {
        let mut seen: HashSet<T> = HashSet::with_capacity(self.data.len());
        let mut out: Vec<T> = Vec::with_capacity(self.data.len());
        for x in self.data.iter() {
            if seen.insert(x.clone()) { out.push(x.clone()); }
        }
        out
    }

    /// Work/Span: Θ(length(&self)), Θ(1).
    fn multiset_range(&self) -> Vec<(N, T)>
    where
        T: Hash,
    {
        let mut counts: HashMap<T, N> = HashMap::with_capacity(self.data.len());
        let mut order: Vec<T> = Vec::new();
        for x in self.data.iter() {
            match counts.entry(x.clone()) {
                Entry::Vacant(e) => { e.insert(1); order.push(x.clone()); }
                Entry::Occupied(mut e) => { *e.get_mut() += 1; }
            }
        }
        order.into_iter().map(|x| (*counts.get(&x).unwrap(), x)).collect()
    }
}

#[macro_export]
macro_rules! MathSeq {
    () => { $crate::MathSeq::MathSeq::MathSeqS::empty() };
    ($x:expr; $n:expr) => { $crate::MathSeq::MathSeq::MathSeqS::with_len($n, $x) };
    ($($x:expr),* $(,)?) => { $crate::MathSeq::MathSeq::MathSeqS::from_vec(vec![$($x),*]) };
}

#[allow(dead_code)]
fn _MathSeq_macro_type_checks() {
    let _ = MathSeq![1];
    let _: crate::MathSeq::MathSeq::MathSeqS<i32> = MathSeq![];
}
}
