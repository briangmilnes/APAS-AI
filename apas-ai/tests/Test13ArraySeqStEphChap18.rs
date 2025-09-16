//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestArraySeqStEphChap18 {
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::ArraySeqStEphChap18::ArraySeqStEphChap18::*;
use apas_ai::ArraySeqStEph; // macro import

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }
    let a = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::tabulate(fib, 10);
        let fib10_head = ArraySeqStEph![
            *a.nth(0), *a.nth(1), *a.nth(2), *a.nth(3), *a.nth(4),
            *a.nth(5), *a.nth(6), *a.nth(7), *a.nth(8), *a.nth(9)
        ];
        assert_eq!(fib10_head, ArraySeqStEph![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArraySeqStEph![1, 2, 3, 4, 5];
    let b = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::map(&a, |x| x + 1);
    assert_eq!(b, ArraySeqStEph![2, 3, 4, 5, 6]);
}

#[test]
fn test_subseq() {
    let a = ArraySeqStEph![10, 20, 30, 40, 50];
    let b = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(b, ArraySeqStEph![20, 30, 40]);
    let c = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq_copy(&a, 2, 0);
    assert_eq!(c.length(), 0);
    let d = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::subseq_copy(&a, 0, 1);
    assert_eq!(d, ArraySeqStEph![10]);
}

#[test]
fn test_append() {
    let a = ArraySeqStEph![1, 2, 3];
    let b = ArraySeqStEph![4, 5, 6];
    let c = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStEph![1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArraySeqStEph![1, 2, 3];
    let b = ArraySeqStEph![4, 5];
    let c = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::append(&a, &b);
    assert_eq!(c, ArraySeqStEph![1, 2, 3, 4, 5]);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::empty();
    let d = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::append(&ArraySeqStEph![1,2,3], &empty);
    assert_eq!(d.length(), 3);
    let e = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::append(&empty, &ArraySeqStEph![1,2,3]);
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArraySeqStEph![1,2,3,4,5,6,7,8,9,10];
    let evens = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::filter(&numbers, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArraySeqStEph![2, 4, 6, 8, 10]);
    let odds_only = ArraySeqStEph![1,3,5,7];
    let no_evens = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::filter(&odds_only, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = ArraySeqStEph![1, 2];
    let b = ArraySeqStEph![3, 4, 5];
    let c = ArraySeqStEph![6];
    let sequences = ArraySeqStEph![a, b, c];
    let flattened = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::flatten(&sequences);
    assert_eq!(flattened, ArraySeqStEph![1, 2, 3, 4, 5, 6]);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::empty();
    let mixed = ArraySeqStEph![ArraySeqStEph![1, 2], empty, ArraySeqStEph![3]];
    let mixed_flat = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::flatten(&mixed);
    assert_eq!(mixed_flat, ArraySeqStEph![1, 2, 3]);
}

#[test]
fn test_update_sequence() {
    // Eph: update mutates a clone (via update helper)
    let a = ArraySeqStEph!["hello", "world", "test"];
    let mut b = a.clone();
    let _ = b.update(Pair(1, "rust").into());
    assert_eq!(b, ArraySeqStEph!["hello", "rust", "test"]);
    let mut c = ArraySeqStEph!["hello", "world", "test"];
    let _ = c.update(Pair(5, "out_of_bounds").into());
    assert_eq!(c, ArraySeqStEph!["hello", "world", "test"]);
}

#[test]
fn test_inject_conflicts_last_wins() {
    let a = ArraySeqStEph!["the", "cat", "in", "the", "hat"];
    let updates = ArraySeqStEph![Pair(0, "a"), Pair(2, "on"), Pair(4, "mat")];
    let injected = <ArraySeqStEphS<&str> as ArraySeqStEphChap18Trait<&str>>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, ArraySeqStEph!["a", "cat", "on", "the", "mat"]);

    let conflicting_updates = ArraySeqStEph![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")];
    let result_last = <ArraySeqStEphS<&str> as ArraySeqStEphChap18Trait<&str>>::inject(&a, &conflicting_updates);
    assert_eq!(result_last, ArraySeqStEph!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_ninject_conflicts_last_wins() {
    let a = ArraySeqStEph!["the", "cat", "in", "the", "hat"];
    let ninjected = <ArraySeqStEphS<&str> as ArraySeqStEphChap18Trait<&str>>::ninject(
        &a,
        &ArraySeqStEph![Pair(1, "dog"), Pair(3, "big"), Pair(6, "hog")],
    );
    assert_eq!(ninjected, ArraySeqStEph!["the", "dog", "in", "big", "hat"]);
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArraySeqStEphS<&str> as ArraySeqStEphChap18Trait<&str>>::ninject(
        &a,
        &ArraySeqStEph![Pair(0, "first"), Pair(0, "second"), Pair(1, "updated")],
    );
    assert_eq!(result_last, ArraySeqStEph!["second", "updated", "in", "the", "hat"]);
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArraySeqStEph![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArraySeqStEphS<N> = <ArraySeqStEphS<N> as ArraySeqStEphTrait<N>>::empty();
    let empty_result = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArraySeqStEph![100];
    let single_result = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArraySeqStEph![1, 2, 3, 4, 5];
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::iterate(&numbers, sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArraySeqStEph![1, 2, 3];
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArraySeqStEphS<N> as ArraySeqStEphChap18Trait<N>>::iteratePrefixes(&numbers, sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    let pairs = ArraySeqStEph![Pair("a", 1_usize), Pair("b", 2_usize), Pair("a", 3_usize)];
    let grouped = <ArraySeqStEphS<Pair<&str, ArraySeqStEphS<N>>> as ArraySeqStEphChap18Trait<Pair<&str, ArraySeqStEphS<N>>>>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let pair0 = grouped.nth(0);
    assert_eq!(pair0.0, "a");
    assert_eq!(pair0.1, ArraySeqStEph![1, 3]);
    let pair1 = grouped.nth(1);
    assert_eq!(pair1.0, "b");
    assert_eq!(pair1.1, ArraySeqStEph![2]);
}

}


