//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.7 (Comprehension with Conditionals): even elements of a paired with vowels of b.
use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_ai::Chap21::Exercise21_7::Exercise21_7::*;
use apas_ai::Types::Types::*;
use apas_ai::{ArraySeqStPerSLit, PairLit};

#[test]
fn test_pair_even_with_vowels_basic() {
    let a = ArraySeqStPerSLit![1, 2, 3, 4];
    let b = ArraySeqStPerSLit!['a', 'b', 'e'];
    let s = pair_even_with_vowels(&a, &b);
    let expect = ArraySeqStPerSLit![Pair(2, 'a'), Pair(2, 'e'), Pair(4, 'a'), Pair(4, 'e')];
    assert_eq!(s, expect);
}

#[test]
fn test_pair_even_with_vowels_debug_shape() {
    let a = ArraySeqStPerSLit![2];
    let b = ArraySeqStPerSLit!['a', 'x'];
    let s = pair_even_with_vowels(&a, &b);
    let dbg_str = format!("{s:?}");
    assert!(!dbg_str.is_empty());
}

#[test]
fn test_is_even() {
    assert!(is_even(&0));
    assert!(!is_even(&1));
    assert!(is_even(&2));
    assert!(!is_even(&3));
    assert!(is_even(&4));
    assert!(!is_even(&5));
    assert!(is_even(&100));
    assert!(!is_even(&99));
}

#[test]
fn test_is_vowel() {
    assert!(is_vowel(&'a'));
    assert!(is_vowel(&'e'));
    assert!(is_vowel(&'i'));
    assert!(is_vowel(&'o'));
    assert!(is_vowel(&'u'));
    assert!(is_vowel(&'A'));
    assert!(is_vowel(&'E'));
    assert!(is_vowel(&'I'));
    assert!(is_vowel(&'O'));
    assert!(is_vowel(&'U'));
    assert!(!is_vowel(&'b'));
    assert!(!is_vowel(&'x'));
    assert!(!is_vowel(&'z'));
    assert!(!is_vowel(&'B'));
    assert!(!is_vowel(&'X'));
}
