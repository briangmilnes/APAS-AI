//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Primitive tree sequence implementation for Chapter 23.
//!
//!  This module defines a single-threaded primitive tree sequence based on the APAS
//! specification: sequences may be exposed as `Zero`, `One`, or `Two` parts, and the
//! corresponding `join` operation reassembles a sequence while keeping it roughly
//! balanced. The balancing here chooses a midpoint split; higher chapters can replace
//! this strategy with more sophisticated tree management when needed.

pub mod PrimTreeSeqSt {
    use crate::Types::Types::*;

    /// The tree representation returned from `expose` and consumed by `join`.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum PrimTreeSeqStTree<T: StT> {
        Zero,
        One(T),
        Two(PrimTreeSeqStS<T>, PrimTreeSeqStS<T>),
    }

    /// Primitive tree sequence stored as a persistent Vec-backed collection.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct PrimTreeSeqStS<T: StT> {
        data: Vec<T>,
    }

    impl<T: StT> PrimTreeSeqStS<T> {
        /// Creates an empty sequence.
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        pub fn empty() -> Self { Self { data: Vec::new() } }

        /// Builds a sequence containing a single element.
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        pub fn singleton(value: T) -> Self { Self { data: vec![value] } }

        /// Constructs a sequence from the provided vector without additional copying.
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - just wraps vector
        pub fn from_vec(vec: Vec<T>) -> Self { Self { data: vec } }

        /// Converts this sequence into its backing vector.
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - just unwraps
        pub fn into_vec(self) -> Vec<T> { self.data }

        /// Provides a shared view of the underlying elements.
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1) - reference operation
        pub fn as_slice(&self) -> &[T] { &self.data }

        /// Returns the number of elements in the sequence.
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        pub fn length(&self) -> N { self.data.len() }

        /// Exposes the internal structure as `Zero`, `One`, or `Two` parts.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential split and copy
        pub fn expose(&self) -> PrimTreeSeqStTree<T> {
            match self.data.len() {
                | 0 => PrimTreeSeqStTree::Zero,
                | 1 => PrimTreeSeqStTree::One(self.data[0].clone()),
                | _ => {
                    let mid = self.data.len() / 2;
                    let left = self.data[..mid].to_vec();
                    let right = self.data[mid..].to_vec();
                    PrimTreeSeqStTree::Two(PrimTreeSeqStS::from_vec(left), PrimTreeSeqStS::from_vec(right))
                }
            }
        }

        /// Reassembles a primitive tree sequence from an exposed tree.
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential append
        pub fn join(tree: PrimTreeSeqStTree<T>) -> Self {
            match tree {
                | PrimTreeSeqStTree::Zero => Self::empty(),
                | PrimTreeSeqStTree::One(value) => Self::singleton(value),
                | PrimTreeSeqStTree::Two(left, right) => {
                    let mut combined = left.into_vec();
                    combined.extend(right.into_vec());
                    PrimTreeSeqStS::from_vec(combined)
                }
            }
        }
    }
}
