//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::ArraySeq::{ArrayS, ArraySeq};
use apas_ai::ArraySeqChap18::ArraySeqChap18;

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }
    let a = <ArrayS<N> as ArraySeqChap18>::tabulate(fib, 10);
    let fib10_head = apas_ai::arrayseq![
        *a.nth(0), *a.nth(1), *a.nth(2), *a.nth(3), *a.nth(4),
        *a.nth(5), *a.nth(6), *a.nth(7), *a.nth(8), *a.nth(9)
    ];
    assert_eq!(fib10_head, apas_ai::arrayseq![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = apas_ai::arrayseq![1, 2, 3, 4, 5];
    let b = <ArrayS<N> as ArraySeqChap18>::map(&a, |x| x + 1);
    assert_eq!(b, apas_ai::arrayseq![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = apas_ai::arrayseq![10, 20, 30, 40, 50];
    let b = a.subseq_copy(1, 3);
    assert_eq!(b, apas_ai::arrayseq![20, 30, 40]);
    let c = a.subseq_copy(2, 0);
    assert_eq!(c.length(), 0);
    let d = a.subseq_copy(0, 1);
    assert_eq!(d, apas_ai::arrayseq![10]);
}

#[test]
fn test_append() {
    let a = apas_ai::arrayseq![1, 2, 3];
    let b = apas_ai::arrayseq![4, 5, 6];
    let c = <ArrayS<N> as ArraySeqChap18>::append(&a, &b);
    assert_eq!(c, apas_ai::arrayseq![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = apas_ai::arrayseq![1, 2, 3];
    let b = apas_ai::arrayseq![4, 5];
    let c = <ArrayS<N> as ArraySeqChap18>::append(&a, &b);
    assert_eq!(c, apas_ai::arrayseq![1, 2, 3, 4, 5]);
    let empty: ArrayS<N> = <ArrayS<N> as ArraySeq<N>>::empty();
    let d = <ArrayS<N> as ArraySeqChap18>::append(&apas_ai::arrayseq![1,2,3], &empty);
    assert_eq!(d.length(), 3);
    let e = <ArrayS<N> as ArraySeqChap18>::append(&empty, &apas_ai::arrayseq![1,2,3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = apas_ai::arrayseq![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = <ArrayS<N> as ArraySeqChap18>::filter(&numbers, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, apas_ai::arrayseq![2, 4, 6, 8, 10]);
    let odds_only = apas_ai::arrayseq![1, 3, 5, 7];
    let no_evens = <ArrayS<N> as ArraySeqChap18>::filter(&odds_only, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = apas_ai::arrayseq![1, 2];
    let b = apas_ai::arrayseq![3, 4, 5];
    let c = apas_ai::arrayseq![6];
    let sequences = apas_ai::arrayseq![a, b, c];
    let flattened = <ArrayS<N> as ArraySeqChap18>::flatten(&sequences);
    assert_eq!(flattened, apas_ai::arrayseq![1, 2, 3, 4, 5, 6]);
    let empty: ArrayS<N> = <ArrayS<N> as ArraySeq<N>>::empty();
    let mixed = apas_ai::arrayseq![apas_ai::arrayseq![1, 2], empty, apas_ai::arrayseq![3]];
    let mixed_flat = <ArrayS<N> as ArraySeqChap18>::flatten(&mixed);
    assert_eq!(mixed_flat, apas_ai::arrayseq![1, 2, 3]);
}

#[test]
fn test_update_sequence() {
    let mut a = apas_ai::arrayseq!["hello", "world", "test"];
    let _ = <ArrayS<&str> as ArraySeqChap18>::update(&mut a, (1, "rust"));
    assert_eq!(a, apas_ai::arrayseq!["hello", "rust", "test"]);
    let mut c = apas_ai::arrayseq!["hello", "world", "test"];
    let _ = <ArrayS<&str> as ArraySeqChap18>::update(&mut c, (5, "out_of_bounds"));
    assert_eq!(c, apas_ai::arrayseq!["hello", "world", "test"]);
}

#[test]
fn test_inject_and_ninject() {
    let a = apas_ai::arrayseq!["the", "cat", "in", "the", "hat"];
    let updates = apas_ai::arrayseq![(0, "a"), (2, "on"), (4, "mat")];
    let injected = <ArrayS<&str> as ArraySeqChap18>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, apas_ai::arrayseq!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = apas_ai::arrayseq![(0, "first"), (0, "second"), (1, "updated")];
    let result_first = <ArrayS<&str> as ArraySeqChap18>::inject(&a, &conflicting_updates);
    assert_eq!(result_first, apas_ai::arrayseq!["first", "updated", "in", "the", "hat"]);

    let ninjected = <ArrayS<&str> as ArraySeqChap18>::ninject(&a, &apas_ai::arrayseq![(1, "dog"), (3, "big"), (6, "hog")]);
    assert_eq!(ninjected, apas_ai::arrayseq!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArrayS<&str> as ArraySeqChap18>::ninject(&a, &apas_ai::arrayseq![(0, "first"), (0, "second"), (1, "updated")]);
    assert_eq!(result_last, apas_ai::arrayseq!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = apas_ai::arrayseq![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArrayS<N> as ArraySeqChap18>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArrayS<N> = <ArrayS<N> as ArraySeq<N>>::empty();
    let empty_result = <ArrayS<N> as ArraySeqChap18>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = apas_ai::arrayseq![100];
    let single_result = <ArrayS<N> as ArraySeqChap18>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArrayS<N> as ArraySeqChap18>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}


