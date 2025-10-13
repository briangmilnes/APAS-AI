#[cfg(test)]
mod tests {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedDirGraphMtEphFloat::WeightedDirGraphMtEphFloat::*;
    use apas_ai::Chap59::JohnsonMtEphFloat::JohnsonMtEphFloat::johnson_apsp;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::OrderedF64;

    #[test]
    fn test_simple_graph() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(5.5)),
            (1, 2, OrderedF64::from(3.2)),
            (0, 2, OrderedF64::from(10.0))
        ];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(0, 1), OrderedF64::from(5.5));
        assert_eq!(result.get_distance(0, 2), OrderedF64::from(8.7));

        assert_eq!(result.get_distance(1, 1), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(1, 2), OrderedF64::from(3.2));
        assert_eq!(result.get_distance(1, 0), OrderedF64::from(f64::INFINITY));
    }

    #[test]
    fn test_negative_weights() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(1.5)),
            (1, 2, OrderedF64::from(-0.8)),
            (0, 2, OrderedF64::from(1.0))
        ];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.7));
    }

    #[test]
    fn test_single_vertex() {
        let vertices = SetLit![0];
        let edges = Set::empty();

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
    }

    #[test]
    fn test_fractional_weights() {
        let vertices = SetLit![0, 1, 2, 3];
        let edges = SetLit![
            (0, 1, OrderedF64::from(0.5)),
            (0, 2, OrderedF64::from(1.5)),
            (1, 2, OrderedF64::from(-0.25)),
            (1, 3, OrderedF64::from(1.0)),
            (2, 3, OrderedF64::from(0.5))
        ];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(0, 1), OrderedF64::from(0.5));
        assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.25));
        assert_eq!(result.get_distance(0, 3), OrderedF64::from(0.75));
    }

    #[test]
    fn test_disconnected_graph() {
        let vertices = SetLit![0, 1, 2, 3];
        let edges = SetLit![(0, 1, OrderedF64::from(2.5)), (2, 3, OrderedF64::from(1.8))];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 1), OrderedF64::from(2.5));
        assert_eq!(result.get_distance(2, 3), OrderedF64::from(1.8));
        assert_eq!(result.get_distance(0, 2), OrderedF64::from(f64::INFINITY));
        assert_eq!(result.get_distance(1, 3), OrderedF64::from(f64::INFINITY));
    }

    #[test]
    fn test_two_vertex_cycle() {
        let vertices = SetLit![0, 1];
        let edges = SetLit![(0, 1, OrderedF64::from(1.0)), (1, 0, OrderedF64::from(2.0))];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 1), OrderedF64::from(1.0));
        assert_eq!(result.get_distance(1, 0), OrderedF64::from(2.0));
    }

    #[test]
    fn test_triangle() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(1.0)),
            (1, 2, OrderedF64::from(1.0)),
            (2, 0, OrderedF64::from(1.0))
        ];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 2), OrderedF64::from(2.0));
        assert_eq!(result.get_distance(1, 0), OrderedF64::from(2.0));
    }

    #[test]
    fn test_zero_weights() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![(0, 1, OrderedF64::from(0.0)), (1, 2, OrderedF64::from(0.0))];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 2), OrderedF64::from(0.0));
    }

    #[test]
    fn test_large_weights() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![(0, 1, OrderedF64::from(1000.5)), (1, 2, OrderedF64::from(2000.3))];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 2), OrderedF64::from(3000.8));
    }

    #[test]
    fn test_self_loop() {
        let vertices = SetLit![0, 1];
        let edges = SetLit![(0, 0, OrderedF64::from(1.0)), (0, 1, OrderedF64::from(2.0))];

        let graph = WeightedDirGraphMtEphFloat::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(0, 1), OrderedF64::from(2.0));
    }
}
