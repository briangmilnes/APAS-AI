use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
use apas_ai::Chap59::JohnsonStEphFloat::JohnsonStEphFloat::johnson_apsp;
use apas_ai::SetLit;
use apas_ai::Types::Types::OrderedF64;

#[test]
fn test_simple_graph() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        (0, 1, OrderedF64::from(5.5)),
        (1, 2, OrderedF64::from(3.2)),
        (0, 2, OrderedF64::from(10.0))
    ];

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(0, 1), OrderedF64::from(5.5));
    assert_eq!(result.get_distance(0, 2), OrderedF64::from(8.7)); // via 1: 5.5 + 3.2 = 8.7 < 10.0

    assert_eq!(result.get_distance(1, 1), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(1, 2), OrderedF64::from(3.2));
    assert_eq!(result.get_distance(1, 0), OrderedF64::from(f64::INFINITY)); // unreachable
}

#[test]
fn test_negative_weights() {
    let vertices = SetLit![0, 1, 2];
    let edges = SetLit![
        (0, 1, OrderedF64::from(1.5)),
        (1, 2, OrderedF64::from(-0.8)),
        (0, 2, OrderedF64::from(1.0))
    ];

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.7)); // via 1: 1.5 + (-0.8) = 0.7 < 1.0
}

#[test]
fn test_single_vertex() {
    let vertices = SetLit![0];
    let edges = Set::empty();

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
}

#[test]
fn test_fractional_weights() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![
        (0, 1, OrderedF64::from(0.5)),
        (0, 2, OrderedF64::from(1.5)),
        (1, 2, OrderedF64::from(-0.25)),
        (1, 3, OrderedF64::from(1.0)),
        (2, 3, OrderedF64::from(0.5))
    ];

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
    assert_eq!(result.get_distance(0, 1), OrderedF64::from(0.5));
    assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.25)); // via 1: 0.5 + (-0.25) = 0.25 < 1.5
    assert_eq!(result.get_distance(0, 3), OrderedF64::from(0.75)); // via 2: 0.25 + 0.5 = 0.75
}

#[test]
fn test_disconnected_graph() {
    let vertices = SetLit![0, 1, 2, 3];
    let edges = SetLit![(0, 1, OrderedF64::from(2.5)), (2, 3, OrderedF64::from(1.8))];

    let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
    let result = johnson_apsp(&graph);

    assert_eq!(result.get_distance(0, 1), OrderedF64::from(2.5));
    assert_eq!(result.get_distance(2, 3), OrderedF64::from(1.8));
    assert_eq!(result.get_distance(0, 2), OrderedF64::from(f64::INFINITY));
    assert_eq!(result.get_distance(1, 3), OrderedF64::from(f64::INFINITY));
}
