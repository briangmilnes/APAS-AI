//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 PQMinMtEph.

use apas_ai::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
use apas_ai::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
use apas_ai::Chap53::PQMinMtEph::PQMinMtEph::*;
use apas_ai::Types::Types::*;

fn vertex_priority() -> ClosurePriority<N, N, impl Fn(&N) -> N + Send + Sync + 'static> {
    ClosurePriority::new(|v: &N| *v)
}

fn test_graph_1() -> impl Fn(&N) -> AVLTreeSetMtEph<N> + Send + Sync + 'static {
    |v: &N| match *v {
        | 1 => AVLTreeSetMtEph::singleton(2).union(&AVLTreeSetMtEph::singleton(3)),
        | 2 => AVLTreeSetMtEph::singleton(4),
        | 3 => AVLTreeSetMtEph::singleton(4).union(&AVLTreeSetMtEph::singleton(5)),
        | 4 => AVLTreeSetMtEph::empty(),
        | 5 => AVLTreeSetMtEph::empty(),
        | _ => AVLTreeSetMtEph::empty(),
    }
}

#[test]
fn test_pq_min_empty_graph() {
    let graph = |_: &N| AVLTreeSetMtEph::empty();
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min(graph, 1, prio_fn);
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_pq_min_single_edge() {
    let graph = |v: &N| {
        if *v == 1 {
            AVLTreeSetMtEph::singleton(2)
        } else {
            AVLTreeSetMtEph::empty()
        }
    };
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min(graph, 1, prio_fn);
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_pq_min_dag() {
    let graph = test_graph_1();
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min(graph, 1, prio_fn);
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_priority_order() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtEph::singleton(2).union(&AVLTreeSetMtEph::singleton(3)),
        | 2 => AVLTreeSetMtEph::singleton(4),
        | 3 => AVLTreeSetMtEph::singleton(5),
        | _ => AVLTreeSetMtEph::empty(),
    };
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min(graph, 1, prio_fn);

    assert_eq!(result.visited.size(), 5);
    assert_eq!(result.priorities.size(), 5);
}

#[test]
fn test_pq_min_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetMtEph::singleton(2).union(&AVLTreeSetMtEph::singleton(5));
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min_multi(graph, sources, prio_fn);

    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
}

#[test]
fn test_pq_min_linear_chain() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtEph::singleton(2),
        | 2 => AVLTreeSetMtEph::singleton(3),
        | 3 => AVLTreeSetMtEph::singleton(4),
        | _ => AVLTreeSetMtEph::empty(),
    };
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min(graph, 1, prio_fn);

    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_cycle() {
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtEph::singleton(2),
        | 2 => AVLTreeSetMtEph::singleton(3),
        | 3 => AVLTreeSetMtEph::singleton(1),
        | _ => AVLTreeSetMtEph::empty(),
    };
    let prio_fn = vertex_priority();
    let result = PQMinMtEph::pq_min(graph, 1, prio_fn);

    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}
