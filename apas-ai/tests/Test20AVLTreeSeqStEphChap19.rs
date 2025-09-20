//! Tests for AVLTreeSeq Eph Chap19-style operations using available Eph APIs.

pub mod TestAVLTreeSeqStEphChap19 {
    use apas_ai::AVLTreeSeqStEph; // macro import
    use apas_ai::AVLTreeSeqStEphSLit;
    use apas_ai::ArraySeqStEphSLit;
    use apas_ai::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use apas_ai::ArraySeqStEph;
    use apas_ai::ArraySeqStEph::ArraySeqStEph::*;
    use apas_ai::Types::Types::*; // macro import

    #[test]
    fn test_tabulate_and_map() {
        let t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2, 3, 4];
        let mapped_v: Vec<N> = t.iter().map(|x| *x * 2).collect();
        let m: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphS::from_vec(mapped_v);
        assert_eq!(m.to_arrayseq(), ArraySeqStEphSLit![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_select_and_append() {
        let a: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2];
        let b: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![3, 4, 5];
        // emulate select behavior
        let select = |a: &AVLTreeSeqStEphS<N>, b: &AVLTreeSeqStEphS<N>, i: N| -> Option<N> {
            if i < a.length() {
                Some(*a.nth(i))
            } else {
                let off = i - a.length();
                if off < b.length() {
                    Some(*b.nth(off))
                } else {
                    None
                }
            }
        };
        assert_eq!(select(&a, &b, 0), Some(0));
        assert_eq!(select(&a, &b, 4), Some(4));
        assert_eq!(select(&a, &b, 6), None);
        let mut vals: Vec<N> = a.iter().map(|x| *x).collect();
        for x in b.iter() {
            if !vals.contains(x) {
                vals.push(*x);
            }
        }
        let c = AVLTreeSeqStEphS::from_vec(vals);
        assert_eq!(c.to_arrayseq(), ArraySeqStEphSLit![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deflate_and_filter() {
        let t: AVLTreeSeqStEphS<N> = AVLTreeSeqStEphSLit![0, 1, 2, 3, 4, 5];
        let d = if 2 % 2 == 0 {
            AVLTreeSeqStEphSLit![2]
        } else {
            AVLTreeSeqStEphSLit![]
        };
        assert_eq!(d.to_arrayseq(), ArraySeqStEphSLit![2]);
        let kept: Vec<N> = t.iter().filter(|x| **x < 3).map(|x| *x).collect();
        let f = AVLTreeSeqStEphS::from_vec(kept);
        assert_eq!(f.to_arrayseq(), ArraySeqStEphSLit![0, 1, 2]);
    }
}
