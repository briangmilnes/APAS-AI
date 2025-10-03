// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Multi-threaded Ephemeral Implementation
//!
//! Implements Algorithm 62.3: Parallel Star Partition using randomized coin flips.
//! Uses Seq.inject for efficient parallel updates.

pub mod StarPartitionMtEph {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    use rand::*;
    use rand::rngs::StdRng;

    pub trait StarPartitionMtEphTrait {
        /// Parallel star partition using randomized coin flips
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        fn parallel_star_partition<V: StT + MtT + Hash + Ord + 'static>(graph: &UnDirGraphMtEph<V>) -> Set<Set<V>>;
    }

    /// Algorithm 62.3: Parallel Star Partition
    ///
    /// Computes a star partition using randomized coin flips:
    /// 1. Flip a coin for each vertex
    /// 2. Find edges from tails (non-centers) to heads (centers)
    /// 3. Use Seq.inject to map satellites to centers
    /// 4. Remaining vertices become singleton centers
    ///
    /// APAS: Work O(n + m), Span O(lg n)
    /// claude-4-sonet: Work O(n + m), Span O(lg n), Parallelism Θ((n+m)/lg n)
    ///
    /// Arguments:
    /// - graph: The undirected graph to partition
    /// - seed: Random seed for coin flips
    ///
    /// Returns:
    /// - (centers, partition_map): Set of center vertices and mapping from each vertex to its center
    pub fn parallel_star_partition<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (Set<V>, HashMap<V, V>) {
        let mut rng = StdRng::seed_from_u64(seed);

        // Create vertex to index mapping for inject operation
        let vertices_vec: std::vec::Vec<V> = graph.vertices().iter().cloned().collect();
        let n = vertices_vec.len() as N;

        let mut vertex_to_index: HashMap<V, N> = HashMap::new();
        for (i, v) in vertices_vec.iter().enumerate() {
            let _ = vertex_to_index.insert(v.clone(), i as N);
        }

        // Phase 1: Flip coins for each vertex (heads = true, tails = false)
        let mut coin_flips: HashMap<V, bool> = HashMap::new();
        for vertex in vertices_vec.iter() {
            let _ = coin_flips.insert(vertex.clone(), rng.random::<bool>());
        }

        // Phase 2: Find edges from tails to heads (TH)
        let mut th_edges: std::vec::Vec<(N, V)> = std::vec::Vec::new();
        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;
            let u_heads = coin_flips.get(u).copied().unwrap_or(false);
            let v_heads = coin_flips.get(v).copied().unwrap_or(false);

            // Add edge if u is tails and v is heads
            if !u_heads && v_heads {
                if let Some(&u_idx) = vertex_to_index.get(u) {
                    th_edges.push((u_idx, v.clone()));
                }
            }
            // Add edge if v is tails and u is heads
            if !v_heads && u_heads {
                if let Some(&v_idx) = vertex_to_index.get(v) {
                    th_edges.push((v_idx, u.clone()));
                }
            }
        }

        // Phase 3: Build base sequence V' where each index maps to itself
        let mut base_seq = <ArraySeqStEphS<V> as ArraySeqStEphTrait<V>>::tabulate(&|i| vertices_vec[i as usize].clone(), n);

        // Phase 4: Convert th_edges to ArraySeqStEphS<Pair<usize, V>>
        let updates_seq = ArraySeqStEphS::from_vec(
            th_edges.into_iter().map(|(idx, vertex)| Pair(idx, vertex)).collect()
        );

        // Phase 5: Apply inject to get partition map P
        let p_seq = base_seq.inject(&updates_seq);

        // Phase 6: Extract centers (vertices where P[v] = v)
        let mut centers: Set<V> = SetLit![];
        let mut partition_map: HashMap<V, V> = HashMap::new();

        for (i, vertex) in vertices_vec.iter().enumerate() {
            let center = p_seq.nth(i as N).clone();
            let _ = partition_map.insert(vertex.clone(), center.clone());

            // A vertex is a center if it maps to itself
            if *vertex == center {
                let _ = centers.insert(vertex.clone());
            }
        }

        (centers, partition_map)
    }
}
