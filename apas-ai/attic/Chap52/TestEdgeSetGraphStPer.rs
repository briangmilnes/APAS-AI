//! Tests for Edge Set Graph - Persistent Single-Threaded

use apas_ai::Chap52::EdgeSetGraphStPer::EdgeSetGraphStPer::*;

#[test]
fn test_empty_graph() {
    let g: EdgeSetGraphStPerS<usize> = EdgeSetGraphStPerS::new();
    assert_eq!(g.vertex_count(), 0);
    assert_eq!(g.edge_count(), 0);
    assert!(g.is_empty());
}

#[test]
fn test_insert_vertices() {
    let g = EdgeSetGraphStPerS::new()
        .insert_vertex(1usize)
        .insert_vertex(2usize)
        .insert_vertex(3usize);
    
    assert_eq!(g.vertex_count(), 3);
    assert_eq!(g.edge_count(), 0);
    assert!(!g.is_empty());
}

#[test]
fn test_insert_edges() {
    let g = EdgeSetGraphStPerS::new()
        .insert_vertex(1usize)
        .insert_vertex(2usize)
        .insert_vertex(3usize)
        .insert_edge(1, 2)
        .insert_edge(2, 3);
    
    assert_eq!(g.vertex_count(), 3);
    assert_eq!(g.edge_count(), 2);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &3));
    assert!(!g.has_edge(&1, &3));
}

#[test]
fn test_directed_graph() {
    let g = EdgeSetGraphStPerS::new()
        .insert_vertex(1usize)
        .insert_vertex(2usize)
        .insert_edge(1, 2);
    
    assert!(g.has_edge(&1, &2));
    assert!(!g.has_edge(&2, &1)); // directed: reverse edge not present
}

#[test]
fn test_persistent_behavior() {
    let g1 = EdgeSetGraphStPerS::new()
        .insert_vertex(1usize)
        .insert_vertex(2usize);
    
    let g2 = g1.insert_edge(1, 2);
    
    // g1 unchanged (persistent)
    assert_eq!(g1.vertex_count(), 2);
    assert_eq!(g1.edge_count(), 0);
    
    // g2 has new edge
    assert_eq!(g2.vertex_count(), 2);
    assert_eq!(g2.edge_count(), 1);
}

#[test]
fn test_display() {
    let g = EdgeSetGraphStPerS::new()
        .insert_vertex(1usize)
        .insert_vertex(2usize)
        .insert_edge(1, 2);
    
    let display_str = format!("{}", g);
    assert!(display_str.contains("EdgeSetGraph"));
    assert!(display_str.contains("v=2"));
    assert!(display_str.contains("e=1"));
}
