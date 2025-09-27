//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::ArraySeqStEphSLit;
use apas_ai::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
use apas_ai::Types::Types::{B, Pair};

#[test]
fn arrayseq_steph_basic_macros() {
    let mut seq = ArraySeqStEphSLit![1, 2, 3];
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::length(&seq), 3);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&seq, 0), &1);
    let _ = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::set(&mut seq, 1, 42);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&seq, 1), &42);
}

#[test]
fn arrayseq_steph_full_pipeline() {
    let seq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::tabulate(&|i| (i + 1) as i32, 5);
    let mapped = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::map(&seq, &|value| value * 2);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&mapped, 4), &10);

    let subseq = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::subseq(&mapped, 1, 3);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&subseq, 2), &8);

    let appended = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::append(&mapped, &ArraySeqStEphSLit![99]);
    let evens = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::filter(&appended, &|value| if *value % 2 == 0 { B::True } else { B::False });
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::length(&evens), 5);

    let nested = ArraySeqStEphSLit![ArraySeqStEphSLit![1, 2], ArraySeqStEphSLit![3], ArraySeqStEphSLit![4, 5]];
    let flattened = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::flatten(&nested);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&flattened, 3), &4);

    let mut writable = flattened.clone();
    let _ = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::update(&mut writable, Pair(0, 100));
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&writable, 0), &100);

    let _ = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::inject(
        &mut writable,
        &ArraySeqStEphSLit![Pair(0, 7), Pair(0, 9), Pair(4, 11)],
    );
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&writable, 0), &9);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&writable, 4), &11);

    let collected = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::collect(
        &ArraySeqStEphSLit![Pair("x", 1), Pair("y", 2), Pair("x", 3)],
        |lhs, rhs| lhs.cmp(rhs),
    );
    assert_eq!(<ArraySeqStEphS<Pair<&str, ArraySeqStEphS<i32>>> as ArraySeqStEphTrait<Pair<&str, ArraySeqStEphS<i32>>>>::length(&collected), 2);

    let iter_sum = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::iterate(&flattened, &|acc, item| acc + item, 0);
    assert_eq!(iter_sum, 15);

    let reduced = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::reduce(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(reduced, 15);

    let (prefixes, total) = <ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::scan(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(total, 15);
    assert_eq!(<ArraySeqStEphS<i32> as ArraySeqStEphTrait<i32>>::nth(&prefixes, 2), &6);
}
