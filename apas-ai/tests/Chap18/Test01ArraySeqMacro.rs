//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqS;
use apas_ai::Chap18::ArraySeq::ArraySeq::{ArraySeqS, ArraySeq};
use apas_ai::Types::Types::{B, Pair};

#[test]
fn arrayseqs_empty_macro() {
    let seq: ArraySeqS<i32> = ArraySeqS![];
    assert_eq!(ArraySeq::length(&seq), 0);
    assert_eq!(ArraySeq::isEmpty(&seq), B::True);
}

#[test]
fn arrayseqs_literal_macro_keeps_order() {
    let seq = ArraySeqS![1, 2, 3, 4];
    assert_eq!(ArraySeq::length(&seq), 4);
    assert_eq!(ArraySeq::nth(&seq, 0), &1);
    assert_eq!(ArraySeq::nth(&seq, 3), &4);
}

#[test]
fn arrayseqs_repeat_macro_clones_element() {
    let seq = ArraySeqS!["hi"; 3];
    assert_eq!(ArraySeq::length(&seq), 3);
    for index in 0..3 {
        assert_eq!(ArraySeq::nth(&seq, index), &"hi");
    }
    assert_eq!(ArraySeq::isSingleton(&seq), B::False);
}

#[test]
fn arrayseq_tabulate_and_map_work() {
    let squares = <ArraySeqS<i32> as ArraySeq<i32>>::tabulate(&|i: usize| (i * i) as i32, 5);
    assert_eq!(ArraySeq::length(&squares), 5);
    assert_eq!(ArraySeq::nth(&squares, 3), &9);

    let doubled = <ArraySeqS<i32> as ArraySeq<i32>>::map(&squares, &|value| value * 2);
    assert_eq!(ArraySeq::nth(&doubled, 0), &0);
    assert_eq!(ArraySeq::nth(&doubled, 4), &32);
}

#[test]
fn arrayseq_subseq_append_filter_flatten() {
    let seq = ArraySeqS![1, 2, 3, 4, 5];
    let middle = <ArraySeqS<i32> as ArraySeq<i32>>::subseq(&seq, 1, 3);
    assert_eq!(ArraySeq::length(&middle), 3);
    assert_eq!(ArraySeq::nth(&middle, 0), &2);
    assert_eq!(ArraySeq::nth(&middle, 2), &4);

    let tail = ArraySeqS![6, 7];
    let combined = <ArraySeqS<i32> as ArraySeq<i32>>::append(&seq, &tail);
    assert_eq!(ArraySeq::length(&combined), 7);
    assert_eq!(ArraySeq::nth(&combined, 5), &6);

    let evens = <ArraySeqS<i32> as ArraySeq<i32>>::filter(&combined, &|value| match value % 2 {
        0 => B::True,
        _ => B::False,
    });
    assert_eq!(ArraySeq::length(&evens), 3);
    assert_eq!(ArraySeq::nth(&evens, 1), &4);

    let nested = ArraySeqS::from_vec(vec![ArraySeqS![1, 2], ArraySeqS![3], ArraySeqS![4, 5]]);
    let flat = <ArraySeqS<i32> as ArraySeq<i32>>::flatten(&nested);
    assert_eq!(ArraySeq::length(&flat), 5);
    assert_eq!(ArraySeq::nth(&flat, 3), &4);
}

#[test]
fn arrayseq_update_and_inject_preserve_original() {
    let original = ArraySeqS![10, 20, 30];
    let updated = <ArraySeqS<i32> as ArraySeq<i32>>::update(&original, Pair(1, 99));

    assert_eq!(ArraySeq::nth(&original, 1), &20);
    assert_eq!(ArraySeq::nth(&updated, 1), &99);

    let updates = ArraySeqS![Pair(0, -1), Pair(0, -5), Pair(2, -3)];
    let injected = <ArraySeqS<i32> as ArraySeq<i32>>::inject(&updated, &updates);
    assert_eq!(ArraySeq::nth(&injected, 0), &-1);
    assert_eq!(ArraySeq::nth(&injected, 1), &99);
    assert_eq!(ArraySeq::nth(&injected, 2), &-3);
}

#[test]
fn arrayseq_collect_iterate_reduce_scan() {
    let pairs = ArraySeqS![Pair("a", 1), Pair("a", 2), Pair("b", 3)];
    let collected = <ArraySeqS<Pair<&str, ArraySeqS<i32>>> as ArraySeq<Pair<&str, ArraySeqS<i32>>>>::collect(&pairs, |lhs, rhs| lhs.cmp(rhs));
    assert_eq!(ArraySeq::<Pair<&str, ArraySeqS<i32>>>::length(&collected), 2);

    let first_group = ArraySeq::<Pair<&str, ArraySeqS<i32>>>::nth(&collected, 0);
    assert_eq!(first_group.0, "a");
    assert_eq!(ArraySeq::length(&first_group.1), 2);

    let seq = ArraySeqS![1, 2, 3, 4];
    let iter_sum = <ArraySeqS<i32> as ArraySeq<i32>>::iterate(&seq, &|acc, item| *acc + *item, 0);
    assert_eq!(iter_sum, 10);

    let reduced = <ArraySeqS<i32> as ArraySeq<i32>>::reduce(&seq, &|lhs, rhs| *lhs + *rhs, 0);
    assert_eq!(reduced, 10);

    let (prefixes, total) = <ArraySeqS<i32> as ArraySeq<i32>>::scan(&seq, &|lhs, rhs| *lhs + *rhs, 0);
    assert_eq!(total, 10);
    assert_eq!(ArraySeq::nth(&prefixes, 0), &1);
    assert_eq!(ArraySeq::nth(&prefixes, 3), &10);
}
