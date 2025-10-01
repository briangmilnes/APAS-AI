#[cfg(test)]
mod tests {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::*;
    use apas_ai::Chap59::JohnsonMtEphInt::JohnsonMtEphInt::johnson_apsp;
    use apas_ai::SetLit;

    #[test]
    fn test_example_59_1() {
        let vertices = SetLit![0, 1, 2, 3];
        let edges = SetLit![
            (0, 1, 3),
            (0, 2, 8),
            (1, 2, -2),
            (1, 3, 1),
            (2, 0, 4),
            (2, 3, 7),
            (3, 1, 2)
        ];

        let graph = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), 0);
        assert_eq!(result.get_distance(0, 1), 3);
        assert_eq!(result.get_distance(0, 2), 1);
        assert_eq!(result.get_distance(0, 3), 4);

        assert_eq!(result.get_distance(1, 0), 2);
        assert_eq!(result.get_distance(1, 1), 0);
        assert_eq!(result.get_distance(1, 2), -2);
        assert_eq!(result.get_distance(1, 3), 1);

        assert_eq!(result.get_distance(2, 0), 4);
        assert_eq!(result.get_distance(2, 1), 7);
        assert_eq!(result.get_distance(2, 2), 0);
        assert_eq!(result.get_distance(2, 3), 7);
    }

    #[test]
    fn test_simple_graph() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![(0, 1, 5), (1, 2, 3), (0, 2, 10)];

        let graph = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), 0);
        assert_eq!(result.get_distance(0, 1), 5);
        assert_eq!(result.get_distance(0, 2), 8);

        assert_eq!(result.get_distance(1, 1), 0);
        assert_eq!(result.get_distance(1, 2), 3);
        assert_eq!(result.get_distance(1, 0), i64::MAX);

        assert_eq!(result.get_distance(2, 2), 0);
        assert_eq!(result.get_distance(2, 0), i64::MAX);
        assert_eq!(result.get_distance(2, 1), i64::MAX);
    }

    #[test]
    fn test_negative_weights() {
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![(0, 1, 1), (1, 2, -5), (0, 2, 3)];

        let graph = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 2), -4);
    }

    #[test]
    fn test_single_vertex() {
        let vertices = SetLit![0];
        let edges = Set::empty();

        let graph = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 0), 0);
    }

    #[test]
    fn test_disconnected_graph() {
        let vertices = SetLit![0, 1, 2, 3];
        let edges = SetLit![(0, 1, 5), (2, 3, 3)];

        let graph = WeightedDirGraphMtEphInt::from_weighted_edges(vertices, edges);
        let result = johnson_apsp(&graph);

        assert_eq!(result.get_distance(0, 1), 5);
        assert_eq!(result.get_distance(2, 3), 3);

        assert_eq!(result.get_distance(0, 2), i64::MAX);
        assert_eq!(result.get_distance(0, 3), i64::MAX);
        assert_eq!(result.get_distance(1, 2), i64::MAX);
    }
}
