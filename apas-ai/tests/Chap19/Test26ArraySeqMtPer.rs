//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod Test26ArraySeqMtPer {
    use apas_ai::Chap19::ArraySeqMtPer::ArraySeqMtPer::*;
    use apas_ai::ArrayMtPerSLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_new_and_set() {
        let a: ArraySeqMtPerS<N> = ArrayMtPerSLit![7; 3];
        assert_eq!(a.length(), 3);
        assert_eq!(*a.nth(0), 7);
        assert_eq!(*a.nth(1), 7);
        assert_eq!(*a.nth(2), 7);
        let b = ArraySeqMtPerS::update(&a, 1, 9);
        let c = ArraySeqMtPerS::update(&b, 0, 2);
        assert_eq!(*c.nth(0), 2);
        assert_eq!(*c.nth(1), 9);
        assert_eq!(*c.nth(2), 7);
    }

    #[test]
    fn test_length_and_nth_basic() {
        let a = ArrayMtPerSLit![10, 20, 30, 40];
        assert_eq!(a.length(), 4);
        assert_eq!(*a.nth(0), 10);
        assert_eq!(*a.nth(3), 40);
    }

    #[test]
    fn test_empty() {
        let empty: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.length() == 0, true);
    }

    #[test]
    fn test_sequence_basic() {
        let a: ArraySeqMtPerS<B> = ArrayMtPerSLit![B::False; 10];
        assert_eq!(a.length() == 0, false);
        assert_eq!(a.length(), 10);
        let b = a.update(0, B::True);
        let c = b.update(1, B::False);
        let d = c.update(2, B::True);
        assert_eq!(d.length(), 10);
        let head4 = ArrayMtPerSLit![*d.nth(0), *d.nth(1), *d.nth(2), *d.nth(3)];
        assert_eq!(head4, ArrayMtPerSLit![B::True, B::False, B::True, B::False]);
    }

    #[test]
    fn test_singleton() {
        let a: ArraySeqMtPerS<N> = ArraySeqMtPerS::singleton(42);
        assert_eq!(a.length(), 1);
        assert_eq!(*a.nth(0), 42);
        assert_eq!(a.length() == 0, false);
    }

    #[test]
    fn test_from_vec() {
        let a = ArrayMtPerSLit![1, 2, 3, 4, 5];
        assert_eq!(a.length(), 5);
        assert_eq!(*a.nth(0), 1);
        assert_eq!(*a.nth(4), 5);
    }

    #[test]
    fn test_subseq() {
        let a = ArrayMtPerSLit![10, 20, 30, 40, 50];
        let sub = a.subseq(1, 3);
        assert_eq!(sub.length(), 3);
        assert_eq!(*sub.nth(0), 20);
        assert_eq!(*sub.nth(1), 30);
        assert_eq!(*sub.nth(2), 40);
    }

    #[test]
    fn test_subseq_view() {
        let a = ArrayMtPerSLit![10, 20, 30, 40, 50];
        let view = a.subseq(1, 3);
        assert_eq!(view.len(), 3);
        assert_eq!(view[0], 20);
        assert_eq!(view[1], 30);
        assert_eq!(view[2], 40);
    }

    #[test]
    fn test_iterators() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let collected: Vec<N> = a.iter().copied().collect();
        // For MT, order might vary, so use set comparison
        assert!(ArraySeqSetEq(
            collected.len(),
            |i| collected[i].clone(),
            3,
            |i| [1, 2, 3][i].clone()
        ));
    }

    #[test]
    fn test_set_out_of_bounds() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let result = ArraySeqMtPerS::update(&a, 5, 99);
        assert!(result.is_err());
    }

    #[test]
    fn test_macro_literals() {
        let empty: ArraySeqMtPerS<N> = ArrayMtPerSLit![];
        assert_eq!(empty.length(), 0);

        let single = ArrayMtPerSLit![42];
        assert_eq!(single.length(), 1);
        assert_eq!(*single.nth(0), 42);

        let multi = ArrayMtPerSLit![1, 2, 3, 4];
        assert_eq!(multi.length(), 4);
        assert_eq!(*multi.nth(0), 1);
        assert_eq!(*multi.nth(3), 4);

        let repeated = ArrayMtPerSLit![7; 5];
        assert_eq!(repeated.length(), 5);
        assert_eq!(*repeated.nth(0), 7);
        assert_eq!(*repeated.nth(4), 7);
    }

    #[test]
    fn test_equality_and_debug() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let b = ArrayMtPerSLit![1, 2, 3];
        let c = ArrayMtPerSLit![1, 2, 4];

        assert_eq!(a, b);
        assert_ne!(a, c);

        // Debug format should work
        let debug_str = format!("{:?}", a);
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_display_format() {
        let a = ArrayMtPerSLit![1, 2, 3];
        let display_str = format!("{}", a);
        assert!(display_str.contains("1"));
        assert!(display_str.contains("2"));
        assert!(display_str.contains("3"));
    }

    #[test]
    fn test_string_sequences() {
        let a = ArrayMtPerSLit!["hello", "world"];
        assert_eq!(a.length(), 2);
        assert_eq!(*a.nth(0), "hello");
        assert_eq!(*a.nth(1), "world");
    }

    #[test]
    fn test_boolean_sequences() {
        let a = ArrayMtPerSLit![B::True, B::False, B::True];
        assert_eq!(a.length(), 3);
        assert_eq!(*a.nth(0), B::True);
        assert_eq!(*a.nth(1), B::False);
        assert_eq!(*a.nth(2), B::True);
    }
}
