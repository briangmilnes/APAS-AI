//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStPerS;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::{ArraySeqStPerS as Seq, ArraySeqStPerTrait};
use apas_ai::Types::Types::{B, Pair};

#[test]
fn arrayseq_stper_macro_empty() {
    let seq: Seq<i32> = ArraySeqStPerS![];
    assert_eq!(ArraySeqStPerTrait::<i32>::length(&seq), 0);
    assert_eq!(ArraySeqStPerTrait::<i32>::isEmpty(&seq), B::True);
}

#[test]
fn arrayseq_stper_macro_literal() {
    let seq = ArraySeqStPerS![1, 2, 3];
    assert_eq!(ArraySeqStPerTrait::<i32>::length(&seq), 3);
    assert_eq!(ArraySeqStPerTrait::<i32>::nth(&seq, 1), &2);
}

#[test]
fn arrayseq_stper_macro_repeat() {
    let seq = ArraySeqStPerS!["rust"; 2];
    assert_eq!(ArraySeqStPerTrait::<&str>::length(&seq), 2);
    assert_eq!(ArraySeqStPerTrait::<&str>::nth(&seq, 0), &"rust");
    assert_eq!(ArraySeqStPerTrait::<&str>::nth(&seq, 1), &"rust");
    assert_eq!(ArraySeqStPerTrait::<&str>::isSingleton(&seq), B::False);
}

#[test]
fn arrayseq_stper_operations() {
    let tabulated = ArraySeqStPerTrait::tabulate(|i| i * 2, 4);
    assert_eq!(ArraySeqStPerTrait::<usize>::nth(&tabulated, 2), &4);

    let mapped = ArraySeqStPerTrait::map(&tabulated, |value| value + 1);
    assert_eq!(ArraySeqStPerTrait::<usize>::nth(&mapped, 3), &7);

    let subseq = ArraySeqStPerTrait::subseq_copy(&mapped, 1, 2);
    assert_eq!(ArraySeqStPerTrait::<usize>::length(&subseq), 2);
    assert_eq!(ArraySeqStPerTrait::<usize>::nth(&subseq, 0), &5);

    let appended = ArraySeqStPerTrait::append(&mapped, &ArraySeqStPerS![42]);
    assert_eq!(ArraySeqStPerTrait::<usize>::length(&appended), 5);
    assert_eq!(ArraySeqStPerTrait::<usize>::nth(&appended, 4), &42);

    let filtered = ArraySeqStPerTrait::filter(&appended, |value| {
        if *value % 2 == 0 { B::True } else { B::False }
    });
    assert_eq!(ArraySeqStPerTrait::<usize>::length(&filtered), 3);

    let nested = ArraySeqStPerS![ArraySeqStPerS![1, 2], ArraySeqStPerS![3], ArraySeqStPerS![4, 5]];
    let flattened = ArraySeqStPerTrait::flatten(&nested);
    assert_eq!(ArraySeqStPerTrait::<i32>::nth(&flattened, 3), &4);

    let injected = ArraySeqStPerTrait::inject(&flattened, &ArraySeqStPerS![Pair(0, 99), Pair(0, 100), Pair(4, 77)]);
    assert_eq!(ArraySeqStPerTrait::<i32>::nth(&injected, 0), &99);
    assert_eq!(ArraySeqStPerTrait::<i32>::nth(&injected, 4), &77);

    let collected = ArraySeqStPerTrait::collect(
        &ArraySeqStPerS![Pair("a", 1), Pair("b", 2), Pair("a", 3)],
        |lhs, rhs| lhs.cmp(rhs),
    );
    assert_eq!(ArraySeqStPerTrait::<Pair<&str, Seq<i32>>>::length(&collected), 2);

    let iterated = ArraySeqStPerTrait::iterate(&flattened, |acc, item| acc + item, 0);
    assert_eq!(iterated, 15);

    let reduced = ArraySeqStPerTrait::reduce(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(reduced, 15);

    let (prefixes, total) = ArraySeqStPerTrait::scan(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(total, 15);
    assert_eq!(ArraySeqStPerTrait::<i32>::nth(&prefixes, 2), &6);
}
