//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: TSP 2-Approximation Tests (Parallel)

use apas_ai::{
    Chap05::SetStEph::SetStEph::*, Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*,
    Chap64::TSPApproxMtEph::TSPApproxMtEph::*, SetLit, Types::Types::*,
};
use ordered_float::OrderedFloat;

fn create_triangle_graph() -> (
    LabUnDirGraphMtEph<N, OrderedFloat<f64>>,
    Set<LabEdge<N, OrderedFloat<f64>>>,
) {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        LabEdge(0, 1, OrderedFloat(1.0)),
        LabEdge(1, 2, OrderedFloat(2.0)),
        LabEdge(2, 0, OrderedFloat(3.0))
    ];
    let graph = <LabUnDirGraphMtEph<N, OrderedFloat<f64>> as LabUnDirGraphMtEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges.clone());

    let spanning_tree = SetLit![LabEdge(0, 1, OrderedFloat(1.0)), LabEdge(1, 2, OrderedFloat(2.0))];

    (graph, spanning_tree)
}

#[test]
fn test_euler_tour_mt() {
    let (graph, tree) = create_triangle_graph();
    let tour = euler_tour_mt(&graph, &0, &tree);

    assert!(tour.len() >= 3);
    assert_eq!(tour[0], 0);
}

#[test]
fn test_shortcut_tour_mt() {
    let tour_with_dups = std::vec![0, 1, 2, 1, 0];
    let shortcut = shortcut_tour_mt(&tour_with_dups);

    assert!(shortcut.contains(&0));
    assert!(shortcut.contains(&1));
    assert!(shortcut.contains(&2));
}

#[test]
fn test_approx_metric_tsp_mt() {
    let (graph, tree) = create_triangle_graph();
    let (tour, weight) = approx_metric_tsp_mt(&graph, &tree, &0);

    assert!(tour.len() >= 3);
    assert_eq!(tour[0], tour[tour.len() - 1]);
    assert!(weight > OrderedFloat(0.0));
}
