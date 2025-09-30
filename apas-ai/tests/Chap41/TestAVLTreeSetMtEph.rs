//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for AVLTreeSetMtEph with parallelism verification

#[cfg(test)]
mod tests_avl_tree_set_mt_eph {
    use apas_ai::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
    use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_empty() {
        let s = AVLTreeSetMtEph::<i32>::empty();
        assert_eq!(s.size(), 0);
    }

    #[test]
    fn test_singleton() {
        let s = AVLTreeSetMtEph::singleton(42);
        assert_eq!(s.size(), 1);
        assert!(s.find(&42));
    }

    #[test]
    fn test_insert_find() {
        let mut s = AVLTreeSetMtEph::<i32>::empty();
        s.insert(1);
        s.insert(2);
        s.insert(3);
        
        assert_eq!(s.size(), 3);
        assert!(s.find(&1));
        assert!(s.find(&2));
        assert!(s.find(&3));
        assert!(!s.find(&4));
    }

    #[test]
    fn test_delete() {
        let mut s = AVLTreeSetMtEph::<i32>::empty();
        s.insert(1);
        s.insert(2);
        s.insert(3);
        
        s.delete(&2);
        assert_eq!(s.size(), 2);
        assert!(!s.find(&2));
        assert!(s.find(&1));
        assert!(s.find(&3));
    }

    #[test]
    fn test_filter_small() {
        let mut s = AVLTreeSetMtEph::<i32>::empty();
        for i in 1..=10 {
            s.insert(i);
        }
        
        let evens = s.filter(|x| x % 2 == 0);
        assert_eq!(evens.size(), 5);
        assert!(evens.find(&2));
        assert!(evens.find(&4));
        assert!(!evens.find(&1));
    }

    #[test]
    fn test_filter_large_parallel() {
        // Test parallel filter with large set (> threshold = 128)
        let mut s = AVLTreeSetMtEph::<i32>::empty();
        for i in 1..=200 {
            s.insert(i);
        }
        
        let evens = s.filter(|x| x % 2 == 0);
        assert_eq!(evens.size(), 100);
        for i in 1..=100 {
            assert!(evens.find(&(i * 2)));
        }
    }

    #[test]
    fn test_union_small() {
        let mut s1 = AVLTreeSetMtEph::<i32>::empty();
        let mut s2 = AVLTreeSetMtEph::<i32>::empty();
        
        for i in 1..=5 {
            s1.insert(i);
        }
        for i in 4..=8 {
            s2.insert(i);
        }
        
        let u = s1.union(&s2);
        assert_eq!(u.size(), 8);
        for i in 1..=8 {
            assert!(u.find(&i));
        }
    }

    #[test]
    fn test_union_large_parallel() {
        // Test parallel union with large sets (> threshold = 128)
        let mut s1 = AVLTreeSetMtEph::<i32>::empty();
        let mut s2 = AVLTreeSetMtEph::<i32>::empty();
        
        for i in 1..=150 {
            s1.insert(i);
        }
        for i in 100..=250 {
            s2.insert(i);
        }
        
        let u = s1.union(&s2);
        assert_eq!(u.size(), 250);
        for i in 1..=250 {
            assert!(u.find(&i));
        }
    }

    #[test]
    fn test_intersection_small() {
        let mut s1 = AVLTreeSetMtEph::<i32>::empty();
        let mut s2 = AVLTreeSetMtEph::<i32>::empty();
        
        for i in 1..=10 {
            s1.insert(i);
        }
        for i in 5..=15 {
            s2.insert(i);
        }
        
        let inter = s1.intersection(&s2);
        assert_eq!(inter.size(), 6);
        for i in 5..=10 {
            assert!(inter.find(&i));
        }
    }

    #[test]
    fn test_intersection_large_parallel() {
        // Test parallel intersection with large sets (> threshold = 128)
        let mut s1 = AVLTreeSetMtEph::<i32>::empty();
        let mut s2 = AVLTreeSetMtEph::<i32>::empty();
        
        for i in 1..=200 {
            s1.insert(i);
        }
        for i in 100..=300 {
            s2.insert(i);
        }
        
        let inter = s1.intersection(&s2);
        assert_eq!(inter.size(), 101);
        for i in 100..=200 {
            assert!(inter.find(&i));
        }
    }

    #[test]
    fn test_difference() {
        let mut s1 = AVLTreeSetMtEph::<i32>::empty();
        let mut s2 = AVLTreeSetMtEph::<i32>::empty();
        
        for i in 1..=10 {
            s1.insert(i);
        }
        for i in 5..=15 {
            s2.insert(i);
        }
        
        let diff = s1.difference(&s2);
        assert_eq!(diff.size(), 4);
        for i in 1..=4 {
            assert!(diff.find(&i));
        }
        for i in 5..=10 {
            assert!(!diff.find(&i));
        }
    }

    #[test]
    fn test_clone() {
        let mut s1 = AVLTreeSetMtEph::<i32>::empty();
        s1.insert(1);
        s1.insert(2);
        
        let s2 = s1.clone();
        assert_eq!(s2.size(), 2);
        assert!(s2.find(&1));
        assert!(s2.find(&2));
    }

    #[test]
    fn test_from_seq() {
        let seq = AVLTreeSeqStEphS::from_vec(vec![3, 1, 4, 1, 5, 9, 2, 6]);
        let s = AVLTreeSetMtEph::from_seq(seq);
        
        assert_eq!(s.size(), 7); // Duplicates removed
        assert!(s.find(&1));
        assert!(s.find(&2));
        assert!(s.find(&3));
        assert!(s.find(&4));
        assert!(s.find(&5));
        assert!(s.find(&6));
        assert!(s.find(&9));
    }
}
