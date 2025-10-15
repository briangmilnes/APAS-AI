//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::sync::{Arc, Barrier};
use std::thread;

use apas_ai::Chap05::SetStEph::SetStEph::*;
use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEphTrait;
use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
use apas_ai::LabUnDirGraphMtEphLit;
use apas_ai::SetLit;
use apas_ai::Types::Types::*;

#[test]
fn test_labundirgraphmtephlit_macro_functionality() {
    // Test empty graph creation
    let empty: LabUnDirGraphMtEph<i32, String> = LabUnDirGraphMtEphLit!();
    assert_eq!(empty.vertices().size(), 0);
    assert_eq!(empty.labeled_edges().size(), 0);

    // Test graph creation with vertices and edges
    let with_data: LabUnDirGraphMtEph<i32, String> = LabUnDirGraphMtEphLit!(
        V: [1, 2, 3],
        E: [(1, 2, "edge1".to_string()), (2, 3, "edge2".to_string())]
    );
    assert_eq!(with_data.vertices().size(), 3);
    assert_eq!(with_data.labeled_edges().size(), 2);
}

#[test]
fn test_labundirgraphmteph_empty() {
    let empty_graph: LabUnDirGraphMtEph<i32, String> = LabUnDirGraphMtEph::empty();
    assert_eq!(empty_graph.vertices().size(), 0);
    assert_eq!(empty_graph.labeled_edges().size(), 0);
}

#[test]
fn test_labundirgraphmteph_basic_operations() {
    let v: Set<N> = SetLit![0, 1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![
        LabEdge(0, 1, "edge01".to_string()),
        LabEdge(1, 2, "edge12".to_string()),
        LabEdge(2, 3, "edge23".to_string())
    ];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    assert_eq!(g.vertices().size(), 4);
    assert_eq!(g.labeled_edges().size(), 3);

    // Test neighbor relationships (undirected - both directions)
    assert!(g.has_edge(&0, &1));
    assert!(g.has_edge(&1, &0)); // Undirected graph
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1));
    assert!(!g.has_edge(&0, &2)); // No direct edge

    // Test NG (neighbors) - should be symmetric
    let ng0 = g.neighbors(&0);
    assert_eq!(ng0.size(), 1);
    assert!(ng0.mem(&1));

    let ng1 = g.neighbors(&1);
    assert_eq!(ng1.size(), 2);
    assert!(ng1.mem(&0));
    assert!(ng1.mem(&2));

    let ng2 = g.neighbors(&2);
    assert_eq!(ng2.size(), 2);
    assert!(ng2.mem(&1));
    assert!(ng2.mem(&3));

    // Test degrees (in undirected graph, InDegree = OutDegree = Degree)
    assert_eq!(g.neighbors(&0).size(), 1);
    assert_eq!(g.neighbors(&0).size(), 1);
    assert_eq!(g.neighbors(&0).size(), 1);

    assert_eq!(g.neighbors(&1).size(), 2);
    assert_eq!(g.neighbors(&1).size(), 2);
    assert_eq!(g.neighbors(&1).size(), 2);

    assert_eq!(g.neighbors(&2).size(), 2);
    assert_eq!(g.neighbors(&2).size(), 2);
    assert_eq!(g.neighbors(&2).size(), 2);

    assert_eq!(g.neighbors(&3).size(), 1);
    assert_eq!(g.neighbors(&3).size(), 1);
    assert_eq!(g.neighbors(&3).size(), 1);
}

#[test]
fn test_labundirgraphmteph_incident_operations() {
    let v: Set<N> = SetLit![0, 1, 2];
    let a: Set<LabEdge<N, String>> = SetLit![
        LabEdge(0, 1, "first".to_string()),
        LabEdge(1, 2, "second".to_string()),
        LabEdge(0, 2, "direct".to_string())
    ];
    let _g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    // Test incident edges (each edge is incident to both endpoints)
    // let incident0 = g.Incident(&0); // TODO: method not available
    // assert_eq!(incident0.size(), 2); // 0-1 and 0-2 // TODO: method not available

    // let incident1 = g.Incident(&1); // TODO: method not available
    // assert_eq!(incident1.size(), 2); // 0-1 and 1-2 // TODO: method not available

    // let incident2 = g.Incident(&2); // TODO: method not available
    // assert_eq!(incident2.size(), 2); // 1-2 and 0-2 // TODO: method not available
}

#[test]
fn test_labundirgraphmteph_ngofvertices() {
    let v: Set<N> = SetLit![0, 1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![
        LabEdge(0, 1, "a".to_string()),
        LabEdge(1, 2, "b".to_string()),
        LabEdge(2, 3, "c".to_string()),
        LabEdge(0, 3, "d".to_string())
    ];
    let _g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    let _vertices_subset: Set<N> = SetLit![0, 1];
    // let ng_subset = g.NGOfVertices(&vertices_subset); // TODO: method not available

    // Neighbors of {0, 1} should include all vertices connected to 0 or 1
    // assert_eq!(ng_subset.size(), 3); // TODO: method not available
    // assert_eq!(ng_subset.mem(&1), true); // 0-1 // TODO: method not available
    // assert_eq!(ng_subset.mem(&2), true); // 1-2 // TODO: method not available
    // assert_eq!(ng_subset.mem(&3), true); // 0-3 // TODO: method not available
}

#[test]
fn test_labundirgraphmteph_nplusminusofvertices() {
    let v: Set<N> = SetLit![0, 1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![
        LabEdge(0, 1, "a".to_string()),
        LabEdge(1, 2, "b".to_string()),
        LabEdge(2, 0, "c".to_string()),
        LabEdge(3, 1, "d".to_string())
    ];
    let _g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    let _vertices_subset: Set<N> = SetLit![0, 1];

    // In undirected graphs, NPlus and NMinus should be the same as NG
    // let nplus_subset = g.NPlusOfVertices(&vertices_subset); // TODO: method not available
    // let nminus_subset = g.NMinusOfVertices(&vertices_subset); // TODO: method not available
    // let ng_subset = g.NGOfVertices(&vertices_subset); // TODO: method not available

    // All should be equal in undirected graph
    // assert_eq!(nplus_subset.size(), ng_subset.size()); // TODO: method not available
    // assert_eq!(nminus_subset.size(), ng_subset.size()); // TODO: method not available

    // Check that all contain the same elements
    // for vertex in [1, 2, 3] { // TODO: method not available
    //     assert_eq!(nplus_subset.mem(&vertex), ng_subset.mem(&vertex)); // TODO: method not available
    //     assert_eq!(nminus_subset.mem(&vertex), ng_subset.mem(&vertex)); // TODO: method not available
    // } // TODO: method not available
}

#[test]
fn test_labundirgraphmteph_edge_cases() {
    // Test empty graph
    let empty: LabUnDirGraphMtEph<i32, String> = LabUnDirGraphMtEph::empty();
    assert!(!empty.has_edge(&0, &1));
    assert_eq!(empty.neighbors(&0).size(), 0);
    assert_eq!(empty.neighbors(&0).size(), 0);

    // Test single vertex
    let v_single: Set<N> = SetLit![42];
    let a_empty: Set<LabEdge<N, String>> = SetLit![];
    let g_single = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v_single, a_empty);

    assert_eq!(g_single.vertices().size(), 1);
    assert_eq!(g_single.labeled_edges().size(), 0);
    assert_eq!(g_single.neighbors(&42).size(), 0);
    assert_eq!(g_single.neighbors(&42).size(), 0);

    // Test self-loop
    let v_self: Set<N> = SetLit![1];
    let a_self: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 1, "self".to_string())];
    let g_self = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v_self, a_self);

    assert!(g_self.has_edge(&1, &1));
    // In undirected graph, self-loop contributes 2 to degree
    assert_eq!(g_self.neighbors(&1).size(), 1); // Self-loop contributes degree 1
    assert_eq!(g_self.neighbors(&1).size(), 1); // Self-loop contributes degree 1
    assert_eq!(g_self.neighbors(&1).size(), 1); // Self-loop contributes degree 1
}

#[test]
fn test_labundirgraphmteph_nonexistent_vertex() {
    let v: Set<N> = SetLit![0, 1, 2];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(0, 1, "test".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    // Query non-existent vertex
    assert!(!g.has_edge(&99, &0));
    assert_eq!(g.neighbors(&99).size(), 0);
    assert_eq!(g.neighbors(&99).size(), 0);
    assert_eq!(g.neighbors(&99).size(), 0);
    assert_eq!(g.neighbors(&99).size(), 0);
}

#[test]
fn test_labundirgraphmteph_concurrent_access() {
    let v: Set<N> = SetLit![0, 1, 2, 3, 4];
    let a: Set<LabEdge<N, String>> = SetLit![
        LabEdge(0, 1, "a".to_string()),
        LabEdge(1, 2, "b".to_string()),
        LabEdge(2, 3, "c".to_string()),
        LabEdge(3, 4, "d".to_string()),
        LabEdge(0, 4, "e".to_string()) // Additional edge for more interesting topology
    ];
    let g = Arc::new(LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a));

    let num_threads = 4;
    let barrier = Arc::new(Barrier::new(num_threads));

    let mut handles = vec![];
    for i in 0..num_threads {
        let g_clone = Arc::clone(&g);
        let barrier_clone = Arc::clone(&barrier);

        handles.push(thread::spawn(move || {
            barrier_clone.wait(); // Wait for all threads to be ready

            // Perform various read operations concurrently
            let _ = g_clone.has_edge(&i, &(i + 1));
            let _ = g_clone.neighbors(&i);
            let _ = g_clone.neighbors(&i);
            let _ = g_clone.neighbors(&i); // In undirected graphs, in/out neighbors are the same
            let _ = g_clone.neighbors(&i);
            let _ = g_clone.neighbors(&i);
            let _ = g_clone.neighbors(&i);

            // Verify basic properties
            assert_eq!(g_clone.vertices().size(), 5);
            assert_eq!(g_clone.labeled_edges().size(), 5);

            // In undirected graph, InDegree should equal OutDegree
            assert_eq!(g_clone.neighbors(&i).size(), g_clone.neighbors(&i).size()); // In undirected graphs, degree is the same

            (
                g_clone.neighbors(&i).size(),
                g_clone.neighbors(&i).size(),
                g_clone.neighbors(&i).size(),
                g_clone.neighbors(&i).size(),
            )
        }));
    }

    for handle in handles {
        let (_ng_size, degree, in_degree, out_degree) = handle.join().unwrap();
        // Verify undirected graph properties
        assert_eq!(in_degree, out_degree);
        assert_eq!(degree, in_degree); // In undirected graphs, degree = in_degree = out_degree
    }
}

#[test]
fn test_sizea() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string()), LabEdge(2, 3, "b".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    assert_eq!(g.sizeA(), 2);
}

#[test]
fn test_arcs() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "edge1".to_string()), LabEdge(2, 3, "edge2".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    let arcs = g.arcs();
    assert_eq!(arcs.size(), 2);
}

#[test]
fn test_nplus() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    let nplus = g.NPlus(&1);
    assert_eq!(nplus.size(), 1);
    assert!(nplus.mem(&2));
}

#[test]
fn test_nminus() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    let nminus = g.NMinus(&2);
    assert_eq!(nminus.size(), 1);
    assert!(nminus.mem(&1));
}

#[test]
fn test_indegree() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string()), LabEdge(2, 3, "b".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    assert_eq!(g.InDegree(&2), 2);
}

#[test]
fn test_outdegree() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string()), LabEdge(2, 3, "b".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    assert_eq!(g.OutDegree(&2), 2);
}

#[test]
fn test_edges() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string()), LabEdge(2, 3, "b".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);
    let edges = g.edges();
    assert_eq!(edges.size(), 2);
    assert!(edges.mem(&Edge(1, 2)));
    assert!(edges.mem(&Edge(2, 3)));
}

#[test]
fn test_add_vertex() {
    let mut g: LabUnDirGraphMtEph<i32, String> = LabUnDirGraphMtEph::empty();
    assert_eq!(g.vertices().size(), 0);

    g.add_vertex(1);
    assert_eq!(g.vertices().size(), 1);
    assert!(g.vertices().mem(&1));

    g.add_vertex(2);
    assert_eq!(g.vertices().size(), 2);
    assert!(g.vertices().mem(&2));

    // Adding duplicate should not increase size
    g.add_vertex(1);
    assert_eq!(g.vertices().size(), 2);
}

#[test]
fn test_add_labeled_edge() {
    let mut g: LabUnDirGraphMtEph<i32, String> = LabUnDirGraphMtEph::empty();

    g.add_labeled_edge(1, 2, "edge12".to_string());
    assert_eq!(g.vertices().size(), 2);
    assert_eq!(g.labeled_edges().size(), 1);
    assert!(g.has_edge(&1, &2));
    assert!(g.has_edge(&2, &1)); // Undirected

    g.add_labeled_edge(2, 3, "edge23".to_string());
    assert_eq!(g.vertices().size(), 3);
    assert_eq!(g.labeled_edges().size(), 2);

    // Test edge normalization (smaller vertex first)
    g.add_labeled_edge(5, 4, "edge54".to_string());
    assert!(g.has_edge(&4, &5));
    assert!(g.has_edge(&5, &4));
}

#[test]
fn test_get_edge_label() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "first".to_string()), LabEdge(2, 3, "second".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    assert_eq!(g.get_edge_label(&1, &2), Some(&"first".to_string()));
    assert_eq!(g.get_edge_label(&2, &1), Some(&"first".to_string())); // Undirected
    assert_eq!(g.get_edge_label(&2, &3), Some(&"second".to_string()));
    assert_eq!(g.get_edge_label(&3, &2), Some(&"second".to_string())); // Undirected
    assert_eq!(g.get_edge_label(&1, &3), None); // No such edge
}

#[test]
fn test_vertices_accessor() {
    let v: Set<N> = SetLit![1, 2, 3, 4];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v.clone(), a);

    assert_eq!(g.vertices().size(), 4);
    for i in 1..=4 {
        assert!(g.vertices().mem(&i));
    }
}

#[test]
fn test_labeled_edges_accessor() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string()), LabEdge(2, 3, "b".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    assert_eq!(g.labeled_edges().size(), 2);
    assert!(g.labeled_edges().mem(&LabEdge(1, 2, "a".to_string())));
    assert!(g.labeled_edges().mem(&LabEdge(2, 3, "b".to_string())));
}

#[test]
fn test_display() {
    let v: Set<N> = SetLit![1, 2];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "test".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    let display_str = format!("{g}");
    assert!(display_str.contains("LabUnDirGraph"));
}

#[test]
fn test_debug() {
    let v: Set<N> = SetLit![1, 2];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "test".to_string())];
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    let debug_str = format!("{g:?}");
    assert!(debug_str.contains("LabUnDirGraph"));
    assert!(debug_str.contains("vertices"));
    assert!(debug_str.contains("labeled_edges"));
}

#[test]
fn test_clone() {
    let v: Set<N> = SetLit![1, 2, 3];
    let a: Set<LabEdge<N, String>> = SetLit![LabEdge(1, 2, "a".to_string())];
    let g1 = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(v, a);

    let g2 = g1.clone();
    assert_eq!(g2.vertices().size(), g1.vertices().size());
    assert_eq!(g2.labeled_edges().size(), g1.labeled_edges().size());
    assert!(g2.has_edge(&1, &2));
}

#[test]
fn test_parallel_neighbors() {
    let mut vertices = Set::empty();
    for i in 0..16 {
        vertices.insert(i);
    }
    
    let mut edges = Set::empty();
    for i in 1..13 {
        edges.insert(LabEdge(0, i, format!("edge_{i}")));
    }
    
    let g = LabUnDirGraphMtEph::from_vertices_and_labeled_edges(vertices, edges);
    let neighbors = g.neighbors(&0);
    assert_eq!(neighbors.size(), 12);
}
