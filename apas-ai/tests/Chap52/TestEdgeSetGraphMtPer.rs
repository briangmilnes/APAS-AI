//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for EdgeSetGraphMtPer

#[cfg(test)]
mod tests_edge_set_graph_mt_per {
    use apas_ai::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use apas_ai::Chap52::EdgeSetGraphMtPer::EdgeSetGraphMtPer::*;
    use apas_ai::Types::Types::*;

    #[test]
    fn test_empty_graph() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        assert_eq!(g.num_vertices(), 0);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_insert_vertex() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        let g = g.insert_vertex(1);
        assert_eq!(g.num_vertices(), 1);
        assert_eq!(g.num_edges(), 0);

        let g = g.insert_vertex(2);
        assert_eq!(g.num_vertices(), 2);
    }

    #[test]
    fn test_insert_edge() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        let g = g.insert_vertex(1);
        let g = g.insert_vertex(2);
        let g = g.insert_edge(1, 2);

        assert_eq!(g.num_edges(), 1);
        assert!(g.has_edge(&1, &2));
        assert!(!g.has_edge(&2, &1));
    }

    #[test]
    fn test_delete_vertex() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        let g = g.insert_vertex(1);
        let g = g.insert_vertex(2);
        let g = g.insert_edge(1, 2);

        let g = g.delete_vertex(&1);
        assert_eq!(g.num_vertices(), 1);
        assert_eq!(g.num_edges(), 0);
    }

    #[test]
    fn test_delete_edge() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        let g = g.insert_vertex(1);
        let g = g.insert_vertex(2);
        let g = g.insert_edge(1, 2);

        let g = g.delete_edge(&1, &2);
        assert_eq!(g.num_edges(), 0);
        assert!(!g.has_edge(&1, &2));
    }

    #[test]
    fn test_out_neighbors() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        let g = g.insert_vertex(1);
        let g = g.insert_vertex(2);
        let g = g.insert_vertex(3);
        let g = g.insert_edge(1, 2);
        let g = g.insert_edge(1, 3);

        let neighbors = g.out_neighbors(&1);
        assert_eq!(neighbors.size(), 2);
        assert!(neighbors.find(&2));
        assert!(neighbors.find(&3));
    }

    #[test]
    fn test_out_degree() {
        let g = EdgeSetGraphMtPer::<i32>::empty();
        let g = g.insert_vertex(1);
        let g = g.insert_vertex(2);
        let g = g.insert_vertex(3);
        let g = g.insert_edge(1, 2);
        let g = g.insert_edge(1, 3);

        assert_eq!(g.out_degree(&1), 2);
        assert_eq!(g.out_degree(&2), 0);
    }
}
