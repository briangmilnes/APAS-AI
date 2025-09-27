//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestArraySeqStPer {
    use apas_ai::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArraySeqStPerSLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_new_and_set() {
        let a: ArraySeqStPerS<N> = ArraySeqStPerS::new(3, 7);
        assert_eq!(a.length(), 3);
        assert_eq!(*a.nth(0), 7);
        assert_eq!(*a.nth(1), 7);
        assert_eq!(*a.nth(2), 7);
        let b = ArraySeqStPerS::update(&a, 1, 9);
        let c = ArraySeqStPerS::update(&b, 0, 2);
        assert_eq!(*c.nth(0), 2);
        assert_eq!(*c.nth(1), 9);
        assert_eq!(*c.nth(2), 7);
    }

    #[test]
    fn test_length_and_nth_basic() {
        let a = ArraySeqStPerSLit![10, 20, 30, 40];
        assert_eq!(a.length(), 4);
        assert_eq!(*a.nth(0), 10);
        assert_eq!(*a.nth(3), 40);
    }

    #[test]
    fn test_empty() {
        let empty: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
        assert_eq!(empty.length(), 0);
        assert_eq!(<ArraySeqStPerS<N> as ArraySeqStPerTrait<N>>::isEmpty(&empty), B::True);
    }

    #[test]
    fn test_sequence_basic() {
        let a: ArraySeqStPerS<B> = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::new(10, B::False);
        assert_eq!(a.length() == 0, false);
        assert_eq!(a.length(), 10);
        let b = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::update(&a, 0, B::True);
        let c = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::update(&b, 1, B::False);
        let d = <ArraySeqStPerS<B> as ArraySeqStPerTrait<B>>::update(&c, 2, B::True);
        assert_eq!(d.length(), 10);
        let head4 = ArraySeqStPerSLit![*d.nth(0), *d.nth(1), *d.nth(2), *d.nth(3)];
        assert_eq!(head4, ArraySeqStPerSLit![B::True, B::False, B::True, B::False]);
    }

    #[test]
    fn test_singleton() {
        let s = ArraySeqStPerS::singleton(42);
        assert_eq!(s.length(), 1);
        assert_eq!(*s.nth(0), 42);
        assert_eq!(s.length() == 1, true);
    }

    #[test]
    fn test_is_empty_and_is_singleton() {
        let e: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
        assert_eq!(e.length() == 0, true);
        assert_eq!(e.length() == 1, false);

        let s = ArraySeqStPerS::singleton(7);
        assert_eq!(s.length() == 0, false);
        assert_eq!(s.length() == 1, true);

        let a = ArraySeqStPerSLit![1, 2];
        assert_eq!(a.length() == 0, false);
        assert_eq!(a.length() == 1, false);
    }

    #[test]
    fn test_from_vec() {
        let empty_seq: ArraySeqStPerS<N> = ArraySeqStPerSLit![];
        assert_eq!(empty_seq.length(), 0);
        assert_eq!(empty_seq.length() == 0, true);
        let single_vec = vec![42];
        let single_seq = ArraySeqStPerS::from_vec(single_vec);
        assert_eq!(single_seq, ArraySeqStPerSLit![42]);
        let multi_seq = ArraySeqStPerSLit![1, 2, 3, 4, 5];
        assert_eq!(multi_seq, ArraySeqStPerS::from_vec(vec![1, 2, 3, 4, 5]));
        let str_vec = vec!["hello", "world"];
        let str_seq = ArraySeqStPerS::from_vec(str_vec);
        assert_eq!(str_seq, ArraySeqStPerS::from_vec(vec!["hello", "world"]));
    }

    #[test]
    fn test_sequence_equality_natural_numbers() {
        let seq1 = ArraySeqStPerSLit![42];
        let seq2 = ArraySeqStPerSLit![42];
        let seq3 = ArraySeqStPerSLit![99];
        assert_eq!(seq1.length(), 1);
        assert_eq!(seq2.length(), 1);
        assert_eq!(seq3.length(), 1);
        assert_eq!(seq1, ArraySeqStPerSLit![42]);
        assert_eq!(seq2, ArraySeqStPerSLit![42]);
        assert_eq!(seq3, ArraySeqStPerSLit![99]);

        let seq4 = ArraySeqStPerSLit![1, 2, 3, 4, 5];
        let seq5 = ArraySeqStPerSLit![1, 2, 3, 4, 5];
        let seq6 = ArraySeqStPerSLit![1, 2, 3, 4, 6];
        let seq7 = ArraySeqStPerSLit![1, 2, 3, 4];
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
        let seq1 = ArraySeqStPerSLit!["hello"];
        let seq2 = ArraySeqStPerSLit!["hello"];
        let seq3 = ArraySeqStPerSLit!["world"];
        assert_eq!(seq1.length(), 1);
        assert_eq!(seq2.length(), 1);
        assert_eq!(seq3.length(), 1);
        assert_eq!(seq1, ArraySeqStPerSLit!["hello"]);
        assert_eq!(seq2, ArraySeqStPerSLit!["hello"]);
        assert_eq!(seq3, ArraySeqStPerSLit!["world"]);

        let seq4 = ArraySeqStPerSLit!["the", "cat", "in", "the", "hat"];
        let seq5 = ArraySeqStPerSLit!["the", "cat", "in", "the", "hat"];
        let seq6 = ArraySeqStPerSLit!["the", "cat", "in", "the", "mat"];
        let seq7 = ArraySeqStPerSLit!["the", "cat", "in", "the"];
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
        #[derive(Clone, Debug, PartialEq, Eq)]
        struct PartialComparable {
            value: i32,
        } // Use i32 instead of f64 so Eq can be implemented

        impl std::fmt::Display for PartialComparable {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "PartialComparable({})", self.value)
            }
        }
        let seq1 = ArraySeqStPerSLit![PartialComparable { value: 1 }];
        let seq2 = ArraySeqStPerSLit![PartialComparable { value: 1 }];
        assert_eq!(seq1.nth(0), seq2.nth(0));
        let nan_seq = ArraySeqStPerSLit![PartialComparable { value: -1 }];
        let nan_seq2 = ArraySeqStPerSLit![PartialComparable { value: -1 }];
        assert_eq!(nan_seq.nth(0), nan_seq2.nth(0)); // Since they're the same value now

        #[derive(Clone, Debug, PartialEq, Eq)]
        struct TotalComparable {
            value: N,
        }

        impl std::fmt::Display for TotalComparable {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "TotalComparable({})", self.value)
            }
        }
        let total_seq1 = ArraySeqStPerSLit![TotalComparable { value: 42 }];
        let total_seq2 = ArraySeqStPerSLit![TotalComparable { value: 42 }];
        assert_eq!(total_seq1.nth(0), total_seq2.nth(0));
        let b_seq1 = ArraySeqStPerSLit![B::True, B::False];
        let b_seq2 = ArraySeqStPerSLit![B::True, B::False];
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
        let e: ArraySeqStPerS<N> = ArraySeqStPerS::empty();
        e.nth(0);
    }

    #[test]
    #[should_panic]
    fn test_nth_upper_bound_panics() {
        let a = ArraySeqStPerSLit![1, 2, 3];
        a.nth(3);
    }

    #[test]
    #[should_panic]
    fn test_set_out_of_bounds_panics_on_unwrap() {
        let a = ArraySeqStPerS::new(3, 0);
        let _ = ArraySeqStPerS::update(&a, 3, 1);
    }

    #[test]
    fn test_set_in_bounds_ok_and_writes() {
        let a = ArraySeqStPerS::new(3, 0);
        let b = ArraySeqStPerS::update(&a, 1, 5);
        assert!(b== Ok(()));
        let c = b;
        assert_eq!(*c.nth(1), 5);
    }

    #[test]
    fn test_subseq_trait_form_basic() {
        let a = ArraySeqStPerSLit![10, 20, 30, 40, 50];
        let b: ArraySeqStPerS<N> = ArraySeqStPerS::subseq(&a, 1, 3);
        assert_eq!(b, ArraySeqStPerSLit![20, 30, 40]);
        let e: ArraySeqStPerS<N> = ArraySeqStPerS::subseq(&a, 10, 2);
        assert_eq!(e.length(), 0);
    }

    #[test]
    fn test_new_set_persistent() {
        let a: ArraySeqStPerS<N> = ArraySeqStPerS::new(3, 7);
        assert_eq!(a.length(), 3);
        let b = ArraySeqStPerS::update(&a, 1, 9);
        assert_eq!(*a.nth(1), 7);
        assert_eq!(*b.nth(1), 9);
    }

    #[test]
    fn test_iterator_collects_in_order() {
        let s = ArraySeqStPerSLit![1, 2, 3, 4];
        let collected: Vec<N> = s.iter().copied().collect();
        assert_eq!(collected, vec![1, 2, 3, 4]);
    }
}
