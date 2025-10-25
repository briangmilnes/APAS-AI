//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for base ArraySeq implementation.

use apas_ai::Chap18::ArraySeq::ArraySeq::*;
use apas_ai::Types::Types::*;

#[test]
fn test_new() {
    let seq: ArraySeqS<i32> = ArraySeqS::new(5, 42);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 42);
    assert_eq!(*seq.nth(4), 42);
}

#[test]
fn test_set() {
    let mut seq: ArraySeqS<i32> = ArraySeqS::new(3, 0);
    seq.set(0, 10).unwrap();
    seq.set(1, 20).unwrap();
    seq.set(2, 30).unwrap();
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_set_out_of_bounds() {
    let mut seq: ArraySeqS<i32> = ArraySeqS::new(3, 0);
    assert!(seq.set(10, 42).is_err());
}

#[test]
fn test_length() {
    let seq: ArraySeqS<i32> = ArraySeqS::new(10, 0);
    assert_eq!(seq.length(), 10);

    let empty: ArraySeqS<i32> = ArraySeqS::empty();
    assert_eq!(empty.length(), 0);
}

#[test]
fn test_nth() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4, 5]);
    assert_eq!(*seq.nth(0), 1);
    assert_eq!(*seq.nth(2), 3);
    assert_eq!(*seq.nth(4), 5);
}

#[test]
fn test_empty() {
    let seq: ArraySeqS<i32> = ArraySeqS::empty();
    assert_eq!(seq.length(), 0);
    assert!(seq.isEmpty());
}

#[test]
fn test_singleton() {
    let seq = ArraySeqS::singleton(42);
    assert_eq!(seq.length(), 1);
    assert_eq!(*seq.nth(0), 42);
    assert!(seq.isSingleton());
}

#[test]
fn test_tabulate() {
    let seq = ArraySeqS::tabulate(&|i| i * 2, 5);
    assert_eq!(seq.length(), 5);
    assert_eq!(*seq.nth(0), 0);
    assert_eq!(*seq.nth(1), 2);
    assert_eq!(*seq.nth(2), 4);
    assert_eq!(*seq.nth(3), 6);
    assert_eq!(*seq.nth(4), 8);
}

#[test]
fn test_map() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4]);
    let doubled = ArraySeqS::map(&seq, &|x| x * 2);
    assert_eq!(doubled.length(), 4);
    assert_eq!(*doubled.nth(0), 2);
    assert_eq!(*doubled.nth(1), 4);
    assert_eq!(*doubled.nth(2), 6);
    assert_eq!(*doubled.nth(3), 8);
}

#[test]
fn test_subseq() {
    let seq = ArraySeqS::from_vec(vec![0, 1, 2, 3, 4, 5]);
    let sub = ArraySeqS::subseq(&seq, 2, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(1), 3);
    assert_eq!(*sub.nth(2), 4);
}

#[test]
fn test_subseq_out_of_bounds() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let sub = ArraySeqS::subseq(&seq, 1, 10);
    assert_eq!(sub.length(), 2); // Truncated
}

#[test]
fn test_append() {
    let a = ArraySeqS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqS::from_vec(vec![4, 5]);
    let c = ArraySeqS::append(&a, &b);
    assert_eq!(c.length(), 5);
    assert_eq!(*c.nth(0), 1);
    assert_eq!(*c.nth(2), 3);
    assert_eq!(*c.nth(3), 4);
    assert_eq!(*c.nth(4), 5);
}

#[test]
fn test_append_empty() {
    let a = ArraySeqS::from_vec(vec![1, 2]);
    let b: ArraySeqS<i32> = ArraySeqS::empty();
    let c = ArraySeqS::append(&a, &b);
    assert_eq!(c.length(), 2);
}

#[test]
fn test_filter() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4, 5, 6]);
    let evens = ArraySeqS::filter(&seq, &|x: &i32| *x % 2 == 0);
    assert_eq!(evens.length(), 3);
    assert_eq!(*evens.nth(0), 2);
    assert_eq!(*evens.nth(1), 4);
    assert_eq!(*evens.nth(2), 6);
}

#[test]
fn test_flatten() {
    let inner1 = ArraySeqS::from_vec(vec![1, 2]);
    let inner2 = ArraySeqS::from_vec(vec![3, 4, 5]);
    let inner3 = ArraySeqS::from_vec(vec![6]);
    let outer = ArraySeqS::from_vec(vec![inner1, inner2, inner3]);

    let flat = ArraySeqS::flatten(&outer);
    assert_eq!(flat.length(), 6);
    assert_eq!(*flat.nth(0), 1);
    assert_eq!(*flat.nth(2), 3);
    assert_eq!(*flat.nth(5), 6);
}

#[test]
fn test_update() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4]);
    let updated = ArraySeqS::update(&seq, Pair(2, 99));
    assert_eq!(*updated.nth(0), 1);
    assert_eq!(*updated.nth(1), 2);
    assert_eq!(*updated.nth(2), 99);
    assert_eq!(*updated.nth(3), 4);
}

#[test]
fn test_inject() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4, 5]);
    let updates = ArraySeqS::from_vec(vec![Pair(1, 10), Pair(3, 30), Pair(1, 99)]);
    let injected = ArraySeqS::inject(&seq, &updates);
    assert_eq!(*injected.nth(0), 1);
    assert_eq!(*injected.nth(1), 10); // First update for index 1
    assert_eq!(*injected.nth(2), 3);
    assert_eq!(*injected.nth(3), 30);
    assert_eq!(*injected.nth(4), 5);
}

#[test]
fn test_isEmpty() {
    let empty: ArraySeqS<i32> = ArraySeqS::empty();
    assert!(empty.isEmpty());

    let not_empty = ArraySeqS::singleton(1);
    assert!(!not_empty.isEmpty());
}

#[test]
fn test_isSingleton() {
    let single = ArraySeqS::singleton(42);
    assert!(single.isSingleton());

    let empty: ArraySeqS<i32> = ArraySeqS::empty();
    assert!(!empty.isSingleton());

    let multi = ArraySeqS::from_vec(vec![1, 2]);
    assert!(!multi.isSingleton());
}

#[test]
fn test_collect() {
    let pairs = ArraySeqS::from_vec(vec![
        Pair('a', 1),
        Pair('b', 2),
        Pair('a', 3),
        Pair('c', 4),
        Pair('b', 5),
    ]);

    let collected = <ArraySeqS<Pair<char, i32>> as ArraySeq<Pair<char, i32>>>::collect::<char, i32>(&pairs, |a, b| {
        if a == b { O::Equal } else { O::Less }
    });
    assert_eq!(collected.length(), 3);
}

#[test]
fn test_iterate() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4]);
    let sum = ArraySeqS::iterate(&seq, &|acc, x| acc + x, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_reduce() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4]);
    let sum = ArraySeqS::reduce(&seq, &|a, b| a + b, 0);
    assert_eq!(sum, 10);
}

#[test]
fn test_reduce_empty() {
    let seq: ArraySeqS<i32> = ArraySeqS::empty();
    let sum = ArraySeqS::reduce(&seq, &|a, b| a + b, 42);
    assert_eq!(sum, 42);
}

#[test]
fn test_scan() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3, 4]);
    let (result, final_val) = ArraySeqS::scan(&seq, &|a, b| a + b, 0);
    assert_eq!(result.length(), 4);
    assert_eq!(*result.nth(0), 1);
    assert_eq!(*result.nth(1), 3);
    assert_eq!(*result.nth(2), 6);
    assert_eq!(*result.nth(3), 10);
    assert_eq!(final_val, 10);
}

#[test]
fn test_from_vec() {
    let vec = vec![10, 20, 30];
    let seq = ArraySeqS::from_vec(vec);
    assert_eq!(seq.length(), 3);
    assert_eq!(*seq.nth(0), 10);
    assert_eq!(*seq.nth(1), 20);
    assert_eq!(*seq.nth(2), 30);
}

#[test]
fn test_partial_eq() {
    let a = ArraySeqS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqS::from_vec(vec![1, 2, 3]);
    let c = ArraySeqS::from_vec(vec![1, 2, 4]);

    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_display() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let s = format!("{}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_debug() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let s = format!("{:?}", seq);
    assert!(!s.is_empty());
}

#[test]
fn test_into_iterator() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let sum: i32 = (&seq).into_iter().sum();
    assert_eq!(sum, 6);
}

#[test]
fn test_into_iterator_mut() {
    let mut seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    for x in &mut seq {
        *x *= 2;
    }
    assert_eq!(*seq.nth(0), 2);
    assert_eq!(*seq.nth(1), 4);
    assert_eq!(*seq.nth(2), 6);
}

#[test]
fn test_into_iterator_owned() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let collected: Vec<i32> = seq.into_iter().collect();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[test]
fn test_clone() {
    let a = ArraySeqS::from_vec(vec![1, 2, 3]);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_subseq_copy() {
    let seq = ArraySeqS::from_vec(vec![0, 1, 2, 3, 4, 5]);
    let sub = seq.subseq_copy(2, 3);
    assert_eq!(sub.length(), 3);
    assert_eq!(*sub.nth(0), 2);
    assert_eq!(*sub.nth(1), 3);
    assert_eq!(*sub.nth(2), 4);
}

#[test]
fn test_iter() {
    let seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    let mut iter = seq.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_iter_mut() {
    let mut seq = ArraySeqS::from_vec(vec![1, 2, 3]);
    for x in seq.iter_mut() {
        *x *= 2;
    }
    assert_eq!(*seq.nth(0), 2);
    assert_eq!(*seq.nth(1), 4);
    assert_eq!(*seq.nth(2), 6);
}
