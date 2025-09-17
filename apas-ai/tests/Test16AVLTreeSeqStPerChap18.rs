//! Tests for AVLTreeSeq Chapter 18 algorithms.

pub mod TestAVLTreeSeqStPerChap18 {
    use apas_ai::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use apas_ai::AVLTreeSeqStPerChap18::AVLTreeSeqStPerChap18::*;
    use apas_ai::ArraySeqStPer; // macro import
    use apas_ai::ArraySeqStPer::ArraySeqStPer::*;
    use apas_ai::ArrayStPerS;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_tabulate_inorder() {
        let t: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i, 5);
        assert_eq!(t.to_arrayseq(), ArraySeqStPer![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_map_increment() {
        let base: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i, 4);
        let mapped: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::map(&base, |x| x + 1);
        assert_eq!(mapped.to_arrayseq(), ArraySeqStPer![1, 2, 3, 4]);
    }

    #[test]
    fn test_append_union() {
        let a: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i, 4); // 0..3
        let b: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i + 2, 4); // 2..5
        let u: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::append(&a, &b);
        assert_eq!(u.to_arrayseq(), ArraySeqStPer![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_filter_even() {
        let base: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::tabulate(|i| i, 6);
        let evens: AVLTreeSeqStPerS<N> = <AVLTreeSeqStPerS<N> as AVLTreeSeqStPerChap18Trait<N>>::filter(&base, |x| {
            if x % 2 == 0 { B::True } else { B::False }
        });
        assert_eq!(evens.to_arrayseq(), ArraySeqStPer![0, 2, 4]);
    }
}
