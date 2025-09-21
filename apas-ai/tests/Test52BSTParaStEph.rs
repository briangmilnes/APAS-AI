use apas_ai::ArrayStPerSLit;
use apas_ai::BSTParaStEph::BSTParaStEph::*;
use apas_ai::*;

fn make_tree(values: &[i32]) -> ParamBST<i32> {
    let tree = ParamBST::new();
    for &value in values {
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
