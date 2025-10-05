//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Types module.

use apas_ai::Types::Types::*;

#[test]
fn test_pair_creation() {
    let p = Pair(1, 2);
    assert_eq!(p.0, 1);
    assert_eq!(p.1, 2);
}

#[test]
fn test_pair_clone() {
    let p1 = Pair(5, 10);
    let p2 = p1.clone();
    assert_eq!(p1, p2);
}

#[test]
fn test_triple_creation() {
    let t = Triple(1, 2, 3);
    assert_eq!(t.0, 1);
    assert_eq!(t.1, 2);
    assert_eq!(t.2, 3);
}

#[test]
fn test_triple_clone() {
    let t1 = Triple(5, 10, 15);
    let t2 = t1.clone();
    assert_eq!(t1, t2);
}

#[test]
fn test_key_val_creation() {
    let kv = KeyVal{ key: 1, val: "test" };
    assert_eq!(kv.key, 1);
    assert_eq!(kv.val, "test");
}

#[test]
fn test_key_val_clone() {
    let kv1 = KeyVal{ key: 5, val: 100 };
    let kv2 = kv1.clone();
    assert_eq!(kv1, kv2);
}

#[test]
fn test_usize_type() {
    let n: N = 42;
    assert_eq!(n, 42);
}

#[test]
fn test_pair_ordering() {
    let p1 = Pair(1, 2);
    let p2 = Pair(1, 3);
    assert!(p1 < p2);
}

#[test]
fn test_triple_ordering() {
    let t1 = Triple(1, 2, 3);
    let t2 = Triple(1, 2, 4);
    assert!(t1 < t2);
}

#[test]
fn test_key_val_ordering() {
    let kv1 = KeyVal { key: 1, val: 2 };
    let kv2 = KeyVal { key: 1, val: 3 };
    assert!(kv1 < kv2);
}

#[test]
fn test_edge_creation() {
    let e = Edge(1, 2);
    assert_eq!(e.0, 1);
    assert_eq!(e.1, 2);
}

#[test]
fn test_edge_clone() {
    let e1 = Edge(1, 2);
    let e2 = e1.clone();
    assert_eq!(e1, e2);
}

#[test]
fn test_edge_from_tuple() {
    let e: Edge<i32> = (1, 2).into();
    assert_eq!(e.0, 1);
    assert_eq!(e.1, 2);
}

#[test]
fn test_edge_to_tuple() {
    let e = Edge(1, 2);
    let t: (i32, i32) = e.into();
    assert_eq!(t, (1, 2));
}

#[test]
fn test_lab_edge_creation() {
    let le = LabEdge(1, 2, "label");
    assert_eq!(le.0, 1);
    assert_eq!(le.1, 2);
    assert_eq!(le.2, "label");
}

#[test]
fn test_lab_edge_from_tuple() {
    let le: LabEdge<i32, &str> = (1, 2, "test").into();
    assert_eq!(le.0, 1);
    assert_eq!(le.1, 2);
    assert_eq!(le.2, "test");
}

#[test]
fn test_lab_edge_to_tuple() {
    let le = LabEdge(1, 2, "label");
    let t: (i32, i32, &str) = le.into();
    assert_eq!(t, (1, 2, "label"));
}

#[test]
fn test_pair_from_tuple() {
    let p: Pair<i32, i32> = (1, 2).into();
    assert_eq!(p.0, 1);
    assert_eq!(p.1, 2);
}

#[test]
fn test_pair_to_tuple() {
    let p = Pair(1, 2);
    let t: (i32, i32) = p.into();
    assert_eq!(t, (1, 2));
}

#[test]
fn test_ordered_float_f32() {
    let f: OrderedF32 = OrderedFloat(3.14);
    assert_eq!(*f, 3.14);
}

#[test]
fn test_ordered_float_f64() {
    let f: OrderedF64 = OrderedFloat(2.718);
    assert_eq!(*f, 2.718);
}

#[test]
fn test_array_seq_set_eq_equal() {
    fn a_nth(i: usize) -> i32 { [1, 2, 3][i] }
    fn b_nth(i: usize) -> i32 { [1, 2, 3][i] }
    let result = ArraySeqSetEq(3, a_nth, 3, b_nth);
    assert!(result);
}

#[test]
fn test_array_seq_set_eq_different_order() {
    fn a_nth(i: usize) -> i32 { [1, 2, 3][i] }
    fn b_nth(i: usize) -> i32 { [3, 1, 2][i] }
    let result = ArraySeqSetEq(3, a_nth, 3, b_nth);
    assert!(result);
}

#[test]
fn test_array_seq_set_eq_different_elements() {
    fn a_nth(i: usize) -> i32 { [1, 2, 3][i] }
    fn b_nth(i: usize) -> i32 { [1, 2, 4][i] }
    let result = ArraySeqSetEq(3, a_nth, 3, b_nth);
    assert!(!result);
}

#[test]
fn test_array_seq_set_eq_different_length() {
    fn a_nth(i: usize) -> i32 { [1, 2, 3][i] }
    fn b_nth(i: usize) -> i32 { [1, 2][i] }
    let result = ArraySeqSetEq(3, a_nth, 2, b_nth);
    assert!(!result);
}

#[test]
fn test_bool_type_alias() {
    let b: B = true;
    assert_eq!(b, true);
    let b: B = false;
    assert_eq!(b, false);
}

#[test]
fn test_ordering_type_alias() {
    use std::cmp::Ordering;
    let o: O = Ordering::Less;
    assert_eq!(o, Ordering::Less);
}

#[test]
fn test_pair_display() {
    let p = Pair(1, 2);
    let s = format!("{}", p);
    assert_eq!(s, "(1 -> 2)");
}

#[test]
fn test_triple_display() {
    let t = Triple(1, 2, 3);
    let s = format!("{}", t);
    assert_eq!(s, "(1, 2, 3)");
}

#[test]
fn test_key_val_display() {
    let kv = KeyVal { key: 1, val: 2 };
    let s = format!("{}", kv);
    assert_eq!(s, "{key: 1, val: 2}");
}

#[test]
fn test_edge_display() {
    let e = Edge(1, 2);
    let s = format!("{}", e);
    assert_eq!(s, "(1, 2)");
}

#[test]
fn test_lab_edge_display() {
    let le = LabEdge(1, 2, "test");
    let s = format!("{}", le);
    assert_eq!(s, "(1, 2, test)");
}
