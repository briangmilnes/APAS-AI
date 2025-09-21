//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

pub mod TestArraySeqStEphChap19 {
    use apas_ai::ArraySeqStEph;
    use apas_ai::ArraySeqStEphSLit;
    use apas_ai::{ArraySeqStEph::ArraySeqStEph::*, ArraySeqStEphChap19::ArraySeqStEphChap19::*, Types::Types::*}; // macro import

    #[test]
    fn test_empty() {
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
        assert_eq!(e.length(), 0);
    }

    #[test]
    fn test_singleton() {
        let s: ArraySeqStEphS<N> = ArraySeqStEphS::singleton(42);
        assert_eq!(s, ArraySeqStEphSLit![42]);
    }

    #[test]
    fn test_map() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 5);
        let b: ArraySeqStEphS<N> = ArraySeqStEphS::map(&a, |x| x * 2);
        assert_eq!(b, ArraySeqStEphSLit![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_append() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 3);
        let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 4, 2);
        let c: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &b);
        assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_append2() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 3);
        let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 4, 2);
        let c: ArraySeqStEphS<N> = ArraySeqStEphS::append2(&a, &b);
        assert_eq!(c, ArraySeqStEphSLit![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deflate_true() {
        let y: ArraySeqStEphS<N> = ArraySeqStEphS::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &6);
        assert_eq!(y, ArraySeqStEphSLit![6]);
    }

    #[test]
    fn test_deflate_false() {
        let y: ArraySeqStEphS<N> = ArraySeqStEphS::deflate(|&x: &N| if x % 2 == 0 { B::True } else { B::False }, &5);
        assert_eq!(y.length(), 0);
    }

    #[test]
    fn test_filter_even_numbers() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 10);
        let evens: ArraySeqStEphS<N> = ArraySeqStEphS::filter(&a, |x| if *x % 2 == 0 { B::True } else { B::False });
        assert_eq!(evens, ArraySeqStEphSLit![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_filter_none() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i, 5);
        let odds_only: ArraySeqStEphS<N> = ArraySeqStEphS::filter(&a, |x| if *x % 2 == 1 { B::True } else { B::False });
        assert_eq!(odds_only, ArraySeqStEphSLit![1, 3]);
    }

    #[test]
    fn test_update_in_bounds() {
        let mut a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 5);
        let _ = a.update(Pair(2, 99).into());
        assert_eq!(a, ArraySeqStEphSLit![1, 2, 99, 4, 5]);
    }

    #[test]
    fn test_update_out_of_bounds() {
        let mut a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 3);
        let _ = a.update(Pair(10, 7).into());
        assert_eq!(a, ArraySeqStEphSLit![1, 2, 3]);
    }

    #[test]
    fn test_isEmpty() {
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
        assert_eq!(e.isEmpty(), B::True);
        let s: ArraySeqStEphS<N> = ArraySeqStEphS::singleton(7);
        assert_eq!(s.isEmpty(), B::False);
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i, 2);
        assert_eq!(a.isEmpty(), B::False);
    }

    #[test]
    fn test_isSingleton() {
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::empty();
        assert_eq!(e.isSingleton(), B::False);
        let s: ArraySeqStEphS<N> = ArraySeqStEphS::singleton(7);
        assert_eq!(s.isSingleton(), B::True);
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i, 2);
        assert_eq!(a.isSingleton(), B::False);
    }

    #[test]
    fn test_iterate_sum() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 5);
        let sum: N = ArraySeqStEphS::iterate(&a, |acc: &N, x: &N| acc + x, 0);
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_iterate_concat() {
        let a: ArraySeqStEphS<&str> = ArraySeqStEphS::tabulate(
            |i| match i {
                | 0 => "a",
                | 1 => "b",
                | _ => "c",
            },
            3,
        );
        let res: String = ArraySeqStEphS::iterate(&a, |acc: &String, x: &&str| format!("{}{}", acc, x), String::new());
        assert_eq!(res, "abc");
    }

    #[test]
    fn test_map_empty() {
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|_| 0, 0);
        let m: ArraySeqStEphS<N> = ArraySeqStEphS::map(&e, |x| x + 1);
        assert_eq!(m.length(), 0);
    }

    #[test]
    fn test_append_with_empty() {
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|_| 0, 0);
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 3);
        let left: ArraySeqStEphS<N> = ArraySeqStEphS::append(&e, &a);
        assert_eq!(left, ArraySeqStEphSLit![1, 2, 3]);
        let right: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &e);
        assert_eq!(right, ArraySeqStEphSLit![1, 2, 3]);
    }

    #[test]
    fn test_append2_equivalence() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 3);
        let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 4, 2);
        let c1: ArraySeqStEphS<N> = ArraySeqStEphS::append(&a, &b);
        let c2: ArraySeqStEphS<N> = ArraySeqStEphS::append2(&a, &b);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_filter_empty_sequence() {
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|_| 0, 0);
        let f: ArraySeqStEphS<N> = ArraySeqStEphS::filter(&e, |_| B::True);
        assert_eq!(f.length(), 0);
    }

    #[test]
    fn test_select_boundary() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 2);
        let b: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 3, 2);
        assert_eq!(ArraySeqStEphS::select(&a, &b, 0), Some(1));
        assert_eq!(ArraySeqStEphS::select(&a, &b, 1), Some(2));
        assert_eq!(ArraySeqStEphS::select(&a, &b, 2), Some(3));
        assert_eq!(ArraySeqStEphS::select(&a, &b, 3), Some(4));
        assert_eq!(ArraySeqStEphS::select(&a, &b, 4), None);
    }

    #[test]
    fn test_subseq_basic() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| (i + 1) * 10, 5);
        let s = a.subseq_copy(1, 3);
        assert_eq!(s, ArraySeqStEphSLit![20, 30, 40]);
    }

    #[test]
    fn test_reduce_sum_basic_ch19() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 5);
        let sum_fn = |x: &N, y: &N| x + y;
        let r: N = ArraySeqStEphS::reduce(&a, &sum_fn, 0);
        assert_eq!(r, 15);
        let e: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|_| 0, 0);
        let re: N = ArraySeqStEphS::reduce(&e, &sum_fn, 42);
        assert_eq!(re, 42);
        let s: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|_| 7, 1);
        let rs: N = ArraySeqStEphS::reduce(&s, &sum_fn, 0);
        assert_eq!(rs, 7);
    }

    #[test]
    fn test_scan_sum_basic_ch19() {
        let a: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 5);
        let sum_fn = |x: &N, y: &N| x + y;
        let (prefixes, total) = ArraySeqStEphS::scan(&a, &sum_fn, 0);
        assert_eq!(prefixes.length(), 5);
        assert_eq!(*prefixes.nth(0), 0);
        assert_eq!(*prefixes.nth(4), 10);
        assert_eq!(total, 15);
    }

    #[test]
    fn test_flatten_ch19() {
        let s1: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 1, 2);
        let s2: ArraySeqStEphS<N> = ArraySeqStEphS::tabulate(|i| i + 3, 2);
        let nested: ArraySeqStEphS<ArraySeqStEphS<N>> = ArraySeqStEphSLit![s1, s2];
        let flat: ArraySeqStEphS<N> = ArraySeqStEphS::flatten(&nested);
        assert_eq!(flat, ArraySeqStEphSLit![1, 2, 3, 4]);
    }
}
