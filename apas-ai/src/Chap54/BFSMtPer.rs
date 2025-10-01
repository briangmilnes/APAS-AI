//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Parallel Persistent (Chapter 54).
//! Layer-by-layer parallel BFS for finding distances and reachability.
//! Work: O(|V| + |E|), Span: O(d·lg n) where d is diameter.

pub mod BFSMtPer {
    use std::collections::VecDeque;

    use crate::Types::Types::*;
    use crate::ParaPair;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::{ArraySeqMtPerS, ArraySeqMtPerTrait};

    const UNREACHABLE: N = N::MAX;

    /// Performs parallel BFS from source vertex s on adjacency list graph G.
    /// Graph is represented as sequence of sequences (adjacency list).
    /// Returns array where result[v] = distance if reachable, UNREACHABLE otherwise.
    pub fn bfs(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> ArraySeqMtPerS<N> {
        let n = graph.length();
        if source >= n {
            return ArraySeqMtPerS::tabulate(&|_| UNREACHABLE, n);
        }

        let mut distances = ArraySeqMtPerS::tabulate(&|_| UNREACHABLE, n);
        distances = ArraySeqMtPerS::update(&distances, Pair(source, 0));

        let mut current_layer = VecDeque::new();
        current_layer.push_back(source);
        let mut current_dist = 0;

        while !current_layer.is_empty() {
            let layer_size = current_layer.len();
            let mut next_layer = VecDeque::new();

            for _ in 0..layer_size {
                if let Some(u) = current_layer.pop_front() {
                    let neighbors = graph.nth(u);
                    for i in 0..neighbors.length() {
                        let v = *neighbors.nth(i);
                        if *distances.nth(v) == UNREACHABLE {
                            distances = ArraySeqMtPerS::update(&distances, Pair(v, current_dist + 1));
                            next_layer.push_back(v);
                        }
                    }
                }
            }

            current_layer = next_layer;
            current_dist += 1;
        }

        distances
    }
}
