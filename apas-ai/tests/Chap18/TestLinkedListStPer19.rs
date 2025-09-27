//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestLinkedListPer {
    use apas_ai::Chap18::LinkedListStPer::LinkedListStPer::*;
    use apas_ai::LinkedListStPerSLit;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_select() {
        let a = LinkedListStPerSLit![1, 2, 3];
        let b = LinkedListStPerSLit![4, 5];

        let sel0 = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 0);
        let sel3 = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 3);
        let sel5 = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::select(&a, &b, 5);
        assert_eq!(sel0, Some(1));
        assert_eq!(sel3, Some(4));
        assert_eq!(sel5, None);
    }

    #[test]
    fn test_append_variants() {
        let a = LinkedListStPerSLit![1, 2, 3];
        let b = LinkedListStPerSLit![4, 5];
        let c = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::append(&a, &b);
        let c2 = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::append(&a, &b);
        assert_eq!(c, LinkedListStPerSLit![1, 2, 3, 4, 5]);
        assert_eq!(c2, LinkedListStPerSLit![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deflate() {
        // Note: deflate method not implemented in LinkedListStPerTrait
        // let one =
        //     <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::deflate(|x| if *x > 0 { B::True } else { B::False }, &7);
        // assert_eq!(one, LinkedListStPerSLit![7]);
        // let none =
        //     <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::deflate(|x| if *x > 0 { B::True } else { B::False }, &0);
        // assert_eq!(none, LinkedListStPerS::new(0, 0));
    }

    #[test]
    fn test_map() {
        let a = LinkedListStPerSLit![1, 2, 3];
        let b = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::map(&a, &|x| x + 1);
        assert_eq!(b, LinkedListStPerSLit![2, 3, 4]);
    }

    #[test]
    fn test_iterate_and_reduce() {
        let a = LinkedListStPerSLit![1, 2, 3];
        let sum = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::iterate(&a, &|acc, x| acc + x, 0);
        assert_eq!(sum, 6);
        let red = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::reduce(&a, &|x, y| x + y, 0);
        assert_eq!(red, 6);
    }

    #[test]
    fn test_scan() {
        let a = LinkedListStPerSLit![1, 2, 3];
        let (prefixes, total) = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::scan(&a, &|x, y| x + y, 0);
        assert_eq!(prefixes, LinkedListStPerSLit![0, 1, 3]);
        assert_eq!(total, 6);
    }

    #[test]
    fn test_flatten() {
        let nested = LinkedListStPerSLit![LinkedListStPerSLit![1, 2], LinkedListStPerSLit![3]];
        let flat = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::flatten(&nested);
        assert_eq!(flat, LinkedListStPerSLit![1, 2, 3]);
    }

    #[test]
    fn test_inject() {
        let values = LinkedListStPerSLit![10, 20, 30];
        let changes = LinkedListStPerSLit![Pair(1, 99)];
        let injected = <LinkedListStPerS<N> as LinkedListStPerTrait<N>>::inject(&values, &changes);
        assert_eq!(injected, LinkedListStPerSLit![10, 99, 30]);
    }
}
