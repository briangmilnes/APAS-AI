//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chap06 WeightedUnDirGraphStEphFloat.

use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait;
use apas_ai::Chap06::WeightedUnDirGraphStEphFloat::WeightedUnDirGraphStEphFloat::*;
use apas_ai::SetLit;
use apas_ai::WeightedUnDirGraphStEphFloatLit;
use ordered_float::OrderedFloat;

#[test]
fn test_weightedundirgraphstephfloatlit_macro_functionality() {
    // Test empty graph creation
    let empty: WeightedUnDirGraphStEphFloat<i32> = WeightedUnDirGraphStEphFloatLit!();
    assert_eq!(empty.vertices().size(), 0);

    // Test graph creation with weighted edges
    let with_data = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [(1, 2, 1.5), (2, 3, 2.5), (3, 1, 3.5)]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.edges().size(), 3);
}

#[test]
fn test_create_empty_graph() {
    let graph: WeightedUnDirGraphStEphFloat<i32> = WeightedUnDirGraphStEphFloat::empty();
    assert_eq!(graph.vertices().size(), 0);
    assert_eq!(graph.edges().size(), 0);
}

#[test]
fn test_add_vertices_and_edges() {
    let mut graph = WeightedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_vertex(3);
    assert_eq!(graph.vertices().size(), 3);

    graph.add_weighted_edge(1, 2, OrderedFloat(3.14));
    graph.add_weighted_edge(2, 3, OrderedFloat(2.71));
    assert_eq!(graph.edges().size(), 2);
}

#[test]
fn test_get_edge_weight() {
    let mut graph = WeightedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, OrderedFloat(42.5));

    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(42.5)));
    assert_eq!(graph.get_edge_weight(&2, &1), Some(OrderedFloat(42.5))); // Undirected
    assert_eq!(graph.get_edge_weight(&1, &3), None);
}

#[test]
fn test_weighted_edges() {
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [(1, 2, 5.5), (2, 3, 10.5)]
    );

    let edges = graph.weighted_edges();
    assert_eq!(edges.size(), 2);
}

#[test]
fn test_neighbors_weighted() {
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [(1, 2, 5.5), (1, 3, 10.5), (2, 4, 15.5)]
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
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [(1, 2, 10.0), (2, 3, 20.0), (3, 1, 30.0)]
    );

    assert!((graph.total_weight().0 - 60.0).abs() < 0.001);
}

#[test]
fn test_vertex_degree() {
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [(1, 2, 5.0), (1, 3, 10.0), (1, 4, 15.0)]
    );

    assert_eq!(graph.vertex_degree(&1), 3);
    assert_eq!(graph.vertex_degree(&2), 1);
    assert_eq!(graph.vertex_degree(&3), 1);
    assert_eq!(graph.vertex_degree(&4), 1);
}

#[test]
fn test_is_connected_single_vertex() {
    let graph = WeightedUnDirGraphStEphFloatLit!(V: [1], E: []);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_two_vertices() {
    let graph = WeightedUnDirGraphStEphFloatLit!(V: [1, 2], E: [(1, 2, 10.0)]);
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_disconnected() {
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![(1, 2, OrderedFloat(5.0)), (3, 4, OrderedFloat(10.0))];
    let graph = WeightedUnDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    assert!(!graph.is_connected());
}

#[test]
fn test_is_connected_fully_connected() {
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3],
        E: [(1, 2, 5.0), (2, 3, 10.0), (3, 1, 15.0)]
    );
    assert!(graph.is_connected());
}

#[test]
fn test_is_connected_empty_graph() {
    let graph: WeightedUnDirGraphStEphFloat<i32> = WeightedUnDirGraphStEphFloatLit!();
    assert!(graph.is_connected()); // Empty graph is considered connected
}

#[test]
fn test_from_weighted_edges() {
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![(1, 2, OrderedFloat(10.5)), (2, 3, OrderedFloat(20.5))];
    let graph = WeightedUnDirGraphStEphFloat::from_weighted_edges(vertices, edges);

    assert_eq!(graph.vertices().size(), 3);
    assert_eq!(graph.edges().size(), 2);
    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(10.5)));
    assert_eq!(graph.get_edge_weight(&2, &3), Some(OrderedFloat(20.5)));
}

#[test]
fn test_zero_weight_edge() {
    let mut graph = WeightedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, OrderedFloat(0.0));

    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(0.0)));
}

#[test]
fn test_negative_weight_edge() {
    let mut graph = WeightedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, OrderedFloat(-10.5));

    assert_eq!(graph.get_edge_weight(&1, &2), Some(OrderedFloat(-10.5)));
}

#[test]
fn test_fractional_weights() {
    let mut graph = WeightedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, OrderedFloat(3.14159));

    let weight = graph.get_edge_weight(&1, &2).unwrap();
    assert!((weight.0 - 3.14159).abs() < 0.00001);
}

#[test]
fn test_min_weight_edge() {
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [(1, 2, 5.0), (2, 3, 2.0), (3, 4, 8.0)]
    );

    let min_edge = graph.min_weight_edge().unwrap();
    assert_eq!(min_edge.2, OrderedFloat(2.0));
}

#[test]
fn test_max_weight_edge() {
    let graph = WeightedUnDirGraphStEphFloatLit!(
        V: [1, 2, 3, 4],
        E: [(1, 2, 5.0), (2, 3, 2.0), (3, 4, 8.0)]
    );

    let max_edge = graph.max_weight_edge().unwrap();
    assert_eq!(max_edge.2, OrderedFloat(8.0));
}

#[test]
fn test_min_max_weight_edge_empty() {
    let graph: WeightedUnDirGraphStEphFloat<i32> = WeightedUnDirGraphStEphFloatLit!();
    assert_eq!(graph.min_weight_edge(), None);
    assert_eq!(graph.max_weight_edge(), None);
}

#[test]
fn test_undirected_edge_symmetry() {
    let mut graph = WeightedUnDirGraphStEphFloat::empty();
    graph.add_vertex(1);
    graph.add_vertex(2);
    graph.add_weighted_edge(1, 2, OrderedFloat(42.5));

    // Should be accessible from both directions with same weight
    assert_eq!(graph.get_edge_weight(&1, &2), graph.get_edge_weight(&2, &1));
}

