//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Topological Sort - Sequential Ephemeral (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using ephemeral structures.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod TopoSortStEph {

    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    pub trait TopoSortStEphTrait {
        /// Computes topological sort of a DAG
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N>;
    }

    /// Computes topological sort of a DAG.
    /// Returns sequence of vertices in topological order (respecting edge directions).
    pub fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<N> {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_| false, n);
        let mut result = Vec::new();

        for start in 0..n {
            if !*visited.nth(start) {
                dfs_finish_order(graph, &mut visited, &mut result, start);
            }
        }
        AVLTreeSeqStEphS::from_vec(result)
    }

    fn dfs_finish_order(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        result: &mut Vec<N>,
        vertex: N,
    ) {
        if *visited.nth(vertex) {
            return;
        }

        let _ = visited.set(vertex, true);
        let neighbors = graph.nth(vertex);

        for i in 0..neighbors.length() {
            let neighbor = *neighbors.nth(i);
            dfs_finish_order(graph, visited, result, neighbor);
        }

        result.insert(0, vertex);
    }
}
