//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Types::Types::*;

#[test]
fn test_arrayseq_mteph_basic_ops() {
    let mut a = ArraySeqMtEphSLit![0; 5];
    for i in 0..a.length() {
        a.set(i, i as i32);
    }
    for i in 0..a.length() {
        assert_eq!(a.nth_cloned(i), i as i32);
    }
    assert_eq!(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::isEmpty(&a), false);
    assert_eq!(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::isSingleton(&a), false);
    let s = a.subseq_copy(1, 3);
    assert_eq!(s.length(), 3);
    assert_eq!(s.nth_cloned(0), 1);
    assert_eq!(s.nth_cloned(2), 3);
}

#[test]
fn test_arrayseq_mteph_append_and_map() {
    let a = ArraySeqMtEphSLit![1, 2, 3];
    let b = ArraySeqMtEphSLit![4, 5];
    let c = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&a, &b);
    assert_eq!(c.length(), 5);
    let d = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::map(&a, &|x| x + 1);
    assert_eq!(d.nth_cloned(0), 2);
    assert_eq!(d.nth_cloned(4), 6);
}
