//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestWeightedUnDirGraphMtEphInt {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedUnDirGraphMtEphInt::WeightedUnDirGraphMtEphInt::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_weightedundirgraphmtephint_empty() {
        let empty_graph: WeightedUnDirGraphMtEphInt<i32> = WeightedUnDirGraphMtEphInt::empty();
        assert_eq!(empty_graph.sizeV(), 0);
        assert_eq!(empty_graph.sizeA(), 0);
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.arcs().size(), 0);
    }

    #[test]
    fn test_weightedundirgraphmtephint_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 10),
            WeightedEdge(1, 2, 20),
            WeightedEdge(2, 3, 30)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        assert_eq!(g.sizeV(), 4);
        assert_eq!(g.sizeA(), 3);
        
        // Test neighbor relationships (undirected - both directions)
        assert_eq!(g.Neighbor(&0, &1), B::True);
        assert_eq!(g.Neighbor(&1, &0), B::True); // Undirected graph
        assert_eq!(g.Neighbor(&1, &2), B::True);
        assert_eq!(g.Neighbor(&2, &1), B::True);
        assert_eq!(g.Neighbor(&0, &2), B::False); // No direct edge
        
        // Test NG (neighbors) - should be symmetric
        let ng0 = g.NG(&0);
        assert_eq!(ng0.size(), 1);
        assert_eq!(ng0.mem(&1), B::True);
        
        let ng1 = g.NG(&1);
        assert_eq!(ng1.size(), 2);
        assert_eq!(ng1.mem(&0), B::True);
        assert_eq!(ng1.mem(&2), B::True);
        
        let ng2 = g.NG(&2);
        assert_eq!(ng2.size(), 2);
        assert_eq!(ng2.mem(&1), B::True);
        assert_eq!(ng2.mem(&3), B::True);
        
        // Test degrees (in undirected graph, InDegree = OutDegree = Degree)
        assert_eq!(g.Degree(&0), 1);
        assert_eq!(g.InDegree(&0), 1);
        assert_eq!(g.OutDegree(&0), 1);
        
        assert_eq!(g.Degree(&1), 2);
        assert_eq!(g.InDegree(&1), 2);
        assert_eq!(g.OutDegree(&1), 2);
        
        assert_eq!(g.Degree(&2), 2);
        assert_eq!(g.InDegree(&2), 2);
        assert_eq!(g.OutDegree(&2), 2);
        
        assert_eq!(g.Degree(&3), 1);
        assert_eq!(g.InDegree(&3), 1);
        assert_eq!(g.OutDegree(&3), 1);
    }

    #[test]
    fn test_weightedundirgraphmtephint_incident_operations() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 100),
            WeightedEdge(1, 2, 200),
            WeightedEdge(0, 2, 300)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        // Test incident edges (each edge is incident to both endpoints)
        let incident0 = g.Incident(&0);
        assert_eq!(incident0.size(), 2); // 0-1 and 0-2
        
        let incident1 = g.Incident(&1);
        assert_eq!(incident1.size(), 2); // 0-1 and 1-2
        
        let incident2 = g.Incident(&2);
        assert_eq!(incident2.size(), 2); // 1-2 and 0-2
    }

    #[test]
    fn test_weightedundirgraphmtephint_ngofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 1),
            WeightedEdge(1, 2, 2),
            WeightedEdge(2, 3, 3),
            WeightedEdge(0, 3, 4)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        let vertices_subset: Set<N> = SetLit![0, 1];
        let ng_subset = g.NGOfVertices(&vertices_subset);
        
        // Neighbors of {0, 1} should include all vertices connected to 0 or 1
        assert_eq!(ng_subset.size(), 3);
        assert_eq!(ng_subset.mem(&1), B::True); // 0-1
        assert_eq!(ng_subset.mem(&2), B::True); // 1-2
        assert_eq!(ng_subset.mem(&3), B::True); // 0-3
    }

    #[test]
    fn test_weightedundirgraphmtephint_nplusminusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 5),
            WeightedEdge(1, 2, 15),
            WeightedEdge(2, 0, 25),
            WeightedEdge(3, 1, 35)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        let vertices_subset: Set<N> = SetLit![0, 1];
        
        // In undirected graphs, NPlus and NMinus should be the same as NG
        let nplus_subset = g.NPlusOfVertices(&vertices_subset);
        let nminus_subset = g.NMinusOfVertices(&vertices_subset);
        let ng_subset = g.NGOfVertices(&vertices_subset);
        
        // All should be equal in undirected graph
        assert_eq!(nplus_subset.size(), ng_subset.size());
        assert_eq!(nminus_subset.size(), ng_subset.size());
        
        // Check that all contain the same elements
        for vertex in [1, 2, 3] {
            assert_eq!(nplus_subset.mem(&vertex), ng_subset.mem(&vertex));
            assert_eq!(nminus_subset.mem(&vertex), ng_subset.mem(&vertex));
        }
    }

    #[test]
    fn test_weightedundirgraphmtephint_edge_cases() {
        // Test empty graph
        let empty: WeightedUnDirGraphMtEphInt<i32> = WeightedUnDirGraphMtEphInt::empty();
        assert_eq!(empty.Neighbor(&0, &1), B::False);
        assert_eq!(empty.NG(&0).size(), 0);
        assert_eq!(empty.Degree(&0), 0);
        
        // Test single vertex
        let v_single: Set<N> = SetLit![42];
        let a_empty: Set<WeightedEdge<N, i32>> = SetLit![];
        let g_single = WeightedUnDirGraphMtEphInt::FromSets(v_single, a_empty);
        
        assert_eq!(g_single.sizeV(), 1);
        assert_eq!(g_single.sizeA(), 0);
        assert_eq!(g_single.Degree(&42), 0);
        assert_eq!(g_single.NG(&42).size(), 0);
        
        // Test self-loop with weight
        let v_self: Set<N> = SetLit![1];
        let a_self: Set<WeightedEdge<N, i32>> = SetLit![WeightedEdge(1, 1, 999)];
        let g_self = WeightedUnDirGraphMtEphInt::FromSets(v_self, a_self);
        
        assert_eq!(g_self.Neighbor(&1, &1), B::True);
        // In undirected graph, self-loop contributes 2 to degree
        assert_eq!(g_self.Degree(&1), 2);
        assert_eq!(g_self.InDegree(&1), 2);
        assert_eq!(g_self.OutDegree(&1), 2);
    }

    #[test]
    fn test_weightedundirgraphmtephint_nonexistent_vertex() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<WeightedEdge<N, i32>> = SetLit![WeightedEdge(0, 1, 777)];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        // Query non-existent vertex
        assert_eq!(g.Neighbor(&99, &0), B::False);
        assert_eq!(g.NG(&99).size(), 0);
        assert_eq!(g.Degree(&99), 0);
        assert_eq!(g.InDegree(&99), 0);
        assert_eq!(g.OutDegree(&99), 0);
    }

    #[test]
    fn test_weightedundirgraphmtephint_weight_variations() {
        // Test with various integer weight values including negative, zero, and extremes
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 0),              // Zero weight
            WeightedEdge(1, 2, -100),           // Negative weight
            WeightedEdge(2, 3, 1),              // Small positive
            WeightedEdge(3, 4, i32::MAX),       // Maximum positive
            WeightedEdge(4, 0, i32::MIN)        // Minimum (most negative)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        assert_eq!(g.sizeV(), 5);
        assert_eq!(g.sizeA(), 5);
        
        // All edges should still be recognized regardless of weight
        assert_eq!(g.Neighbor(&0, &1), B::True);
        assert_eq!(g.Neighbor(&1, &0), B::True); // Undirected
        assert_eq!(g.Neighbor(&1, &2), B::True);
        assert_eq!(g.Neighbor(&2, &1), B::True); // Undirected
        assert_eq!(g.Neighbor(&2, &3), B::True);
        assert_eq!(g.Neighbor(&3, &2), B::True); // Undirected
        assert_eq!(g.Neighbor(&3, &4), B::True);
        assert_eq!(g.Neighbor(&4, &3), B::True); // Undirected
        assert_eq!(g.Neighbor(&4, &0), B::True);
        assert_eq!(g.Neighbor(&0, &4), B::True); // Undirected
        
        // Each vertex should have degree 2 (connected to 2 neighbors)
        for vertex in [0, 1, 2, 3, 4] {
            assert_eq!(g.Degree(&vertex), 2);
            assert_eq!(g.InDegree(&vertex), 2);  // Same as degree in undirected
            assert_eq!(g.OutDegree(&vertex), 2); // Same as degree in undirected
        }
    }

    #[test]
    fn test_weightedundirgraphmtephint_large_weights() {
        // Test with large integer weights to ensure no overflow issues
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 1_000_000),
            WeightedEdge(1, 2, -1_000_000),
            WeightedEdge(2, 0, 999_999_999)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        assert_eq!(g.sizeV(), 3);
        assert_eq!(g.sizeA(), 3);
        
        // Verify all connections work with large weights (undirected)
        assert_eq!(g.Neighbor(&0, &1), B::True);
        assert_eq!(g.Neighbor(&1, &0), B::True);
        assert_eq!(g.Neighbor(&1, &2), B::True);
        assert_eq!(g.Neighbor(&2, &1), B::True);
        assert_eq!(g.Neighbor(&2, &0), B::True);
        assert_eq!(g.Neighbor(&0, &2), B::True);
        
        // Each vertex should have degree 2 (connected to 2 neighbors)
        for vertex in [0, 1, 2] {
            assert_eq!(g.Degree(&vertex), 2);
            assert_eq!(g.InDegree(&vertex), 2);
            assert_eq!(g.OutDegree(&vertex), 2);
        }
    }

    #[test]
    fn test_weightedundirgraphmtephint_concurrent_access() {
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 11),
            WeightedEdge(1, 2, 22),
            WeightedEdge(2, 3, 33),
            WeightedEdge(3, 4, 44),
            WeightedEdge(0, 4, 55) // Additional edge for more interesting topology
        ];
        let g = Arc::new(WeightedUnDirGraphMtEphInt::FromSets(v, a));
        
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let mut handles = vec![];
        for i in 0..num_threads {
            let g_clone = Arc::clone(&g);
            let barrier_clone = Arc::clone(&barrier);
            
            handles.push(thread::spawn(move || {
                barrier_clone.wait(); // Wait for all threads to be ready
                
                // Perform various read operations concurrently
                let _ = g_clone.Neighbor(&i, &(i + 1));
                let _ = g_clone.NG(&i);
                let _ = g_clone.NPlus(&i);
                let _ = g_clone.NMinus(&i);
                let _ = g_clone.Degree(&i);
                let _ = g_clone.InDegree(&i);
                let _ = g_clone.OutDegree(&i);

                // Verify basic properties
                assert_eq!(g_clone.sizeV(), 5);
                assert_eq!(g_clone.sizeA(), 5);
                
                // In undirected graph, InDegree should equal OutDegree
                assert_eq!(g_clone.InDegree(&i), g_clone.OutDegree(&i));
                
                (g_clone.NG(&i).size(), g_clone.Degree(&i), g_clone.InDegree(&i), g_clone.OutDegree(&i))
            }));
        }
        
        for handle in handles {
            let (ng_size, degree, in_degree, out_degree) = handle.join().unwrap();
            // Verify undirected graph properties
            assert_eq!(in_degree, out_degree);
            assert_eq!(degree, in_degree + out_degree);
        }
    }

    #[test]
    fn test_weightedundirgraphmtephint_complete_graph() {
        // Test complete graph K4 with integer weights
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, i32>> = SetLit![
            WeightedEdge(0, 1, 1), WeightedEdge(0, 2, 2), WeightedEdge(0, 3, 3),
            WeightedEdge(1, 2, 12), WeightedEdge(1, 3, 13),
            WeightedEdge(2, 3, 23)
        ];
        let g = WeightedUnDirGraphMtEphInt::FromSets(v, a);
        
        assert_eq!(g.sizeV(), 4);
        assert_eq!(g.sizeA(), 6);
        
        // Every vertex should have degree 3 in K4
        for vertex in [0, 1, 2, 3] {
            assert_eq!(g.Degree(&vertex), 3);
            assert_eq!(g.NG(&vertex).size(), 3);
            assert_eq!(g.InDegree(&vertex), 3);
            assert_eq!(g.OutDegree(&vertex), 3);
        }
        
        // Every pair should be neighbors
        for i in [0, 1, 2, 3] {
            for j in [0, 1, 2, 3] {
                if i != j {
                    assert_eq!(g.Neighbor(&i, &j), B::True);
                }
            }
        }
    }
}
