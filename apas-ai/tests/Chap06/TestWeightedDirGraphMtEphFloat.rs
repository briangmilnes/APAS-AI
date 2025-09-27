//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestWeightedDirGraphMtEphFloat {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_weighteddirgraphmtephfloat_empty() {
        let empty_graph: WeightedDirGraphMtEphFloat<i32> = WeightedDirGraphMtEphFloat::empty();
        assert_eq!(empty_graph.sizeV(), 0);
        assert_eq!(empty_graph.sizeA(), 0);
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.arcs().size(), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, f64>> = SetLit![
            WeightedEdge(0, 1, 1.5),
            WeightedEdge(1, 2, 2.7),
            WeightedEdge(2, 3, 0.8),
            WeightedEdge(0, 3, 4.2)
        ];
        let g = WeightedDirGraphMtEphFloat::FromSets(v, a);
        
        assert_eq!(g.sizeV(), 4);
        assert_eq!(g.sizeA(), 4);
        
        // Test neighbor relationships
        assert_eq!(g.Neighbor(&0, &1), B::True);
        assert_eq!(g.Neighbor(&1, &0), B::False); // Directed graph
        assert_eq!(g.Neighbor(&1, &2), B::True);
        assert_eq!(g.Neighbor(&2, &1), B::False);
        assert_eq!(g.Neighbor(&0, &3), B::True);
        
        // Test NG (neighbors)
        let ng0 = g.NG(&0);
        assert_eq!(ng0.size(), 2);
        assert_eq!(ng0.mem(&1), B::True);
        assert_eq!(ng0.mem(&3), B::True);
        
        let ng1 = g.NG(&1);
        assert_eq!(ng1.size(), 1);
        assert_eq!(ng1.mem(&2), B::True);
        
        // Test NPlus (out-neighbors)
        let nplus0 = g.NPlus(&0);
        assert_eq!(nplus0.size(), 2);
        assert_eq!(nplus0.mem(&1), B::True);
        assert_eq!(nplus0.mem(&3), B::True);
        
        // Test NMinus (in-neighbors)
        let nminus1 = g.NMinus(&1);
        assert_eq!(nminus1.size(), 1);
        assert_eq!(nminus1.mem(&0), B::True);
        
        let nminus3 = g.NMinus(&3);
        assert_eq!(nminus3.size(), 2);
        assert_eq!(nminus3.mem(&0), B::True);
        assert_eq!(nminus3.mem(&2), B::True);
        
        // Test degrees
        assert_eq!(g.Degree(&0), 2);
        assert_eq!(g.InDegree(&0), 0);
        assert_eq!(g.OutDegree(&0), 2);
        
        assert_eq!(g.Degree(&1), 2);
        assert_eq!(g.InDegree(&1), 1);
        assert_eq!(g.OutDegree(&1), 1);
        
        assert_eq!(g.Degree(&3), 2);
        assert_eq!(g.InDegree(&3), 2);
        assert_eq!(g.OutDegree(&3), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_incident_operations() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<WeightedEdge<N, f64>> = SetLit![
            WeightedEdge(0, 1, 3.14),
            WeightedEdge(1, 2, 2.71),
            WeightedEdge(0, 2, 1.41)
        ];
        let g = WeightedDirGraphMtEphFloat::FromSets(v, a);
        
        // Test incident edges
        let incident0 = g.Incident(&0);
        assert_eq!(incident0.size(), 2); // 0->1 and 0->2
        
        let incident1 = g.Incident(&1);
        assert_eq!(incident1.size(), 2); // 0->1 (incoming) and 1->2 (outgoing)
        
        let incident2 = g.Incident(&2);
        assert_eq!(incident2.size(), 2); // 1->2 and 0->2 (both incoming)
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_ngofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, f64>> = SetLit![
            WeightedEdge(0, 1, 1.0),
            WeightedEdge(1, 2, 2.0),
            WeightedEdge(2, 3, 3.0),
            WeightedEdge(0, 3, 4.0)
        ];
        let g = WeightedDirGraphMtEphFloat::FromSets(v, a);
        
        let vertices_subset: Set<N> = SetLit![0, 1];
        let ng_subset = g.NGOfVertices(&vertices_subset);
        
        // Neighbors of {0, 1} should be {1, 2, 3}
        assert_eq!(ng_subset.size(), 3);
        assert_eq!(ng_subset.mem(&1), B::True);
        assert_eq!(ng_subset.mem(&2), B::True);
        assert_eq!(ng_subset.mem(&3), B::True);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_nplusminusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<WeightedEdge<N, f64>> = SetLit![
            WeightedEdge(0, 1, 0.5),
            WeightedEdge(1, 2, 1.5),
            WeightedEdge(2, 0, 2.5),
            WeightedEdge(3, 1, 3.5)
        ];
        let g = WeightedDirGraphMtEphFloat::FromSets(v, a);
        
        let vertices_subset: Set<N> = SetLit![0, 1];
        
        // Test NPlusOfVertices (out-neighbors)
        let nplus_subset = g.NPlusOfVertices(&vertices_subset);
        assert_eq!(nplus_subset.size(), 2);
        assert_eq!(nplus_subset.mem(&1), B::True); // 0->1
        assert_eq!(nplus_subset.mem(&2), B::True); // 1->2
        
        // Test NMinusOfVertices (in-neighbors)
        let nminus_subset = g.NMinusOfVertices(&vertices_subset);
        assert_eq!(nminus_subset.size(), 2);
        assert_eq!(nminus_subset.mem(&2), B::True); // 2->0
        assert_eq!(nminus_subset.mem(&3), B::True); // 3->1
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_edge_cases() {
        // Test empty graph
        let empty: WeightedDirGraphMtEphFloat<i32> = WeightedDirGraphMtEphFloat::empty();
        assert_eq!(empty.Neighbor(&0, &1), B::False);
        assert_eq!(empty.NG(&0).size(), 0);
        assert_eq!(empty.Degree(&0), 0);
        
        // Test single vertex
        let v_single: Set<N> = SetLit![42];
        let a_empty: Set<WeightedEdge<N, f64>> = SetLit![];
        let g_single = WeightedDirGraphMtEphFloat::FromSets(v_single, a_empty);
        
        assert_eq!(g_single.sizeV(), 1);
        assert_eq!(g_single.sizeA(), 0);
        assert_eq!(g_single.Degree(&42), 0);
        assert_eq!(g_single.NG(&42).size(), 0);
        
        // Test self-loop with weight
        let v_self: Set<N> = SetLit![1];
        let a_self: Set<WeightedEdge<N, f64>> = SetLit![WeightedEdge(1, 1, 99.9)];
        let g_self = WeightedDirGraphMtEphFloat::FromSets(v_self, a_self);
        
        assert_eq!(g_self.Neighbor(&1, &1), B::True);
        assert_eq!(g_self.Degree(&1), 2); // Self-loop counts as both in and out
        assert_eq!(g_self.InDegree(&1), 1);
        assert_eq!(g_self.OutDegree(&1), 1);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_nonexistent_vertex() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<WeightedEdge<N, f64>> = SetLit![WeightedEdge(0, 1, 7.77)];
        let g = WeightedDirGraphMtEphFloat::FromSets(v, a);
        
        // Query non-existent vertex
        assert_eq!(g.Neighbor(&99, &0), B::False);
        assert_eq!(g.NG(&99).size(), 0);
        assert_eq!(g.Degree(&99), 0);
        assert_eq!(g.InDegree(&99), 0);
        assert_eq!(g.OutDegree(&99), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_weight_variations() {
        // Test with various weight values including negative, zero, and very small/large
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<WeightedEdge<N, f64>> = SetLit![
            WeightedEdge(0, 1, 0.0),           // Zero weight
            WeightedEdge(1, 2, -1.5),          // Negative weight
            WeightedEdge(2, 3, 1e-10),         // Very small positive
            WeightedEdge(3, 4, 1e10),          // Very large positive
            WeightedEdge(4, 0, f64::INFINITY)  // Infinity
        ];
        let g = WeightedDirGraphMtEphFloat::FromSets(v, a);
        
        assert_eq!(g.sizeV(), 5);
        assert_eq!(g.sizeA(), 5);
        
        // All edges should still be recognized regardless of weight
        assert_eq!(g.Neighbor(&0, &1), B::True);
        assert_eq!(g.Neighbor(&1, &2), B::True);
        assert_eq!(g.Neighbor(&2, &3), B::True);
        assert_eq!(g.Neighbor(&3, &4), B::True);
        assert_eq!(g.Neighbor(&4, &0), B::True);
        
        // Each vertex should have degree 2 (one in, one out)
        for vertex in [0, 1, 2, 3, 4] {
            assert_eq!(g.Degree(&vertex), 2);
            assert_eq!(g.InDegree(&vertex), 1);
            assert_eq!(g.OutDegree(&vertex), 1);
        }
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_concurrent_access() {
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<WeightedEdge<N, f64>> = SetLit![
            WeightedEdge(0, 1, 1.1),
            WeightedEdge(1, 2, 2.2),
            WeightedEdge(2, 3, 3.3),
            WeightedEdge(3, 4, 4.4)
        ];
        let g = Arc::new(WeightedDirGraphMtEphFloat::FromSets(v, a));
        
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
                assert_eq!(g_clone.sizeA(), 4);
                
                (g_clone.NG(&i).size(), g_clone.Degree(&i), g_clone.InDegree(&i), g_clone.OutDegree(&i))
            }));
        }
        
        for handle in handles {
            let _ = handle.join().unwrap();
        }
    }
}
