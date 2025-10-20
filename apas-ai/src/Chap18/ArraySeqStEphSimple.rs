//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Simplified single-threaded ephemeral array sequence - TRAIT ONLY, NO DELEGATION.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::slice::Iter;
use std::vec::IntoIter;

use crate::Types::Types::*;

#[derive(Clone)]
pub struct ArraySeqStEphSimpleS<T: StT> {
    data: Box<[T]>,
}

// ============================================================================
// TRAIT ONLY - No inherent impl needed!
// All logic lives in the trait implementation.
// You can still call methods like: seq.length(), seq.nth(0), etc.
// Rust automatically makes trait methods available as regular methods!
// ============================================================================

pub trait ArraySeqStEphSimpleTrait<T: StT> {
    fn new(length: N, init_value: T) -> Self;
    fn empty() -> Self;
    fn singleton(item: T) -> Self;
    fn from_vec(elts: Vec<T>) -> Self;
    fn length(&self) -> N;
    fn nth(&self, index: N) -> &T;
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
    fn update(&mut self, update: Pair<N, T>) -> &mut Self;
    fn iter(&self) -> Iter<'_, T>;
}

impl<T: StT> ArraySeqStEphSimpleTrait<T> for ArraySeqStEphSimpleS<T> {
    fn from_vec(elts: Vec<T>) -> Self {
        Self {
            data: elts.into_boxed_slice(),
        }
    }

    fn new(length: N, init_value: T) -> Self {
        Self::from_vec(vec![init_value; length])
    }

    fn empty() -> Self {
        Self::from_vec(Vec::new())
    }

    fn singleton(item: T) -> Self {
        Self::from_vec(vec![item])
    }

    fn length(&self) -> N {
        self.data.len()
    }

    fn nth(&self, index: N) -> &T {
        &self.data[index]
    }

    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
        if index < self.data.len() {
            self.data[index] = item;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }

    fn update(&mut self, update: Pair<N, T>) -> &mut Self {
        let Pair(index, item) = update;
        let _ = self.set(index, item);
        self
    }

    fn iter(&self) -> Iter<'_, T> {
        self.data.iter()
    }
}

// Standard trait impls for convenience
impl<T: StT> PartialEq for ArraySeqStEphSimpleS<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data[..] == other.data[..]
    }
}

impl<T: StT> Eq for ArraySeqStEphSimpleS<T> {}

impl<T: StT> Debug for ArraySeqStEphSimpleS<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl<T: StT> Display for ArraySeqStEphSimpleS<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[")?;
        for (i, item) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphSimpleS<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T: StT> IntoIterator for ArraySeqStEphSimpleS<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}
