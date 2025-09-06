//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Types::{N, B, O};
use apas_ai::{ArrayPerS, ArraySeqPerTrait};

#[test]
fn test_new_and_set() {
    let a: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::new(3, 7);
    assert_eq!(a.length(), 3);
    assert_eq!(*a.nth(0), 7);
    assert_eq!(*a.nth(1), 7);
    assert_eq!(*a.nth(2), 7);
    let b = <ArrayPerS<N> as ArraySeqPerTrait<N>>::set(&a, 1, 9).unwrap();
    let c = <ArrayPerS<N> as ArraySeqPerTrait<N>>::set(&b, 0, 2).unwrap();
    assert_eq!(*c.nth(0), 2);
    assert_eq!(*c.nth(1), 9);
    assert_eq!(*c.nth(2), 7);
}

#[test]
fn test_length_and_nth_basic() {
    let a = ArrayPerS::from_vec(vec![10, 20, 30, 40]);
    assert_eq!(a.length(), 4);
    assert_eq!(*a.nth(0), 10);
    assert_eq!(*a.nth(3), 40);
}

#[test]
fn test_empty() {
    let empty: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::empty();
    assert_eq!(empty.length(), 0);
    assert_eq!(empty.isEmpty(), B::True);
}

#[test]
fn test_sequence_basic() {
    let a: ArrayPerS<B> = <ArrayPerS<B> as ArraySeqPerTrait<B>>::new(10, B::False);
    assert_eq!(a.isEmpty(), B::False);
    assert_eq!(a.length(), 10);
    let b = <ArrayPerS<B> as ArraySeqPerTrait<B>>::set(&a, 0, B::True).unwrap();
    let c = <ArrayPerS<B> as ArraySeqPerTrait<B>>::set(&b, 1, B::False).unwrap();
    let d = <ArrayPerS<B> as ArraySeqPerTrait<B>>::set(&c, 2, B::True).unwrap();
    assert_eq!(d.length(), 10);
    let head4 = ArrayPerS::from_vec(vec![*d.nth(0), *d.nth(1), *d.nth(2), *d.nth(3)]);
    assert_eq!(head4, ArrayPerS::from_vec(vec![B::True, B::False, B::True, B::False]));
}

#[test]
fn test_singleton() {
    let s = <ArrayPerS<N> as ArraySeqPerTrait<N>>::singleton(42);
    assert_eq!(s.length(), 1);
    assert_eq!(*s.nth(0), 42);
    assert_eq!(s.isSingleton(), B::True);
}

#[test]
fn test_is_empty_and_is_singleton() {
    let e: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::empty();
    assert_eq!(e.isEmpty(), B::True);
    assert_eq!(e.isSingleton(), B::False);

    let s = <ArrayPerS<N> as ArraySeqPerTrait<N>>::singleton(7);
    assert_eq!(s.isEmpty(), B::False);
    assert_eq!(s.isSingleton(), B::True);

    let a = ArrayPerS::from_vec(vec![1, 2]);
    assert_eq!(a.isEmpty(), B::False);
    assert_eq!(a.isSingleton(), B::False);
}

#[test]
fn test_from_vec() {
    let empty_vec: Vec<N> = vec![];
    let empty_seq = ArrayPerS::from_vec(empty_vec);
    assert_eq!(empty_seq.length(), 0);
    assert_eq!(empty_seq.isEmpty(), B::True);
    let single_vec = vec![42];
    let single_seq = ArrayPerS::from_vec(single_vec);
    assert_eq!(single_seq, ArrayPerS::from_vec(vec![42]));
    let multi_vec = vec![1, 2, 3, 4, 5];
    let multi_seq = ArrayPerS::from_vec(multi_vec);
    assert_eq!(multi_seq, ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]));
    let str_vec = vec!["hello", "world"];
    let str_seq = ArrayPerS::from_vec(str_vec);
    assert_eq!(str_seq, ArrayPerS::from_vec(vec!["hello", "world"]));
}

#[test]
fn test_sequence_equality_natural_numbers() {
    let seq1 = ArrayPerS::from_vec(vec![42]);
    let seq2 = ArrayPerS::from_vec(vec![42]);
    let seq3 = ArrayPerS::from_vec(vec![99]);
    assert_eq!(seq1.length(), 1);
    assert_eq!(seq2.length(), 1);
    assert_eq!(seq3.length(), 1);
    assert_eq!(seq1, ArrayPerS::from_vec(vec![42]));
    assert_eq!(seq2, ArrayPerS::from_vec(vec![42]));
    assert_eq!(seq3, ArrayPerS::from_vec(vec![99]));

    let seq4 = ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let seq5 = ArrayPerS::from_vec(vec![1, 2, 3, 4, 5]);
    let seq6 = ArrayPerS::from_vec(vec![1, 2, 3, 4, 6]);
    let seq7 = ArrayPerS::from_vec(vec![1, 2, 3, 4]);
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
    let seq1 = ArrayPerS::from_vec(vec!["hello"]);
    let seq2 = ArrayPerS::from_vec(vec!["hello"]);
    let seq3 = ArrayPerS::from_vec(vec!["world"]);
    assert_eq!(seq1.length(), 1);
    assert_eq!(seq2.length(), 1);
    assert_eq!(seq3.length(), 1);
    assert_eq!(seq1, ArrayPerS::from_vec(vec!["hello"]));
    assert_eq!(seq2, ArrayPerS::from_vec(vec!["hello"]));
    assert_eq!(seq3, ArrayPerS::from_vec(vec!["world"]));

    let seq4 = ArrayPerS::from_vec(vec!["the", "cat", "in", "the", "hat"]);
    let seq5 = ArrayPerS::from_vec(vec!["the", "cat", "in", "the", "hat"]);
    let seq6 = ArrayPerS::from_vec(vec!["the", "cat", "in", "the", "mat"]);
    let seq7 = ArrayPerS::from_vec(vec!["the", "cat", "in", "the"]);
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
    let seq1 = ArrayPerS::from_vec(vec![PartialComparable { value: 1.0 }]);
    let seq2 = ArrayPerS::from_vec(vec![PartialComparable { value: 1.0 }]);
    assert_eq!(seq1.nth(0), seq2.nth(0));
    let nan_seq = ArrayPerS::from_vec(vec![PartialComparable { value: f64::NAN }]);
    let nan_seq2 = ArrayPerS::from_vec(vec![PartialComparable { value: f64::NAN }]);
    assert_ne!(nan_seq.nth(0), nan_seq2.nth(0));

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TotalComparable { value: N }
    let total_seq1 = ArrayPerS::from_vec(vec![TotalComparable { value: 42 }]);
    let total_seq2 = ArrayPerS::from_vec(vec![TotalComparable { value: 42 }]);
    assert_eq!(total_seq1.nth(0), total_seq2.nth(0));
    let b_seq1 = ArrayPerS::from_vec(vec![B::True, B::False]);
    let b_seq2 = ArrayPerS::from_vec(vec![B::True, B::False]);
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
    let e: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::empty();
    e.nth(0);
}

#[test]
#[should_panic]
fn test_nth_upper_bound_panics() {
    let a = ArrayPerS::from_vec(vec![1, 2, 3]);
    a.nth(3);
}

#[test]
#[should_panic]
fn test_set_out_of_bounds_panics_on_unwrap() {
    let a = <ArrayPerS<N> as ArraySeqPerTrait<N>>::new(3, 0);
    let _ = <ArrayPerS<N> as ArraySeqPerTrait<N>>::set(&a, 3, 1).unwrap();
}

#[test]
fn test_set_in_bounds_ok_and_writes() {
    let a = <ArrayPerS<N> as ArraySeqPerTrait<N>>::new(3, 0);
    let b = <ArrayPerS<N> as ArraySeqPerTrait<N>>::set(&a, 1, 5);
    assert!(b.is_ok());
    let c = b.unwrap();
    assert_eq!(*c.nth(1), 5);
}

#[test]
fn test_subseq_copy_trait_form_basic() {
    let a = ArrayPerS::from_vec(vec![10, 20, 30, 40, 50]);
    let b: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::subseq_copy(&a, 1, 3);
    assert_eq!(b, ArrayPerS::from_vec(vec![20, 30, 40]));
    let e: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::subseq_copy(&a, 10, 2);
    assert_eq!(e.length(), 0);
}

#[test]
fn test_new_set_persistent() {
    let a: ArrayPerS<N> = <ArrayPerS<N> as ArraySeqPerTrait<N>>::new(3, 7);
    assert_eq!(a.length(), 3);
    let b = <ArrayPerS<N> as ArraySeqPerTrait<N>>::set(&a, 1, 9).unwrap();
    assert_eq!(*a.nth(1), 7);
    assert_eq!(*b.nth(1), 9);
}


#[test]
fn test_iterator_collects_in_order() {
    let s = ArrayPerS::from_vec(vec![1, 2, 3, 4]);
    let collected: Vec<N> = s.iter().copied().collect();
    assert_eq!(collected, vec![1, 2, 3, 4]);
}


