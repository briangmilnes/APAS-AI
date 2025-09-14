//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestArraySeqEphChap19 {
use apas_ai::Types::Types::*;
use apas_ai::ArraySeqEph::ArraySeqEph::*;
use apas_ai::ArraySeqEphChap18::ArraySeqEphChap18Trait;
use apas_ai::ArraySeqEphChap19::ArraySeqEphChap19Trait;
use apas_ai::ArraySeqEph; // macro import

#[test]
fn test_empty() {
    let e = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::empty();
    assert_eq!(e.length(), 0);
}

#[test]
fn test_singleton() {
    let s: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::singleton(42);
    assert_eq!(s, ArraySeqEph![42]);
}

#[test]
fn test_map() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 5);
    let b = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::map(&a, |x| x * 2);
    assert_eq!(b, ArraySeqEph![2, 4, 6, 8, 10]);
}

#[test]
fn test_append() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 4, 2);
    let c = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::append(&a, &b);
    assert_eq!(c, ArraySeqEph![1, 2, 3, 4, 5]);
}

#[test]
fn test_append2() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 4, 2);
    let c = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::append2(&a, &b);
    assert_eq!(c, ArraySeqEph![1, 2, 3, 4, 5]);
}

#[test]
fn test_deflate_true() {
    let y = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &6);
    assert_eq!(y, ArraySeqEph![6]);
}

#[test]
fn test_deflate_false() {
    let y = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &5);
    assert_eq!(y.length(), 0);
}

#[test]
fn test_filter_even_numbers() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 10);
    let evens = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
    assert_eq!(evens, ArraySeqEph![2, 4, 6, 8, 10]);
}

#[test]
fn test_filter_none() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i, 5);
    let odds_only = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::filter(&a, |x| if *x % 2 == 1 { B::True } else { B::False });
    assert_eq!(odds_only, ArraySeqEph![1, 3]);
}

#[test]
fn test_update_in_bounds() {
    let mut a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 5);
    let _ = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::update(&mut a, (2, 99));
    assert_eq!(a, ArraySeqEph![1, 2, 99, 4, 5]);
}

#[test]
fn test_update_out_of_bounds() {
    let mut a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 3);
    let _ = <ArraySeqEphS<N> as ArraySeqEphChap18Trait>::update(&mut a, (10, 7));
    assert_eq!(a, ArraySeqEph![1, 2, 3]);
}

#[test]
fn test_isEmpty() {
    let e = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    let s = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::singleton(7);
    assert_eq!(s.isEmpty(), B::False);
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i, 2);
    assert_eq!(a.isEmpty(), B::False);
}

#[test]
fn test_isSingleton() {
    let e = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::empty();
    assert_eq!(e.isSingleton(), B::False);
    let s = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::singleton(7);
    assert_eq!(s.isSingleton(), B::True);
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i, 2);
    assert_eq!(a.isSingleton(), B::False);
}

#[test]
fn test_iterate_sum() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 5);
    let sum = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::iterate(&a, |acc: &N, x: &N| acc + x, 0);
    assert_eq!(sum, 15);
}

#[test]
fn test_iterate_concat() {
    let a = <ArraySeqEphS<&str> as ArraySeqEphChap19Trait>::tabulate(
        |i| match i { 0 => "a", 1 => "b", _ => "c" },
        3,
    );
    let res = <ArraySeqEphS<&str> as ArraySeqEphChap19Trait>::iterate(
        &a,
        |acc: &String, x: &&str| format!("{}{}", acc, x),
        String::new(),
    );
    assert_eq!(res, "abc");
}

#[test]
fn test_map_empty() {
    let e: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|_| 0, 0);
    let m = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::map(&e, |x| x + 1);
    assert_eq!(m.length(), 0);
}

#[test]
fn test_append_with_empty() {
    let e: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|_| 0, 0);
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 3);
    let left = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::append(&e, &a);
    assert_eq!(left, ArraySeqEph![1, 2, 3]);
    let right = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::append(&a, &e);
    assert_eq!(right, ArraySeqEph![1, 2, 3]);
}

#[test]
fn test_append2_equivalence() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 3);
    let b = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 4, 2);
    let c1 = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::append(&a, &b);
    let c2 = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::append2(&a, &b);
    assert_eq!(c1, c2);
}

#[test]
fn test_filter_empty_sequence() {
    let e: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|_| 0, 0);
    let f = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::filter(&e, |_| B::True);
    assert_eq!(f.length(), 0);
}

#[test]
fn test_select_boundary() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 2);
    let b = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 3, 2);
    assert_eq!(<ArraySeqEphS<N> as ArraySeqEphChap19Trait>::select(&a, &b, 0), Some(1));
    assert_eq!(<ArraySeqEphS<N> as ArraySeqEphChap19Trait>::select(&a, &b, 1), Some(2));
    assert_eq!(<ArraySeqEphS<N> as ArraySeqEphChap19Trait>::select(&a, &b, 2), Some(3));
    assert_eq!(<ArraySeqEphS<N> as ArraySeqEphChap19Trait>::select(&a, &b, 3), Some(4));
    assert_eq!(<ArraySeqEphS<N> as ArraySeqEphChap19Trait>::select(&a, &b, 4), None);
}

#[test]
fn test_subseq_basic() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| (i + 1) * 10, 5);
    let s = <ArraySeqEphS<N> as ArraySeqEphTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(s, ArraySeqEph![20, 30, 40]);
}

#[test]
fn test_reduce_sum_basic_ch19() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let r = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::reduce(&a, &sum_fn, 0);
    assert_eq!(r, 15);
    let e: ArraySeqEphS<N> = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|_| 0, 0);
    let re = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::reduce(&e, &sum_fn, 42);
    assert_eq!(re, 42);
    let s = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|_| 7, 1);
    let rs = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::reduce(&s, &sum_fn, 0);
    assert_eq!(rs, 7);
}

#[test]
fn test_scan_sum_basic_ch19() {
    let a = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 5);
    let sum_fn = |x: &N, y: &N| x + y;
    let (prefixes, total) = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::scan(&a, &sum_fn, 0);
    assert_eq!(prefixes.length(), 5);
    assert_eq!(*prefixes.nth(0), 0);
    assert_eq!(*prefixes.nth(4), 10);
    assert_eq!(total, 15);
}

#[test]
fn test_flatten_ch19() {
    let s1 = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 1, 2);
    let s2 = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::tabulate(|i| i + 3, 2);
    let nested: ArraySeqEphS<ArraySeqEphS<N>> = ArraySeqEph![s1, s2];
    let flat = <ArraySeqEphS<N> as ArraySeqEphChap19Trait>::flatten(&nested);
    assert_eq!(flat, ArraySeqEph![1, 2, 3, 4]);
}

}
