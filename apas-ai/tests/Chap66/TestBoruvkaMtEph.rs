//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for Chapter 66: Borůvka's MST Algorithm (Parallel Ephemeral)

use ordered_float::OrderedFloat;

use apas_ai::Chap66::BoruvkaMtEph::BoruvkaMtEph::*;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;

#[test]
fn test_boruvka_mt_triangle() {
    // Triangle: 1-2 (w=3), 2-3 (w=2), 3-1 (w=1)
    // MST: edges 2 (3-1, w=1) and 1 (2-3, w=2), total weight = 3
    let vertices = SetLit![1, 2, 3];
    let edges = SetLit![
        (1, 2, OrderedFloat(3.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 1, OrderedFloat(1.0), 2),
    ];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 2);
    assert_eq!(mst_w, OrderedFloat(3.0));
    assert!(mst_labels.mem(&1));
    assert!(mst_labels.mem(&2));
}

#[test]
fn test_boruvka_mt_square() {
    // Square: 1-2 (w=1), 2-3 (w=2), 3-4 (w=3), 4-1 (w=4)
    // MST: edges 0, 1, 2 (no diagonal)
    let vertices = SetLit![1, 2, 3, 4];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 4, OrderedFloat(3.0), 2),
        (4, 1, OrderedFloat(4.0), 3),
    ];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, OrderedFloat(6.0));
}

#[test]
fn test_boruvka_mt_complete_4() {
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

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 3);
    assert_eq!(mst_w, OrderedFloat(6.0)); // 1+2+3 = 6
}

#[test]
fn test_boruvka_mt_star() {
    // Star: center 0 connected to 1, 2, 3, 4 with equal weights
    // MST: all edges from center
    let vertices = SetLit![0, 1, 2, 3, 4];
    let edges = SetLit![
        (0, 1, OrderedFloat(1.0), 0),
        (0, 2, OrderedFloat(1.0), 1),
        (0, 3, OrderedFloat(1.0), 2),
        (0, 4, OrderedFloat(1.0), 3),
    ];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(4.0));
}

#[test]
fn test_boruvka_mt_path() {
    // Path: 1-2-3-4-5 with weights 1, 2, 3, 4
    // MST: all edges (it's a tree already)
    let vertices = SetLit![1, 2, 3, 4, 5];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 4, OrderedFloat(3.0), 2),
        (4, 5, OrderedFloat(4.0), 3),
    ];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(10.0)); // 1+2+3+4 = 10
}

#[test]
fn test_boruvka_mt_single_vertex() {
    // Single vertex: no edges
    let vertices = SetLit![1];
    let edges = SetLit![];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);

    assert_eq!(mst_labels.size(), 0);
}

#[test]
fn test_boruvka_mt_two_vertices() {
    // Two vertices with one edge
    let vertices = SetLit![1, 2];
    let edges = SetLit![(1, 2, OrderedFloat(5.0), 0),];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 1);
    assert_eq!(mst_w, OrderedFloat(5.0));
    assert!(mst_labels.mem(&0));
}

#[test]
fn test_boruvka_mt_cycle_5() {
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

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 4);
    assert_eq!(mst_w, OrderedFloat(10.0)); // 1+2+3+4 = 10
    assert!(!mst_labels.mem(&4)); // heaviest edge not in MST
}

#[test]
fn test_boruvka_mt_larger_graph() {
    // Larger graph: 8 vertices with specific structure
    let vertices = SetLit![1, 2, 3, 4, 5, 6, 7, 8];
    let edges = SetLit![
        (1, 2, OrderedFloat(1.0), 0),
        (2, 3, OrderedFloat(2.0), 1),
        (3, 4, OrderedFloat(3.0), 2),
        (4, 5, OrderedFloat(4.0), 3),
        (5, 6, OrderedFloat(5.0), 4),
        (6, 7, OrderedFloat(6.0), 5),
        (7, 8, OrderedFloat(7.0), 6),
        (8, 1, OrderedFloat(8.0), 7),
        (1, 5, OrderedFloat(9.0), 8),
        (2, 6, OrderedFloat(10.0), 9),
    ];

    let mst_labels = boruvka_mst_mt_with_seed(&vertices, &edges, 42);
    let mst_w = mst_weight(&edges, &mst_labels);

    assert_eq!(mst_labels.size(), 7); // n-1 edges for n vertices
    assert!(mst_w < OrderedFloat(40.0)); // Should be less than sum of all edges
}
