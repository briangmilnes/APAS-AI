//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Sequential Ephemeral (Chapter 54, Algorithm 54.3).
//! Queue-based BFS for finding distances and reachability from a source vertex.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod BFSStEph {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Types::Types::*;
    use std::collections::VecDeque;

    const UNREACHABLE: N = N::MAX;

    /// Performs BFS from source vertex s on adjacency list graph G.
    /// Graph is represented as sequence of sequences (adjacency list).
    /// Returns array where result[v] = distance if reachable, UNREACHABLE otherwise.
    pub fn bfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> ArraySeqStEphS<N> {
        let n = graph.length();
        if source >= n {
            return ArraySeqStEphS::tabulate(&|_| UNREACHABLE, n);
        }

        let mut distances = ArraySeqStEphS::tabulate(&|_| UNREACHABLE, n);
        let _ = distances.set(source, 0);

        let mut queue = VecDeque::new();
        queue.push_back(source);

        while let Some(u) = queue.pop_front() {
            let dist = *distances.nth(u);
            if dist != UNREACHABLE {
                let neighbors = graph.nth(u);
                for i in 0..neighbors.length() {
                    let v = *neighbors.nth(i);
                    if *distances.nth(v) == UNREACHABLE {
                        let _ = distances.set(v, dist + 1);
                        queue.push_back(v);
                    }
                }
            }
        }

        distances
    }
}
