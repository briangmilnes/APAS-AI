//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! CLEAN: One-line defaults in trait, multi-line implementations in impl block.

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::slice::Iter;
use std::vec::IntoIter;

use crate::Types::Types::*;

#[derive(Clone)]
pub struct ArraySeqStEphCleanS<T: StT> {
    pub data: Box<[T]>,
}

pub trait ArraySeqStEphCleanTrait<T: StT>: Sized {
    // REQUIRED PRIMITIVES: Implementer must provide these
    fn from_vec(elts: Vec<T>)                -> Self;
    fn data(&self)                           -> &[T];
    fn data_mut(&mut self)                   -> &mut [T];

    // ONE-LINE DEFAULTS: Readable directly in trait
    fn new(length: N, init_value: T)         -> Self {
        Self::from_vec(vec![init_value; length])
    }
    fn empty()                               -> Self {
        Self::from_vec(Vec::new())
    }
    fn singleton(item: T)                    -> Self {
        Self::from_vec(vec![item])
    }
    fn length(&self)                         -> N {
        self.data().len()
    }
    fn nth(&self, index: N)                  -> &T {
        &self.data()[index]
    }
    fn iter(&self)                           -> Iter<'_, T> {
        self.data().iter()
    }

    // MULTI-LINE DEFAULTS: Type signature here, implementation in impl block below
    fn set(&mut self, index: N, item: T)     -> Result<&mut Self, &'static str>;
    fn update(&mut self, update: Pair<N, T>) -> &mut Self;
}

impl<T: StT> ArraySeqStEphCleanTrait<T> for ArraySeqStEphCleanS<T> {
    // REQUIRED PRIMITIVES
    fn from_vec(elts: Vec<T>) -> Self {
        Self {
            data: elts.into_boxed_slice(),
        }
    }

    fn data(&self) -> &[T] {
        &self.data
    }

    fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }

    // MULTI-LINE DEFAULTS (trait only has signature, we provide body here)
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
}

// Standard trait impls
impl<T: StT> PartialEq for ArraySeqStEphCleanS<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data[..] == other.data[..]
    }
}

impl<T: StT> Eq for ArraySeqStEphCleanS<T> {}

impl<T: StT> Debug for ArraySeqStEphCleanS<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.data.iter()).finish()
    }
}

impl<T: StT> Display for ArraySeqStEphCleanS<T> {
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

impl<'a, T: StT> IntoIterator for &'a ArraySeqStEphCleanS<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<T: StT> IntoIterator for ArraySeqStEphCleanS<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_vec().into_iter()
    }
}
