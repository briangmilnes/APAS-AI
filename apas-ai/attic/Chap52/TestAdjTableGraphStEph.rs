#[cfg(test)]
mod tests {
    use apas_ai::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::*;

    #[test]
    fn test_new_graph() {
        let graph = AdjTableGraphStEphS::<i32>::new();
        assert_eq!(graph.vertex_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_insert_vertices() {
        let mut graph = AdjTableGraphStEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_vertex(3);
        assert_eq!(graph.vertex_count(), 3);
    }

    #[test]
    fn test_insert_edges() {
        let mut graph = AdjTableGraphStEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        assert_eq!(graph.vertex_count(), 2);
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn test_directed_semantics() {
        let mut graph = AdjTableGraphStEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn test_delete_edge() {
        let mut graph = AdjTableGraphStEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        assert!(graph.has_edge(&1, &2));
        graph.delete_edge(&1, &2);
        assert!(!graph.has_edge(&1, &2));
    }

    #[test]
    fn test_delete_vertex() {
        let mut graph = AdjTableGraphStEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        graph.delete_vertex(&1);
        assert_eq!(graph.vertex_count(), 1);
    }
}
