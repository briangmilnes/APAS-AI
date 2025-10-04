//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap06 WeightedUnDirGraphStEphInt.

use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait;
use apas_ai::Chap06::WeightedUnDirGraphStEphInt::WeightedUnDirGraphStEphInt::*;
use apas_ai::SetLit;
use apas_ai::WeightedUnDirGraphStEphIntLit;

#[test]
fn test_weightedundirgraphstephintlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeightedUnDirGraphStEphInt<i32> = WeightedUnDirGraphStEphIntLit!();
    assert_eq!(empty.vertices().size(), 0);

    // Test graph creation with weighted edges
    let with_data = WeightedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [(1, 2, 10), (2, 3, 20), (3, 1, 30)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.edges().size(), 3);
}

#[test]
fn test_create_empty_graph() {
    let graph: WeightedUnDirGraphStEphInt<i32> = WeightedUnDirGraphStEphInt::empty();
    assert_eq!(graph.vertices().size(), 0);
    assert_eq!(graph.edges().size(), 0);
}

#[test]
fn test_add_vertices_and_edges() {
    let mut graph = WeightedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    assert_eq!(graph.vertices().size(), 3);

    graph.add_weighted_edge(1, 2, 10);
    graph.add_weighted_edge(2, 3, 20);
    assert_eq!(graph.edges().size(), 2);
}

#[test]
fn test_get_edge_weight() {
    let mut graph = WeightedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, 42);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(42));
    assert_eq!(graph.get_edge_weight(&2, &1), Some(42)); // Undirected
    assert_eq!(graph.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighted_edges() {
    let graph = WeightedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [(1, 2, 5), (2, 3, 10)]
    );

    let edges = graph.weighted_edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_neighbors_weighted() {
    let graph = WeightedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [(1, 2, 5), (1, 3, 10), (2, 4, 15)]
    );

    let neighbors_1 = graph.neighbors_weighted(&1);
    assert_eq!(neighbors_1.size(), 2);

    let neighbors_2 = graph.neighbors_weighted(&2);
    assert_eq!(neighbors_2.size(), 2);

    let neighbors_4 = graph.neighbors_weighted(&4);
    assert_eq!(neighbors_4.size(), 1);
}

#[test]
fn test_total_weight() {
    let graph = WeightedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [(1, 2, 10), (2, 3, 20), (3, 1, 30)]
    );

    assert_eq!(graph.total_weight(), 60);
}

#[test]
fn test_vertex_degree() {
    let graph = WeightedUnDirGraphStEphIntLit!(
        V: [1, 2, 3, 4],
        E: [(1, 2, 5), (1, 3, 10), (1, 4, 15)]
    );

    assert_eq!(graph.vertex_degree(&1), 3);
    assert_eq!(graph.vertex_degree(&2), 1);
    assert_eq!(graph.vertex_degree(&3), 1);
    assert_eq!(graph.vertex_degree(&4), 1);
}

#[test]
fn test_is_connected_single_vertex() {
    let graph = WeightedUnDirGraphStEphIntLit!(V: [1], E: []);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_two_vertices() {
    let graph = WeightedUnDirGraphStEphIntLit!(V: [1, 2], E: [(1, 2, 10)]);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_disconnected() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![(1, 2, 5), (3, 4, 10)];
    let graph = WeightedUnDirGraphStEphInt::from_weighted_edges(vertices, edges);
    assert!(!graph.is_connected());
}

#[test]
fn test_is_connected_fully_connected() {
    let graph = WeightedUnDirGraphStEphIntLit!(
        V: [1, 2, 3],
        E: [(1, 2, 5), (2, 3, 10), (3, 1, 15)]
    );
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_empty_graph() {
    let graph: WeightedUnDirGraphStEphInt<i32> = WeightedUnDirGraphStEphIntLit!();
    assert!(graph.is_connected()); // Empty graph is considered connected
}

#[test]
fn test_from_weighted_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![(1, 2, 10), (2, 3, 20)];
    let graph = WeightedUnDirGraphStEphInt::from_weighted_edges(vertices, edges);

    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 2);
    assert_eq!(graph.get_edge_weight(&1, &2), Some(10));
    assert_eq!(graph.get_edge_weight(&2, &3), Some(20));
}

#[test]
fn test_zero_weight_edge() {
    let mut graph = WeightedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, 0);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(0));
}

#[test]
fn test_negative_weight_edge() {
    let mut graph = WeightedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, -10);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(-10));
}

#[test]
fn test_large_weights() {
    let mut graph = WeightedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, i32::MAX);

    assert_eq!(graph.get_edge_weight(&1, &2), Some(i32::MAX));
}

#[test]
fn test_undirected_edge_symmetry() {
    let mut graph = WeightedUnDirGraphStEphInt::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, 42);

    // Should be accessible from both directions with same weight
    assert_eq!(graph.get_edge_weight(&1, &2), graph.get_edge_weight(&2, &1));
}

