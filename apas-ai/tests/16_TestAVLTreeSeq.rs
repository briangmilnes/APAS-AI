//! Tests for AVLTreeSeq: implicit-order AVL with O(lg n) nth/set and ArraySeq parity.

use apas_ai::Types::{N, B, O};
use apas_ai::AVLTreeSeq::{AVLTreeSeq, AVLTreeS as AVLTree3};
use apas_ai::ArraySeq::{ArrayS, ArraySeq};

#[test]
fn test_empty_singleton_and_predicates() {
    let e: AVLTree3<N> = apas_ai::avltreeseq![];
    assert_eq!(e.length(), 0);
    assert_eq!(e.isEmpty(), B::True);
    assert_eq!(e.isSingleton(), B::False);

    let s = <AVLTree3<N> as AVLTreeSeq<N>>::singleton(42);
    assert_eq!(s.length(), 1);
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&s, 0), 42);
    assert_eq!(s.isEmpty(), B::False);
    assert_eq!(s.isSingleton(), B::True);
}

#[test]
fn test_new_and_set() {
    let mut t: AVLTree3<N> = apas_ai::avltreeseq![0; 3];
    assert_eq!(t.length(), 3);
    // defaults are 0 for N
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 0), 0);
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 1), 0);
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 2), 0);
    let _ = <AVLTree3<N> as AVLTreeSeq<N>>::set(&mut t, 1, 9).unwrap();
    let _ = t.update((0, 2));
    let _ = <AVLTree3<N> as AVLTreeSeq<N>>::set(&mut t, 2, 7).unwrap();
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 0), 2);
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 1), 9);
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 2), 7);
}

#[test]
fn test_from_vec_and_to_arrayseq() {
    let a: AVLTree3<N> = AVLTree3::from_vec(vec![10, 20, 30, 40]);
    assert_eq!(a.length(), 4);
    assert_eq!(*<AVLTree3<N> as AVLTreeSeq<N>>::nth(&a, 0), 10);
    let arr: ArrayS<N> = a.to_arrayseq();
    assert_eq!(arr, apas_ai::arrayseq![10, 20, 30, 40]);
}

#[test]
fn test_subseq_copy_basic() {
    let a: AVLTree3<N> = AVLTree3::from_vec(vec![10, 20, 30, 40, 50]);
    let b: AVLTree3<N> = <AVLTree3<N> as AVLTreeSeq<N>>::subseq_copy(&a, 1, 3);
    let arr = b.to_arrayseq();
    assert_eq!(arr, apas_ai::arrayseq![20, 30, 40]);
    let e: AVLTree3<N> = <AVLTree3<N> as AVLTreeSeq<N>>::subseq_copy(&a, 10, 2);
    assert_eq!(e.length(), 0);
}

#[test]
fn test_ordering_numbers_and_strings_parity() {
    let a: N = 1; let b: N = 2;
    assert!(matches!(a.cmp(&b), O::Less));
    assert!(matches!(b.cmp(&a), O::Greater));
    assert!(matches!(a.cmp(&a), O::Equal));
    let s = "foo";
    assert!(matches!(s.cmp(&s), O::Equal));
}

#[test]
fn test_out_of_bounds_behaviors() {
    use std::panic;
    let e: AVLTree3<N> = <AVLTree3<N> as AVLTreeSeq<N>>::empty();
    let panicked = panic::catch_unwind(|| { let _ = <AVLTree3<N> as AVLTreeSeq<N>>::nth(&e, 0); }).is_err();
    assert!(panicked);
    let mut s: AVLTree3<N> = <AVLTree3<N> as AVLTreeSeq<N>>::new();
    assert!(<AVLTree3<N> as AVLTreeSeq<N>>::set(&mut s, 3, 1).is_err());
}

#[test]
fn test_equality_and_debug() {
    let x = AVLTree3::from_vec(vec![1,2,3]);
    let y = AVLTree3::from_vec(vec![1,2,3]);
    let z = AVLTree3::from_vec(vec![1,2,4]);
    assert_eq!(x, y);
    assert_ne!(x, z);
    let _ = format!("{:?}", x);
}

#[test]
fn test_sequence_equality_natural_numbers() {
    let a = apas_ai::avltreeseq![1, 2, 3, 4, 5];
    let b = apas_ai::avltreeseq![1, 2, 3, 4, 5];
    let c = apas_ai::avltreeseq![1, 2, 3, 4, 6];
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_sequence_equality_strings() {
    let a = apas_ai::avltreeseq!["the", "cat", "in", "the", "hat"];
    let b = apas_ai::avltreeseq!["the", "cat", "in", "the", "hat"];
    let c = apas_ai::avltreeseq!["the", "cat", "in", "the", "mat"];
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_sequence_basic_booleans() {
    let mut t = apas_ai::avltreeseq![apas_ai::Types::B::False; 4];
    let _ = <AVLTree3<apas_ai::Types::B> as AVLTreeSeq<apas_ai::Types::B>>::set(&mut t, 0, apas_ai::Types::B::True);
    let head = apas_ai::avltreeseq![*<AVLTree3<apas_ai::Types::B> as AVLTreeSeq<apas_ai::Types::B>>::nth(&t, 0),
                                  *<AVLTree3<apas_ai::Types::B> as AVLTreeSeq<apas_ai::Types::B>>::nth(&t, 1)];
    let expected = apas_ai::arrayseq![apas_ai::Types::B::True, apas_ai::Types::B::False];
    assert_eq!(head.to_arrayseq(), expected);
}

#[test]
fn test_from_vec_strings() {
    let s = AVLTree3::from_vec(vec!["hello", "world"]);
    let arr = s.to_arrayseq();
    assert_eq!(arr, apas_ai::arrayseq!["hello", "world"]);
}

#[test]
#[should_panic]
fn test_nth_upper_bound_panics_avl3() {
    let t = AVLTree3::from_vec(vec![1usize, 2, 3]);
    let _ = <AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 3);
}

#[test]
#[should_panic]
fn test_nth_on_empty_panics_avl3() {
    let t: AVLTree3<N> = <AVLTree3<N> as AVLTreeSeq<N>>::empty();
    let _ = <AVLTree3<N> as AVLTreeSeq<N>>::nth(&t, 0);
}

#[test]
#[should_panic]
fn test_set_out_of_bounds_panics_on_unwrap_avl3() {
    let mut t: AVLTree3<N> = apas_ai::avltreeseq![0; 3];
    let _ = <AVLTree3<N> as AVLTreeSeq<N>>::set(&mut t, 3, 1).unwrap();
}

#[test]
fn test_eq_vs_partial_eq_difference_avl3() {
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct PartialComparable { value: f64 }
    let s1 = AVLTree3::from_vec(vec![PartialComparable { value: 1.0 }]);
    let s2 = AVLTree3::from_vec(vec![PartialComparable { value: 1.0 }]);
    assert_eq!(<AVLTree3<PartialComparable> as AVLTreeSeq<PartialComparable>>::nth(&s1, 0),
               <AVLTree3<PartialComparable> as AVLTreeSeq<PartialComparable>>::nth(&s2, 0));
    let n1 = AVLTree3::from_vec(vec![PartialComparable { value: f64::NAN }]);
    let n2 = AVLTree3::from_vec(vec![PartialComparable { value: f64::NAN }]);
    assert_ne!(<AVLTree3<PartialComparable> as AVLTreeSeq<PartialComparable>>::nth(&n1, 0),
               <AVLTree3<PartialComparable> as AVLTreeSeq<PartialComparable>>::nth(&n2, 0));
}

#[test]
fn test_iterator() {
    // Non 0..n order
    let s = apas_ai::avltreeseq![3usize, 1, 4, 2];
    let collected: Vec<usize> = s.iter().copied().collect();
    assert_eq!(collected, vec![3, 1, 4, 2]);

    // Empty
    let e: apas_ai::AVLTreeSeq::AVLTreeS<usize> = apas_ai::avltreeseq![];
    assert!(e.iter().copied().collect::<Vec<_>>().is_empty());

    // Singleton
    let one = apas_ai::avltreeseq![7usize];
    assert_eq!(one.iter().copied().collect::<Vec<_>>(), vec![7]);
}

