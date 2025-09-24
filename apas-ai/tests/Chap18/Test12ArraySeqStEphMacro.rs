//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStEphS;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
use apas_ai::Types::Types::{B, Pair};

#[test]
fn arrayseq_steph_basic_macros() {
    let mut seq = ArraySeqStEphS![1, 2, 3];
    assert_eq!(ArraySeqStEphTrait::<i32>::length(&seq), 3);
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&seq, 0), &1);
    let _ = ArraySeqStEphTrait::<i32>::set(&mut seq, 1, 42).unwrap();
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&seq, 1), &42);
}

#[test]
fn arrayseq_steph_full_pipeline() {
    let seq = ArraySeqStEphTrait::tabulate(|i| (i + 1) as i32, 5);
    let mapped = ArraySeqStEphTrait::map(&seq, |value| value * 2);
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&mapped, 4), &10);

    let subseq = ArraySeqStEphTrait::subseq(&mapped, 1, 3);
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&subseq, 2), &8);

    let appended = ArraySeqStEphTrait::append(&mapped, &ArraySeqStEphS![99]);
    let evens = ArraySeqStEphTrait::filter(&appended, |value| if *value % 2 == 0 { B::True } else { B::False });
    assert_eq!(ArraySeqStEphTrait::<i32>::length(&evens), 5);

    let nested = ArraySeqStEphS![ArraySeqStEphS![1, 2], ArraySeqStEphS![3], ArraySeqStEphS![4, 5]];
    let flattened = ArraySeqStEphTrait::flatten(&nested);
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&flattened, 3), &4);

    let mut writable = flattened.clone();
    let _ = ArraySeqStEphTrait::update(&mut writable, Pair(0, 100));
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&writable, 0), &100);

    let _ = ArraySeqStEphTrait::inject(
        &mut writable,
        &ArraySeqStEphS![Pair(0, 7), Pair(0, 9), Pair(4, 11)],
    );
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&writable, 0), &7);
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&writable, 4), &11);

    let collected = ArraySeqStEphTrait::collect(
        &ArraySeqStEphS![Pair("x", 1), Pair("y", 2), Pair("x", 3)],
        |lhs, rhs| lhs.cmp(rhs),
    );
    assert_eq!(ArraySeqStEphTrait::<Pair<&str, ArraySeqStEphS<i32>>>::length(&collected), 2);

    let iter_sum = ArraySeqStEphTrait::iterate(&flattened, |acc, item| acc + item, 0);
    assert_eq!(iter_sum, 15);

    let reduced = ArraySeqStEphTrait::reduce(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(reduced, 15);

    let (prefixes, total) = ArraySeqStEphTrait::scan(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(total, 15);
    assert_eq!(ArraySeqStEphTrait::<i32>::nth(&prefixes, 2), &6);
}
