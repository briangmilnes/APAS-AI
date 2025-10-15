//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 66: Borůvka's MST Algorithm (Sequential Ephemeral)

use ordered_float::OrderedFloat;

use apas_ai::Chap66::BoruvkaStEph::BoruvkaStEph::*;
use apas_ai::SetLit;

#[test]
fn test_boruvka_triangle() {
    // Triangle: 1-2 (w=3), 2-3 (w=2), 3-1 (w=1)
    // MST: edges 2 (3-1, w=1) and 1 (2-3, w=2), total weight = 3
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        (1, 2, OrderedFloat(3.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 1, OrderedFloat(1.0), 2),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 2);
    assert_eq!(mst_w, OrderedFloat(3.0));
    assert!(mst_labels.mem(&1));
    assert!(mst_labels.mem(&2));
}

#[test]
fn test_boruvka_square() {
    // Square: 1-2 (w=1), 2-3 (w=2), 3-4 (w=3), 4-1 (w=4)
    // MST: edges 0, 1, 2 (no diagonal)
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 4, OrderedFloat(3.0), 2),
        (4, 1, OrderedFloat(4.0), 3),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, OrderedFloat(6.0));
}

#[test]
fn test_boruvka_complete_4() {
    // Complete graph on 4 vertices with increasing weights
    // MST should have 3 edges with minimum total weight
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (1, 3, OrderedFloat(2.0), 1),
        (1, 4, OrderedFloat(3.0), 2),
        (2, 3, OrderedFloat(4.0), 3),
        (2, 4, OrderedFloat(5.0), 4),
        (3, 4, OrderedFloat(6.0), 5),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, OrderedFloat(6.0)); // 1+2+3 = 6
}

#[test]
fn test_boruvka_star() {
    // Star: center 0 connected to 1, 2, 3, 4 with equal weights
    // MST: all edges from center
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        (0, 1, OrderedFloat(1.0), 0),
        (0, 2, OrderedFloat(1.0), 1),
        (0, 3, OrderedFloat(1.0), 2),
        (0, 4, OrderedFloat(1.0), 3),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(4.0));
}

#[test]
fn test_boruvka_path() {
    // Path: 1-2-3-4-5 with weights 1, 2, 3, 4
    // MST: all edges (it's a tree already)
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 4, OrderedFloat(3.0), 2),
        (4, 5, OrderedFloat(4.0), 3),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(10.0)); // 1+2+3+4 = 10
}

#[test]
fn test_boruvka_single_vertex() {
    // Single vertex: no edges
    let vertices = SetLit![1];
    let edges = SetLit![];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);

    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_two_vertices() {
    // Two vertices with one edge
    let vertices = SetLit![1, 2];
    let edges = SetLit![(1, 2, OrderedFloat(5.0), 0),];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 1);
    assert_eq!(mst_w, OrderedFloat(5.0));
    assert!(mst_labels.mem(&0));
}

#[test]
fn test_boruvka_cycle_5() {
    // Cycle: 1-2-3-4-5-1 with weights 1, 2, 3, 4, 10
    // MST: omit the heaviest edge (label 4)
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 4, OrderedFloat(3.0), 2),
        (4, 5, OrderedFloat(4.0), 3),
        (5, 1, OrderedFloat(10.0), 4),
    ];

    let mst_labels = boruvka_mst_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(10.0)); // 1+2+3+4 = 10
    assert!(!mst_labels.mem(&4)); // heaviest edge not in MST
}
