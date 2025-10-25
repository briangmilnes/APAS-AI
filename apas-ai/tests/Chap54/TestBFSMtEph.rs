//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_ai::ArraySeqMtEphSLit;
use apas_ai::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_ai::Chap54::BFSMtEph::*;
use apas_ai::Types::Types::*;

const UNREACHABLE: usize = usize::MAX;

#[test]
fn test_empty_graph() {
    let graph: ArraySeqMtEphS<ArraySeqMtEphS<usize>> = ArraySeqMtEphSLit![];
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.length(), 0);
}

#[test]
fn test_single_vertex() {
    let graph = ArraySeqMtEphS::from_vec(vec![ArraySeqMtEphSLit![]]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.nth_cloned(0), 0);
}

#[test]
fn test_line_graph() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphSLit![1],
        ArraySeqMtEphSLit![2],
        ArraySeqMtEphSLit![3],
        ArraySeqMtEphSLit![],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.nth_cloned(0), 0);
    assert_eq!(distances.nth_cloned(1), 1);
    assert_eq!(distances.nth_cloned(2), 2);
    assert_eq!(distances.nth_cloned(3), 3);
}

#[test]
fn test_dag() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphSLit![1, 2],
        ArraySeqMtEphSLit![2, 3, 4],
        ArraySeqMtEphSLit![4],
        ArraySeqMtEphSLit![5, 6],
        ArraySeqMtEphSLit![0, 4, 6],
        ArraySeqMtEphSLit![],
        ArraySeqMtEphSLit![],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.nth_cloned(0), 0);
    assert_eq!(distances.nth_cloned(1), 1);
    assert_eq!(distances.nth_cloned(2), 1);
    assert_eq!(distances.nth_cloned(3), 2);
    assert_eq!(distances.nth_cloned(4), 2);
    assert_eq!(distances.nth_cloned(5), 3);
    assert_eq!(distances.nth_cloned(6), 3);
}

#[test]
fn test_unreachable() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphSLit![1],
        ArraySeqMtEphSLit![],
        ArraySeqMtEphSLit![3],
        ArraySeqMtEphSLit![],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.nth_cloned(0), 0);
    assert_eq!(distances.nth_cloned(1), 1);
    assert_eq!(distances.nth_cloned(2), UNREACHABLE);
    assert_eq!(distances.nth_cloned(3), UNREACHABLE);
}

#[test]
fn test_cycle() {
    let graph = ArraySeqMtEphS::from_vec(vec![
        ArraySeqMtEphSLit![1],
        ArraySeqMtEphSLit![2],
        ArraySeqMtEphSLit![0],
    ]);
    let distances = BFSMtEph::bfs(&graph, 0);
    assert_eq!(distances.nth_cloned(0), 0);
    assert_eq!(distances.nth_cloned(1), 1);
    assert_eq!(distances.nth_cloned(2), 2);
}

#[test]
fn test_invalid_source() {
    let graph = ArraySeqMtEphS::from_vec(vec![ArraySeqMtEphSLit![1], ArraySeqMtEphSLit![]]);
    let distances = BFSMtEph::bfs(&graph, 5);
    assert_eq!(distances.length(), 2);
    assert_eq!(distances.nth_cloned(0), UNREACHABLE);
    assert_eq!(distances.nth_cloned(1), UNREACHABLE);
}
