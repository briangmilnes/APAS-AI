//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap52 AdjMatrixGraphStEph.

use apas_ai::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_ai::Chap52::AdjMatrixGraphStEph::AdjMatrixGraphStEph::*;
use apas_ai::Types::Types::*;

#[test]
fn test_new() {
    let graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(5);
    assert_eq!(graph.num_vertices(), 5);
}

#[test]
fn test_has_edge() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(3);
    graph.set_edge(0, 1, true);
    assert!(graph.has_edge(0, 1));
    assert!(!graph.has_edge(1, 0));
}

#[test]
fn test_out_neighbors() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(0, 2, true);
    let neighbors = graph.out_neighbors(0);
    assert_eq!(neighbors.length(), 2);
}

#[test]
fn test_out_degree() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(0, 2, true);
    assert_eq!(graph.out_degree(0), 2);
}

#[test]
fn test_set_edge() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(3);
    graph.set_edge(0, 1, true);
    assert!(graph.has_edge(0, 1));
    graph.set_edge(0, 1, false);
    assert!(!graph.has_edge(0, 1));
}

#[test]
fn test_complement() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(3);
    graph.set_edge(0, 1, true);
    let comp = graph.complement();
    assert!(!comp.has_edge(0, 1));
}

#[test]
fn test_num_edges() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(4);
    graph.set_edge(0, 1, true);
    graph.set_edge(1, 2, true);
    assert_eq!(graph.num_edges(), 2);
}

#[test]
fn test_self_loop() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(3);
    graph.set_edge(1, 1, true);
    assert!(graph.has_edge(1, 1));
}

#[test]
fn test_complete_graph() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(4);
    for i in 0..4 {
        for j in 0..4 {
            if i != j {
                graph.set_edge(i, j, true);
            }
        }
    }
    assert_eq!(graph.num_edges(), 12);
}

#[test]
fn test_empty_graph() {
    let graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(5);
    assert_eq!(graph.num_edges(), 0);
}

#[test]
fn test_num_edges_large() {
    let mut graph: AdjMatrixGraphStEph = AdjMatrixGraphStEphTrait::new(5);
    for i in 0..5 {
        for j in 0..5 {
            if i != j {
                graph.set_edge(i, j, true);
            }
        }
    }
    assert_eq!(graph.num_edges(), 20);
}
