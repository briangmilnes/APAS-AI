//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use apas_ai::ArraySeqStPerSLit;
use apas_ai::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
use apas_ai::*;

fn make_tree(values: &[i32]) -> ParamTreap<i32> {
    let tree = ParamTreap::new();
    for &value in values {
        tree.insert(value);
    }
    tree
}

fn make_range_tree(start: i32, end: i32) -> ParamTreap<i32> {
    let tree = ParamTreap::new();
    for value in start..end {
        tree.insert(value);
    }
    tree
}

#[test]
fn treap_basic_insert_find() {
    let tree = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    assert_eq!(tree.size(), 7);
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&8), None);
    assert_eq!(tree.is_empty(), B::False);
    assert_eq!(tree.in_order(), ArraySeqStPerSLit![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn treap_split_join_pair() {
    let tree = make_tree(&[0, 1, 2, 3, 4, 5]);
    let (left, present, right) = tree.split(&3);
    assert_eq!(present, B::True);
    assert_eq!(left.in_order(), ArraySeqStPerSLit![0, 1, 2]);
    assert_eq!(right.in_order(), ArraySeqStPerSLit![4, 5]);

    let rejoined = left.join_pair(right);
    assert_eq!(rejoined.in_order(), ArraySeqStPerSLit![0, 1, 2, 4, 5]);
}

#[test]
fn treap_union_intersect_difference() {
    let a = make_range_tree(0, 200);
    let b = make_range_tree(100, 300);

    let union = a.union(&b);
    let values: Vec<_> = union.in_order().iter().copied().collect();
    assert_eq!(values, (0..300).collect::<Vec<_>>());

    let intersection = make_range_tree(0, 200).intersect(&make_range_tree(100, 300));
    let intersection_values: Vec<_> = intersection.in_order().iter().copied().collect();
    assert_eq!(intersection_values, (100..200).collect::<Vec<_>>());

    let difference = make_range_tree(0, 200).difference(&make_range_tree(100, 300));
    let difference_values: Vec<_> = difference.in_order().iter().copied().collect();
    assert_eq!(difference_values, (0..100).collect::<Vec<_>>());
}

#[test]
fn treap_filter_reduce() {
    let tree = make_range_tree(0, 64);

    let evens = tree.filter(|v| v % 2 == 0);
    let evens_vec: Vec<_> = evens.in_order().iter().copied().collect();
    assert_eq!(evens_vec, (0..64).filter(|v| v % 2 == 0).collect::<Vec<_>>());

    let sum = tree.reduce(|acc, v| acc + v, 0);
    let expected_sum = (0..64).sum();
    assert_eq!(sum, expected_sum);
}

#[test]
fn treap_join_mid_roundtrip() {
    let empty = ParamTreap::<i32>::join_mid(Exposed::Leaf);
    matches!(empty.expose(), Exposed::Leaf);

    let left = ParamTreap::<i32>::join_mid(Exposed::Leaf);
    let right = ParamTreap::<i32>::join_mid(Exposed::Leaf);
    let combined = ParamTreap::<i32>::join_mid(Exposed::Node(left, 10, right));

    match combined.expose() {
        | Exposed::Leaf => panic!("expected node"),
        | Exposed::Node(l, key, r) => {
            assert_eq!(key, 10);
            assert_eq!(l.size(), 0);
            assert_eq!(r.size(), 0);
        }
    }
}

#[test]
fn treap_invariants_priority_heap() {
    fn check_heap(tree: &ParamTreap<i32>) {
        if let Some((left, _key, priority, right)) = tree.expose_with_priority() {
            if let Some((_, _, left_priority, _)) = left.expose_with_priority() {
                assert!(priority >= left_priority);
            }
            if let Some((_, _, right_priority, _)) = right.expose_with_priority() {
                assert!(priority >= right_priority);
            }
            check_heap(&left);
            check_heap(&right);
        }
    }

    let tree = make_range_tree(0, 128);
    check_heap(&tree);
}
