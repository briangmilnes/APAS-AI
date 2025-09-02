//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Sequences::*;
use apas_ai::APAS18::APAS18;

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }
    let a = <S<N> as APAS18>::tabulate(fib, 10);
    let fib10_head = apas_ai::seq![*a.nth(0), *a.nth(1), *a.nth(2), *a.nth(3), *a.nth(4), *a.nth(5), *a.nth(6), *a.nth(7), *a.nth(8), *a.nth(9)];
    assert_eq!(fib10_head, apas_ai::seq![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = apas_ai::seq![1, 2, 3, 4, 5];
    let b = <S<i32> as APAS18>::map(&a, |x| x + 1);
    assert_eq!(b, apas_ai::seq![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = apas_ai::seq![10, 20, 30, 40, 50];
    let b = a.subseq(1, 3);
    assert_eq!(b, apas_ai::seq![20, 30, 40]);
    let c = a.subseq(2, 0);
    assert_eq!(c.length(), 0);
    let d = a.subseq(0, 1);
    assert_eq!(d, apas_ai::seq![10]);
}

#[test]
fn test_append() {
    let a = apas_ai::seq![1, 2, 3];
    let b = apas_ai::seq![4, 5, 6];
    let c = <S<i32> as APAS18>::append(&a, &b);
    assert_eq!(c, apas_ai::seq![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = apas_ai::seq![1, 2, 3];
    let b = apas_ai::seq![4, 5];
    let c = <S<i32> as APAS18>::append(&a, &b);
    assert_eq!(c, apas_ai::seq![1, 2, 3, 4, 5]);
    let empty: S<N> = <S<N> as Sequence<N>>::empty();
    let d = <S<N> as APAS18>::append(&apas_ai::seq![1,2,3], &empty);
    assert_eq!(d.length(), 3);
    let e = <S<N> as APAS18>::append(&empty, &apas_ai::seq![1,2,3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = apas_ai::seq![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let evens = <S<i32> as APAS18>::filter(&numbers, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, apas_ai::seq![2, 4, 6, 8, 10]);
    let odds_only = apas_ai::seq![1, 3, 5, 7];
    let no_evens = <S<i32> as APAS18>::filter(&odds_only, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = apas_ai::seq![1, 2];
    let b = apas_ai::seq![3, 4, 5];
    let c = apas_ai::seq![6];
    let sequences = apas_ai::seq![a, b, c];
    let flattened = <S<i32> as APAS18>::flatten(&sequences);
    assert_eq!(flattened, apas_ai::seq![1, 2, 3, 4, 5, 6]);
    let empty: S<i32> = <S<i32> as Sequence<i32>>::empty();
    let mixed = apas_ai::seq![apas_ai::seq![1, 2], empty, apas_ai::seq![3]];
    let mixed_flat = <S<i32> as APAS18>::flatten(&mixed);
    assert_eq!(mixed_flat, apas_ai::seq![1, 2, 3]);
}

#[test]
fn test_update_sequence() {
    let mut a = apas_ai::seq!["hello", "world", "test"];
    let _ = <S<&str> as APAS18>::update(&mut a, (1, "rust"));
    assert_eq!(a, apas_ai::seq!["hello", "rust", "test"]);
    let mut c = apas_ai::seq!["hello", "world", "test"];
    let _ = <S<&str> as APAS18>::update(&mut c, (5, "out_of_bounds"));
    assert_eq!(c, apas_ai::seq!["hello", "world", "test"]);
}

#[test]
fn test_inject_and_ninject() {
    let a = apas_ai::seq!["the", "cat", "in", "the", "hat"];
    let updates = apas_ai::seq![(0, "a"), (2, "on"), (4, "mat")];
    let injected = <S<&str> as APAS18>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, apas_ai::seq!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = apas_ai::seq![(0, "first"), (0, "second"), (1, "updated")];
    let result_first = <S<&str> as APAS18>::inject(&a, &conflicting_updates);
    assert_eq!(result_first, apas_ai::seq!["first", "updated", "in", "the", "hat"]);

    let ninjected = <S<&str> as APAS18>::ninject(&a, &apas_ai::seq![(1, "dog"), (3, "big"), (6, "hog")]);
    assert_eq!(ninjected, apas_ai::seq!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let result_last = <S<&str> as APAS18>::ninject(&a, &apas_ai::seq![(0, "first"), (0, "second"), (1, "updated")]);
    assert_eq!(result_last, apas_ai::seq!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = apas_ai::seq![1, 2, 3, 4, 5];
    let sum_fn = |a: &i32, b: &i32| a + b;
    let result = <S<i32> as APAS18>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: S<i32> = <S<i32> as Sequence<i32>>::empty();
    let empty_result = <S<i32> as APAS18>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = apas_ai::seq![100];
    let single_result = <S<i32> as APAS18>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <S<i32> as APAS18>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}


