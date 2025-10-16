//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap18::LinkedListStPer::LinkedListStPer::*;
use apas_ai::LinkedListStPerSLit;
use apas_ai::Types::Types::*; // macro import

#[test]
fn test_empty_singleton_and_predicates() {
    let l: LinkedListStPerS<N> = LinkedListStPerSLit![];
    assert_eq!(l.length(), 0);
    assert!(l.length() == 0);
    let one = LinkedListStPerSLit![7];
    assert!(one.length() == 1);
}

#[test]
fn test_new_and_nth_set() {
    let l = LinkedListStPerSLit![1; 3];
    assert_eq!(*l.nth(0), 1);
    assert_eq!(*l.nth(2), 1);
    // LinkedListStPer is persistent - no update method
    // let l2 = l.update(1, 9);
    // original remains unchanged (persistent semantics)
    assert_eq!(*l.nth(1), 1);
    // updated copy has the change
    // assert_eq!(*l2.nth(1), 9);
}

#[test]
fn test_subseq() {
    let l = LinkedListStPerSLit![2; 5];
    let sub = l.subseq_copy(1, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(2), 2);
}

#[test]
fn test_from_vec_and_debug_format() {
    let l = LinkedListStPerSLit![1, 2, 3];
    assert_eq!(l.length(), 3);
    assert_eq!(format!("{l:?}"), "[1, 2, 3]");
}

#[test]
fn test_iter_inorder_collect() {
    let l = LinkedListStPerSLit![10, 20, 30];
    let vals: Vec<N> = (0..l.length()).map(|i| *l.nth(i)).collect();
    assert_eq!(vals, vec![10, 20, 30]);
}

#[test]
#[should_panic]
fn test_nth_out_of_bounds_panics() {
    let l = LinkedListStPerSLit![0; 2];
    let _ = l.nth(2);
}

#[test]
fn test_display_impl() {
    let l = LinkedListStPerSLit![1, 2, 3];
    assert_eq!(format!("{l}"), "[1, 2, 3]");
}

#[test]
fn test_tabulate_constructor() {
    let l = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i * 2, 4);
    assert_eq!(l.length(), 4);
    assert_eq!(*l.nth(0), 0);
    assert_eq!(*l.nth(1), 2);
    assert_eq!(*l.nth(2), 4);
    assert_eq!(*l.nth(3), 6);
}

#[test]
fn test_map_transform() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let doubled = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::map(&l, &|x| x * 2);
    assert_eq!(doubled.length(), 3);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(1), 4);
    assert_eq!(*doubled.nth(2), 6);
}

#[test]
fn test_append_concatenation() {
    let a = LinkedListStPerSLit![1, 2];
    let b = LinkedListStPerSLit![3, 4];
    let c = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::append(&a, &b);
    assert_eq!(c.length(), 4);
    assert_eq!(*c.nth(0), 1);
    assert_eq!(*c.nth(1), 2);
    assert_eq!(*c.nth(2), 3);
    assert_eq!(*c.nth(3), 4);
}

#[test]
fn test_select_from_concatenated() {
    let a = LinkedListStPerSLit![10, 20];
    let b = LinkedListStPerSLit![30, 40];

    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 0),
        Some(10)
    );
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 1),
        Some(20)
    );
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 2),
        Some(30)
    );
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 3),
        Some(40)
    );
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 4),
        None
    );
}

#[test]
fn test_filter_predicate() {
    let l = LinkedListStPerSLit![1, 2, 3, 4, 5];
    let evens = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::filter(&l, &|x| *x % 2 == 0);
    assert_eq!(evens.length(), 2);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
}

#[test]
fn test_update_persistent() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let updated = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::update(&l, Pair(1, 99));

    // Original unchanged (persistent)
    assert_eq!(*l.nth(1), 2);

    // Updated copy has change
    assert_eq!(updated.length(), 3);
    assert_eq!(*updated.nth(0), 1);
    assert_eq!(*updated.nth(1), 99);
    assert_eq!(*updated.nth(2), 3);
}

#[test]
fn test_inject_first_update_wins() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let updates = LinkedListStPerSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)]; // Duplicate index 1
    let result = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::inject(&l, &updates);

    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 1);
    assert_eq!(*result.nth(1), 10); // First update wins
    assert_eq!(*result.nth(2), 20);
}

#[test]
fn test_ninject_last_update_wins() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let updates = LinkedListStPerSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)]; // Duplicate index 1
    let result = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::ninject(&l, &updates);

    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(0), 1);
    assert_eq!(*result.nth(1), 30); // Last update wins
    assert_eq!(*result.nth(2), 20);
}

#[test]
fn test_iterate_accumulate() {
    let l = LinkedListStPerSLit![1, 2, 3, 4];
    let sum = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::iterate(&l, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_iteratePrefixes_scan_like() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::iteratePrefixes(&l, &|acc, x| acc + x, 0);

    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0); // Initial value
    assert_eq!(*prefixes.nth(1), 1); // 0 + 1
    assert_eq!(*prefixes.nth(2), 3); // 1 + 2
    assert_eq!(total, 6); // 3 + 3
}

#[test]
fn test_reduce_divide_conquer() {
    let l = LinkedListStPerSLit![1, 2, 3, 4];
    let sum = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::reduce(&l, &|a, b| a + b, 0);
    assert_eq!(sum, 10);

    let empty: LinkedListStPerS<N> = LinkedListStPerSLit![];
    let empty_sum = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::reduce(&empty, &|a, b| a + b, 42);
    assert_eq!(empty_sum, 42);

    let single = LinkedListStPerSLit![5];
    let single_sum = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::reduce(&single, &|a, b| a + b, 0);
    assert_eq!(single_sum, 5);
}

#[test]
fn test_scan_prefix_sums() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::scan(&l, &|a, b| a + b, 0);

    assert_eq!(prefixes.length(), 3);
    assert_eq!(*prefixes.nth(0), 0); // reduce([], +, 0) = 0
    assert_eq!(*prefixes.nth(1), 1); // reduce([1], +, 0) = 1
    assert_eq!(*prefixes.nth(2), 3); // reduce([1,2], +, 0) = 3
    assert_eq!(total, 6); // reduce([1,2,3], +, 0) = 6
}

#[test]
fn test_flatten_nested_lists() {
    let inner1 = LinkedListStPerSLit![1, 2];
    let inner2 = LinkedListStPerSLit![3];
    let inner3 = LinkedListStPerSLit![4, 5];
    let nested = LinkedListStPerSLit![inner1, inner2, inner3];

    let flattened = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::flatten(&nested);
    assert_eq!(flattened.length(), 5);
    assert_eq!(*flattened.nth(0), 1);
    assert_eq!(*flattened.nth(1), 2);
    assert_eq!(*flattened.nth(2), 3);
    assert_eq!(*flattened.nth(3), 4);
    assert_eq!(*flattened.nth(4), 5);
}

#[test]
fn test_collect_group_by_key() {
    let pairs = LinkedListStPerSLit![Pair(1, "a"), Pair(2, "b"), Pair(1, "c"), Pair(2, "d")];
    let grouped = <LinkedListStPerS<Pair<N, &str>> as LinkedListStPerTrait<Pair<N, &str>>>::collect(&pairs, |a, b| {
        if a < b {
            O::Less
        } else if a > b {
            O::Greater
        } else {
            O::Equal
        }
    });

    assert_eq!(grouped.length(), 2);

    let Pair(key1, vals1) = grouped.nth(0);
    assert_eq!(*key1, 1);
    assert_eq!(vals1.length(), 2);
    assert_eq!(*vals1.nth(0), "a");
    assert_eq!(*vals1.nth(1), "c");

    let Pair(key2, vals2) = grouped.nth(1);
    assert_eq!(*key2, 2);
    assert_eq!(vals2.length(), 2);
    assert_eq!(*vals2.nth(0), "b");
    assert_eq!(*vals2.nth(1), "d");
}

#[test]
fn test_equality_comparison() {
    let l1 = LinkedListStPerSLit![1, 2, 3];
    let l2 = LinkedListStPerSLit![1, 2, 3];
    let l3 = LinkedListStPerSLit![1, 2, 4];

    assert_eq!(l1, l2);
    assert_ne!(l1, l3);
}

#[test]
fn test_subseq_copy_edge_cases() {
    let l = LinkedListStPerSLit![1, 2, 3, 4, 5];

    // Zero length
    let empty_sub = l.subseq_copy(2, 0);
    assert_eq!(empty_sub.length(), 0);

    // Start beyond bounds
    let beyond_sub = l.subseq_copy(10, 3);
    assert_eq!(beyond_sub.length(), 0);

    // Length extends beyond bounds
    let partial_sub = l.subseq_copy(3, 10);
    assert_eq!(partial_sub.length(), 2);
    assert_eq!(*partial_sub.nth(0), 4);
    assert_eq!(*partial_sub.nth(1), 5);
}

// ========== Merged from _Advanced file ==========

#[test]
fn test_tabulate() {
    let a = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i * 2, 5);
    assert_eq!(a.length(), 5);
    assert_eq!(a.nth(3), &6);
}

#[test]
fn test_map() {
    let a = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i * 2, 5);
    let b = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::map(&a, &|x| x + 1);
    assert_eq!(b.nth(0), &1);
    assert_eq!(b.nth(4), &9);
}

#[test]
fn test_filter() {
    let a = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i + 1, 5);
    let c = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::filter(&a, &|x: &N| *x % 2 == 1);
    assert_eq!(c, LinkedListStPerSLit![1, 3, 5]);
}

#[test]
fn test_append() {
    let a = LinkedListStPerSLit![0, 2, 4, 6, 8];
    let b = LinkedListStPerSLit![1, 3, 5, 7, 9];
    let d = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::append(&a, &b);
    assert_eq!(d, LinkedListStPerSLit![0, 2, 4, 6, 8, 1, 3, 5, 7, 9]);
}

#[test]
fn test_update() {
    let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
    let upd = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::update(&a, Pair(1, 9));
    assert_eq!(upd, LinkedListStPerSLit![1, 9, 3, 2, 1]);
}

#[test]
fn test_inject() {
    let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
    let changes = LinkedListStPerSLit![Pair(0, 7), Pair(3, 5)];
    let inj = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::inject(&a, &changes);
    assert_eq!(inj, LinkedListStPerSLit![7, 2, 3, 5, 1]);
}

#[test]
fn test_ninject() {
    let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
    let changes = LinkedListStPerSLit![Pair(0, 7), Pair(3, 5)];
    let ninj = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::ninject(&a, &changes);
    assert_eq!(ninj, LinkedListStPerSLit![7, 2, 3, 5, 1]);
}

#[test]
fn test_iterate() {
    let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
    let sum = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::iterate(&a, &|acc, x| acc + x, 0);
    assert_eq!(sum, 9);
}

#[test]
fn test_reduce() {
    let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
    let red = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::reduce(&a, &|x, y| x + y, 0);
    assert_eq!(red, 9);
}

#[test]
fn test_scan() {
    let a = LinkedListStPerSLit![1, 2, 3, 2, 1];
    let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::scan(&a, &|x, y| x + y, 0);
    assert_eq!(prefixes, LinkedListStPerSLit![0, 1, 3, 6, 8]);
    assert_eq!(total, 9);
}

#[test]
fn test_flatten() {
    let nested = LinkedListStPerSLit![
        LinkedListStPerSLit![1, 2],
        LinkedListStPerSLit![3],
        LinkedListStPerSLit![4, 5],
    ];
    let flat = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::flatten(&nested);
    assert_eq!(flat, LinkedListStPerSLit![1, 2, 3, 4, 5]);
}

#[test]
fn test_collect() {
    let pairs = LinkedListStPerSLit![Pair(1, 10), Pair(2, 20), Pair(1, 30)];
    let grouped = <LinkedListStPerS<Pair<N, N>> as LinkedListStPerTrait<Pair<N, N>>>::collect(&pairs, |a, b| {
        if a < b {
            O::Less
        } else if a > b {
            O::Greater
        } else {
            O::Equal
        }
    });
    // Expect two groups with keys 1 and 2
    assert_eq!(grouped.length(), 2);
}

// ========== Merged from TestLinkedListStPerChap18_Advanced.rs above ==========

#[test]
fn test_large_list_operations() {
    let large = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::tabulate(&|i| i, 100);
    assert_eq!(large.length(), 100);
    assert_eq!(*large.nth(0), 0);
    assert_eq!(*large.nth(50), 50);
    assert_eq!(*large.nth(99), 99);

    // Test map on large list
    let doubled = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::map(&large, &|x| x * 2);
    assert_eq!(*doubled.nth(50), 100);
}

#[test]
fn test_empty_list_operations() {
    let empty: LinkedListStPerS<N> = LinkedListStPerSLit![];

    let mapped = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::map(&empty, &|x| x * 2);
    assert_eq!(mapped.length(), 0);

    let filtered = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::filter(&empty, &|_| true);
    assert_eq!(filtered.length(), 0);

    let reduced = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::reduce(&empty, &|a, b| a + b, 10);
    assert_eq!(reduced, 10);
}

#[test]
fn test_select_boundary_cases() {
    let a = LinkedListStPerSLit![1, 2];
    let b = LinkedListStPerSLit![3, 4];

    // First element
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 0),
        Some(1)
    );

    // Last element
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 3),
        Some(4)
    );

    // Just beyond
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 4),
        None
    );

    // Empty first list
    let empty: LinkedListStPerS<N> = LinkedListStPerSLit![];
    assert_eq!(
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&empty, &b, 0),
        Some(3)
    );
}

#[test]
fn test_iteratePrefixes_empty() {
    let empty: LinkedListStPerS<N> = LinkedListStPerSLit![];
    let (prefixes, total) =
        <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::iteratePrefixes(&empty, &|acc, x| acc + x, 5);

    assert_eq!(prefixes.length(), 0);
    assert_eq!(total, 5); // Only initial value
}

#[test]
fn test_scan_empty() {
    let empty: LinkedListStPerS<N> = LinkedListStPerSLit![];
    let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::scan(&empty, &|a, b| a + b, 10);

    assert_eq!(prefixes.length(), 0);
    assert_eq!(total, 10);
}

#[test]
fn test_flatten_empty_sublists() {
    let empty_sub: LinkedListStPerS<N> = LinkedListStPerSLit![];
    let nested = LinkedListStPerSLit![empty_sub.clone(), empty_sub.clone()];

    let flattened = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::flatten(&nested);
    assert_eq!(flattened.length(), 0);
}

#[test]
fn test_collect_empty() {
    let empty: LinkedListStPerS<Pair<N, N>> = LinkedListStPerSLit![];
    let grouped = <LinkedListStPerS<Pair<N, N>> as LinkedListStPerTrait<Pair<N, N>>>::collect(&empty, |a, b| {
        if a < b {
            O::Less
        } else if a > b {
            O::Greater
        } else {
            O::Equal
        }
    });

    assert_eq!(grouped.length(), 0);
}

#[test]
fn test_multiple_updates_persistence() {
    let l = LinkedListStPerSLit![1, 2, 3, 4, 5];

    let u1 = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::update(&l, Pair(1, 10));
    let u2 = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::update(&u1, Pair(3, 30));

    // Original unchanged
    assert_eq!(*l.nth(1), 2);
    assert_eq!(*l.nth(3), 4);

    // First update
    assert_eq!(*u1.nth(1), 10);
    assert_eq!(*u1.nth(3), 4);

    // Second update
    assert_eq!(*u2.nth(1), 10);
    assert_eq!(*u2.nth(3), 30);
}

#[test]
fn test_inject_out_of_bounds_ignored() {
    let l = LinkedListStPerSLit![1, 2, 3];
    let updates = LinkedListStPerSLit![Pair(1, 10), Pair(10, 99)]; // Index 10 out of bounds
    let result = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::inject(&l, &updates);

    assert_eq!(result.length(), 3);
    assert_eq!(*result.nth(1), 10);
}
