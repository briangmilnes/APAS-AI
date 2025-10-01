// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 63: Graph Connectivity - Multi-threaded Ephemeral Tests

#[cfg(test)]
mod tests {
    use apas_ai::{
        Chap05::SetStEph::SetStEph::*,
        Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
        Chap63::ConnectivityMtEph::ConnectivityMtEph::*,
        SetLit,
        Types::Types::*,
    };

    fn create_connected_graph() -> UnDirGraphMtEph<N> {
        let mut vertices = SetLit![];
        for i in 0..6 {
            let _ = vertices.insert(i);
        }
        let mut edges = SetLit![];
        for i in 0..6 {
            let _ = edges.insert(Edge(i, (i + 1) % 6));
        }
        <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges)
    }

    fn create_multi_component_graph() -> UnDirGraphMtEph<N> {
        let mut vertices = SetLit![];
        for i in 0..8 {
            let _ = vertices.insert(i);
        }
        let mut edges = SetLit![];
        let _ = edges.insert(Edge(0, 1));
        let _ = edges.insert(Edge(1, 2));
        let _ = edges.insert(Edge(3, 4));
        let _ = edges.insert(Edge(5, 6));
        let _ = edges.insert(Edge(6, 7));
        <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges)
    }

    #[test]
    fn test_count_components_mt_single() {
        let graph = create_connected_graph();
        let count = count_components_mt(&graph, 123);
        assert_eq!(count, 1);
    }

    #[test]
    fn test_count_components_mt_multiple() {
        let graph = create_multi_component_graph();
        let count = count_components_mt(&graph, 456);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_connected_components_mt_single() {
        let graph = create_connected_graph();
        let (reps, comp_map) = connected_components_mt(&graph, 123);
        
        assert_eq!(reps.size(), 1);
        
        let first_comp = comp_map.get(&0).unwrap();
        for i in 1..6 {
            assert_eq!(comp_map.get(&i).unwrap(), first_comp);
        }
    }

    #[test]
    fn test_connected_components_mt_multiple() {
        let graph = create_multi_component_graph();
        let (reps, comp_map) = connected_components_mt(&graph, 789);
        
        assert_eq!(reps.size(), 3);
        
        // Vertices in same component map to same representative
        let comp0 = comp_map.get(&0).unwrap();
        assert_eq!(comp_map.get(&1).unwrap(), comp0);
        assert_eq!(comp_map.get(&2).unwrap(), comp0);
        
        let comp3 = comp_map.get(&3).unwrap();
        assert_eq!(comp_map.get(&4).unwrap(), comp3);
        
        let comp5 = comp_map.get(&5).unwrap();
        assert_eq!(comp_map.get(&6).unwrap(), comp5);
        assert_eq!(comp_map.get(&7).unwrap(), comp5);
        
        assert_ne!(comp0, comp3);
        assert_ne!(comp0, comp5);
        assert_ne!(comp3, comp5);
    }

    #[test]
    fn test_count_components_hof_mt() {
        let graph = create_multi_component_graph();
        let count_hof = count_components_hof(&graph, 999);
        let count_direct = count_components_mt(&graph, 999);
        assert_eq!(count_hof, count_direct);
        assert_eq!(count_hof, 3);
    }
}

