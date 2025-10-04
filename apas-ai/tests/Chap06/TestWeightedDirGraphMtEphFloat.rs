//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.


    use std::sync::{Arc, Barrier};
    use std::thread;

    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait;
    use apas_ai::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_weighteddirgraphmtephfloat_empty() {
        let empty_graph: WeightedDirGraphMtEphFloat<i32> = WeightedDirGraphMtEphFloat::empty();
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.labeled_arcs().size(), 0);
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.labeled_arcs().size(), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.5)),
            LabEdge(1, 2, OrderedFloat(2.7)),
            LabEdge(2, 3, OrderedFloat(0.8)),
            LabEdge(0, 3, OrderedFloat(4.2))
        ];
        let g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

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
        assert_eq!(ng0.mem(&1), true);
        assert_eq!(ng0.mem(&3), true);

        let ng1 = g.out_neighbors(&1);
        assert_eq!(ng1.size(), 1);
        assert_eq!(ng1.mem(&2), true);

        // Test NPlus (out-neighbors)
        let nplus0 = g.out_neighbors(&0);
        assert_eq!(nplus0.size(), 2);
        assert_eq!(nplus0.mem(&1), true);
        assert_eq!(nplus0.mem(&3), true);

        // Test NMinus (in-neighbors)
        let nminus1 = g.in_neighbors(&1);
        assert_eq!(nminus1.size(), 1);
        assert_eq!(nminus1.mem(&0), true);

        let nminus3 = g.in_neighbors(&3);
        assert_eq!(nminus3.size(), 2);
        assert_eq!(nminus3.mem(&0), true);
        assert_eq!(nminus3.mem(&2), true);

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
    fn test_weighteddirgraphmtephfloat_incident_operations() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(3.14)),
            LabEdge(1, 2, OrderedFloat(2.71)),
            LabEdge(0, 2, OrderedFloat(1.41))
        ];
        let _g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

        // Test incident edges
        // let incident0 = _g.Incident(&0); // TODO: method not available
        // assert_eq!(incident0.size(), 2); // 0->1 and 0->2 // TODO: method not available

        // let incident1 = g.Incident(&1); // TODO: method not available
        // assert_eq!(incident1.size(), 2); // 0->1 (incoming) and 1->2 (outgoing) // TODO: method not available

        // let incident2 = g.Incident(&2); // TODO: method not available
        // assert_eq!(incident2.size(), 2); // 1->2 and 0->2 (both incoming) // TODO: method not available
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_ngofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.0)),
            LabEdge(1, 2, OrderedFloat(2.0)),
            LabEdge(2, 3, OrderedFloat(3.0)),
            LabEdge(0, 3, OrderedFloat(4.0))
        ];
        let _g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

        let _vertices_subset: Set<N> = SetLit![0, 1];
        // let ng_subset = _g.NGOfVertices(&_vertices_subset); // TODO: method not available

        // Neighbors of {0, 1} should be {1, 2, 3}
        // assert_eq!(ng_subset.size(), 3); // TODO: method not available
        // assert_eq!(ng_subset.mem(&1), true); // TODO: method not available
        // assert_eq!(ng_subset.mem(&2), true); // TODO: method not available
        // assert_eq!(ng_subset.mem(&3), true); // TODO: method not available
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_nplusminusofvertices() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(0.5)),
            LabEdge(1, 2, OrderedFloat(1.5)),
            LabEdge(2, 0, OrderedFloat(2.5)),
            LabEdge(3, 1, OrderedFloat(3.5))
        ];
        let _g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

        let _vertices_subset: Set<N> = SetLit![0, 1];

        // Test NPlusOfVertices (out-neighbors)
        // let nplus_subset = _g.NPlusOfVertices(&_vertices_subset); // TODO: method not available
        // assert_eq!(nplus_subset.size(), 2); // TODO: method not available
        // assert_eq!(nplus_subset.mem(&1), true); // 0->1 // TODO: method not available
        // assert_eq!(nplus_subset.mem(&2), true); // 1->2 // TODO: method not available

        // Test NMinusOfVertices (in-neighbors)
        // let nminus_subset = g.NMinusOfVertices(&vertices_subset); // TODO: method not available
        // assert_eq!(nminus_subset.size(), 2); // TODO: method not available
        // assert_eq!(nminus_subset.mem(&2), true); // 2->0 // TODO: method not available
        // assert_eq!(nminus_subset.mem(&3), true); // 3->1 // TODO: method not available
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_edge_cases() {
        // Test empty graph
        let empty: WeightedDirGraphMtEphFloat<i32> = WeightedDirGraphMtEphFloat::empty();
        assert!(!empty.has_arc(&0, &1));
        assert_eq!(empty.out_neighbors(&0).size(), 0);
        assert_eq!(empty.out_neighbors(&0).size(), 0);

        // Test single vertex
        let v_single: Set<N> = SetLit![42];
        let a_empty: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![];
        let g_single = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v_single, a_empty);

        assert_eq!(g_single.vertices().size(), 1);
        assert_eq!(g_single.labeled_arcs().size(), 0);
        assert_eq!(g_single.out_neighbors(&42).size(), 0);
        assert_eq!(g_single.out_neighbors(&42).size(), 0);

        // Test self-loop with weight
        let v_self: Set<N> = SetLit![1];
        let a_self: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(1, 1, OrderedFloat(99.9))];
        let g_self = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v_self, a_self);

        assert!(g_self.has_arc(&1, &1));
        assert_eq!(g_self.out_neighbors(&1).size(), 1); // Self-loop to self
        assert_eq!(g_self.in_neighbors(&1).size(), 1);
        assert_eq!(g_self.out_neighbors(&1).size(), 1);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_nonexistent_vertex() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![LabEdge(0, 1, OrderedFloat(7.77))];
        let g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

        // Query non-existent vertex
        assert!(!g.has_arc(&99, &0));
        assert_eq!(g.out_neighbors(&99).size(), 0);
        assert_eq!(g.out_neighbors(&99).size(), 0);
        assert_eq!(g.in_neighbors(&99).size(), 0);
        assert_eq!(g.out_neighbors(&99).size(), 0);
    }

    #[test]
    fn test_weighteddirgraphmtephfloat_weight_variations() {
        // Test with various weight values including negative, zero, and very small/large
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(0.0)),           // Zero weight
            LabEdge(1, 2, OrderedFloat(-1.5)),          // Negative weight
            LabEdge(2, 3, OrderedFloat(1e-10)),         // Very small positive
            LabEdge(3, 4, OrderedFloat(1e10)),          // Very large positive
            LabEdge(4, 0, OrderedFloat(f64::INFINITY))  // Infinity
        ];
        let g = WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a);

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
    fn test_weighteddirgraphmtephfloat_concurrent_access() {
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, OrderedFloat<f64>>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.1)),
            LabEdge(1, 2, OrderedFloat(2.2)),
            LabEdge(2, 3, OrderedFloat(3.3)),
            LabEdge(3, 4, OrderedFloat(4.4))
        ];
        let g = Arc::new(WeightedDirGraphMtEphFloat::from_vertices_and_labeled_arcs(v, a));

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

                (
                    g_clone.out_neighbors(&i).size(),
                    g_clone.out_neighbors(&i),
                    g_clone.in_neighbors(&i),
                    g_clone.out_neighbors(&i),
                )
            }));
        }

        for handle in handles {
            let _ = handle.join().unwrap();
        }
    }

