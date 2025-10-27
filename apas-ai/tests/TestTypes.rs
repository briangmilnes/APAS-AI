//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Comprehensive tests for Types module to achieve 100% function coverage

use apas_ai::Types::Types::*;
use apas_ai::{EdgeLit, PairLit, EdgeList, PairList, ParaPair};
use std::sync::Mutex;

// ============================================================================
// Test MtT trait implementations - clone_mt() and new_mt()
// ============================================================================

#[test]
fn test_mtt_mutex() {
    let mutex = Mutex::new(42);
    let cloned = mutex.clone_mt();
    assert_eq!(*cloned.lock().unwrap(), 42);
    
    let new_mutex = <Mutex<i32> as MtT>::new_mt(100);
    assert_eq!(*new_mutex.lock().unwrap(), 100);
}

#[test]
fn test_mtt_pair() {
    let pair = Pair(1, 2);
    let cloned = pair.clone_mt();
    assert_eq!(cloned, Pair(1, 2));
    
    let new_pair = <Pair<i32, i32> as MtT>::new_mt(Pair(3, 4));
    assert_eq!(new_pair, Pair(3, 4));
}

#[test]
fn test_mtt_usize() {
    let val: usize = 42;
    let cloned = val.clone_mt();
    assert_eq!(cloned, 42);
    
    let new_val = <usize as MtT>::new_mt(100);
    assert_eq!(new_val, 100);
}

#[test]
fn test_mtt_isize() {
    let val: isize = -42;
    let cloned = val.clone_mt();
    assert_eq!(cloned, -42);
    
    let new_val = <isize as MtT>::new_mt(-100);
    assert_eq!(new_val, -100);
}

#[test]
fn test_mtt_i32() {
    let val: i32 = -42;
    let cloned = val.clone_mt();
    assert_eq!(cloned, -42);
    
    let new_val = <i32 as MtT>::new_mt(-100);
    assert_eq!(new_val, -100);
}

#[test]
fn test_mtt_u32() {
    let val: u32 = 42;
    let cloned = val.clone_mt();
    assert_eq!(cloned, 42);
    
    let new_val = <u32 as MtT>::new_mt(100);
    assert_eq!(new_val, 100);
}

#[test]
fn test_mtt_i64() {
    let val: i64 = -42;
    let cloned = val.clone_mt();
    assert_eq!(cloned, -42);
    
    let new_val = <i64 as MtT>::new_mt(-100);
    assert_eq!(new_val, -100);
}

#[test]
fn test_mtt_u64() {
    let val: u64 = 42;
    let cloned = val.clone_mt();
    assert_eq!(cloned, 42);
    
    let new_val = <u64 as MtT>::new_mt(100);
    assert_eq!(new_val, 100);
}

#[test]
fn test_mtt_bool() {
    let val: bool = true;
    let cloned = val.clone_mt();
    assert_eq!(cloned, true);
    
    let new_val = <bool as MtT>::new_mt(false);
    assert_eq!(new_val, false);
}

#[test]
fn test_mtt_char() {
    let val: char = 'x';
    let cloned = val.clone_mt();
    assert_eq!(cloned, 'x');
    
    let new_val = <char as MtT>::new_mt('z');
    assert_eq!(new_val, 'z');
}

#[test]
fn test_mtt_string() {
    let val = String::from("hello");
    let cloned = val.clone_mt();
    assert_eq!(cloned, "hello");
    
    let new_val = <String as MtT>::new_mt(String::from("world"));
    assert_eq!(new_val, "world");
}

#[test]
fn test_mtt_str() {
    let val: &str = "hello";
    let cloned = val.clone_mt();
    assert_eq!(cloned, "hello");
    
    let new_val = <&str as MtT>::new_mt("world");
    assert_eq!(new_val, "world");
}

// ============================================================================
// Test Display implementations
// ============================================================================

#[test]
fn test_edge_display() {
    let edge = Edge(1, 2);
    assert_eq!(format!("{}", edge), "(1, 2)");
    
    let edge_str = Edge("a", "b");
    assert_eq!(format!("{}", edge_str), "(a, b)");
}

#[test]
fn test_labedge_display() {
    let edge = LabEdge(1, 2, "label");
    assert_eq!(format!("{}", edge), "(1, 2, label)");
    
    let edge_num = LabEdge("a", "b", 42);
    assert_eq!(format!("{}", edge_num), "(a, b, 42)");
}

#[test]
fn test_pair_display() {
    let pair = Pair(1, 2);
    assert_eq!(format!("{}", pair), "(1 -> 2)");
    
    let pair_str = Pair("key", "value");
    assert_eq!(format!("{}", pair_str), "(key -> value)");
}

#[test]
fn test_triple_display() {
    let triple = Triple(1, 2, 3);
    assert_eq!(format!("{}", triple), "(1, 2, 3)");
    
    let triple_str = Triple("a", "b", "c");
    assert_eq!(format!("{}", triple_str), "(a, b, c)");
}

#[test]
fn test_quadruple_display() {
    let quad = Quadruple(1, 2, 3, 4);
    assert_eq!(format!("{}", quad), "(1, 2, 3, 4)");
    
    let quad_str = Quadruple("a", "b", "c", "d");
    assert_eq!(format!("{}", quad_str), "(a, b, c, d)");
}

#[test]
fn test_keyval_display() {
    let kv = KeyVal { key: 1, val: 2 };
    assert_eq!(format!("{}", kv), "{key: 1, val: 2}");
    
    let kv_str = KeyVal { key: "name", val: "Alice" };
    assert_eq!(format!("{}", kv_str), "{key: name, val: Alice}");
}

// ============================================================================
// Test From implementations (conversions)
// ============================================================================

#[test]
fn test_edge_from_tuple() {
    let edge: Edge<i32> = (1, 2).into();
    assert_eq!(edge, Edge(1, 2));
}

#[test]
fn test_edge_into_tuple() {
    let edge = Edge(1, 2);
    let tuple: (i32, i32) = edge.into();
    assert_eq!(tuple, (1, 2));
}

#[test]
fn test_labedge_from_tuple() {
    let edge: LabEdge<i32, &str> = (1, 2, "label").into();
    assert_eq!(edge, LabEdge(1, 2, "label"));
}

#[test]
fn test_labedge_into_tuple() {
    let edge = LabEdge(1, 2, "label");
    let tuple: (i32, i32, &str) = edge.into();
    assert_eq!(tuple, (1, 2, "label"));
}

#[test]
fn test_pair_from_tuple() {
    let pair: Pair<i32, i32> = (1, 2).into();
    assert_eq!(pair, Pair(1, 2));
}

#[test]
fn test_pair_into_tuple() {
    let pair = Pair(1, 2);
    let tuple: (i32, i32) = pair.into();
    assert_eq!(tuple, (1, 2));
}

// ============================================================================
// Test macros
// ============================================================================

#[test]
fn test_edgelit_macro() {
    let edge = EdgeLit!(1, 2);
    assert_eq!(edge, Edge(1, 2));
}

#[test]
fn test_pairlit_macro() {
    let pair = PairLit!(1, 2);
    assert_eq!(pair, Pair(1, 2));
}

#[test]
fn test_edgelist_macro() {
    let edges = EdgeList![(1, 2), (3, 4), (5, 6)];
    assert_eq!(edges.len(), 3);
    assert_eq!(edges[0], Edge(1, 2));
    assert_eq!(edges[1], Edge(3, 4));
    assert_eq!(edges[2], Edge(5, 6));
    
    // Test empty
    let empty: Vec<Edge<i32>> = EdgeList![];
    assert_eq!(empty.len(), 0);
}

#[test]
fn test_pairlist_macro() {
    let pairs = PairList![(1, 2), (3, 4), (5, 6)];
    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs[0], Pair(1, 2));
    assert_eq!(pairs[1], Pair(3, 4));
    assert_eq!(pairs[2], Pair(5, 6));
    
    // Test empty
    let empty: Vec<Pair<i32, i32>> = PairList![];
    assert_eq!(empty.len(), 0);
}

// ============================================================================
// Test struct properties (Clone, PartialEq, Hash, etc.)
// ============================================================================

#[test]
fn test_pair_properties() {
    let p1 = Pair(1, 2);
    let p2 = Pair(1, 2);
    let p3 = Pair(2, 1);
    
    // PartialEq
    assert_eq!(p1, p2);
    assert_ne!(p1, p3);
    
    // Clone
    let cloned = p1.clone();
    assert_eq!(cloned, p1);
    
    // Ord
    assert!(p1 < p3);
}

#[test]
fn test_triple_properties() {
    let t1 = Triple(1, 2, 3);
    let t2 = Triple(1, 2, 3);
    
    assert_eq!(t1, t2);
    assert_eq!(t1.clone(), t1);
}

#[test]
fn test_quadruple_properties() {
    let q1 = Quadruple(1, 2, 3, 4);
    let q2 = Quadruple(1, 2, 3, 4);
    
    assert_eq!(q1, q2);
    assert_eq!(q1.clone(), q1);
}

#[test]
fn test_keyval_properties() {
    let kv1 = KeyVal { key: 1, val: 2 };
    let kv2 = KeyVal { key: 1, val: 2 };
    
    assert_eq!(kv1, kv2);
    assert_eq!(kv1.clone(), kv1);
}

#[test]
fn test_edge_properties() {
    let e1 = Edge(1, 2);
    let e2 = Edge(1, 2);
    
    assert_eq!(e1, e2);
    assert_eq!(e1.clone(), e1);
}

#[test]
fn test_labedge_properties() {
    let e1 = LabEdge(1, 2, "label");
    let e2 = LabEdge(1, 2, "label");
    
    assert_eq!(e1, e2);
    assert_eq!(e1.clone(), e1);
}

// ============================================================================
// Test type aliases and trait bounds work correctly
// ============================================================================

#[test]
fn test_type_aliases() {
    let n: N = 42;
    assert_eq!(n, 42usize);
    
    let b: B = true;
    assert_eq!(b, true);
    
    let o: O = std::cmp::Ordering::Less;
    assert_eq!(o, std::cmp::Ordering::Less);
}

#[test]
fn test_ordered_float() {
    use ordered_float::OrderedFloat;
    
    let f1: OrderedF32 = OrderedFloat(3.14);
    let f2: OrderedF32 = OrderedFloat(2.71);
    
    assert!(f1 > f2);
    assert_eq!(f1.clone(), f1);
}

