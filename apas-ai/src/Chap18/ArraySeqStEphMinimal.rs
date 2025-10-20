//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! MINIMAL: Trait with default implementations - impl block is nearly empty!

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::slice::Iter;
use std::vec::IntoIter;

use crate::Types::Types::*;

#[derive(Clone)]
pub struct ArraySeqStEphMinimalS<T: StT> {
    pub data: Box<[T]>, // Public so trait defaults can access it
}

// ============================================================================
// TRAIT WITH DEFAULT IMPLEMENTATIONS
// The impl block below only needs to provide the primitive operations!
// All other methods have default implementations using those primitives.
// ============================================================================

pub trait ArraySeqStEphMinimalTrait<T: StT>: Sized {
    // REQUIRED: Implementer must provide these
    fn from_vec(elts: Vec<T>) -> Self;
    fn data(&self) -> &[T];
    fn data_mut(&mut self) -> &mut [T];

    // DEFAULT IMPLEMENTATIONS: Free for any implementer!
    fn new(length: N, init_value: T) -> Self { Self::from_vec(vec![init_value; length]) }

    fn empty() -> Self { Self::from_vec(Vec::new()) }

    fn singleton(item: T) -> Self { Self::from_vec(vec![item]) }

    fn length(&self) -> N { self.data().len() }

    fn nth(&self, index: N) -> &T { &self.data()[index] }

    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
        let data = self.data_mut();
        if index < data.len() {
            data[index] = item;
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

    fn iter(&self) -> Iter<'_, T> { self.data().iter() }
}

// ============================================================================
// MINIMAL IMPL BLOCK - only provides the 3 required primitive operations!
// Everything else comes from the trait's default implementations.
// ============================================================================

impl<T: StT> ArraySeqStEphMinimalTrait<T> for ArraySeqStEphMinimalS<T> {
    fn from_vec(elts: Vec<T>) -> Self {
        Self {
            data: elts.into_boxed_slice(),
        }
    }

    fn data(&self) -> &[T] { &self.data }

    fn data_mut(&mut self) -> &mut [T] { &mut self.data }

    // That's it! All other methods (new, empty, singleton, length, nth, set,
    // update, iter) are automatically provided by the trait's defaults.
}

// Standard trait impls
impl<T: StT> PartialEq for ArraySeqStEphMinimalS<T> {
    fn eq(&self, other: &Self) -> bool { self.data[..] == other.data[..] }
}

impl<T: StT> Eq for ArraySeqStEphMinimalS<T> {}

impl<T: StT> Debug for ArraySeqStEphMinimalS<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { f.debug_list().entries(self.data.iter()).finish() }
}

impl<T: StT> Display for ArraySeqStEphMinimalS<T> {
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

impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphMinimalS<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.data.iter() }
}

impl<T: StT> IntoIterator for ArraySeqStEphMinimalS<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter { self.data.into_vec().into_iter() }
}
