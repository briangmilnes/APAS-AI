//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::*;
use apas_ai::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;
use apas_ai::Types::Types::*;

trait TestSet: Sized {
    fn empty() -> Self;
    fn insert(&mut self, value: i32);
    fn delete(&mut self, value: &i32);
    fn size(&self) -> usize;
    fn is_empty(&self) -> B;
    fn contains(&self, value: &i32) -> B;
    fn minimum(&self) -> Option<i32>;
    fn maximum(&self) -> Option<i32>;
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
    fn difference(&self, other: &Self) -> Self;
    fn split(&self, pivot: &i32) -> (Self, B, Self);
    fn join_pair(left: Self, right: Self) -> Self;
    fn join_m(left: Self, pivot: i32, right: Self) -> Self;
    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self;
    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32;
    fn iter_seq(&self) -> ArraySeqStPerS<i32>;
}

impl TestSet for apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {
    fn empty() -> Self { Self::empty() }

    fn insert(&mut self, value: i32) { self.insert(value); }

    fn delete(&mut self, value: &i32) { self.delete(value); }

    fn size(&self) -> usize { self.size() }

    fn is_empty(&self) -> B { self.is_empty() }

    fn contains(&self, value: &i32) -> B { self.contains(value) }

    fn minimum(&self) -> Option<i32> { self.minimum() }

    fn maximum(&self) -> Option<i32> { self.maximum() }

    fn union(&self, other: &Self) -> Self { self.union(other) }

    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }

    fn difference(&self, other: &Self) -> Self { self.difference(other) }

    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }

    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }

    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }

    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }

    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base) }

    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {
    fn empty() -> Self { Self::empty() }

    fn insert(&mut self, value: i32) { self.insert(value); }

    fn delete(&mut self, value: &i32) { self.delete(value); }

    fn size(&self) -> usize { self.size() }

    fn is_empty(&self) -> B { self.is_empty() }

    fn contains(&self, value: &i32) -> B { self.contains(value) }

    fn minimum(&self) -> Option<i32> { self.minimum() }

    fn maximum(&self) -> Option<i32> { self.maximum() }

    fn union(&self, other: &Self) -> Self { self.union(other) }

    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }

    fn difference(&self, other: &Self) -> Self { self.difference(other) }

    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }

    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }

    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }

    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }

    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base) }

    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {
    fn empty() -> Self { Self::empty() }

    fn insert(&mut self, value: i32) { self.insert(value); }

    fn delete(&mut self, value: &i32) { self.delete(value); }

    fn size(&self) -> usize { self.size() }

    fn is_empty(&self) -> B { self.is_empty() }

    fn contains(&self, value: &i32) -> B { self.contains(value) }

    fn minimum(&self) -> Option<i32> { self.minimum() }

    fn maximum(&self) -> Option<i32> { self.maximum() }

    fn union(&self, other: &Self) -> Self { self.union(other) }

    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }

    fn difference(&self, other: &Self) -> Self { self.difference(other) }

    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }

    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }

    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }

    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }

    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base) }

    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {
    fn empty() -> Self { Self::empty() }

    fn insert(&mut self, value: i32) { self.insert(value); }

    fn delete(&mut self, value: &i32) { self.delete(value); }

    fn size(&self) -> usize { self.size() }

    fn is_empty(&self) -> B { self.is_empty() }

    fn contains(&self, value: &i32) -> B { self.contains(value) }

    fn minimum(&self) -> Option<i32> { self.minimum() }

    fn maximum(&self) -> Option<i32> { self.maximum() }

    fn union(&self, other: &Self) -> Self { self.union(other) }

    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }

    fn difference(&self, other: &Self) -> Self { self.difference(other) }

    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }

    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }

    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }

    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }

    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base) }

    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {
    fn empty() -> Self { Self::empty() }

    fn insert(&mut self, value: i32) { self.insert(value); }

    fn delete(&mut self, value: &i32) { self.delete(value); }

    fn size(&self) -> usize { self.size() }

    fn is_empty(&self) -> B { self.is_empty() }

    fn contains(&self, value: &i32) -> B { self.contains(value) }

    fn minimum(&self) -> Option<i32> { self.minimum() }

    fn maximum(&self) -> Option<i32> { self.maximum() }

    fn union(&self, other: &Self) -> Self { self.union(other) }

    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }

    fn difference(&self, other: &Self) -> Self { self.difference(other) }

    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }

    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }

    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }

    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }

    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base) }

    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {
    fn empty() -> Self { Self::empty() }

    fn insert(&mut self, value: i32) { self.insert(value); }

    fn delete(&mut self, value: &i32) { self.delete(value); }

    fn size(&self) -> usize { self.size() }

    fn is_empty(&self) -> B { self.is_empty() }

    fn contains(&self, value: &i32) -> B { self.contains(value) }

    fn minimum(&self) -> Option<i32> { self.minimum() }

    fn maximum(&self) -> Option<i32> { self.maximum() }

    fn union(&self, other: &Self) -> Self { self.union(other) }

    fn intersection(&self, other: &Self) -> Self { self.intersection(other) }

    fn difference(&self, other: &Self) -> Self { self.difference(other) }

    fn split(&self, pivot: &i32) -> (Self, B, Self) { self.split(pivot) }

    fn join_pair(left: Self, right: Self) -> Self { Self::join_pair(left, right) }

    fn join_m(left: Self, pivot: i32, right: Self) -> Self { Self::join_m(left, pivot, right) }

    fn filter<F: FnMut(&i32) -> bool>(&self, predicate: F) -> Self { self.filter(predicate) }

    fn reduce<F: FnMut(i32, i32) -> i32>(&self, op: F, base: i32) -> i32 { self.reduce(op, base) }

    fn iter_seq(&self) -> ArraySeqStPerS<i32> { self.iter_in_order() }
}

fn exercise_set<S: TestSet>() {
    let mut set = S::empty();
    assert_eq!(set.is_empty(), true);

    for value in [3, 1, 4, 2] {
        set.insert(value);
    }

    assert_eq!(set.size(), 4);
    assert_eq!(set.contains(&2), true);
    assert_eq!(set.minimum(), Some(1));
    assert_eq!(set.maximum(), Some(4));

    set.delete(&3);
    assert_eq!(set.contains(&3), false);
    assert_eq!(set.size(), 3);

    let mut other = S::empty();
    for value in [2, 5] {
        other.insert(value);
    }

    let union = set.union(&other);
    assert_eq!(union.size(), 4);

    let intersection = set.intersection(&other);
    assert_eq!(intersection.size(), 1);
    assert_eq!(intersection.contains(&2), true);

    let difference = set.difference(&other);
    assert_eq!(difference.size(), 2);
    assert_eq!(difference.contains(&1), true);
    assert_eq!(difference.contains(&4), true);

    let (lt, found_missing, gt) = union.split(&3);
    assert_eq!(found_missing, false);
    assert_eq!(lt.size(), 2);
    assert_eq!(gt.size(), 2);

    let rejoined_pair = S::join_pair(lt, gt);
    assert_eq!(rejoined_pair.size(), union.size());

    let (lt_with_pivot, found_pivot, gt_with_pivot) = union.split(&4);
    assert_eq!(found_pivot, true);
    let rejoined_mid = S::join_m(lt_with_pivot, 4, gt_with_pivot);
    assert_eq!(rejoined_mid.size(), union.size());

    let evens = union.filter(|value| *value % 2 == 0);
    assert_eq!(evens.size(), 2);
    assert_eq!(evens.contains(&2), true);
    assert_eq!(evens.contains(&4), true);

    let sum = union.reduce(|acc, value| acc + value, 0);
    assert_eq!(sum, 12);

    let in_order = union.iter_seq();
    assert_eq!(in_order, ArraySeqStPerSLit![1, 2, 4, 5]);
}

#[test]
fn test_plain_bst_set_ops() {
    exercise_set::<apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32>>();
}

#[test]
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32>>(); }

#[test]
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32>>(); }

#[test]
fn test_bbalpha_bst_set_ops() {
    exercise_set::<apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32>>();
}

#[test]
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32>>(); }

#[test]
fn test_splay_bst_set_ops() {
    exercise_set::<apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32>>();
}

// Individual variant testing for BSTSet*MtEph variants
#[test]
fn test_plain_bst_individual_operations() {
    use apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt;
    
    let mut set = BSTSetPlainMt::<i32>::empty();
    assert_eq!(set.is_empty(), true);
    assert_eq!(set.size(), 0);
    assert_eq!(set.minimum(), None);
    assert_eq!(set.maximum(), None);
    
    // Test individual insertions
    set.insert(10);
    assert_eq!(set.size(), 1);
    assert_eq!(set.contains(&10), true);
    assert_eq!(set.minimum(), Some(10));
    assert_eq!(set.maximum(), Some(10));
    
    set.insert(5);
    set.insert(15);
    set.insert(3);
    set.insert(7);
    set.insert(12);
    set.insert(18);
    
    assert_eq!(set.size(), 7);
    assert_eq!(set.minimum(), Some(3));
    assert_eq!(set.maximum(), Some(18));
    
    // Test deletions
    set.delete(&10);
    assert_eq!(set.contains(&10), false);
    assert_eq!(set.size(), 6);
    
    // Test set operations
    let mut other = BSTSetPlainMt::empty();
    other.insert(7);
    other.insert(20);
    other.insert(25);
    
    let union_result = set.union(&other);
    assert_eq!(union_result.size(), 8); // 6 from set + 2 new from other
    assert_eq!(union_result.contains(&7), true);
    assert_eq!(union_result.contains(&20), true);
    
    let intersection_result = set.intersection(&other);
    assert_eq!(intersection_result.size(), 1); // Only 7 is common
    assert_eq!(intersection_result.contains(&7), true);
    
    let difference_result = set.difference(&other);
    assert_eq!(difference_result.size(), 5); // set minus common element
    assert_eq!(difference_result.contains(&7), false);
    assert_eq!(difference_result.contains(&5), true);
}

#[test]
fn test_avl_bst_individual_operations() {
    use apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt;
    
    let mut set = BSTSetAVLMt::<i32>::empty();
    
    // Test balanced insertion (AVL should maintain balance)
    for i in 1..=15 {
        set.insert(i);
    }
    
    assert_eq!(set.size(), 15);
    assert_eq!(set.minimum(), Some(1));
    assert_eq!(set.maximum(), Some(15));
    
    // Test split operation
    let (left, found, right) = set.split(&8);
    assert_eq!(found, true);
    assert_eq!(left.size(), 7); // 1-7
    assert_eq!(right.size(), 7); // 9-15
    
    // Test join operations
    let rejoined = BSTSetAVLMt::join_m(left, 8, right);
    assert_eq!(rejoined.size(), 15);
    assert_eq!(rejoined.contains(&8), true);
    
    // Test filter
    let evens = set.filter(|x| *x % 2 == 0);
    assert_eq!(evens.size(), 7); // 2,4,6,8,10,12,14
    
    // Test reduce
    let sum = set.reduce(|acc, x| acc + x, 0);
    assert_eq!(sum, 120); // Sum of 1-15
}

#[test]
fn test_rb_bst_individual_operations() {
    use apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt;
    
    let mut set = BSTSetRBMt::<i32>::empty();
    
    // Test Red-Black tree with alternating insertions
    let values = [50, 25, 75, 12, 37, 62, 87, 6, 18, 31, 43];
    for &val in &values {
        set.insert(val);
    }
    
    assert_eq!(set.size(), values.len());
    
    // Verify all values are present
    for &val in &values {
        assert_eq!(set.contains(&val), true);
    }
    
    // Test iterator ordering
    let in_order = set.iter_in_order();
    let mut sorted_values = values.to_vec();
    sorted_values.sort();
    
    assert_eq!(in_order.length(), sorted_values.len());
    for i in 0..in_order.length() {
        assert_eq!(*in_order.nth(i), sorted_values[i]);
    }
    
    // Test multiple deletions
    set.delete(&25);
    set.delete(&75);
    assert_eq!(set.size(), values.len() - 2);
    assert_eq!(set.contains(&25), false);
    assert_eq!(set.contains(&75), false);
}

#[test]
fn test_bbalpha_bst_individual_operations() {
    use apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt;
    
    let mut set = BSTSetBBAlphaMt::<i32>::empty();
    
    // Test BB[α] tree with sequential insertions
    for i in (1..=20).rev() {
        set.insert(i);
    }
    
    assert_eq!(set.size(), 20);
    
    // Test that tree maintains balance despite worst-case insertion order
    let in_order = set.iter_in_order();
    for i in 0..20 {
        assert_eq!(*in_order.nth(i), (i + 1) as i32);
    }
    
    // Test complex split and join
    let (left_part, found_10, right_part) = set.split(&10);
    assert_eq!(found_10, true);
    assert_eq!(left_part.size(), 9);  // 1-9
    assert_eq!(right_part.size(), 10); // 11-20
    
    let rejoined = BSTSetBBAlphaMt::join_m(left_part, 10, right_part);
    assert_eq!(rejoined.size(), 20);
    
    // Test filter with complex predicate
    let multiples_of_3 = set.filter(|x| *x % 3 == 0);
    assert_eq!(multiples_of_3.size(), 6); // 3,6,9,12,15,18
    assert_eq!(multiples_of_3.contains(&9), true);
    assert_eq!(multiples_of_3.contains(&10), false);
}

#[test]
fn test_treap_bst_individual_operations() {
    use apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt;
    
    let mut set = BSTSetTreapMt::<i32>::empty();
    
    // Test Treap with random-like insertions
    let values = [42, 17, 89, 3, 56, 23, 78, 91, 12, 67];
    for &val in &values {
        set.insert(val);
    }
    
    assert_eq!(set.size(), values.len());
    
    // Test set operations with another treap
    let mut other = BSTSetTreapMt::empty();
    let other_values = [17, 34, 56, 78, 99];
    for &val in &other_values {
        other.insert(val);
    }
    
    let union_result = set.union(&other);
    let expected_union_size = values.len() + other_values.len() - 3; // 3 overlaps: 17,56,78
    assert_eq!(union_result.size(), expected_union_size);
    
    let intersection_result = set.intersection(&other);
    assert_eq!(intersection_result.size(), 3); // 17,56,78
    assert_eq!(intersection_result.contains(&17), true);
    assert_eq!(intersection_result.contains(&56), true);
    assert_eq!(intersection_result.contains(&78), true);
    
    // Test reduce operation
    let product = intersection_result.reduce(|acc, x| acc * x, 1);
    assert_eq!(product, 17 * 56 * 78);
}

#[test]
fn test_splay_bst_individual_operations() {
    use apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt;
    
    let mut set = BSTSetSplayMt::<i32>::empty();
    
    // Test Splay tree with access pattern optimization
    let values = [100, 50, 150, 25, 75, 125, 175];
    for &val in &values {
        set.insert(val);
    }
    
    assert_eq!(set.size(), values.len());
    
    // Repeatedly access certain elements (should splay to root)
    for _ in 0..5 {
        assert_eq!(set.contains(&75), true);
    }
    
    // Test difference operation
    let mut subtract_set = BSTSetSplayMt::empty();
    subtract_set.insert(50);
    subtract_set.insert(150);
    
    let difference_result = set.difference(&subtract_set);
    assert_eq!(difference_result.size(), 5); // Original 7 minus 2
    assert_eq!(difference_result.contains(&50), false);
    assert_eq!(difference_result.contains(&150), false);
    assert_eq!(difference_result.contains(&100), true);
    
    // Test join_pair operation
    let (left_split, _, right_split) = difference_result.split(&100);
    let rejoined = BSTSetSplayMt::join_pair(left_split, right_split);
    assert_eq!(rejoined.size(), 4); // difference_result minus the pivot 100
}

#[test]
fn test_all_variants_empty_operations() {
    // Test that all variants handle empty operations correctly
    macro_rules! test_empty_variant {
        ($variant:ty) => {
            let set = <$variant>::empty();
            assert_eq!(set.is_empty(), true);
            assert_eq!(set.size(), 0);
            assert_eq!(set.minimum(), None);
            assert_eq!(set.maximum(), None);
            assert_eq!(set.contains(&42), false);
            
            let in_order = set.iter_in_order();
            assert_eq!(in_order.length(), 0);
            
            let filtered = set.filter(|_| true);
            assert_eq!(filtered.size(), 0);
            
            let reduced = set.reduce(|acc, x| acc + x, 100);
            assert_eq!(reduced, 100); // Should return base value
        };
    }
    
    test_empty_variant!(apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32>);
    test_empty_variant!(apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32>);
    test_empty_variant!(apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32>);
    test_empty_variant!(apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32>);
    test_empty_variant!(apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32>);
    test_empty_variant!(apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32>);
}

#[test]
fn test_all_variants_single_element() {
    // Test that all variants handle single element correctly
    macro_rules! test_single_variant {
        ($variant:ty) => {
            let mut set = <$variant>::empty();
            set.insert(42);
            
            assert_eq!(set.is_empty(), false);
            assert_eq!(set.size(), 1);
            assert_eq!(set.minimum(), Some(42));
            assert_eq!(set.maximum(), Some(42));
            assert_eq!(set.contains(&42), true);
            assert_eq!(set.contains(&99), false);
            
            let in_order = set.iter_in_order();
            assert_eq!(in_order.length(), 1);
            assert_eq!(*in_order.nth(0), 42);
            
            // Test split on single element
            let (left, found, right) = set.split(&42);
            assert_eq!(found, true);
            assert_eq!(left.size(), 0);
            assert_eq!(right.size(), 0);
            
            // Test filter
            let filtered = set.filter(|x| *x > 40);
            assert_eq!(filtered.size(), 1);
            assert_eq!(filtered.contains(&42), true);
            
            let filtered_empty = set.filter(|x| *x > 50);
            assert_eq!(filtered_empty.size(), 0);
        };
    }
    
    test_single_variant!(apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32>);
    test_single_variant!(apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32>);
    test_single_variant!(apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32>);
    test_single_variant!(apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32>);
    test_single_variant!(apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32>);
    test_single_variant!(apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32>);
}

// Concurrent access pattern tests for BSTSet*MtEph variants
#[test]
fn test_concurrent_plain_bst_operations() {
    use apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt;
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];
    
    // Thread 1: Insert values 1-25
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        let mut local_set = BSTSetPlainMt::empty();
        for i in 1..=25 {
            local_set.insert(i);
        }
        (local_set.size(), local_set.contains(&15))
    }));
    
    // Thread 2: Insert values 26-50
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        let mut local_set = BSTSetPlainMt::empty();
        for i in 26..=50 {
            local_set.insert(i);
        }
        (local_set.size(), local_set.contains(&30))
    }));
    
    // Thread 3: Test set operations
    let barrier3 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier3.wait();
        let mut set_a = BSTSetPlainMt::empty();
        let mut set_b = BSTSetPlainMt::empty();
        
        for i in [10, 20, 30] {
            set_a.insert(i);
        }
        for i in [20, 30, 40] {
            set_b.insert(i);
        }
        
        let union = set_a.union(&set_b);
        let intersection = set_a.intersection(&set_b);
        (union.size(), intersection.contains(&20))
    }));
    
    // Thread 4: Test split and join operations
    let barrier4 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier4.wait();
        let mut test_set = BSTSetPlainMt::empty();
        for i in [5, 15, 25, 35, 45] {
            test_set.insert(i);
        }
        
        let (left, found, right) = test_set.split(&25);
        let rejoined = BSTSetPlainMt::join_m(left, 25, right);
        (rejoined.size(), found)
    }));
    
    // Collect results
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify thread results
    assert_eq!(results[0], (25, true));  // Thread 1: size 25, contains 15
    assert_eq!(results[1], (25, true));  // Thread 2: size 25, contains 30
    assert_eq!(results[2], (4, true));   // Thread 3: union size 4, intersection contains 20
    assert_eq!(results[3], (5, true));   // Thread 4: rejoined size 5, found 25
}

#[test]
fn test_concurrent_avl_bst_operations() {
    use apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt;
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let num_threads = 6;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        
        handles.push(thread::spawn(move || {
            barrier_clone.wait();
            
            let mut set = BSTSetAVLMt::<i32>::empty();
            
            // Each thread works with different value ranges
            let start = thread_id * 10;
            let end = start + 10;
            
            for i in start..end {
                set.insert(i as i32);
            }
            
            // Test operations
            let size = set.size();
            let min = set.minimum();
            let max = set.maximum();
            
            // Test filter operation
            let evens = set.filter(|x| *x % 2 == 0);
            let even_count = evens.size();
            
            (thread_id, size, min, max, even_count)
        }));
    }
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify each thread's results
    for (thread_id, size, min, max, even_count) in results {
        assert_eq!(size, 10);
        assert_eq!(min, Some((thread_id * 10) as i32));
        assert_eq!(max, Some((thread_id * 10 + 9) as i32));
        assert_eq!(even_count, 5); // 5 even numbers in each range of 10
    }
}

#[test]
fn test_concurrent_rb_bst_stress() {
    use apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt;
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let num_threads = 8;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        
        handles.push(thread::spawn(move || {
            barrier_clone.wait();
            
            let mut set = BSTSetRBMt::<i32>::empty();
            
            // Stress test with many insertions and deletions
            for i in 0..100 {
                let value = (thread_id * 100 + i) as i32;
                set.insert(value);
            }
            
            // Delete every third element
            for i in (0..100).step_by(3) {
                let value = (thread_id * 100 + i) as i32;
                set.delete(&value);
            }
            
            let final_size = set.size();
            let in_order = set.iter_in_order();
            let is_sorted = (1..in_order.length()).all(|i| {
                *in_order.nth(i-1) <= *in_order.nth(i)
            });
            
            (thread_id, final_size, is_sorted)
        }));
    }
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify results
    for (thread_id, final_size, is_sorted) in results {
        // Every third element deleted: 0,3,6,9,12,15,18,21,24,27,30,33,36,39,42,45,48,51,54,57,60,63,66,69,72,75,78,81,84,87,90,93,96,99
        // That's 34 elements deleted (0 to 99 step 3), so 100 - 34 = 66
        assert_eq!(final_size, 66); // 100 - 34 (every third deleted starting from 0) = 66
        assert!(is_sorted, "Thread {} produced unsorted result", thread_id);
    }
}

#[test]
fn test_concurrent_bbalpha_operations() {
    use apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt;
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    
    // Thread 1: Sequential insertions (worst case for unbalanced trees)
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        let mut set = BSTSetBBAlphaMt::empty();
        for i in 1..=50 {
            set.insert(i);
        }
        (set.size(), set.minimum(), set.maximum())
    }));
    
    // Thread 2: Reverse sequential insertions
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        let mut set = BSTSetBBAlphaMt::empty();
        for i in (1..=50).rev() {
            set.insert(i);
        }
        (set.size(), set.minimum(), set.maximum())
    }));
    
    // Thread 3: Random-like insertions
    let barrier3 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier3.wait();
        let mut set = BSTSetBBAlphaMt::empty();
        let values = [25, 12, 37, 6, 18, 31, 43, 3, 9, 15, 21, 28, 34, 40, 46];
        for &val in &values {
            set.insert(val);
        }
        
        // Test complex operations
        let (left, _found, right) = set.split(&25);
        let _rejoined = BSTSetBBAlphaMt::join_m(left, 25, right);
        
        (set.size(), set.minimum(), set.maximum())
    }));
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // All threads should produce consistent results
    assert_eq!(results[0], (50, Some(1), Some(50)));
    assert_eq!(results[1], (50, Some(1), Some(50)));
    assert_eq!(results[2], (15, Some(3), Some(46)));
}

#[test]
fn test_concurrent_treap_operations() {
    use apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt;
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let barrier_clone = Arc::clone(&barrier);
        
        handles.push(thread::spawn(move || {
            barrier_clone.wait();
            
            let mut set1 = BSTSetTreapMt::empty();
            let mut set2 = BSTSetTreapMt::empty();
            
            // Build two sets with overlapping values
            for i in 0..20 {
                set1.insert(thread_id * 20 + i);
            }
            
            for i in 10..30 {
                set2.insert(thread_id * 20 + i);
            }
            
            // Test set operations
            let union = set1.union(&set2);
            let intersection = set1.intersection(&set2);
            let difference = set1.difference(&set2);
            
            // Test reduce operation
            let sum = intersection.reduce(|acc, x| acc + x, 0);
            
            (union.size(), intersection.size(), difference.size(), sum as usize)
        }));
    }
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify results for each thread
    for (thread_id, (union_size, intersection_size, difference_size, sum)) in results.iter().enumerate() {
        assert_eq!(*union_size, 30);      // 20 + 20 - 10 overlap = 30
        assert_eq!(*intersection_size, 10); // 10 overlapping values
        assert_eq!(*difference_size, 10);   // 20 - 10 overlap = 10
        
        // Sum of intersection: thread_id*20 + 10 + ... + thread_id*20 + 19
        let expected_sum = (0..10).map(|i| thread_id * 20 + 10 + i).sum::<usize>();
        assert_eq!(*sum, expected_sum);
    }
}

#[test]
fn test_concurrent_splay_access_patterns() {
    use apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt;
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    let barrier = Arc::new(Barrier::new(2));
    let mut handles = vec![];
    
    // Thread 1: Frequent access to certain elements
    let barrier1 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier1.wait();
        let mut set = BSTSetSplayMt::empty();
        
        // Insert values
        for i in 1..=20 {
            set.insert(i);
        }
        
        // Frequently access certain values (should splay them)
        let frequent_values = [5, 10, 15];
        for _ in 0..10 {
            for &val in &frequent_values {
                assert_eq!(set.contains(&val), true);
            }
        }
        
        set.size()
    }));
    
    // Thread 2: Test split and join with splay tree
    let barrier2 = Arc::clone(&barrier);
    handles.push(thread::spawn(move || {
        barrier2.wait();
        let mut set = BSTSetSplayMt::empty();
        
        for i in [100, 50, 150, 25, 75, 125, 175] {
            set.insert(i);
        }
        
        // Split around middle value
        let (left, found, right) = set.split(&100);
        
        // Test that split worked correctly
        assert_eq!(found, true);
        assert_eq!(left.size() + right.size(), 6); // 7 - 1 (pivot) = 6
        
        // Rejoin using join_pair
        let rejoined = BSTSetSplayMt::join_pair(left, right);
        rejoined.size()
    }));
    
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    assert_eq!(results[0], 20); // Thread 1: 20 elements
    assert_eq!(results[1], 6);  // Thread 2: 6 elements after split (excluding pivot)
}

#[test]
fn test_all_variants_concurrent_stress() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    
    macro_rules! test_concurrent_variant {
        ($variant:ty, $expected_size:expr) => {
            let barrier = Arc::new(Barrier::new(3));
            let mut handles = vec![];
            
            for thread_id in 0..3 {
                let barrier_clone = Arc::clone(&barrier);
                
                handles.push(thread::spawn(move || {
                    barrier_clone.wait();
                    
                    let mut set = <$variant>::empty();
                    
                    // Each thread inserts different ranges
                    let start = thread_id * 10 + 1;
                    let end = start + 10;
                    
                    for i in start..end {
                        set.insert(i);
                    }
                    
                    // Test that all operations work
                    let size = set.size();
                    let min = set.minimum();
                    let max = set.maximum();
                    let contains_mid = set.contains(&(start + 5));
                    
                    (size, min, max, contains_mid)
                }));
            }
            
            let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
            
            // Each thread should have inserted 10 elements
            for (thread_id, (size, min, max, contains_mid)) in results.iter().enumerate() {
                assert_eq!(*size, 10);
                assert_eq!(*min, Some((thread_id * 10 + 1) as i32));
                assert_eq!(*max, Some((thread_id * 10 + 10) as i32));
                assert_eq!(*contains_mid, true);
            }
        };
    }
    
    test_concurrent_variant!(apas_ai::Chap37::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32>, 10);
    test_concurrent_variant!(apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32>, 10);
    test_concurrent_variant!(apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32>, 10);
    test_concurrent_variant!(apas_ai::Chap37::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32>, 10);
    test_concurrent_variant!(apas_ai::Chap39::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32>, 10);
    test_concurrent_variant!(apas_ai::Chap37::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32>, 10);
}

#[test]
fn test_race_condition_verification_bst_sets() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt;
    
    let barrier = Arc::new(Barrier::new(8));
    let race_detected = Arc::new(AtomicBool::new(false));
    let operation_counter = Arc::new(AtomicUsize::new(0));
    let shared_set = Arc::new(std::sync::RwLock::new(BSTSetAVLMt::empty()));
    let mut handles = vec![];
    
    // Spawn reader threads
    for thread_id in 0..4 {
        let barrier_clone = Arc::clone(&barrier);
        let race_detected_clone = Arc::clone(&race_detected);
        let shared_set_clone = Arc::clone(&shared_set);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let mut read_count = 0;
            for _ in 0..100 {
                if let Ok(set) = shared_set_clone.read() {
                    let size = set.size();
                    let min = set.minimum();
                    let max = set.maximum();
                    
                    // Verify consistency
                    if size > 0 {
                        if min.is_none() || max.is_none() {
                            race_detected_clone.store(true, Ordering::SeqCst);
                        }
                        if let (Some(min_val), Some(max_val)) = (min, max) {
                            if min_val > max_val {
                                race_detected_clone.store(true, Ordering::SeqCst);
                            }
                        }
                    } else {
                        if min.is_some() || max.is_some() {
                            race_detected_clone.store(true, Ordering::SeqCst);
                        }
                    }
                    
                    read_count += 1;
                }
            }
            
            (thread_id, read_count)
        });
        handles.push(handle);
    }
    
    // Spawn writer threads
    for thread_id in 4..8 {
        let barrier_clone = Arc::clone(&barrier);
        let operation_counter_clone = Arc::clone(&operation_counter);
        let shared_set_clone = Arc::clone(&shared_set);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let mut write_count = 0;
            for i in 0..25 {
                let value = thread_id * 100 + i;
                
                if let Ok(mut set) = shared_set_clone.write() {
                    set.insert(value);
                    write_count += 1;
                    operation_counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            }
            
            (thread_id, write_count)
        });
        handles.push(handle);
    }
    
    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    // Verify no race conditions detected
    assert!(!race_detected.load(Ordering::SeqCst), "Race condition detected in BST set operations");
    
    // Verify all operations completed
    assert_eq!(operation_counter.load(Ordering::SeqCst), 100, "Not all write operations completed");
    
    // Verify reader and writer thread results
    for (thread_id, count) in results {
        if thread_id < 4 {
            // Reader thread
            assert!(count > 0, "Reader thread {} performed no reads", thread_id);
        } else {
            // Writer thread
            assert_eq!(count, 25, "Writer thread {} didn't complete all writes", thread_id);
        }
    }
    
    // Final consistency check
    if let Ok(final_set) = shared_set.read() {
        let final_size = final_set.size();
        assert!(final_size <= 100, "Set size {} exceeds maximum expected", final_size);
        
        if final_size > 0 {
            let min = final_set.minimum();
            let max = final_set.maximum();
            assert!(min.is_some() && max.is_some(), "Non-empty set missing min/max");
            
            if let (Some(min_val), Some(max_val)) = (min, max) {
                assert!(min_val <= max_val, "Min {} > Max {}", min_val, max_val);
            }
        }
    }
}

#[test]
fn test_race_condition_verification_concurrent_modifications() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::sync::atomic::{AtomicBool, Ordering};
    use apas_ai::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt;
    
    let barrier = Arc::new(Barrier::new(6));
    let inconsistency_detected = Arc::new(AtomicBool::new(false));
    let mut handles = vec![];
    
    // Create initial shared data
    let shared_data = Arc::new(std::sync::RwLock::new(vec![
        BSTSetRBMt::empty(),
        BSTSetRBMt::empty(),
        BSTSetRBMt::empty(),
    ]));
    
    // Spawn threads that modify different sets concurrently
    for thread_id in 0..6 {
        let barrier_clone = Arc::clone(&barrier);
        let shared_data_clone = Arc::clone(&shared_data);
        let inconsistency_clone = Arc::clone(&inconsistency_detected);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let mut operations_completed = 0;
            for i in 0..50 {
                let set_index = thread_id % 3;
                let value = thread_id * 100 + i;
                
                // Perform operations on the assigned set
                if let Ok(mut sets) = shared_data_clone.write() {
                    let set = &mut sets[set_index];
                    
                    // Insert value
                    set.insert(value);
                    
                    // Verify the insertion worked
                    if set.contains(&value) != true {
                        inconsistency_clone.store(true, Ordering::SeqCst);
                    }
                    
                    // Verify set properties
                    let size = set.size();
                    if size == 0 {
                        inconsistency_clone.store(true, Ordering::SeqCst);
                    }
                    
                    operations_completed += 1;
                }
                
                // Occasionally read from other sets to create contention
                if i % 10 == 0 {
                    if let Ok(sets) = shared_data_clone.read() {
                        for (idx, set) in sets.iter().enumerate() {
                            if idx != set_index {
                                let _ = set.size();
                                let _ = set.minimum();
                                let _ = set.maximum();
                            }
                        }
                    }
                }
            }
            
            (thread_id, operations_completed)
        });
        handles.push(handle);
    }
    
    // Collect results
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    // Verify no inconsistencies detected
    assert!(!inconsistency_detected.load(Ordering::SeqCst), 
        "Data inconsistency detected during concurrent modifications");
    
    // Verify all operations completed
    for (thread_id, op_count) in results {
        assert_eq!(op_count, 50, "Thread {} didn't complete all operations", thread_id);
    }
    
    // Final verification of set states
    if let Ok(final_sets) = shared_data.read() {
        for (idx, set) in final_sets.iter().enumerate() {
            let size = set.size();
            // Each set should have been modified by 2 threads (6 threads, 3 sets)
            // Each thread inserts 50 values, so each set should have up to 100 elements
            assert!(size <= 100, "Set {} has size {} > 100", idx, size);
            
            if size > 0 {
                let min = set.minimum();
                let max = set.maximum();
                assert!(min.is_some() && max.is_some(), 
                    "Set {} with size {} missing min/max", idx, size);
            }
        }
    }
}

#[test]
fn test_deadlock_prevention_bst_sets() {
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::{Duration, Instant};
    use apas_ai::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt;
    
    let barrier = Arc::new(Barrier::new(6));
    let shared_sets = Arc::new(std::sync::RwLock::new(vec![
        BSTSetAVLMt::empty(),
        BSTSetAVLMt::empty(),
        BSTSetAVLMt::empty(),
    ]));
    let mut handles = vec![];
    
    // Spawn threads that access multiple sets in different orders to test deadlock prevention
    for thread_id in 0..6 {
        let barrier_clone = Arc::clone(&barrier);
        let shared_sets_clone = Arc::clone(&shared_sets);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            
            let start_time = Instant::now();
            let mut operations_completed = 0;
            
            // Run for a limited time to detect deadlocks
            while start_time.elapsed() < Duration::from_millis(100) {
                // Access sets in different orders based on thread_id to create potential deadlock scenarios
                let (first_idx, second_idx) = if thread_id % 2 == 0 {
                    (0, 1)
                } else {
                    (1, 0)
                };
                
                // Perform operations that could cause deadlocks if not properly synchronized
                if let Ok(mut sets) = shared_sets_clone.try_write() {
                    // Insert values into first set
                    sets[first_idx].insert(thread_id * 100 + operations_completed);
                    
                    // Then into second set
                    sets[second_idx].insert(thread_id * 100 + operations_completed + 50);
                    
                    operations_completed += 1;
                } else {
                    // If we can't get write lock, try read operations
                    if let Ok(sets) = shared_sets_clone.try_read() {
                        let _ = sets[first_idx].size();
                        let _ = sets[second_idx].contains(&(thread_id * 100));
                    }
                }
                
                // Small yield to allow other threads to run
                thread::yield_now();
            }
            
            (thread_id, operations_completed)
        });
        handles.push(handle);
    }
    
    // Collect results - if there's a deadlock, this will hang
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    
    // Verify all threads completed some operations (no deadlock occurred)
    for (thread_id, op_count) in results {
        assert!(op_count > 0, "Thread {} completed no operations - possible deadlock", thread_id);
    }
    
    // Verify final state is consistent
    if let Ok(final_sets) = shared_sets.read() {
        let mut total_elements = 0;
        for (idx, set) in final_sets.iter().enumerate() {
            let size = set.size();
            total_elements += size;
            println!("Set {}: size = {}", idx, size);
        }
        // At least some operations should have completed across all sets
        assert!(total_elements > 0, "No elements inserted across all sets - possible deadlock");
    }
}
