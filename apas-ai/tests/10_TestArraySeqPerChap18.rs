use apas_ai::Types::{N, B, O};
use apas_ai::{ArrayPerS, ArraySeqPerTrait, ArraySeqPerChap18Trait};

#[test]
fn test_tabulate_fibonacci() {
    fn fib(n: N) -> N { match n { 0 => 0, 1 => 1, _ => fib(n - 1) + fib(n - 2) } }
    let a = <ArrayPerS<N> as ArraySeqPerChap18Trait>::tabulate(fib, 10);
    let fib10_head = ArrayPerS::from_vec(vec![
        *a.nth(0), *a.nth(1), *a.nth(2), *a.nth(3), *a.nth(4),
        *a.nth(5), *a.nth(6), *a.nth(7), *a.nth(8), *a.nth(9)
    ]);
    assert_eq!(fib10_head, ArrayPerS::from_vec(vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]));
    assert_eq!(a.length(), 10);
}

#[test]
fn test_map_increment() {
    let a = ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let b = <ArrayPerS<N> as ArraySeqPerChap18Trait>::map(&a, |x| x + 1);
    assert_eq!(b, ArrayPerS::from_vec(vec![2, 3, 4, 5, 6]));
}

#[test]
fn test_subseq() {
    let a = ArrayPerS::from_vec(vec![10, 20, 30, 40, 50]);
    let b = <ArrayPerS<N> as ArraySeqPerTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(b, ArrayPerS::from_vec(vec![20, 30, 40]));
    let c = <ArrayPerS<N> as ArraySeqPerTrait<N>>::subseq_copy(&a, 2, 0);
    assert_eq!(c.length(), 0);
    let d = <ArrayPerS<N> as ArraySeqPerTrait<N>>::subseq_copy(&a, 0, 1);
    assert_eq!(d, ArrayPerS::from_vec(vec![10]));
}

#[test]
fn test_append() {
    let a = ArrayPerS::from_vec(vec![1, 2, 3]);
    let b = ArrayPerS::from_vec(vec![4, 5, 6]);
    let c = <ArrayPerS<N> as ArraySeqPerChap18Trait>::append(&a, &b);
    assert_eq!(c, ArrayPerS::from_vec(vec![1, 2, 3, 4, 5, 6]));
}

#[test]
fn test_sequence_literals_and_append() {
    let a = ArrayPerS::from_vec(vec![1, 2, 3]);
    let b = ArrayPerS::from_vec(vec![4, 5]);
    let c = <ArrayPerS<N> as ArraySeqPerChap18Trait>::append(&a, &b);
    assert_eq!(c, ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]));
    let empty: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::empty();
    let d = <ArrayPerS<N> as ArraySeqPerChap18Trait>::append(&ArrayPerS::from_vec(vec![1,2,3]), &empty);
    assert_eq!(d.length(), 3);
    let e = <ArrayPerS<N> as ArraySeqPerChap18Trait>::append(&empty, &ArrayPerS::from_vec(vec![1,2,3]));
    assert_eq!(e.length(), 3);
}

#[test]
fn test_filter_even() {
    let numbers = ArrayPerS::from_vec(vec![1,2,3,4,5,6,7,8,9,10]);
    let evens = <ArrayPerS<N> as ArraySeqPerChap18Trait>::filter(&numbers, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArrayPerS::from_vec(vec![2, 4, 6, 8, 10]));
    let odds_only = ArrayPerS::from_vec(vec![1,3,5,7]);
    let no_evens = <ArrayPerS<N> as ArraySeqPerChap18Trait>::filter(&odds_only, |&x| if x % 2 == 0 { B::True } else { B::False });
    assert_eq!(no_evens.length(), 0);
}

#[test]
fn test_flatten() {
    let a = ArrayPerS::from_vec(vec![1, 2]);
    let b = ArrayPerS::from_vec(vec![3, 4, 5]);
    let c = ArrayPerS::from_vec(vec![6]);
    let sequences = ArrayPerS::from_vec(vec![a, b, c]);
    let flattened = <ArrayPerS<N> as ArraySeqPerChap18Trait>::flatten(&sequences);
    assert_eq!(flattened, ArrayPerS::from_vec(vec![1, 2, 3, 4, 5, 6]));
    let empty: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::empty();
    let mixed = ArrayPerS::from_vec(vec![ArrayPerS::from_vec(vec![1, 2]), empty, ArrayPerS::from_vec(vec![3])]);
    let mixed_flat = <ArrayPerS<N> as ArraySeqPerChap18Trait>::flatten(&mixed);
    assert_eq!(mixed_flat, ArrayPerS::from_vec(vec![1, 2, 3]));
}

#[test]
fn test_update_sequence() {
    let a = ArrayPerS::from_vec(vec!["hello", "world", "test"]);
    let b = <ArrayPerS<&str> as ArraySeqPerChap18Trait>::update(&a, (1, "rust"));
    assert_eq!(b, ArrayPerS::from_vec(vec!["hello", "rust", "test"]));
    let c = ArrayPerS::from_vec(vec!["hello", "world", "test"]);
    let d = <ArrayPerS<&str> as ArraySeqPerChap18Trait>::update(&c, (5, "out_of_bounds"));
    assert_eq!(d, ArrayPerS::from_vec(vec!["hello", "world", "test"]));
}

#[test]
fn test_inject_and_ninject() {
    let a = ArrayPerS::from_vec(vec!["the", "cat", "in", "the", "hat"]);
    let updates = ArrayPerS::from_vec(vec![(0, "a"), (2, "on"), (4, "mat")]);
    let injected = <ArrayPerS<&str> as ArraySeqPerChap18Trait>::inject(&a, &updates);
    assert_eq!(injected.length(), 5);
    assert_eq!(injected, ArrayPerS::from_vec(vec!["a", "cat", "on", "the", "mat"]));

    let conflicting_updates = ArrayPerS::from_vec(vec![(0, "first"), (0, "second"), (1, "updated")]);
    let result_first = <ArrayPerS<&str> as ArraySeqPerChap18Trait>::inject(&a, &conflicting_updates);
    assert_eq!(result_first, ArrayPerS::from_vec(vec!["first", "updated", "in", "the", "hat"]));

    let ninjected = <ArrayPerS<&str> as ArraySeqPerChap18Trait>::ninject(&a, &ArrayPerS::from_vec(vec![(1, "dog"), (3, "big"), (6, "hog")]));
    assert_eq!(ninjected, ArrayPerS::from_vec(vec!["the", "dog", "in", "big", "hat"]));
    assert_eq!(ninjected.length(), 5);
    let result_last = <ArrayPerS<&str> as ArraySeqPerChap18Trait>::ninject(&a, &ArrayPerS::from_vec(vec![(0, "first"), (0, "second"), (1, "updated")]));
    assert_eq!(result_last, ArrayPerS::from_vec(vec!["second", "updated", "in", "the", "hat"]));
}

#[test]
fn test_iterate_and_prefixes_and_reduce_and_scan() {
    let numbers = ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let sum_fn = |a: &N, b: &N| a + b;
    let result = <ArrayPerS<N> as ArraySeqPerChap18Trait>::reduce(&numbers, &sum_fn, 0);
    assert_eq!(result, 15);
    let empty: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::empty();
    let empty_result = <ArrayPerS<N> as ArraySeqPerChap18Trait>::reduce(&empty, &sum_fn, 42);
    assert_eq!(empty_result, 42);
    let single = ArrayPerS::from_vec(vec![100]);
    let single_result = <ArrayPerS<N> as ArraySeqPerChap18Trait>::reduce(&single, &sum_fn, 0);
    assert_eq!(single_result, 100);
    let (prefixes, final_result) = <ArrayPerS<N> as ArraySeqPerChap18Trait>::scan(&numbers, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(final_result, 15);
}

#[test]
fn test_iterate_sum_basic() {
    let numbers = ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let sum_fn = |a: &N, x: &N| a + x;
    let r = <ArrayPerS<N> as ArraySeqPerChap18Trait>::iterate(&numbers, sum_fn, 0);
    assert_eq!(r, 15);
}

#[test]
fn test_iterate_prefixes_sum() {
    let numbers = ArrayPerS::from_vec(vec![1, 2, 3]);
    let sum_fn = |a: &N, x: &N| a + x;
    let (prefixes, total) = <ArrayPerS<N> as ArraySeqPerChap18Trait>::iteratePrefixes(&numbers, sum_fn, 0);
    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(1), 1);
    assert_eq!(*prefixes.nth(2), 3);
    assert_eq!(total, 6);
}

#[test]
fn test_collect_groups_by_key() {
    let pairs = ArrayPerS::from_vec(vec![("a", 1_usize), ("b", 2_usize), ("a", 3_usize)]);
    let grouped = <ArrayPerS<(&str, N)> as ArraySeqPerChap18Trait>::collect(&pairs, |k1, k2| k1.cmp(k2));
    assert_eq!(grouped.length(), 2);
    let (k0, v0) = grouped.nth(0);
    assert_eq!(*k0, "a");
    assert_eq!(*v0, ArrayPerS::from_vec(vec![1, 3]));
    let (k1, v1) = grouped.nth(1);
    assert_eq!(*k1, "b");
    assert_eq!(*v1, ArrayPerS::from_vec(vec![2]));
}


