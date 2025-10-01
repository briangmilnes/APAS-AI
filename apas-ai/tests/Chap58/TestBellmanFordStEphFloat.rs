//! Copyright © 2025 APAS-VERUS. All rights reserved.
//!
//! Tests for Bellman-Ford Algorithm (Float Weights)

#[cfg(test)]
mod tests {
    use apas_ai::Chap05::SetStEph::SetStEph::*;
    use apas_ai::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat;
    use apas_ai::Chap58::BellmanFordStEphFloat::BellmanFordStEphFloat::bellman_ford;
    use apas_ai::SetLit;
    use apas_ai::Types::Types::OrderedF64;

    #[test]
    fn test_basic_path() {
        // Simple path s -> a -> b
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(1.5)),
            (1, 2, OrderedF64::from(2.5))
        ];

        let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(result.get_distance(0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(1), OrderedF64::from(1.5));
        assert_eq!(result.get_distance(2), OrderedF64::from(4.0));
    }

    #[test]
    fn test_negative_edges() {
        // Test with negative edge weights
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(5.0)),
            (0, 2, OrderedF64::from(2.0)),
            (1, 2, OrderedF64::from(-4.0))
        ];

        let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(result.get_distance(0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(1), OrderedF64::from(5.0));
        assert_eq!(result.get_distance(2), OrderedF64::from(1.0)); // via 1
    }

    #[test]
    fn test_negative_cycle() {
        // Cycle with negative total weight
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(1.0)),
            (1, 2, OrderedF64::from(2.0)),
            (2, 1, OrderedF64::from(-4.0))
        ];

        let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
        let result = bellman_ford(&graph, 0);

        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Negative-weight cycle"));
    }

    #[test]
    fn test_fractional_weights() {
        // Test with fractional weights
        let vertices = SetLit![0, 1, 2, 3];
        let edges = SetLit![
            (0, 1, OrderedF64::from(0.5)),
            (1, 2, OrderedF64::from(1.25)),
            (2, 3, OrderedF64::from(-0.75))
        ];

        let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(result.get_distance(0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(1), OrderedF64::from(0.5));
        assert_eq!(result.get_distance(2), OrderedF64::from(1.75));
        assert_eq!(result.get_distance(3), OrderedF64::from(1.0));
    }

    #[test]
    fn test_unreachable() {
        // Test with unreachable vertex
        let vertices = SetLit![0, 1, 2];
        let edges = SetLit![
            (0, 1, OrderedF64::from(1.0))
        ];

        let graph = WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges);
        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(result.get_distance(0), OrderedF64::from(0.0));
        assert_eq!(result.get_distance(1), OrderedF64::from(1.0));
        assert_eq!(result.get_distance(2), OrderedF64::from(f64::INFINITY));
    }
}
