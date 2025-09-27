//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ArraySeqMtPerChap18 multithreaded algorithms.

pub mod Test27ArraySeqMtPerChap {
    use apas_ai::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use apas_ai::ArrayMtPerSLit;
    use apas_ai::Types::Types::*; // macro import

    // Helper functions for common patterns
    fn identity(i: N) -> N { i }
    fn double(i: N) -> N { i * 2 }
    fn square(i: N) -> N { i * i }
    fn add_100(i: N) -> N { i + 100 }
    fn const_42(_i: N) -> N { 42 }
    fn format_item(i: N) -> String { format!("item_{}", i) }
    fn is_even_bool(i: N) -> B { if i % 2 == 0 { B::True } else { B::False } }

    // Helper function for set equality comparison
    fn assert_set_eq<T: PartialEq + std::fmt::Debug>(actual: &[T], expected: &[T]) {
        assert_eq!(actual.len(), expected.len());
        for val in actual {
            assert!(expected.contains(val), "Value {:?} not found in expected", val);
        }
        for val in expected {
            assert!(actual.contains(val), "Expected value {:?} not found in actual", val);
        }
    }

    #[test]
    fn test_tabulate_basic() {
        let a: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&identity, 5);
        assert_eq!(a.length(), 5);

        // Use set comparison since MT results may vary in order
        let a_vec: Vec<N> = (0..a.length()).map(|i| *a.nth(i)).collect();
        let expected_vec: Vec<N> = (0..5).collect();
        assert_set_eq(&a_vec, &expected_vec);
    }

    #[test]
    fn test_tabulate_fibonacci() {
        fn fib(n: N) -> N {
            match n {
                | 0 => 0,
                | 1 => 1,
                | _ => {
                    let mut a = 0;
                    let mut b = 1;
                    for _ in 2..=n {
                        let temp = a + b;
                        a = b;
                        b = temp;
                    }
                    b
                }
            }
        }

        let fibs: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&fib, 8);
        assert_eq!(fibs.length(), 8);

        // Expected fibonacci sequence: [0, 1, 1, 2, 3, 5, 8, 13]
        let expected = [0, 1, 1, 2, 3, 5, 8, 13];
        let fibs_vec: Vec<N> = (0..fibs.length()).map(|i| *fibs.nth(i)).collect();
        assert_set_eq(&fibs_vec, &expected);
    }

    #[test]
    fn test_tabulate_empty() {
        let empty: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&double, 0);
        assert_eq!(empty.length(), 0);
        assert_eq!(empty.length() == 0, true);
    }

    #[test]
    fn test_tabulate_single() {
        let single: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(const_42, 1);
        assert_eq!(single.length(), 1);
        assert_eq!(*single.nth(0), 42);
    }

    #[test]
    fn test_tabulate_string() {
        let strings: ArraySeqMtPerS<String> = ArraySeqMtPerS::tabulate(format_item, 4);
        assert_eq!(strings.length(), 4);

        // Check that all expected strings are present (order may vary in MT)
        let expected = ["item_0", "item_1", "item_2", "item_3"];
        let strings_vec: Vec<String> = (0..strings.length()).map(|i| strings.nth(i).clone()).collect();
        let expected_vec: Vec<String> = expected.iter().map(|s| s.to_string()).collect();
        assert_set_eq(&strings_vec, &expected_vec);
    }

    #[test]
    fn test_tabulate_boolean() {
        let bools: ArraySeqMtPerS<B> = ArraySeqMtPerS::tabulate(is_even_bool, 6);
        assert_eq!(bools.length(), 6);

        // Expected: [True, False, True, False, True, False]
        let expected = [B::True, B::False, B::True, B::False, B::True, B::False];
        let bools_vec: Vec<B> = (0..bools.length()).map(|i| *bools.nth(i)).collect();
        assert_set_eq(&bools_vec, &expected);
    }

    #[test]
    fn test_tabulate_squares() {
        let squares: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(square, 5);
        assert_eq!(squares.length(), 5);

        // Expected: [0, 1, 4, 9, 16]
        let expected = [0, 1, 4, 9, 16];
        let squares_vec: Vec<N> = (0..squares.length()).map(|i| *squares.nth(i)).collect();
        assert_set_eq(&squares_vec, &expected);
    }

    #[test]
    fn test_tabulate_large() {
        let large: ArraySeqMtPerS<N> = ArraySeqMtPerS::tabulate(&add_100, 1000);
        assert_eq!(large.length(), 1000);
        assert_eq!(*large.nth(0), 100);
        assert_eq!(*large.nth(999), 1099);

        // Check a few random elements
        assert_eq!(*large.nth(500), 600);
        assert_eq!(*large.nth(123), 223);
    }
}
