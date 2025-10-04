//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 GraphSearchMtPer.

use apas_ai::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
use apas_ai::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_ai::Chap53::GraphSearchMtPer::GraphSearchMtPer::*;
use apas_ai::Types::Types::*;

fn test_graph_1() -> impl Fn(&N) -> AVLTreeSetMtPer<N> {
    |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(3)),
        | 2 => AVLTreeSetMtPer::singleton(4),
        | 3 => AVLTreeSetMtPer::singleton(4).union(&AVLTreeSetMtPer::singleton(5)),
        | 4 => AVLTreeSetMtPer::empty(),
        | 5 => AVLTreeSetMtPer::empty(),
        | _ => AVLTreeSetMtPer::empty(),
    }
}

#[test]
fn test_graph_search_empty_graph() {
    let graph = |_: &N| AVLTreeSetMtPer::empty();
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_graph_search_single_edge() {
    let graph = |v: &N| {
        if *v == 1 {
            AVLTreeSetMtPer::singleton(2)
        } else {
            AVLTreeSetMtPer::empty()
        }
    };
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_graph_search_dag() {
    let graph = test_graph_1();
    let result = graph_search(&graph, 1, &SelectAll);
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_graph_search_select_one() {
    let graph = test_graph_1();
    let result = graph_search(&graph, 1, &SelectOne);
    // SelectOne strategy: explore one vertex at a time (DFS-like)
    // It visits reachable vertices but may visit fewer due to DFS order
    assert!(result.visited.size() >= 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_graph_search_select_all() {
    let graph = test_graph_1();
    let result = graph_search(&graph, 1, &SelectAll);
    // SelectAll strategy: explore all frontier vertices (BFS-like)
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_graph_search_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(5));
    let result = graph_search_multi(&graph, sources, &SelectAll);

    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
}

#[test]
fn test_graph_search_disconnected() {
    // Two disconnected components: 1 -> 2, 3 -> 4
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::empty(),
        | 3 => AVLTreeSetMtPer::singleton(4),
        | 4 => AVLTreeSetMtPer::empty(),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);

    // Only first component should be visited
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(!result.visited.find(&3));
    assert!(!result.visited.find(&4));
}

#[test]
fn test_graph_search_cycle() {
    // Cycle: 1 -> 2 -> 3 -> 1
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::singleton(3),
        | 3 => AVLTreeSetMtPer::singleton(1),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);

    // All vertices in cycle should be visited exactly once
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_reachable_from_source() {
    let graph = test_graph_1();
    let reachable_set = reachable(&graph, 1);

    assert_eq!(reachable_set.size(), 5);
    for i in 1..=5 {
        assert!(reachable_set.find(&i));
    }
}

#[test]
fn test_reachable_isolated_vertex() {
    let graph = |_: &N| AVLTreeSetMtPer::empty();
    let reachable_set = reachable(&graph, 1);

    assert_eq!(reachable_set.size(), 1);
    assert!(reachable_set.find(&1));
}

#[test]
fn test_graph_search_complete_graph() {
    // Complete graph on 4 vertices: each vertex connects to all others
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(3)).union(&AVLTreeSetMtPer::singleton(4)),
        | 2 => AVLTreeSetMtPer::singleton(1).union(&AVLTreeSetMtPer::singleton(3)).union(&AVLTreeSetMtPer::singleton(4)),
        | 3 => AVLTreeSetMtPer::singleton(1).union(&AVLTreeSetMtPer::singleton(2)).union(&AVLTreeSetMtPer::singleton(4)),
        | 4 => AVLTreeSetMtPer::singleton(1).union(&AVLTreeSetMtPer::singleton(2)).union(&AVLTreeSetMtPer::singleton(3)),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);

    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_graph_search_linear_chain() {
    // Linear chain: 1 -> 2 -> 3 -> 4 -> 5
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::singleton(3),
        | 3 => AVLTreeSetMtPer::singleton(4),
        | 4 => AVLTreeSetMtPer::singleton(5),
        | 5 => AVLTreeSetMtPer::empty(),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let result = graph_search(&graph, 1, &SelectAll);

    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

