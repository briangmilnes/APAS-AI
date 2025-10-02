// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: TSP 2-Approximation Tests (Sequential)

#[cfg(test)]
mod tests {
    use apas_ai::{
        Chap05::SetStEph::SetStEph::*, Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*,
        Chap64::TSPApproxStEph::TSPApproxStEph::*, SetLit, Types::Types::*,
    };
    use ordered_float::OrderedFloat;

    fn create_triangle_graph() -> (
        LabUnDirGraphStEph<N, OrderedFloat<f64>>,
        Set<LabEdge<N, OrderedFloat<f64>>>,
    ) {
        let mut vertices = SetLit![0, 1, 2];
        let mut edges = SetLit![
            LabEdge(0, 1, OrderedFloat(1.0)),
            LabEdge(1, 2, OrderedFloat(2.0)),
            LabEdge(2, 0, OrderedFloat(3.0))
        ];
        let graph = <LabUnDirGraphStEph<N, OrderedFloat<f64>> as LabUnDirGraphStEphTrait<N, OrderedFloat<f64>>>::from_vertices_and_labeled_edges(vertices, edges.clone());

        let spanning_tree = SetLit![LabEdge(0, 1, OrderedFloat(1.0)), LabEdge(1, 2, OrderedFloat(2.0))];

        (graph, spanning_tree)
    }

    #[test]
    fn test_euler_tour() {
        let (graph, tree) = create_triangle_graph();
        let tour = euler_tour(&graph, &0, &tree);

        assert!(tour.len() >= 3);
        assert_eq!(tour[0], 0);
    }

    #[test]
    fn test_shortcut_tour() {
        let tour_with_dups = std::vec![0, 1, 2, 1, 0, 0];
        let shortcut = shortcut_tour(&tour_with_dups);

        assert_eq!(shortcut, std::vec![0, 1, 2, 0]);
    }

    #[test]
    fn test_tour_weight() {
        let (graph, _) = create_triangle_graph();
        let tour = std::vec![0, 1, 2, 0];
        let weight = tour_weight(&graph, &tour);

        assert_eq!(weight, OrderedFloat(6.0));
    }

    #[test]
    fn test_approx_metric_tsp() {
        let (graph, tree) = create_triangle_graph();
        let (tour, weight) = approx_metric_tsp(&graph, &tree, &0);

        assert!(tour.len() >= 3);
        assert_eq!(tour[0], tour[tour.len() - 1]);
        assert!(weight > OrderedFloat(0.0));
    }
}
