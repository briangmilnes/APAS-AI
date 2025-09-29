//! Chap41Codex Example 41.3: sequential iteration versus parallel union construction.

pub mod Example41_3 {
    use std::thread;

    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS as ArraySeqSt;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS as ArraySeqMt;
    use crate::Chap41Codex::AVLTreeSetMtEph::AVLTreeSetMtEph::{AVLTreeSetMtEph, AVLTreeSetMtEphTrait};
    use crate::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{AVLTreeSetStEph, AVLTreeSetStEphTrait};
    use crate::Chap41Codex::ArraySetEnumMtEph::ArraySetEnumMtEph::{ArraySetEnumMtEph, ArraySetEnumMtEphTrait};
    use crate::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::{ArraySetEnumStEph, ArraySetEnumStEphTrait};
    use crate::Types::Types::*;

    /// Sequential construction via iterative inserts (matching textbook `Seq.iterate`).
    pub fn array_enum_from_seq_iter(universe: N, seq: &ArraySeqSt<N>) -> ArraySetEnumStEph {
        let mut set = ArraySetEnumStEph::empty(universe);
        for index in 0..seq.length() {
            let value = seq.nth(index);
            let _ = set.insert(*value);
        }
        set
    }

    fn array_enum_reduce_span(universe: N, seq: &ArraySeqMt<N>, start: N, end: N) -> ArraySetEnumMtEph {
        if start >= end {
            return ArraySetEnumMtEph::empty(universe);
        }
        if end - start == 1 {
            return ArraySetEnumMtEph::singleton(universe, seq.nth_cloned(start));
        }
        let mid = start + (end - start) / 2;
        let left = array_enum_reduce_span(universe, seq, start, mid);
        let right = array_enum_reduce_span(universe, seq, mid, end);
        left.union(&right)
    }

    /// Parallel-friendly construction using divide-and-conquer unions.
    pub fn array_enum_from_seq_reduce(universe: N, seq: &ArraySeqMt<N>) -> ArraySetEnumMtEph {
        if seq.length() <= 1 {
            return array_enum_reduce_span(universe, seq, 0, seq.length());
        }
        thread::scope(|scope| {
            let mid = seq.length() / 2;
            let left_handle = scope.spawn(move || array_enum_reduce_span(universe, seq, 0, mid));
            let right = array_enum_reduce_span(universe, seq, mid, seq.length());
            let left = left_handle.join().unwrap();
            left.union(&right)
        })
    }

    fn avl_reduce_span<T: StT + Ord>(seq: &ArraySeqSt<T>, start: N, end: N) -> AVLTreeSetStEph<T> {
        if start >= end {
            return AVLTreeSetStEph::empty();
        }
        if end - start == 1 {
            return AVLTreeSetStEph::singleton(seq.nth(start).clone());
        }
        let mid = start + (end - start) / 2;
        let left = avl_reduce_span(seq, start, mid);
        let right = avl_reduce_span(seq, mid, end);
        left.union(&right)
    }

    /// Single-threaded iterate baseline for AVL sets.
    pub fn avl_from_seq_iter<T: StT + Ord>(seq: &ArraySeqSt<T>) -> AVLTreeSetStEph<T> {
        let mut acc = AVLTreeSetStEph::empty();
        for index in 0..seq.length() {
            acc.insert(seq.nth(index).clone());
        }
        acc
    }

    /// Divide-and-conquer (parallel-style) union for AVL sets.
    pub fn avl_from_seq_reduce<T: StT + Ord>(seq: &ArraySeqSt<T>) -> AVLTreeSetStEph<T> {
        avl_reduce_span(seq, 0, seq.length())
    }

    fn avl_mt_reduce_span<T: MtKey>(seq: &ArraySeqMt<T>, start: N, end: N) -> AVLTreeSetMtEph<T> {
        if start >= end {
            return AVLTreeSetMtEph::empty();
        }
        if end - start == 1 {
            return AVLTreeSetMtEph::singleton(seq.nth_cloned(start));
        }
        let mid = start + (end - start) / 2;
        let left = avl_mt_reduce_span(seq, start, mid);
        let right = avl_mt_reduce_span(seq, mid, end);
        left.union(&right)
    }

    /// Parallel AVL set construction using scoped threads for the top split.
    pub fn avl_mt_from_seq_reduce<T: MtKey>(seq: &ArraySeqMt<T>) -> AVLTreeSetMtEph<T> {
        if seq.length() <= 1 {
            return avl_mt_reduce_span(seq, 0, seq.length());
        }
        thread::scope(|scope| {
            let mid = seq.length() / 2;
            let left_handle = scope.spawn(move || avl_mt_reduce_span(seq, 0, mid));
            let right = avl_mt_reduce_span(seq, mid, seq.length());
            let left = left_handle.join().unwrap();
            left.union(&right)
        })
    }
}
