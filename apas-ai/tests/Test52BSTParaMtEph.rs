use apas_ai::BSTParaMtEph::BSTParaMtEph::*;
use apas_ai::ArrayStPerSLit;
use apas_ai::*;

fn make_tree(values: &[i32]) -> ParamBST<i32> {
    let tree = ParamBST::new();
    for &value in values {
        tree.insert(value);
    }
    tree
}

fn make_range_tree(start: i32, end: i32) -> ParamBST<i32> {
    let tree = ParamBST::new();
    for value in start..end {
        tree.insert(value);
    }
    tree
}

#[test]
fn para_basic_insert_find() {
    let tree = make_tree(&[4, 2, 6, 1, 3, 5, 7]);
    assert_eq!(tree.size(), 7);
    assert_eq!(tree.find(&3), Some(3));
    assert_eq!(tree.find(&8), None);
    assert_eq!(tree.is_empty(), B::False);
    assert_eq!(tree.in_order(), ArrayStPerSLit![1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn para_split_and_join_pair() {
    let tree = make_tree(&[0, 1, 2, 3, 4, 5]);
    let (less, present, greater) = tree.split(&3);
    assert_eq!(present, B::True);
    assert_eq!(less.in_order(), ArrayStPerSLit![0, 1, 2]);
    assert_eq!(greater.in_order(), ArrayStPerSLit![4, 5]);

    let rejoined = less.join_pair(greater);
    assert_eq!(rejoined.in_order(), ArrayStPerSLit![0, 1, 2, 4, 5]);
}

#[test]
fn para_union_and_delete() {
    let a = make_tree(&[1, 3, 5, 7]);
    let b = make_tree(&[0, 2, 4, 6, 8]);
    let union = a.union(&b);
    assert_eq!(union.in_order(), ArrayStPerSLit![0, 1, 2, 3, 4, 5, 6, 7, 8]);

    union.delete(&4);
    union.delete(&7);
    assert_eq!(union.find(&4), None);
    assert_eq!(union.find(&7), None);
    assert_eq!(union.in_order(), ArrayStPerSLit![0, 1, 2, 3, 5, 6, 8]);
}

#[test]
fn para_join_mid_expose_roundtrip() {
    let empty = ParamBST::<i32>::join_mid(Exposed::Leaf);
    match empty.expose() {
        | Exposed::Leaf => {}
        | Exposed::Node(..) => panic!("expected leaf"),
    }

    let left = ParamBST::<i32>::join_mid(Exposed::Leaf);
    let right = ParamBST::<i32>::join_mid(Exposed::Leaf);
    let combined = ParamBST::<i32>::join_mid(Exposed::Node(left, 10, right));

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
fn para_intersect_and_difference() {
    let a = make_tree(&[1, 2, 3, 4, 5, 6]);
    let b = make_tree(&[4, 5, 6, 7, 8]);

    let intersection = a.intersect(&b);
    assert_eq!(intersection.in_order(), ArrayStPerSLit![4, 5, 6]);

    let difference = a.difference(&b);
    assert_eq!(difference.in_order(), ArrayStPerSLit![1, 2, 3]);
}

#[test]
fn para_filter_and_reduce() {
    let tree = make_tree(&[1, 2, 3, 4, 5, 6]);

    let evens = tree.filter(|v| v % 2 == 0);
    assert_eq!(evens.in_order(), ArrayStPerSLit![2, 4, 6]);

    let sum = tree.reduce(|a, b| a + b, 0);
    assert_eq!(sum, 21);

    let empty_sum = ParamBST::<i32>::new().reduce(|a, b| a + b, 0);
    assert_eq!(empty_sum, 0);
}

#[test]
fn para_union_large_balanced() {
    let a = make_range_tree(0, 200);
    let b = make_range_tree(100, 300);

    let union = a.union(&b);
    let values: Vec<_> = union.in_order().iter().copied().collect();
    let expected: Vec<_> = (0..300).collect();
    assert_eq!(values, expected);
}

#[test]
fn para_intersect_and_difference_large() {
    let a = make_range_tree(0, 256);
    let b = make_range_tree(128, 384);

    let intersection = a.intersect(&b);
    let intersect_values: Vec<_> = intersection.in_order().iter().copied().collect();
    let expected_intersection: Vec<_> = (128..256).collect();
    assert_eq!(intersect_values, expected_intersection);

    let difference = a.difference(&b);
    let diff_values: Vec<_> = difference.in_order().iter().copied().collect();
    let expected_difference: Vec<_> = (0..128).collect();
    assert_eq!(diff_values, expected_difference);
}

#[test]
fn para_filter_and_reduce_edge_cases() {
    let tree = make_range_tree(0, 64);

    let odds = tree.filter(|v| v % 2 == 1);
    let odd_values: Vec<_> = odds.in_order().iter().copied().collect();
    let expected_odds: Vec<_> = (0..64).filter(|v| v % 2 == 1).collect();
    assert_eq!(odd_values, expected_odds);

    let sum_squares = tree.reduce(|acc, v| acc + v * v, 0);
    let expected_sum_squares = (63 * 64 * 127) / 6; // sum_{i=0}^{63} i^2
    assert_eq!(sum_squares, expected_sum_squares);

    let single = make_tree(&[42]);
    let filtered_single = single.filter(|v| *v == 42);
    assert_eq!(filtered_single.in_order().iter().copied().collect::<Vec<_>>(), vec![42]);
    let reduced_single = single.reduce(|a, b| a + b, 0);
    assert_eq!(reduced_single, 42);
}
