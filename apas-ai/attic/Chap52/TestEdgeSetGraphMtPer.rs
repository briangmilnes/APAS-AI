#[cfg(test)]
mod tests {
    use apas_ai::Chap52::EdgeSetGraphMtPer::EdgeSetGraphMtPer::*;

    #[test]
    fn test_new_graph() {
        let graph = EdgeSetGraphMtPerS::<i32>::new();
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_insert_vertices() {
        let graph = EdgeSetGraphMtPerS::new();
        let graph = graph.insert_vertex(1);
        let graph = graph.insert_vertex(2);
        let graph = graph.insert_vertex(3);
        assert_eq!(graph.vertex_count(), 3);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_insert_edges() {
        let graph = EdgeSetGraphMtPerS::new();
        let graph = graph.insert_vertex(1);
        let graph = graph.insert_vertex(2);
        let graph = graph.insert_edge(1, 2);
        assert_eq!(graph.vertex_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn test_directed_semantics() {
        let graph = EdgeSetGraphMtPerS::new();
        let graph = graph.insert_vertex(1);
        let graph = graph.insert_vertex(2);
        let graph = graph.insert_edge(1, 2);
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn test_delete_edge() {
        let graph = EdgeSetGraphMtPerS::new();
        let graph = graph.insert_vertex(1);
        let graph = graph.insert_vertex(2);
        let graph = graph.insert_edge(1, 2);
        assert!(graph.has_edge(&1, &2));
        let graph = graph.delete_edge(&1, &2);
        assert!(!graph.has_edge(&1, &2));
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_delete_vertex() {
        let graph = EdgeSetGraphMtPerS::new();
        let graph = graph.insert_vertex(1);
        let graph = graph.insert_vertex(2);
        let graph = graph.insert_edge(1, 2);
        let graph = graph.delete_vertex(&1);
        assert_eq!(graph.vertex_count(), 1);
    }

    #[test]
    fn test_persistence() {
        let graph1 = EdgeSetGraphMtPerS::new();
        let graph2 = graph1.clone().insert_vertex(1);
        let graph3 = graph2.clone().insert_vertex(2);
        assert_eq!(graph1.vertex_count(), 0);
        assert_eq!(graph2.vertex_count(), 1);
        assert_eq!(graph3.vertex_count(), 2);
    }
}
