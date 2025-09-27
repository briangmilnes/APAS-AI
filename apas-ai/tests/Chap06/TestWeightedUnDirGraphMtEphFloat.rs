//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestWeightedUnDirGraphMtEphFloat {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedUnDirGraphMtEphFloat::WeightedUnDirGraphMtEphFloat::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use ordered_float::OrderedFloat;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_weightedundirgraphmtephfloat_empty() {
        let empty_graph: WeightedUnDirGraphMtEphFloat<i32> = WeightedUnDirGraphMtEphFloat::empty();
        assert_eq!(empty_graph.sizeV(), 0);
        assert_eq!(empty_graph.sizeA(), 0);
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.arcs().size(), 0);
    }

    #[test]
    fn test_weightedundirgraphmtephfloat_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.5)),
            LabEdge(1, 2, OrderedFloat(2.7)),
            LabEdge(2, 3, OrderedFloat(0.8))
        ];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
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
    fn test_weightedundirgraphmtephfloat_incident_operations() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(3.14)),
            LabEdge(1, 2, OrderedFloat(2.71)),
            LabEdge(0, 2, OrderedFloat(1.41))
        ];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
        // Test incident edges (each edge is incident to both endpoints)
        let incident0 = g.Incident(&0);
        assert_eq!(incident0.size(), 2); // 0-1 and 0-2
        
        let incident1 = g.Incident(&1);
        assert_eq!(incident1.size(), 2); // 0-1 and 1-2
        
        let incident2 = g.Incident(&2);
        assert_eq!(incident2.size(), 2); // 1-2 and 0-2
    }

    #[test]
    fn test_weightedundirgraphmtephfloat_ngofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.0)),
            LabEdge(1, 2, OrderedFloat(2.0)),
            LabEdge(2, 3, OrderedFloat(3.0)),
            LabEdge(0, 3, OrderedFloat(4.0))
        ];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
        let vertices_subset: Set<N> = SetLit![0, 1];
        let ng_subset = g.NGOfVertices(&vertices_subset);
        
        // Neighbors of {0, 1} should include all vertices connected to 0 or 1
        assert_eq!(ng_subset.size(), 3);
        assert_eq!(ng_subset.mem(&1), B::True); // 0-1
        assert_eq!(ng_subset.mem(&2), B::True); // 1-2
        assert_eq!(ng_subset.mem(&3), B::True); // 0-3
    }

    #[test]
    fn test_weightedundirgraphmtephfloat_nplusminusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(0.5)),
            LabEdge(1, 2, OrderedFloat(1.5)),
            LabEdge(2, 0, OrderedFloat(2.5)),
            LabEdge(3, 1, OrderedFloat(3.5))
        ];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
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
    fn test_weightedundirgraphmtephfloat_edge_cases() {
        // Test empty graph
        let empty: WeightedUnDirGraphMtEphFloat<i32> = WeightedUnDirGraphMtEphFloat::empty();
        assert_eq!(empty.Neighbor(&0, &1), B::False);
        assert_eq!(empty.NG(&0).size(), 0);
        assert_eq!(empty.Degree(&0), 0);
        
        // Test single vertex
        let v_single: Set<N> = SetLit![42];
        let a_empty: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![];
        let g_single = WeightedUnDirGraphMtEphFloat::FromSets(v_single, a_empty);
        
        assert_eq!(g_single.sizeV(), 1);
        assert_eq!(g_single.sizeA(), 0);
        assert_eq!(g_single.Degree(&42), 0);
        assert_eq!(g_single.NG(&42).size(), 0);
        
        // Test self-loop with weight
        let v_self: Set<N> = SetLit![1];
        let a_self: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(1, 1, OrderedFloat(99.9))];
        let g_self = WeightedUnDirGraphMtEphFloat::FromSets(v_self, a_self);
        
        assert_eq!(g_self.Neighbor(&1, &1), B::True);
        // In undirected graph, self-loop contributes 2 to degree
        assert_eq!(g_self.Degree(&1), 2);
        assert_eq!(g_self.InDegree(&1), 2);
        assert_eq!(g_self.OutDegree(&1), 2);
    }

    #[test]
    fn test_weightedundirgraphmtephfloat_nonexistent_vertex() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(0, 1, OrderedFloat(7.77))];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
        // Query non-existent vertex
        assert_eq!(g.Neighbor(&99, &0), B::False);
        assert_eq!(g.NG(&99).size(), 0);
        assert_eq!(g.Degree(&99), 0);
        assert_eq!(g.InDegree(&99), 0);
        assert_eq!(g.OutDegree(&99), 0);
    }

    #[test]
    fn test_weightedundirgraphmtephfloat_weight_variations() {
        // Test with various weight values including negative, zero, and very small/large
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(0.0)),           // Zero weight
            LabEdge(1, 2, OrderedFloat(-1.5)),          // Negative weight
            LabEdge(2, 3, OrderedFloat(1e-10)),         // Very small positive
            LabEdge(3, 4, OrderedFloat(1e10)),          // Very large positive
            LabEdge(4, 0, OrderedFloat(f64::INFINITY))  // Infinity
        ];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
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
    fn test_weightedundirgraphmtephfloat_concurrent_access() {
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.1)),
            LabEdge(1, 2, OrderedFloat(2.2)),
            LabEdge(2, 3, OrderedFloat(3.3)),
            LabEdge(3, 4, OrderedFloat(4.4)),
            LabEdge(0, 4, OrderedFloat(5.5)) // Additional edge for more interesting topology
        ];
        let g = Arc::new(WeightedUnDirGraphMtEphFloat::FromSets(v, a));
        
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
    fn test_weightedundirgraphmtephfloat_complete_graph() {
        // Test complete graph K4 with weights
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(0.1)), LabEdge(0, 2, OrderedFloat(0.2)), LabEdge(0, 3, OrderedFloat(0.3)),
            LabEdge(1, 2, OrderedFloat(1.2)), LabEdge(1, 3, OrderedFloat(1.3)),
            LabEdge(2, 3, OrderedFloat(2.3))
        ];
        let g = WeightedUnDirGraphMtEphFloat::FromSets(v, a);
        
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
