//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap53 PQMinMtPer.

use apas_ai::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
use apas_ai::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_ai::Chap53::PQMinMtPer::PQMinMtPer::*;
use apas_ai::Types::Types::*;

// Simple priority function: vertex value itself as priority
fn vertex_priority() -> ClosurePriority<N, N, impl Fn(&N) -> N> {
    ClosurePriority::new(|v: &N| *v)
}

// Distance-based priority (for simulating shortest path)
fn distance_priority(distances: AVLTreeSetMtPer<Pair<N, N>>) -> impl PriorityFn<N, N> {
    ClosurePriority::new(move |v: &N| {
        let seq = distances.to_seq();
        for i in 0..seq.length() {
            let pair = seq.nth(i);
            if pair.0 == *v {
                return pair.1;
            }
        }
        999999 // Large value for unreachable
    })
}

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
fn test_pq_min_empty_graph() {
    let graph = |_: &N| AVLTreeSetMtPer::empty();
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);
    assert_eq!(result.visited.size(), 1);
    assert!(result.visited.find(&1));
}

#[test]
fn test_pq_min_single_edge() {
    let graph = |v: &N| {
        if *v == 1 {
            AVLTreeSetMtPer::singleton(2)
        } else {
            AVLTreeSetMtPer::empty()
        }
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}

#[test]
fn test_pq_min_dag() {
    let graph = test_graph_1();
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);
    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_priority_order() {
    // Graph: 1 -> {2, 3}, 2 -> {4}, 3 -> {5}
    // With vertex value as priority, lower values visited first
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(3)),
        | 2 => AVLTreeSetMtPer::singleton(4),
        | 3 => AVLTreeSetMtPer::singleton(5),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    // All reachable vertices should be visited
    assert_eq!(result.visited.size(), 5);

    // Check priorities were recorded
    assert_eq!(result.priorities.size(), 5);
}

#[test]
fn test_pq_min_multi_source() {
    let graph = test_graph_1();
    let sources = AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(5));
    let prio_fn = vertex_priority();
    let result = pq_min_multi(&graph, sources, &prio_fn);

    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&4));
    assert!(result.visited.find(&5));
}

#[test]
fn test_pq_min_disconnected_graph() {
    // Two disconnected components: 1 -> 2, 3 -> 4
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::empty(),
        | 3 => AVLTreeSetMtPer::singleton(4),
        | 4 => AVLTreeSetMtPer::empty(),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    // Only first component should be visited
    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(!result.visited.find(&3));
    assert!(!result.visited.find(&4));
}

#[test]
fn test_pq_min_cycle() {
    // Cycle: 1 -> 2 -> 3 -> 1
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::singleton(3),
        | 3 => AVLTreeSetMtPer::singleton(1),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    // All vertices in cycle should be visited exactly once
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_pq_min_custom_priority() {
    // Set up distances: 1->0, 2->5, 3->10
    let distances = AVLTreeSetMtPer::singleton(Pair(1, 0))
        .union(&AVLTreeSetMtPer::singleton(Pair(2, 5)))
        .union(&AVLTreeSetMtPer::singleton(Pair(3, 10)));

    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(3)),
        | _ => AVLTreeSetMtPer::empty(),
    };

    let prio_fn = distance_priority(distances);
    let result = pq_min(&graph, 1, &prio_fn);

    // All reachable vertices should be visited
    assert_eq!(result.visited.size(), 3);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
    assert!(result.visited.find(&3));
}

#[test]
fn test_pq_min_linear_chain() {
    // Linear chain: 1 -> 2 -> 3 -> 4 -> 5
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2),
        | 2 => AVLTreeSetMtPer::singleton(3),
        | 3 => AVLTreeSetMtPer::singleton(4),
        | 4 => AVLTreeSetMtPer::singleton(5),
        | 5 => AVLTreeSetMtPer::empty(),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_complete_graph() {
    // Complete graph on 4 vertices: each vertex connects to all others
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2)
            .union(&AVLTreeSetMtPer::singleton(3))
            .union(&AVLTreeSetMtPer::singleton(4)),
        | 2 => AVLTreeSetMtPer::singleton(1)
            .union(&AVLTreeSetMtPer::singleton(3))
            .union(&AVLTreeSetMtPer::singleton(4)),
        | 3 => AVLTreeSetMtPer::singleton(1)
            .union(&AVLTreeSetMtPer::singleton(2))
            .union(&AVLTreeSetMtPer::singleton(4)),
        | 4 => AVLTreeSetMtPer::singleton(1)
            .union(&AVLTreeSetMtPer::singleton(2))
            .union(&AVLTreeSetMtPer::singleton(3)),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 4);
    for i in 1..=4 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_star_graph() {
    // Star graph: 1 is center, connects to 2, 3, 4, 5
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2)
            .union(&AVLTreeSetMtPer::singleton(3))
            .union(&AVLTreeSetMtPer::singleton(4))
            .union(&AVLTreeSetMtPer::singleton(5)),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 5);
    for i in 1..=5 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_binary_tree() {
    // Binary tree: 1 -> {2, 3}, 2 -> {4, 5}, 3 -> {6, 7}
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(2).union(&AVLTreeSetMtPer::singleton(3)),
        | 2 => AVLTreeSetMtPer::singleton(4).union(&AVLTreeSetMtPer::singleton(5)),
        | 3 => AVLTreeSetMtPer::singleton(6).union(&AVLTreeSetMtPer::singleton(7)),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 7);
    for i in 1..=7 {
        assert!(result.visited.find(&i));
    }
}

#[test]
fn test_pq_min_self_loop() {
    // Graph with self-loop: 1 -> {1, 2}
    let graph = |v: &N| match *v {
        | 1 => AVLTreeSetMtPer::singleton(1).union(&AVLTreeSetMtPer::singleton(2)),
        | _ => AVLTreeSetMtPer::empty(),
    };
    let prio_fn = vertex_priority();
    let result = pq_min(&graph, 1, &prio_fn);

    assert_eq!(result.visited.size(), 2);
    assert!(result.visited.find(&1));
    assert!(result.visited.find(&2));
}
