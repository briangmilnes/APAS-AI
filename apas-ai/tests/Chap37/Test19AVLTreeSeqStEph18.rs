//! Tests for AVLTreeSeq Eph (Chap18-style operations built using base APIs).

pub mod TestAVLTreeSeqStEph {
    use apas_ai::Chap37::AVLTreeSeqStEph;
    use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphS;
    use apas_ai::AVLTreeSeqStEphSLit;
    use apas_ai::ArraySeqStEph; // macro import
    use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::ArraySeqStEphSLit;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_tabulate_inorder() {
        let t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2, 3, 4];
        assert_eq!(t.to_arrayseq(), ArraySeqStEphSLit![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_map_increment() {
        let base: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2, 3];
        let mapped_v: Vec<N> = base.iter().map(|x| *x + 1).collect();
        let mapped: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::from_vec(mapped_v);
        assert_eq!(mapped.to_arrayseq(), ArraySeqStEphSLit![1, 2, 3, 4]);
    }

    #[test]
    fn test_append_union() {
        let a: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2, 3];
        let b: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![2, 3, 4, 5];
        let mut vals: Vec<N> = a.iter().map(|x| *x).collect();
        for x in b.iter() {
            if !vals.contains(x) {
                vals.push(*x);
            }
        }
        let u = AVLTreeSeqStEphS::from_vec(vals);
        assert_eq!(u.to_arrayseq(), ArraySeqStEphSLit![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_filter_even() {
        let base: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2, 3, 4, 5];
        let kept: Vec<N> = base.iter().filter(|x| **x % 2 == 0).map(|x| *x).collect();
        let evens: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::from_vec(kept);
        assert_eq!(evens.to_arrayseq(), ArraySeqStEphSLit![0, 2, 4]);
    }
}
