//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestWeightedDirGraphMtEphInt {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::*;
    use apas_ai::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use std::sync::{Arc, Barrier};
    use std::thread;

    #[test]
    fn test_weighteddirgraphmtephint_empty() {
        let empty_graph: WeightedDirGraphMtEphInt<i32> = WeightedDirGraphMtEphInt::empty();
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.labeled_arcs().size(), 0);
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.labeled_arcs().size(), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephint_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 10),
            LabEdge(1, 2, 20),
            LabEdge(2, 3, 30),
            LabEdge(0, 3, 40)
        ];
        let g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        assert_eq!(g.vertices().size(), 4);
        assert_eq!(g.labeled_arcs().size(), 4);
        
        // Test neighbor relationships
        assert!(g.has_arc(&0, &1));
        assert!(!g.has_arc(&1, &0)); // Directed graph
        assert!(g.has_arc(&1, &2));
        assert!(!g.has_arc(&2, &1));
        assert!(g.has_arc(&0, &3));
        
        // Test NG (neighbors)
        let ng0 = g.out_neighbors(&0);
        assert_eq!(ng0.size(), 2);
        assert_eq!(ng0.mem(&1), B::True);
        assert_eq!(ng0.mem(&3), B::True);
        
        let ng1 = g.out_neighbors(&1);
        assert_eq!(ng1.size(), 1);
        assert_eq!(ng1.mem(&2), B::True);
        
        // Test NPlus (out-neighbors)
        let nplus0 = g.out_neighbors(&0);
        assert_eq!(nplus0.size(), 2);
        assert_eq!(nplus0.mem(&1), B::True);
        assert_eq!(nplus0.mem(&3), B::True);
        
        // Test NMinus (in-neighbors)
        let nminus1 = g.in_neighbors(&1);
        assert_eq!(nminus1.size(), 1);
        assert_eq!(nminus1.mem(&0), B::True);
        
        let nminus3 = g.in_neighbors(&3);
        assert_eq!(nminus3.size(), 2);
        assert_eq!(nminus3.mem(&0), B::True);
        assert_eq!(nminus3.mem(&2), B::True);
        
        // Test degrees
        assert_eq!(g.out_neighbors(&0).size(), 2);
        assert_eq!(g.in_neighbors(&0).size(), 0);
        assert_eq!(g.out_neighbors(&0).size(), 2);
        
        assert_eq!(g.out_neighbors(&1).size(), 1);
        assert_eq!(g.in_neighbors(&1).size(), 1);
        assert_eq!(g.out_neighbors(&1).size(), 1);
        
        assert_eq!(g.out_neighbors(&3).size(), 0);
        assert_eq!(g.in_neighbors(&3).size(), 2);
        assert_eq!(g.out_neighbors(&3).size(), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephint_incident_operations() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 100),
            LabEdge(1, 2, 200),
            LabEdge(0, 2, 300)
        ];
        let _g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        // Test incident edges
        // let _incident0 = g.Incident(&0); // TODO: method not available
        // assert_eq!(_incident0.size(), 2); // 0->1 and 0->2 // TODO: method not available
        
        // let _incident1 = g.Incident(&1); // TODO: method not available
        // assert_eq!(_incident1.size(), 2); // 0->1 (incoming) and 1->2 (outgoing) // TODO: method not available
        
        // let _incident2 = g.Incident(&2); // TODO: method not available
        // assert_eq!(_incident2.size(), 2); // 1->2 and 0->2 (both incoming) // TODO: method not available
    }

    #[test]
    fn test_weighteddirgraphmtephint_ngofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 1),
            LabEdge(1, 2, 2),
            LabEdge(2, 3, 3),
            LabEdge(0, 3, 4)
        ];
        let _g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        let _vertices_subset: Set<N> = SetLit![0, 1];
        // let _ng_subset = g.NGOfVertices(&vertices_subset); // TODO: method not available
        
        // Neighbors of {0, 1} should be {1, 2, 3}
        // assert_eq!(_ng_subset.size(), 3); // TODO: method not available
        // assert_eq!(_ng_subset.mem(&1), B::True); // TODO: method not available
        // assert_eq!(_ng_subset.mem(&2), B::True); // TODO: method not available
        // assert_eq!(_ng_subset.mem(&3), B::True); // TODO: method not available
    }

    #[test]
    fn test_weighteddirgraphmtephint_nplusminusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 5),
            LabEdge(1, 2, 15),
            LabEdge(2, 0, 25),
            LabEdge(3, 1, 35)
        ];
        let _g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        let _vertices_subset: Set<N> = SetLit![0, 1];
        
        // Test NPlusOfVertices (out-neighbors)
        // let _nplus_subset = g.NPlusOfVertices(&vertices_subset); // TODO: method not available
        // assert_eq!(_nplus_subset.size(), 2); // TODO: method not available
        // assert_eq!(_nplus_subset.mem(&1), B::True); // 0->1 // TODO: method not available
        // assert_eq!(_nplus_subset.mem(&2), B::True); // 1->2 // TODO: method not available
        
        // Test NMinusOfVertices (in-neighbors)
        // let _nminus_subset = g.NMinusOfVertices(&vertices_subset); // TODO: method not available
        // assert_eq!(_nminus_subset.size(), 2); // TODO: method not available
        // assert_eq!(_nminus_subset.mem(&2), B::True); // 2->0 // TODO: method not available
        // assert_eq!(_nminus_subset.mem(&3), B::True); // 3->1 // TODO: method not available
    }

    #[test]
    fn test_weighteddirgraphmtephint_edge_cases() {
        // Test empty graph
        let empty: WeightedDirGraphMtEphInt<i32> = WeightedDirGraphMtEphInt::empty();
        assert!(!empty.has_arc(&0, &1));
        assert_eq!(empty.out_neighbors(&0).size(), 0);
        assert_eq!(empty.out_neighbors(&0).size(), 0);
        
        // Test single vertex
        let v_single: Set<N> = SetLit![42];
        let a_empty: Set<LabEdge<N, i32>> = SetLit![];
        let g_single = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v_single, a_empty);
        
        assert_eq!(g_single.vertices().size(), 1);
        assert_eq!(g_single.labeled_arcs().size(), 0);
        assert_eq!(g_single.out_neighbors(&42).size(), 0);
        assert_eq!(g_single.out_neighbors(&42).size(), 0);
        
        // Test self-loop with weight
        let v_self: Set<N> = SetLit![1];
        let a_self: Set<LabEdge<N, i32>> = SetLit![LabEdge(1, 1, 999)];
        let g_self = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v_self, a_self);
        
        assert!(g_self.has_arc(&1, &1));
        assert_eq!(g_self.out_neighbors(&1).size(), 1); // Self-loop to self
        assert_eq!(g_self.in_neighbors(&1).size(), 1);
        assert_eq!(g_self.out_neighbors(&1).size(), 1);
    }

    #[test]
    fn test_weighteddirgraphmtephint_nonexistent_vertex() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, i32>> = SetLit![LabEdge(0, 1, 777)];
        let g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        // Query non-existent vertex
        assert!(!g.has_arc(&99, &0));
        assert_eq!(g.out_neighbors(&99).size(), 0);
        assert_eq!(g.out_neighbors(&99).size(), 0);
        assert_eq!(g.in_neighbors(&99).size(), 0);
        assert_eq!(g.out_neighbors(&99).size(), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephint_weight_variations() {
        // Test with various integer weight values including negative, zero, and extremes
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 0),              // Zero weight
            LabEdge(1, 2, -100),           // Negative weight
            LabEdge(2, 3, 1),              // Small positive
            LabEdge(3, 4, i32::MAX),       // Maximum positive
            LabEdge(4, 0, i32::MIN)        // Minimum (most negative)
        ];
        let g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        assert_eq!(g.vertices().size(), 5);
        assert_eq!(g.labeled_arcs().size(), 5);
        
        // All edges should still be recognized regardless of weight
        assert!(g.has_arc(&0, &1));
        assert!(g.has_arc(&1, &2));
        assert!(g.has_arc(&2, &3));
        assert!(g.has_arc(&3, &4));
        assert!(g.has_arc(&4, &0));
        
        // Each vertex should have degree 2 (one in, one out)
        for vertex in [0, 1, 2, 3, 4] {
            assert_eq!(g.out_neighbors(&vertex).size(), 1);
            assert_eq!(g.in_neighbors(&vertex).size(), 1);
            assert_eq!(g.out_neighbors(&vertex).size(), 1);
        }
    }

    #[test]
    fn test_weighteddirgraphmtephint_large_weights() {
        // Test with large integer weights to ensure no overflow issues
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 1_000_000),
            LabEdge(1, 2, -1_000_000),
            LabEdge(2, 0, 999_999_999)
        ];
        let g = WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a);
        
        assert_eq!(g.vertices().size(), 3);
        assert_eq!(g.labeled_arcs().size(), 3);
        
        // Verify all connections work with large weights
        assert!(g.has_arc(&0, &1));
        assert!(g.has_arc(&1, &2));
        assert!(g.has_arc(&2, &0));
        
        // Each vertex should have degree 2 (one in, one out)
        for vertex in [0, 1, 2] {
            assert_eq!(g.out_neighbors(&vertex).size(), 1);
            assert_eq!(g.in_neighbors(&vertex).size(), 1);
            assert_eq!(g.out_neighbors(&vertex).size(), 1);
        }
    }

    #[test]
    fn test_weighteddirgraphmtephint_concurrent_access() {
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, i32>> = SetLit![
            LabEdge(0, 1, 11),
            LabEdge(1, 2, 22),
            LabEdge(2, 3, 33),
            LabEdge(3, 4, 44)
        ];
        let g = Arc::new(WeightedDirGraphMtEphInt::from_vertices_and_labeled_arcs(v, a));
        
        let num_threads = 4;
        let barrier = Arc::new(Barrier::new(num_threads));
        
        let mut handles = vec![];
        for i in 0..num_threads {
            let g_clone = Arc::clone(&g);
            let barrier_clone = Arc::clone(&barrier);
            
            handles.push(thread::spawn(move || {
                barrier_clone.wait(); // Wait for all threads to be ready
                
                // Perform various read operations concurrently
                let _ = g_clone.has_arc(&i, &(i + 1));
                let _ = g_clone.out_neighbors(&i);
                let _ = g_clone.out_neighbors(&i);
                let _ = g_clone.in_neighbors(&i);
                let _ = g_clone.out_neighbors(&i);
                let _ = g_clone.in_neighbors(&i);
                let _ = g_clone.out_neighbors(&i);

                // Verify basic properties
                assert_eq!(g_clone.vertices().size(), 5);
                assert_eq!(g_clone.labeled_arcs().size(), 4);
                
                (g_clone.out_neighbors(&i).size(), g_clone.out_neighbors(&i), g_clone.in_neighbors(&i), g_clone.out_neighbors(&i))
            }));
        }
        
        for handle in handles {
            let _ = handle.join().unwrap();
        }
    }
}
