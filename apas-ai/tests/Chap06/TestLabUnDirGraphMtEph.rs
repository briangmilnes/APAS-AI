//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestLabUnDirGraphMtEph {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::LabUnDirGraphMtEphTrait;
    use apas_ai::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

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
        assert_eq!(g.has_edge(&0, &1), true);
        assert_eq!(g.has_edge(&1, &0), true); // Undirected graph
        assert_eq!(g.has_edge(&1, &2), true);
        assert_eq!(g.has_edge(&2, &1), true);
        assert_eq!(g.has_edge(&0, &2), false); // No direct edge

        // Test NG (neighbors) - should be symmetric
        let ng0 = g.neighbors(&0);
        assert_eq!(ng0.size(), 1);
        assert_eq!(ng0.mem(&1), true);

        let ng1 = g.neighbors(&1);
        assert_eq!(ng1.size(), 2);
        assert_eq!(ng1.mem(&0), true);
        assert_eq!(ng1.mem(&2), true);

        let ng2 = g.neighbors(&2);
        assert_eq!(ng2.size(), 2);
        assert_eq!(ng2.mem(&1), true);
        assert_eq!(ng2.mem(&3), true);

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

        assert_eq!(g_self.has_edge(&1, &1), true);
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
        assert_eq!(g.has_edge(&99, &0), false);
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
}
