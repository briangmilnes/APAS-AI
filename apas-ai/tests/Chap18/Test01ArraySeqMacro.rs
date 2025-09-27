//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqS;
use apas_ai::Chap18::ArraySeq::ArraySeq::{ArraySeqS, ArraySeq};
use apas_ai::Types::Types::{B, Pair};

#[test]
fn arrayseqs_empty_macro() {
    let seq: ArraySeqS<i32> = ArraySeqS![];
    assert_eq!(ArraySeq::<i32>::length(&seq), 0);
    assert_eq!(ArraySeq::<i32>::isEmpty(&seq), B::True);
}

#[test]
fn arrayseqs_literal_macro_keeps_order() {
    let seq = ArraySeqS![1, 2, 3, 4];
    assert_eq!(ArraySeq::<i32>::length(&seq), 4);
    assert_eq!(ArraySeq::<i32>::nth(&seq, 0), &1);
    assert_eq!(ArraySeq::<i32>::nth(&seq, 3), &4);
}

#[test]
fn arrayseqs_repeat_macro_clones_element() {
    let seq = ArraySeqS!["hi"; 3];
    assert_eq!(ArraySeq::<&str>::length(&seq), 3);
    for index in 0..3 {
        assert_eq!(ArraySeq::<&str>::nth(&seq, index), &"hi");
    }
    assert_eq!(ArraySeq::<&str>::isSingleton(&seq), B::False);
}

#[test]
fn arrayseq_tabulate_and_map_work() {
    let squares = ArraySeq::<usize>::tabulate(&|i| i * i, 5);
    assert_eq!(ArraySeq::<usize>::length(&squares), 5);
    assert_eq!(ArraySeq::<usize>::nth(&squares, 3), &9);

    let doubled = ArraySeq::map(&squares, &|value| value * 2);
    assert_eq!(ArraySeq::<usize>::nth(&doubled, 0), &0);
    assert_eq!(ArraySeq::<usize>::nth(&doubled, 4), &32);
}

#[test]
fn arrayseq_subseq_append_filter_flatten() {
    let seq = ArraySeqS![1, 2, 3, 4, 5];
    let middle = ArraySeq::<i32>::subseq(&seq, 1, 3);
    assert_eq!(ArraySeq::<i32>::length(&middle), 3);
    assert_eq!(ArraySeq::<i32>::nth(&middle, 0), &2);
    assert_eq!(ArraySeq::<i32>::nth(&middle, 2), &4);

    let tail = ArraySeqS![6, 7];
    let combined = ArraySeq::<i32>::append(&seq, &tail);
    assert_eq!(ArraySeq::<i32>::length(&combined), 7);
    assert_eq!(ArraySeq::<i32>::nth(&combined, 5), &6);

    let evens = ArraySeq::<i32>::filter(&combined, &|value| match value % 2 {
        0 => B::True,
        _ => B::False,
    });
    assert_eq!(ArraySeq::<i32>::length(&evens), 3);
    assert_eq!(ArraySeq::<i32>::nth(&evens, 1), &4);

    let nested = ArraySeqS::from_vec(vec![ArraySeqS![1, 2], ArraySeqS![3], ArraySeqS![4, 5]]);
    let flat = ArraySeq::<i32>::flatten(&nested);
    assert_eq!(ArraySeq::<i32>::length(&flat), 5);
    assert_eq!(ArraySeq::<i32>::nth(&flat, 3), &4);
}

#[test]
fn arrayseq_update_and_inject_preserve_original() {
    let original = ArraySeqS![10, 20, 30];
    let updated = ArraySeq::<i32>::update(&original, Pair(1, 99));

    assert_eq!(ArraySeq::<i32>::nth(&original, 1), &20);
    assert_eq!(ArraySeq::<i32>::nth(&updated, 1), &99);

    let updates = ArraySeqS![Pair(0, -1), Pair(0, -5), Pair(2, -3)];
    let injected = ArraySeq::<i32>::inject(&updated, &updates);
    assert_eq!(ArraySeq::<i32>::nth(&injected, 0), &-1);
    assert_eq!(ArraySeq::<i32>::nth(&injected, 1), &99);
    assert_eq!(ArraySeq::<i32>::nth(&injected, 2), &-3);
}

#[test]
fn arrayseq_collect_iterate_reduce_scan() {
    let pairs = ArraySeqS![Pair("a", 1), Pair("a", 2), Pair("b", 3)];
    let collected = ArraySeq::collect(&pairs, |lhs, rhs| lhs.cmp(rhs));
    assert_eq!(ArraySeq::<Pair<&str, ArraySeqS<i32>>>::length(&collected), 2);

    let first_group = ArraySeq::<Pair<&str, ArraySeqS<i32>>>::nth(&collected, 0);
    assert_eq!(first_group.0, "a");
    assert_eq!(ArraySeq::<i32>::length(&first_group.1), 2);

    let seq = ArraySeqS![1, 2, 3, 4];
    let iter_sum = ArraySeq::<i32>::iterate(&seq, &|acc, item| *acc + *item, 0);
    assert_eq!(iter_sum, 10);

    let reduced = ArraySeq::<i32>::reduce(&seq, &|lhs, rhs| *lhs + *rhs, 0);
    assert_eq!(reduced, 10);

    let (prefixes, total) = ArraySeq::<i32>::scan(&seq, &|lhs, rhs| *lhs + *rhs, 0);
    assert_eq!(total, 10);
    assert_eq!(ArraySeq::<i32>::nth(&prefixes, 0), &1);
    assert_eq!(ArraySeq::<i32>::nth(&prefixes, 3), &10);
}
