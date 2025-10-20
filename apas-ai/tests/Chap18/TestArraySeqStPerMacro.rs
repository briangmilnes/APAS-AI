//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStPerSLit;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::{
    ArraySeqStPerBaseTrait, ArraySeqStPerRedefinableTrait, ArraySeqStPerS, ArraySeqStPerS as Seq,
};
use apas_ai::Types::Types::*;

#[test]
fn arrayseq_stper_macro_empty() {
    let seq: Seq<i32> = ArraySeqStPerSLit![];
    assert_eq!(<ArraySeqStPerS<i32> as ArraySeqStPerBaseTrait<i32>>::length(&seq), 0);
    assert!(<ArraySeqStPerS<i32> as ArraySeqStPerRedefinableTrait<i32>>::isEmpty(
        &seq
    ));
}

#[test]
fn arrayseq_stper_macro_literal() {
    let seq = ArraySeqStPerSLit![1, 2, 3];
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::length(&seq),
        3
    );
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&seq, 1),
        &2
    );
}

#[test]
fn arrayseq_stper_macro_repeat() {
    let seq = ArraySeqStPerSLit!["rust"; 2];
    assert_eq!(<ArraySeqStPerS<&str> as ArraySeqStPerBaseTrait<&str>>::length(&seq), 2);
    assert_eq!(
        <ArraySeqStPerS<&str> as ArraySeqStPerBaseTrait<&str>>::nth(&seq, 0),
        &"rust"
    );
    assert_eq!(
        <ArraySeqStPerS<&str> as ArraySeqStPerBaseTrait<&str>>::nth(&seq, 1),
        &"rust"
    );
    assert!(!<ArraySeqStPerS<&str> as ArraySeqStPerRedefinableTrait<&str>>::isSingleton(&seq));
}

#[test]
fn arrayseq_stper_operations() {
    let tabulated = <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::tabulate(&|i| i * 2, 4);
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&tabulated, 2),
        &4
    );

    let mapped = <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::map(&tabulated, &|value| value + 1);
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&mapped, 3),
        &7
    );

    let subseq = mapped.subseq_copy(1, 2);
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::length(&subseq),
        2
    );
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&subseq, 0),
        &3
    );

    let appended =
        <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::append(&mapped, &ArraySeqStPerSLit![42]);
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::length(&appended),
        5
    );
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&appended, 4),
        &42
    );

    let a = ArraySeqStPerSLit![1, 2, 3, 4, 5];
    let filtered =
        <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::filter(&a, &|value| *value % 2 == 0);
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::length(&filtered),
        2
    );

    let nested = ArraySeqStPerSLit![
        ArraySeqStPerSLit![1, 2],
        ArraySeqStPerSLit![3],
        ArraySeqStPerSLit![4, 5]
    ];
    let flattened = <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::flatten(&nested);
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&flattened, 3),
        &4
    );

    let injected = <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::inject(
        &flattened,
        &ArraySeqStPerSLit![Pair(0, 99), Pair(0, 100), Pair(4, 77)],
    );
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&injected, 0),
        &99
    );
    assert_eq!(
        <ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&injected, 4),
        &77
    );

    let collected = <ArraySeqStPerS<Pair<&str, usize>> as ArraySeqStPerBaseTrait<Pair<&str, usize>>>::collect(
        &ArraySeqStPerSLit![Pair("a", 1), Pair("b", 2), Pair("a", 3)],
        |lhs: &&str, rhs: &&str| lhs.cmp(rhs),
    );
    assert_eq!(
        <ArraySeqStPerS<Pair<&str, ArraySeqStPerS<usize>>> as ArraySeqStPerBaseTrait<
            Pair<&str, ArraySeqStPerS<usize>>,
        >>::length(&collected),
        2
    );

    let iterated =
        <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::iterate(&a, &|acc, item| acc + item, 0);
    assert_eq!(iterated, 15);

    let reduced =
        <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::reduce(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(reduced, 15);

    let (prefixes, total) =
        <ArraySeqStPerS<usize> as ArraySeqStPerRedefinableTrait<usize>>::scan(&flattened, &|lhs, rhs| lhs + rhs, 0);
    assert_eq!(total, 15);
    // Convert prefixes to Vec for set equality comparison
    let prefixes_vec: Vec<usize> = (0..<ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::length(&prefixes))
        .map(|i| *<ArraySeqStPerS<usize> as ArraySeqStPerBaseTrait<usize>>::nth(&prefixes, i))
        .collect();
    let mut prefixes_sorted = prefixes_vec.clone();
    prefixes_sorted.sort();
    let mut expected_sorted = vec![0, 1, 3, 6, 10];
    expected_sorted.sort();
    assert_eq!(prefixes_sorted, expected_sorted);
}
