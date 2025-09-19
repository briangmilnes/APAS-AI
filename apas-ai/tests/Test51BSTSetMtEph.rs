use apas_ai::*;

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
    fn iter_seq(&self) -> ArrayStPerS<i32>;
}

impl TestSet for apas_ai::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32> {
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

    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32> {
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

    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32> {
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

    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32> {
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

    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32> {
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

    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }
}

impl TestSet for apas_ai::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32> {
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

    fn iter_seq(&self) -> ArrayStPerS<i32> { self.iter_in_order() }
}

fn exercise_set<S: TestSet>() {
    let mut set = S::empty();
    assert_eq!(set.is_empty(), B::True);

    for value in [3, 1, 4, 2] {
        set.insert(value);
    }

    assert_eq!(set.size(), 4);
    assert_eq!(set.contains(&2), B::True);
    assert_eq!(set.minimum(), Some(1));
    assert_eq!(set.maximum(), Some(4));

    set.delete(&3);
    assert_eq!(set.contains(&3), B::False);
    assert_eq!(set.size(), 3);

    let mut other = S::empty();
    for value in [2, 5] {
        other.insert(value);
    }

    let union = set.union(&other);
    assert_eq!(union.size(), 4);

    let intersection = set.intersection(&other);
    assert_eq!(intersection.size(), 1);
    assert_eq!(intersection.contains(&2), B::True);

    let difference = set.difference(&other);
    assert_eq!(difference.size(), 2);
    assert_eq!(difference.contains(&1), B::True);
    assert_eq!(difference.contains(&4), B::True);

    let (lt, found_missing, gt) = union.split(&3);
    assert_eq!(found_missing, B::False);
    assert_eq!(lt.size(), 2);
    assert_eq!(gt.size(), 2);

    let rejoined_pair = S::join_pair(lt, gt);
    assert_eq!(rejoined_pair.size(), union.size());

    let (lt_with_pivot, found_pivot, gt_with_pivot) = union.split(&4);
    assert_eq!(found_pivot, B::True);
    let rejoined_mid = S::join_m(lt_with_pivot, 4, gt_with_pivot);
    assert_eq!(rejoined_mid.size(), union.size());

    let evens = union.filter(|value| *value % 2 == 0);
    assert_eq!(evens.size(), 2);
    assert_eq!(evens.contains(&2), B::True);
    assert_eq!(evens.contains(&4), B::True);

    let sum = union.reduce(|acc, value| acc + value, 0);
    assert_eq!(sum, 12);

    let in_order = union.iter_seq();
    assert_eq!(in_order, ArraySeqStPer![1, 2, 4, 5]);
}

#[test]
fn test_plain_bst_set_ops() { exercise_set::<apas_ai::BSTSetPlainMtEph::BSTSetPlainMtEph::BSTSetPlainMt<i32>>(); }

#[test]
fn test_avl_bst_set_ops() { exercise_set::<apas_ai::BSTSetAVLMtEph::BSTSetAVLMtEph::BSTSetAVLMt<i32>>(); }

#[test]
fn test_rb_bst_set_ops() { exercise_set::<apas_ai::BSTSetRBMtEph::BSTSetRBMtEph::BSTSetRBMt<i32>>(); }

#[test]
fn test_bbalpha_bst_set_ops() {
    exercise_set::<apas_ai::BSTSetBBAlphaMtEph::BSTSetBBAlphaMtEph::BSTSetBBAlphaMt<i32>>();
}

#[test]
fn test_treap_bst_set_ops() { exercise_set::<apas_ai::BSTSetTreapMtEph::BSTSetTreapMtEph::BSTSetTreapMt<i32>>(); }

#[test]
fn test_splay_bst_set_ops() { exercise_set::<apas_ai::BSTSetSplayMtEph::BSTSetSplayMtEph::BSTSetSplayMt<i32>>(); }
