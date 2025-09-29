//! Chap41Codex Example 41.3: building sets from sequences via singleton union reduction.

pub mod Example41_3 {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS as ArraySeqStEph;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS as ArraySeqMtEph;
    use crate::Chap41Codex::AVLTreeSetMtEph::AVLTreeSetMtEph::{AVLTreeSetMtEph, AVLTreeSetMtEphTrait};
    use crate::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{AVLTreeSetStEph, AVLTreeSetStEphTrait};
    use crate::Chap41Codex::ArraySetEnumMtEph::ArraySetEnumMtEph::{
        ArraySetEnumMtEph, ArraySetEnumMtEphS, ArraySetEnumMtEphTrait,
    };
    use crate::Types::Types::*;

    /// Array-set version: parallel-friendly reduce of singleton sets over `[0, u)` universe.
    pub fn array_enum_from_seq(universe: N, seq: &ArraySeqMtEph<N>) -> ArraySetEnumMtEphS {
        if seq.length() == 0 {
            return ArraySetEnumMtEph::empty(universe);
        }
        let mut accumulator = ArraySetEnumMtEph::empty(universe);
        for index in 0..seq.length() {
            let value = seq.nth_cloned(index);
            let singleton = ArraySetEnumMtEph::singleton(universe, value);
            accumulator = accumulator.union(&singleton);
        }
        accumulator
    }

    /// AVL-tree set (single-threaded) constructed via singleton union reduction.
    pub fn avl_from_seq<T: StT + Ord>(seq: &ArraySeqStEph<T>) -> AVLTreeSetStEph<T> {
        if seq.length() == 0 {
            return AVLTreeSetStEph::empty();
        }
        let mut accumulator = AVLTreeSetStEph::empty();
        for index in 0..seq.length() {
            let value = seq.nth(index).clone();
            let singleton = AVLTreeSetStEph::singleton(value.clone());
            accumulator = accumulator.union(&singleton);
        }
        accumulator
    }

    /// AVL-tree set (multi-threaded) constructed via singleton union reduction.
    pub fn avl_mt_from_seq<T: StTInMtT + Ord + 'static>(seq: &ArraySeqMtEph<T>) -> AVLTreeSetMtEph<T> {
        if seq.length() == 0 {
            return AVLTreeSetMtEph::empty();
        }
        let mut accumulator = AVLTreeSetMtEph::empty();
        for index in 0..seq.length() {
            let value = seq.nth_cloned(index);
            let singleton = AVLTreeSetMtEph::singleton(value.clone());
            accumulator = accumulator.union(&singleton);
        }
        accumulator
    }
}
