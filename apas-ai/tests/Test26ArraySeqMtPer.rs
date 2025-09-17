//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod Test26ArraySeqMtPer {
    use apas_ai::ArraySeqMtPer;
    use apas_ai::{ArraySeqMtPer::ArraySeqMtPer::*, Types::Types::*}; // macro import

    #[test]
    fn test_new_and_set() {
        let a: ArrayMtPerS<N> = ArrayMtPerS::new(3, 7);
        assert_eq!(a.length(), 3);
        assert_eq!(*a.nth(0), 7);
        assert_eq!(*a.nth(1), 7);
        assert_eq!(*a.nth(2), 7);
        let b = ArrayMtPerS::set(&a, 1, 9).unwrap();
        let c = ArrayMtPerS::set(&b, 0, 2).unwrap();
        assert_eq!(*c.nth(0), 2);
        assert_eq!(*c.nth(1), 9);
        assert_eq!(*c.nth(2), 7);
    }

    #[test]
    fn test_length_and_nth_basic() {
        let a = ArraySeqMtPer![10, 20, 30, 40];
        assert_eq!(a.length(), 4);
        assert_eq!(*a.nth(0), 10);
        assert_eq!(*a.nth(3), 40);
    }

    #[test]
    fn test_empty() {
        let empty: ArrayMtPerS<N> = ArrayMtPerS::empty();
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.isEmpty(), B::True);
    }

    #[test]
    fn test_sequence_basic() {
        let a: ArrayMtPerS<B> = ArrayMtPerS::new(10, B::False);
        assert_eq!(a.isEmpty(), B::False);
        assert_eq!(a.length(), 10);
        let b = a.set(0, B::True).unwrap();
        let c = b.set(1, B::False).unwrap();
        let d = c.set(2, B::True).unwrap();
        assert_eq!(d.length(), 10);
        let head4 = ArraySeqMtPer![*d.nth(0), *d.nth(1), *d.nth(2), *d.nth(3)];
        assert_eq!(head4, ArraySeqMtPer![B::True, B::False, B::True, B::False]);
    }

    #[test]
    fn test_singleton() {
        let a: ArrayMtPerS<N> = ArrayMtPerS::singleton(42);
        assert_eq!(a.length(), 1);
        assert_eq!(*a.nth(0), 42);
        assert_eq!(a.isEmpty(), B::False);
    }

    #[test]
    fn test_from_vec() {
        let v = vec![1, 2, 3, 4, 5];
        let a = ArrayMtPerS::from_vec(v);
        assert_eq!(a.length(), 5);
        assert_eq!(*a.nth(0), 1);
        assert_eq!(*a.nth(4), 5);
    }

    #[test]
    fn test_subseq_copy() {
        let a = ArraySeqMtPer![10, 20, 30, 40, 50];
        let sub = a.subseq_copy(1, 3);
        assert_eq!(sub.length(), 3);
        assert_eq!(*sub.nth(0), 20);
        assert_eq!(*sub.nth(1), 30);
        assert_eq!(*sub.nth(2), 40);
    }

    #[test]
    fn test_subseq_view() {
        let a = ArraySeqMtPer![10, 20, 30, 40, 50];
        let view = a.subseq(1, 3);
        assert_eq!(view.len(), 3);
        assert_eq!(view[0], 20);
        assert_eq!(view[1], 30);
        assert_eq!(view[2], 40);
    }

    #[test]
    fn test_iterators() {
        let a = ArraySeqMtPer![1, 2, 3];
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
        let a = ArraySeqMtPer![1, 2, 3];
        let result = ArrayMtPerS::set(&a, 5, 99);
        assert!(result.is_err());
    }

    #[test]
    fn test_macro_literals() {
        let empty: ArrayMtPerS<N> = ArraySeqMtPer![];
        assert_eq!(empty.length(), 0);

        let single = ArraySeqMtPer![42];
        assert_eq!(single.length(), 1);
        assert_eq!(*single.nth(0), 42);

        let multi = ArraySeqMtPer![1, 2, 3, 4];
        assert_eq!(multi.length(), 4);
        assert_eq!(*multi.nth(0), 1);
        assert_eq!(*multi.nth(3), 4);

        let repeated = ArraySeqMtPer![7; 5];
        assert_eq!(repeated.length(), 5);
        assert_eq!(*repeated.nth(0), 7);
        assert_eq!(*repeated.nth(4), 7);
    }

    #[test]
    fn test_equality_and_debug() {
        let a = ArraySeqMtPer![1, 2, 3];
        let b = ArraySeqMtPer![1, 2, 3];
        let c = ArraySeqMtPer![1, 2, 4];

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
        let a = ArraySeqMtPer![1, 2, 3];
        let display_str = format!("{}", a);
        assert!(display_str.contains("1"));
        assert!(display_str.contains("2"));
        assert!(display_str.contains("3"));
    }

    #[test]
    fn test_string_sequences() {
        let a = ArraySeqMtPer!["hello", "world"];
        assert_eq!(a.length(), 2);
        assert_eq!(*a.nth(0), "hello");
        assert_eq!(*a.nth(1), "world");
    }

    #[test]
    fn test_boolean_sequences() {
        let a = ArraySeqMtPer![B::True, B::False, B::True];
        assert_eq!(a.length(), 3);
        assert_eq!(*a.nth(0), B::True);
        assert_eq!(*a.nth(1), B::False);
        assert_eq!(*a.nth(2), B::True);
    }
}
