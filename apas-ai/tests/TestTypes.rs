//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Types module.

use std::f32::consts::PI;
use std::f64::consts::E;

use apas_ai::Types::Types::*;
use apas_ai::{EdgeList, EdgeLit, PairList, PairLit, ParaPair};

#[test]
fn test_pair_creation() {
    let p = Pair(1, 2);
    assert_eq!(p.0, 1);
    assert_eq!(p.1, 2);
}

#[test]
fn test_pair_clone() {
    let p1 = Pair(5, 10);
    let p2 = p1;
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
    let t2 = t1;
    assert_eq!(t1, t2);
}

#[test]
fn test_key_val_creation() {
    let kv = KeyVal { key: 1, val: "test" };
    assert_eq!(kv.key, 1);
    assert_eq!(kv.val, "test");
}

#[test]
fn test_key_val_clone() {
    let kv1 = KeyVal { key: 5, val: 100 };
    let kv2 = kv1;
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
    let e2 = e1;
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
    let f: OrderedF32 = OrderedFloat(PI);
    assert_eq!(*f, PI);
}

#[test]
fn test_ordered_float_f64() {
    let f: OrderedF64 = OrderedFloat(E);
    assert_eq!(*f, E);
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
    assert!(b);
    let b: B = false;
    assert!(!b);
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
    let s = format!("{p}");
    assert_eq!(s, "(1 -> 2)");
}

#[test]
fn test_triple_display() {
    let t = Triple(1, 2, 3);
    let s = format!("{t}");
    assert_eq!(s, "(1, 2, 3)");
}

#[test]
fn test_key_val_display() {
    let kv = KeyVal { key: 1, val: 2 };
    let s = format!("{kv}");
    assert_eq!(s, "{key: 1, val: 2}");
}

#[test]
fn test_edge_display() {
    let e = Edge(1, 2);
    let s = format!("{e}");
    assert_eq!(s, "(1, 2)");
}

#[test]
fn test_lab_edge_display() {
    let le = LabEdge(1, 2, "test");
    let s = format!("{le}");
    assert_eq!(s, "(1, 2, test)");
}

#[test]
fn test_edgelit_macro() {
    let e = EdgeLit!(1, 2);
    assert_eq!(e.0, 1);
    assert_eq!(e.1, 2);
}

#[test]
fn test_edgelit_macro_type_inference() {
    let e = EdgeLit!(10, 20);
    assert_eq!(e, Edge(10, 20));
}

#[test]
fn test_pairlit_macro() {
    let p = PairLit!(3, 4);
    assert_eq!(p.0, 3);
    assert_eq!(p.1, 4);
}

#[test]
fn test_pairlit_macro_different_types() {
    let p = PairLit!("key", 100);
    assert_eq!(p.0, "key");
    assert_eq!(p.1, 100);
}

#[test]
fn test_edgelist_macro_empty() {
    let edges: Vec<Edge<i32>> = EdgeList![];
    assert_eq!(edges.len(), 0);
}

#[test]
fn test_edgelist_macro_single() {
    let edges = EdgeList![(1, 2)];
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0], Edge(1, 2));
}

#[test]
fn test_edgelist_macro_multiple() {
    let edges = EdgeList![(1, 2), (3, 4), (5, 6)];
    assert_eq!(edges.len(), 3);
    assert_eq!(edges[0], Edge(1, 2));
    assert_eq!(edges[1], Edge(3, 4));
    assert_eq!(edges[2], Edge(5, 6));
}

#[test]
fn test_edgelist_macro_trailing_comma() {
    let edges = EdgeList![(1, 2), (3, 4),];
    assert_eq!(edges.len(), 2);
}

#[test]
fn test_pairlist_macro_empty() {
    let pairs: Vec<Pair<i32, i32>> = PairList![];
    assert_eq!(pairs.len(), 0);
}

#[test]
fn test_pairlist_macro_single() {
    let pairs = PairList![(10, 20)];
    assert_eq!(pairs.len(), 1);
    assert_eq!(pairs[0], Pair(10, 20));
}

#[test]
fn test_pairlist_macro_multiple() {
    let pairs = PairList![(1, 2), (3, 4), (5, 6)];
    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs[0], Pair(1, 2));
    assert_eq!(pairs[1], Pair(3, 4));
    assert_eq!(pairs[2], Pair(5, 6));
}

#[test]
fn test_pairlist_macro_different_types() {
    let pairs = PairList![("a", 1), ("b", 2)];
    assert_eq!(pairs[0], Pair("a", 1));
    assert_eq!(pairs[1], Pair("b", 2));
}

#[test]
fn test_parapair_macro() {
    let result = ParaPair!(|| { 10 + 20 }, || { 5 * 6 });
    assert_eq!(result.0, 30);
    assert_eq!(result.1, 30);
}

#[test]
fn test_parapair_macro_complex() {
    let result = ParaPair!(
        || {
            let mut sum = 0;
            for i in 1..=100 {
                sum += i;
            }
            sum
        },
        || {
            let mut product = 1;
            for i in 1..=10 {
                product *= i;
            }
            product
        }
    );
    assert_eq!(result.0, 5050);
    assert_eq!(result.1, 3628800);
}

// MtT trait tests
#[test]
fn test_mtt_usize_clone_mt() {
    let val: usize = 42;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_usize_new_mt() {
    let val = <usize as MtT>::new_mt(42usize);
    assert_eq!(val, 42);
}

#[test]
fn test_mtt_isize_clone_mt() {
    let val: isize = -42;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_isize_new_mt() {
    let val = <isize as MtT>::new_mt(-42isize);
    assert_eq!(val, -42);
}

#[test]
fn test_mtt_i32_clone_mt() {
    let val: i32 = 100;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_i32_new_mt() {
    let val = <i32 as MtT>::new_mt(100i32);
    assert_eq!(val, 100);
}

#[test]
fn test_mtt_u32_clone_mt() {
    let val: u32 = 200;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_u32_new_mt() {
    let val = <u32 as MtT>::new_mt(200u32);
    assert_eq!(val, 200);
}

#[test]
fn test_mtt_i64_clone_mt() {
    let val: i64 = 1000;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_i64_new_mt() {
    let val = <i64 as MtT>::new_mt(1000i64);
    assert_eq!(val, 1000);
}

#[test]
fn test_mtt_u64_clone_mt() {
    let val: u64 = 2000;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_u64_new_mt() {
    let val = <u64 as MtT>::new_mt(2000u64);
    assert_eq!(val, 2000);
}

#[test]
fn test_mtt_bool_clone_mt() {
    let val: bool = true;
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_bool_new_mt() {
    let val = <bool as MtT>::new_mt(true);
    assert!(val);
}

#[test]
fn test_mtt_char_clone_mt() {
    let val: char = 'A';
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_char_new_mt() {
    let val = <char as MtT>::new_mt('Z');
    assert_eq!(val, 'Z');
}

#[test]
fn test_mtt_string_clone_mt() {
    let val = String::from("test");
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_string_new_mt() {
    let val = <String as MtT>::new_mt(String::from("hello"));
    assert_eq!(val, "hello");
}

#[test]
fn test_mtt_str_clone_mt() {
    let val: &str = "test";
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_str_new_mt() {
    let val = <&str as MtT>::new_mt("world");
    assert_eq!(val, "world");
}

#[test]
fn test_mtt_mutex_clone_mt() {
    use std::sync::Mutex;
    let val = Mutex::new(42);
    let cloned = MtT::clone_mt(&val);
    assert_eq!(*val.lock().unwrap(), *cloned.lock().unwrap());
}

#[test]
fn test_mtt_mutex_new_mt() {
    use std::sync::Mutex;
    let val: Mutex<i32> = MtT::new_mt(99);
    assert_eq!(*val.lock().unwrap(), 99);
}

#[test]
fn test_mtt_pair_clone_mt() {
    let val = Pair(1, 2);
    let cloned = MtT::clone_mt(&val);
    assert_eq!(val, cloned);
}

#[test]
fn test_mtt_pair_new_mt() {
    let val: Pair<i32, i32> = MtT::new_mt(Pair(5, 10));
    assert_eq!(val, Pair(5, 10));
}
