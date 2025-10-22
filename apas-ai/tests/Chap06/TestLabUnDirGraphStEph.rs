//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_ai::Types::Types::*;
use apas_ai::LabUnDirGraphStEphLit;
use apas_ai::SetLit;

#[test]
fn test_labelled_undir_graph_empty() {
    let g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_edges().size(), 0);
    assert_eq!(format!("{g}"), "LabUnDirGraph(V: {}, E: {})");
}

#[test]
fn test_labelled_undir_graph_add_vertex() {
    let mut g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    g.add_vertex(1);
    g.add_vertex(2);

    assert_eq!(g.vertices().size(), 2);
    assert!(g.vertices().mem(&1));
    assert!(g.vertices().mem(&2));
    assert_eq!(g.labeled_edges().size(), 0);
}

#[test]
fn test_labelled_undir_graph_add_labeled_edge() {
    let mut g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    g.add_labeled_edge(1, 2, "edge12");
    g.add_labeled_edge(2, 3, "edge23");

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_edges().size(), 2);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1)); // Undirected - both directions
    assert!(g.has_edge(&2, &3));
    assert!(g.has_edge(&3, &2)); // Undirected - both directions
    assert!(!g.has_edge(&1, &3));

    assert_eq!(g.get_edge_label(&1, &2), Some(&"edge12"));
    assert_eq!(g.get_edge_label(&2, &1), Some(&"edge12")); // Undirected
    assert_eq!(g.get_edge_label(&2, &3), Some(&"edge23"));
    assert_eq!(g.get_edge_label(&3, &2), Some(&"edge23")); // Undirected
    assert_eq!(g.get_edge_label(&1, &3), None);
}

#[test]
fn test_labelled_undir_graph_neighbors() {
    let mut g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    g.add_labeled_edge(1, 2, "a");
    g.add_labeled_edge(1, 3, "b");
    g.add_labeled_edge(2, 3, "c");

    let neighbors_1 = g.neighbors(&1);
    assert_eq!(neighbors_1.size(), 2);
    assert!(neighbors_1.mem(&2));
    assert!(neighbors_1.mem(&3));

    let neighbors_2 = g.neighbors(&2);
    assert_eq!(neighbors_2.size(), 2);
    assert!(neighbors_2.mem(&1));
    assert!(neighbors_2.mem(&3));

    let neighbors_3 = g.neighbors(&3);
    assert_eq!(neighbors_3.size(), 2);
    assert!(neighbors_3.mem(&1));
    assert!(neighbors_3.mem(&2));
}

#[test]
fn test_labelled_undir_graph_edges() {
    let mut g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    g.add_labeled_edge(1, 2, "label");
    g.add_labeled_edge(2, 3, "another");

    let edges = g.edges();
    assert_eq!(edges.size(), 2);
    // Note: edges are normalized, so (1,2) and (2,3) should be stored as is
    // since 1 < 2 and 2 < 3
    assert!(edges.mem(&Edge(1, 2)));
    assert!(edges.mem(&Edge(2, 3)));
}

#[test]
fn test_labelled_undir_graph_macro_empty() {
    let g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEphLit!();
    assert_eq!(g.vertices().size(), 0);
    assert_eq!(g.labeled_edges().size(), 0);
}

#[test]
fn test_labelled_undir_graph_macro_with_data() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "first"), (2, 3, "second")]
    );

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_edges().size(), 2);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1)); // Undirected
    assert!(g.has_edge(&2, &3));
    assert!(g.has_edge(&3, &2)); // Undirected
    assert_eq!(g.get_edge_label(&1, &2), Some(&"first"));
    assert_eq!(g.get_edge_label(&2, &3), Some(&"second"));
}

#[test]
fn test_labelled_undir_graph_edge_normalization() {
    // Test that edges (2,1) get normalized to (1,2) since 1 < 2
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2],
        E: [(2, 1, "test")] // This should be normalized to (1, 2)
    );

    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1)); // Both directions should work
    assert_eq!(g.get_edge_label(&1, &2), Some(&"test"));
    assert_eq!(g.get_edge_label(&2, &1), Some(&"test"));
}

#[test]
fn test_labelled_undir_graph_different_label_types() {
    // Test with integer labels
    let g1 = LabUnDirGraphStEphLit!(
        V: ["a", "b"],
        E: [("a", "b", 42)]
    );
    assert_eq!(g1.get_edge_label(&"a", &"b"), Some(&42));
    assert_eq!(g1.get_edge_label(&"b", &"a"), Some(&42)); // Undirected

    // Test with string labels
    let g2 = LabUnDirGraphStEphLit!(
        V: [1, 2],
        E: [(1, 2, "hello")]
    );
    assert_eq!(g2.get_edge_label(&1, &2), Some(&"hello"));
    assert_eq!(g2.get_edge_label(&2, &1), Some(&"hello")); // Undirected
}

#[test]
fn test_labelled_undir_graph_display() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2],
        E: [(1, 2, "test")]
    );
    let display_str = format!("{g}");
    assert!(display_str.contains("LabUnDirGraph"));
    assert!(display_str.contains("V:"));
    assert!(display_str.contains("E:"));
}

#[test]
fn test_labelled_undir_graph_debug() {
    let g = LabUnDirGraphStEphLit!(
        V: [1],
        E: [(1, 1, "self")]
    );
    let debug_str = format!("{g:?}");
    assert!(debug_str.contains("LabUnDirGraph"));
    assert!(debug_str.contains("vertices"));
    assert!(debug_str.contains("labeled_edges"));
}

#[test]
fn test_labelled_undir_graph_self_loop() {
    let mut g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    g.add_labeled_edge(1, 1, "self_loop");

    assert!(g.has_edge(&1, &1));
    assert_eq!(g.get_edge_label(&1, &1), Some(&"self_loop"));

    let neighbors = g.neighbors(&1);
    assert_eq!(neighbors.size(), 1);
    assert!(neighbors.mem(&1));
}

#[test]
fn test_labelled_undir_graph_multiple_edges_same_vertices() {
    // This test checks if we can have multiple edges between same vertices
    // Current implementation allows multiple labeled edges between same vertices
    let mut g: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEph::empty();
    g.add_labeled_edge(1, 2, "first");
    g.add_labeled_edge(1, 2, "second"); // This creates a second labeled edge

    assert_eq!(g.labeled_edges().size(), 2);
    // Both edges should exist in the graph
    assert!(g.labeled_edges().mem(&LabEdge(1, 2, "first")));
    assert!(g.labeled_edges().mem(&LabEdge(1, 2, "second")));
}

#[test]
fn test_sizea() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "a"), (2, 3, "b")]
    );
    assert_eq!(g.labeled_edges().size(), 2);
}

#[test]
fn test_arcs() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "first"), (2, 3, "second")]
    );
    let arcs = g.labeled_edges();
    assert_eq!(arcs.size(), 2);
    assert!(arcs.mem(&LabEdge(1, 2, "first")));
    assert!(arcs.mem(&LabEdge(2, 3, "second")));
}

#[test]
fn test_nplus() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "a")]
    );
    let nplus = g.neighbors(&1);
    assert_eq!(nplus.size(), 1);
    assert!(nplus.mem(&2));
}

#[test]
fn test_nminus() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "a")]
    );
    let nminus = g.neighbors(&2);
    assert_eq!(nminus.size(), 1);
    assert!(nminus.mem(&1));
}

#[test]
fn test_indegree() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "a"), (2, 3, "b")]
    );
    assert_eq!(g.neighbors(&2).size(), 2);
    assert_eq!(g.neighbors(&1).size(), 1);
    assert_eq!(g.neighbors(&3).size(), 1);
}

#[test]
fn test_outdegree() {
    let g = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "a"), (2, 3, "b")]
    );
    assert_eq!(g.neighbors(&2).size(), 2);
    assert_eq!(g.neighbors(&1).size(), 1);
    assert_eq!(g.neighbors(&3).size(), 1);
}

#[test]
fn test_from_vertices_and_labeled_edges() {
    use apas_ai::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;

    let v: SetStEph<i32> = SetLit![1, 2, 3];
    let e: SetStEph<LabEdge<i32, &str>> = SetLit![LabEdge(1, 2, "edge")];
    let g =
        <LabUnDirGraphStEph<i32, &str> as LabUnDirGraphStEphTrait<i32, &str>>::from_vertices_and_labeled_edges(v, e);

    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_edges().size(), 1);
}

#[test]
fn test_clone_graph() {
    let g1 = LabUnDirGraphStEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "test")]
    );

    let g2 = g1.clone();
    assert_eq!(g2.vertices().size(), g1.vertices().size());
    assert_eq!(g2.labeled_edges().size(), g1.labeled_edges().size());
    assert!(g2.has_edge(&1, &2));
}
