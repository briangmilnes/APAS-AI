// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 64: Spanning Tree via Star Contraction Tests (Parallel)

#[cfg(test)]
mod tests {
    use apas_ai::{
        Chap05::SetStEph::SetStEph::*, Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*,
        Chap62::StarPartitionMtEph::StarPartitionMtEph::*, Chap64::SpanTreeMtEph::SpanTreeMtEph::*, SetLit,
        Types::Types::*,
    };

    fn create_cycle_graph(n: N) -> UnDirGraphMtEph<N> {
        let mut vertices = SetLit![];
        for i in 0..n {
            let _ = vertices.insert(i);
        }
        let mut edges = SetLit![];
        for i in 0..n {
            let _ = edges.insert(Edge(i, (i + 1) % n));
        }
        <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges)
    }

    #[test]
    fn test_spanning_tree_mt_cycle() {
        let graph = create_cycle_graph(6);
        let tree = spanning_tree_star_contraction_mt(&graph, 123);

        assert_eq!(tree.size(), 5);
        assert!(verify_spanning_tree(&graph, &tree));
    }

    #[test]
    fn test_spanning_tree_mt_larger() {
        let graph = create_cycle_graph(10);
        let tree = spanning_tree_star_contraction_mt(&graph, 456);

        assert_eq!(tree.size(), 9);
        assert!(verify_spanning_tree(&graph, &tree));
    }

    #[test]
    fn test_spanning_tree_mt_empty() {
        let vertices = SetLit![];
        let edges = SetLit![];
        let graph = <UnDirGraphMtEph<N> as UnDirGraphMtEphTrait<N>>::FromSets(vertices, edges);
        let tree = spanning_tree_star_contraction_mt(&graph, 789);

        assert_eq!(tree.size(), 0);
    }
}
