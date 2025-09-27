//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
pub mod TestWeightedDirGraphStEphFloat {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use apas_ai::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::*;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_weighteddirgraphstephfloat_empty() {
        let empty_graph: WeightedDirGraphStEphFloat<i32> = WeightedDirGraphStEphFloat::empty();
        assert_eq!(empty_graph.vertices().size(), 0);
        assert_eq!(empty_graph.labeled_arcs().size(), 0);
        assert_eq!(empty_graph.arcs().size(), 0);
    }

    #[test]
    fn test_weighteddirgraphstephfloat_basic_operations() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedF64>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.5)),
            LabEdge(1, 2, OrderedFloat(2.7)),
            LabEdge(2, 3, OrderedFloat(0.8))
        ];
        let g = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);
        
        assert_eq!(g.vertices().size(), 4);
        assert_eq!(g.labeled_arcs().size(), 3);
        
        // Test arc relationships
        assert_eq!(g.has_arc(&0, &1), true);
        assert_eq!(g.has_arc(&1, &0), false); // Directed graph
        assert_eq!(g.has_arc(&1, &2), true);
        assert_eq!(g.has_arc(&2, &1), false);
        
        // Test out-neighbors
        let out0 = g.out_neighbors(&0);
        assert_eq!(out0.size(), 1);
        assert_eq!(out0.mem(&1), B::True);
        
        let out1 = g.out_neighbors(&1);
        assert_eq!(out1.size(), 1);
        assert_eq!(out1.mem(&2), B::True);
        
        // Test in-neighbors
        let in1 = g.in_neighbors(&1);
        assert_eq!(in1.size(), 1);
        assert_eq!(in1.mem(&0), B::True);
        
        let in3 = g.in_neighbors(&3);
        assert_eq!(in3.size(), 1);
        assert_eq!(in3.mem(&2), B::True);
        
        // Test arc weights
        assert_eq!(g.get_arc_label(&0, &1), Some(&OrderedFloat(1.5)));
        assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(2.7)));
        assert_eq!(g.get_arc_label(&2, &3), Some(&OrderedFloat(0.8)));
        assert_eq!(g.get_arc_label(&0, &2), None); // No direct arc
    }

    #[test]
    fn test_weighteddirgraphstephfloat_mutable_operations() {
        let mut g: WeightedDirGraphStEphFloat<i32> = WeightedDirGraphStEphFloat::empty();
        
        // Add vertices
        g.add_vertex(0);
        g.add_vertex(1);
        g.add_vertex(2);
        
        assert_eq!(g.vertices().size(), 3);
        assert_eq!(g.vertices().mem(&0), B::True);
        assert_eq!(g.vertices().mem(&1), B::True);
        assert_eq!(g.vertices().mem(&2), B::True);
        
        // Add weighted arcs
        g.add_labeled_arc(0, 1, OrderedFloat(3.14));
        g.add_labeled_arc(1, 2, OrderedFloat(2.71));
        
        assert_eq!(g.labeled_arcs().size(), 2);
        assert_eq!(g.has_arc(&0, &1), true);
        assert_eq!(g.has_arc(&1, &2), true);
        assert_eq!(g.has_arc(&0, &2), false);
        
        // Test weights
        assert_eq!(g.get_arc_label(&0, &1), Some(&OrderedFloat(3.14)));
        assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(2.71)));
    }

    #[test]
    fn test_weighteddirgraphstephfloat_weight_variations() {
        // Test with various weight values including negative, zero, and very small/large
        let v: Set<N> = SetLit![0, 1, 2, 3, 4];
        let a: Set<LabEdge<N, OrderedF64>> = SetLit![
            LabEdge(0, 1, OrderedFloat(0.0)),           // Zero weight
            LabEdge(1, 2, OrderedFloat(-1.5)),          // Negative weight
            LabEdge(2, 3, OrderedFloat(1e-10)),         // Very small positive
            LabEdge(3, 4, OrderedFloat(1e10)),          // Very large positive
            LabEdge(4, 0, OrderedFloat(f64::INFINITY))  // Infinity
        ];
        let g = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);
        
        assert_eq!(g.vertices().size(), 5);
        assert_eq!(g.labeled_arcs().size(), 5);
        
        // All edges should still be recognized regardless of weight
        assert_eq!(g.has_arc(&0, &1), true);
        assert_eq!(g.has_arc(&1, &2), true);
        assert_eq!(g.has_arc(&2, &3), true);
        assert_eq!(g.has_arc(&3, &4), true);
        assert_eq!(g.has_arc(&4, &0), true);
        
        // Test specific weights
        assert_eq!(g.get_arc_label(&0, &1), Some(&OrderedFloat(0.0)));
        assert_eq!(g.get_arc_label(&1, &2), Some(&OrderedFloat(-1.5)));
        assert_eq!(g.get_arc_label(&2, &3), Some(&OrderedFloat(1e-10)));
        assert_eq!(g.get_arc_label(&3, &4), Some(&OrderedFloat(1e10)));
        assert_eq!(g.get_arc_label(&4, &0), Some(&OrderedFloat(f64::INFINITY)));
    }

    #[test]
    fn test_weighteddirgraphstephfloat_nan_handling() {
        // Test with NaN weights (OrderedFloat handles NaN consistently)
        let v: Set<N> = SetLit![0, 1];
        let a: Set<LabEdge<N, OrderedF64>> = SetLit![
            LabEdge(0, 1, OrderedFloat(f64::NAN))
        ];
        let g = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);
        
        assert_eq!(g.vertices().size(), 2);
        assert_eq!(g.labeled_arcs().size(), 1);
        assert_eq!(g.has_arc(&0, &1), true);
        
        // NaN should be handled consistently by OrderedFloat
        let weight = g.get_arc_label(&0, &1);
        assert!(weight.is_some());
        assert!(weight.unwrap().is_nan());
    }

    #[test]
    fn test_weighteddirgraphstephfloat_edge_cases() {
        // Test empty graph
        let empty: WeightedDirGraphStEphFloat<i32> = WeightedDirGraphStEphFloat::empty();
        assert_eq!(empty.has_arc(&0, &1), false);
        assert_eq!(empty.out_neighbors(&0).size(), 0);
        assert_eq!(empty.in_neighbors(&0).size(), 0);
        assert_eq!(empty.get_arc_label(&0, &1), None);
        
        // Test single vertex
        let v_single: Set<N> = SetLit![42];
        let a_empty: Set<LabEdge<N, OrderedF64>> = SetLit![];
        let g_single = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v_single, a_empty);
        
        assert_eq!(g_single.vertices().size(), 1);
        assert_eq!(g_single.labeled_arcs().size(), 0);
        assert_eq!(g_single.out_neighbors(&42).size(), 0);
        assert_eq!(g_single.in_neighbors(&42).size(), 0);
        
        // Test self-loop with weight
        let v_self: Set<N> = SetLit![1];
        let a_self: Set<LabEdge<N, OrderedF64>> = SetLit![LabEdge(1, 1, OrderedFloat(99.9))];
        let g_self = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v_self, a_self);
        
        assert_eq!(g_self.has_arc(&1, &1), true);
        assert_eq!(g_self.out_neighbors(&1).size(), 1);
        assert_eq!(g_self.in_neighbors(&1).size(), 1);
        assert_eq!(g_self.get_arc_label(&1, &1), Some(&OrderedFloat(99.9)));
    }

    #[test]
    fn test_weighteddirgraphstephfloat_nonexistent_vertex() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, OrderedF64>> = SetLit![LabEdge(0, 1, OrderedFloat(7.77))];
        let g = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);
        
        // Query non-existent vertex
        assert_eq!(g.has_arc(&99, &0), false);
        assert_eq!(g.out_neighbors(&99).size(), 0);
        assert_eq!(g.in_neighbors(&99).size(), 0);
        assert_eq!(g.get_arc_label(&99, &0), None);
    }

    #[test]
    fn test_weighteddirgraphstephfloat_arcs_conversion() {
        let v: Set<N> = SetLit![0, 1, 2];
        let a: Set<LabEdge<N, OrderedF64>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.1)),
            LabEdge(1, 2, OrderedFloat(2.2))
        ];
        let g = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);
        
        // Test arcs() method that converts weighted arcs to unlabeled edges
        let arcs = g.arcs();
        assert_eq!(arcs.size(), 2);
        assert_eq!(arcs.mem(&Edge(0, 1)), B::True);
        assert_eq!(arcs.mem(&Edge(1, 2)), B::True);
        assert_eq!(arcs.mem(&Edge(0, 2)), B::False);
    }

    #[test]
    fn test_weighteddirgraphstephfloat_complex_topology() {
        let v: Set<N> = SetLit![0, 1, 2, 3];
        let a: Set<LabEdge<N, OrderedF64>> = SetLit![
            LabEdge(0, 1, OrderedFloat(1.0)),
            LabEdge(1, 2, OrderedFloat(2.0)),
            LabEdge(2, 3, OrderedFloat(3.0)),
            LabEdge(0, 3, OrderedFloat(4.0)),
            LabEdge(1, 3, OrderedFloat(5.0))
        ];
        let g = WeightedDirGraphStEphFloat::from_vertices_and_labeled_arcs(v, a);
        
        // Test multiple paths and weights
        assert_eq!(g.vertices().size(), 4);
        assert_eq!(g.labeled_arcs().size(), 5);
        
        // Test out-neighbors with multiple edges
        let out0 = g.out_neighbors(&0);
        assert_eq!(out0.size(), 2);
        assert_eq!(out0.mem(&1), B::True);
        assert_eq!(out0.mem(&3), B::True);
        
        let out1 = g.out_neighbors(&1);
        assert_eq!(out1.size(), 2);
        assert_eq!(out1.mem(&2), B::True);
        assert_eq!(out1.mem(&3), B::True);
        
        // Test in-neighbors with multiple edges
        let in3 = g.in_neighbors(&3);
        assert_eq!(in3.size(), 3);
        assert_eq!(in3.mem(&0), B::True);
        assert_eq!(in3.mem(&1), B::True);
        assert_eq!(in3.mem(&2), B::True);
        
        // Test different path weights to same destination
        assert_eq!(g.get_arc_label(&0, &3), Some(&OrderedFloat(4.0))); // Direct path
        assert_eq!(g.get_arc_label(&1, &3), Some(&OrderedFloat(5.0))); // Via vertex 1
    }
}
