#[cfg(test)]
mod tests {
    use apas_ai::Chap52::EdgeSetGraphMtEph::EdgeSetGraphMtEph::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_new_graph() {
        let graph = EdgeSetGraphMtEphS::<i32>::new();
        assert_eq!(graph.vertex_count(), 0);
        assert_eq!(graph.edge_count(), 0);
        assert!(graph.is_empty());
    }

    #[test]
    fn test_insert_vertices() {
        let graph = EdgeSetGraphMtEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_vertex(3);
        assert_eq!(graph.vertex_count(), 3);
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_insert_edges() {
        let graph = EdgeSetGraphMtEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        assert_eq!(graph.vertex_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn test_directed_semantics() {
        let graph = EdgeSetGraphMtEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        assert!(graph.has_edge(&1, &2));
        assert!(!graph.has_edge(&2, &1));
    }

    #[test]
    fn test_delete_edge() {
        let graph = EdgeSetGraphMtEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        assert!(graph.has_edge(&1, &2));
        graph.delete_edge(&1, &2);
        assert!(!graph.has_edge(&1, &2));
        assert_eq!(graph.edge_count(), 0);
    }

    #[test]
    fn test_delete_vertex() {
        let graph = EdgeSetGraphMtEphS::new();
        graph.insert_vertex(1);
        graph.insert_vertex(2);
        graph.insert_edge(1, 2);
        graph.delete_vertex(&1);
        assert_eq!(graph.vertex_count(), 1);
    }

    #[test]
    fn test_thread_safety() {
        let graph = Arc::new(EdgeSetGraphMtEphS::new());
        let handles: Vec<_> = (0..4).map(|i| {
            let g = Arc::clone(&graph);
            thread::spawn(move || {
                let g_local = (*g).clone();
                g_local.insert_vertex(i);
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
    }
}
