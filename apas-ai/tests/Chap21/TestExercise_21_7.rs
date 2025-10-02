//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
/// Exercise 21.7 (Comprehension with Conditionals): even elements of a paired with vowels of b.

pub mod Test33Exercise_21_7 {

    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::Chap21::Exercise21_7::Exercise21_7::*;
    use apas_ai::PairLit;
    use apas_ai::Types::Types::*;

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
        let dbg_str = format!("{:?}", s);
        assert!(!dbg_str.is_empty());
    }
}
