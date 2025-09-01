//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Sequences::*;
#[test]
fn test_new_and_update() {
    let mut s = <S<i32> as Sequence<i32>>::new(3, 7);
    assert_eq!(s.length(), 3);
    assert_eq!(*s.nth(0), 7);
    assert_eq!(*s.nth(1), 7);
    assert_eq!(*s.nth(2), 7);
    let _ = s.update((1, 9)).update((0, 2));
    assert_eq!(*s.nth(0), 2);
    assert_eq!(*s.nth(1), 9);
    assert_eq!(*s.nth(2), 7);
}

#[test]
fn test_length_and_nth_basic() {
    let a = apas_ai::seq![10, 20, 30, 40];
    assert_eq!(a.length(), 4);
    assert_eq!(*a.nth(0), 10);
    assert_eq!(*a.nth(3), 40);
}

#[test]
fn test_empty() {
    let empty: S<i32> = <S<i32> as Sequence<i32>>::empty();
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), B::True);
}

#[test]
fn test_sequence_basic() {
    let mut seq: S<B> = <S<B> as Sequence<B>>::new(10, B::False);
    assert_eq!(seq.isEmpty(), B::False);
    assert_eq!(seq.length(), 10);
    let _ = seq.update((0, B::True)).update((1, B::False)).update((2, B::True));
    assert_eq!(seq.length(), 10);
    let head4 = apas_ai::seq![*seq.nth(0), *seq.nth(1), *seq.nth(2), *seq.nth(3)];
    assert_eq!(head4, apas_ai::seq![B::True, B::False, B::True, B::False]);
}

#[test]
fn test_singleton() {
    let s = <S<i32> as Sequence<i32>>::singleton(42);
    assert_eq!(s.length(), 1);
    assert_eq!(*s.nth(0), 42);
    assert_eq!(s.isSingleton(), B::True);
}

#[test]
fn test_is_empty_and_is_singleton() {
    let e: S<i32> = <S<i32> as Sequence<i32>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    assert_eq!(e.isSingleton(), B::False);

    let s = <S<i32> as Sequence<i32>>::singleton(7);
    assert_eq!(s.isEmpty(), B::False);
    assert_eq!(s.isSingleton(), B::True);

    let a = apas_ai::seq![1, 2];
    assert_eq!(a.isEmpty(), B::False);
    assert_eq!(a.isSingleton(), B::False);
}

#[test]
fn test_from_vec() {
    let empty_vec: Vec<N> = vec![];
    let empty_seq = S::from_vec(empty_vec);
    assert_eq!(empty_seq.length(), 0);
    assert_eq!(empty_seq.isEmpty(), B::True);
    let single_vec = vec![42];
    let single_seq = S::from_vec(single_vec);
    assert_eq!(single_seq, apas_ai::seq![42]);
    let multi_vec = vec![1, 2, 3, 4, 5];
    let multi_seq = S::from_vec(multi_vec);
    assert_eq!(multi_seq, apas_ai::seq![1, 2, 3, 4, 5]);
    let str_vec = vec!["hello", "world"];
    let str_seq = S::from_vec(str_vec);
    assert_eq!(str_seq, apas_ai::seq!["hello", "world"]);
}

#[test]
fn test_sequence_equality_natural_numbers() {
    let seq1 = apas_ai::seq![42];
    let seq2 = apas_ai::seq![42];
    let seq3 = apas_ai::seq![99];
    assert_eq!(seq1.length(), 1);
    assert_eq!(seq2.length(), 1);
    assert_eq!(seq3.length(), 1);
    assert_eq!(seq1, apas_ai::seq![42]);
    assert_eq!(seq2, apas_ai::seq![42]);
    assert_eq!(seq3, apas_ai::seq![99]);

    let seq4 = apas_ai::seq![1, 2, 3, 4, 5];
    let seq5 = apas_ai::seq![1, 2, 3, 4, 5];
    let seq6 = apas_ai::seq![1, 2, 3, 4, 6];
    let seq7 = apas_ai::seq![1, 2, 3, 4];
    assert_eq!(seq4.length(), 5);
    assert_eq!(seq5.length(), 5);
    assert_eq!(seq6.length(), 5);
    assert_eq!(seq7.length(), 4);
    assert_eq!(seq4, seq5);
    assert_ne!(seq4, seq6);
    assert_ne!(seq4.length(), seq7.length());
}

#[test]
fn test_sequence_equality_strings() {
    let seq1 = apas_ai::seq!["hello"];
    let seq2 = apas_ai::seq!["hello"];
    let seq3 = apas_ai::seq!["world"];
    assert_eq!(seq1.length(), 1);
    assert_eq!(seq2.length(), 1);
    assert_eq!(seq3.length(), 1);
    assert_eq!(seq1, apas_ai::seq!["hello"]);
    assert_eq!(seq2, apas_ai::seq!["hello"]);
    assert_eq!(seq3, apas_ai::seq!["world"]);

    let seq4 = apas_ai::seq!["the", "cat", "in", "the", "hat"];
    let seq5 = apas_ai::seq!["the", "cat", "in", "the", "hat"];
    let seq6 = apas_ai::seq!["the", "cat", "in", "the", "mat"];
    let seq7 = apas_ai::seq!["the", "cat", "in", "the"];
    assert_eq!(seq4.length(), 5);
    assert_eq!(seq5.length(), 5);
    assert_eq!(seq6.length(), 5);
    assert_eq!(seq7.length(), 4);
    assert_eq!(seq4, seq5);
    assert_ne!(seq4, seq6);
    assert_ne!(seq4.length(), seq7.length());
}

#[test]
fn test_eq_vs_partial_eq_difference() {
    #[derive(Clone, Debug, PartialEq)]
    struct PartialComparable { value: f64 }
    let seq1 = apas_ai::seq![PartialComparable { value: 1.0 }];
    let seq2 = apas_ai::seq![PartialComparable { value: 1.0 }];
    assert_eq!(seq1.nth(0), seq2.nth(0));
    let nan_seq = apas_ai::seq![PartialComparable { value: f64::NAN }];
    let nan_seq2 = apas_ai::seq![PartialComparable { value: f64::NAN }];
    assert_ne!(nan_seq.nth(0), nan_seq2.nth(0));

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TotalComparable { value: i32 }
    let total_seq1 = apas_ai::seq![TotalComparable { value: 42 }];
    let total_seq2 = apas_ai::seq![TotalComparable { value: 42 }];
    assert_eq!(total_seq1.nth(0), total_seq2.nth(0));
    let b_seq1 = apas_ai::seq![B::True, B::False];
    let b_seq2 = apas_ai::seq![B::True, B::False];
    assert_eq!(b_seq1.nth(0), b_seq2.nth(0));
    assert_eq!(b_seq1.nth(1), b_seq2.nth(1));
    assert_ne!(b_seq1.nth(0), b_seq1.nth(1));
}

#[test]
fn test_ordering_numbers_basic() {
    let a: N = 1;
    let b: N = 2;
    assert!(matches!(a.cmp(&b), O::Less));
    assert!(matches!(b.cmp(&a), O::Greater));
    assert!(matches!(a.cmp(&a), O::Equal));
}

#[test]
fn test_numbers_equal_is_equal() {
    let x: N = 7;
    assert!(matches!(x.cmp(&x), O::Equal));
}

#[test]
fn test_ordering_strings_basic() {
    let a = "a";
    let b = "b";
    assert!(matches!(a.cmp(&b), O::Less));
    assert!(matches!(b.cmp(&a), O::Greater));
    assert!(matches!(a.cmp(&a), O::Equal));
}

#[test]
fn test_strings_equal_is_equal() {
    let s = "foo";
    assert!(matches!(s.cmp(&s), O::Equal));
}

#[test]
#[should_panic]
fn test_nth_on_empty_panics() {
    let e: S<i32> = <S<i32> as Sequence<i32>>::empty();
    e.nth(0);
}

#[test]
#[should_panic]
fn test_nth_upper_bound_panics() {
    let a = apas_ai::seq![1, 2, 3];
    a.nth(3);
}
