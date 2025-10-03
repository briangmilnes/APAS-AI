// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 66: Borůvka's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 66.2 and 66.3: Borůvka's algorithm for computing Minimum Spanning Trees
//! using vertex bridges and graph contraction with randomized star contraction.

pub mod BoruvkaStEph {

    use std::collections::HashMap;
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    use ordered_float::OrderedFloat;
    use rand::rngs::StdRng;
    use rand::*;

    pub trait BoruvkaStEphTrait {
        /// Find vertex bridges for Borůvka's algorithm
        /// APAS: Work O(|E|), Span O(|E|)
        fn vertex_bridges<V: StT + Hash + Ord>(edges: &Set<LabeledEdge<V>>) -> Set<(V, LabeledEdge<V>)>;

        /// Bridge-based star partition
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn bridge_star_partition<V: StT + Hash + Ord>(
            vertices: &Set<V>,
            bridges: &Set<(V, LabeledEdge<V>)>,
        ) -> Set<Set<V>>;

        /// Borůvka's MST algorithm
        /// APAS: Work O(m log n), Span O(m log n)
        fn boruvka_mst<V: StT + Hash + Ord>(edges: &Set<LabeledEdge<V>>) -> Set<LabeledEdge<V>>;

        /// Borůvka's MST with random seed
        /// APAS: Work O(m log n), Span O(m log n)
        fn boruvka_mst_with_seed<V: StT + Hash + Ord>(edges: &Set<LabeledEdge<V>>, seed: u64) -> Set<LabeledEdge<V>>;

        /// Compute total weight of MST
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(mst: &Set<LabeledEdge<V>>) -> OrderedFloat<f64>;
    }

    /// Edge with label: (u, v, weight, label)
    /// Vertices u,v change during contraction, but weight and label are immutable
    pub type LabeledEdge<V> = (V, V, OrderedFloat<f64>, usize);

    /// Algorithm 66.3: Find vertex bridges
    ///
    /// For each vertex, find the minimum weight edge incident on it.
    /// Returns a table mapping each vertex to (neighbor, weight, label).
    ///
    /// APAS: Work O(m), Span O(m)
    /// claude-4-sonet: Work O(m), Span O(m) [sequential iteration]
    ///
    /// Arguments:
    /// - edges: Set of labeled edges
    ///
    /// Returns:
    /// - HashMap mapping vertex → (neighbor, weight, label) for minimum edge
    pub fn vertex_bridges<V: StT + Hash + Ord>(
        edges: &Set<LabeledEdge<V>>,
    ) -> HashMap<V, (V, OrderedFloat<f64>, usize)> {
        let mut bridges: HashMap<V, (V, OrderedFloat<f64>, usize)> = HashMap::new();

        for edge in edges.iter() {
            let (u, v, w, label) = edge.clone();

            // Update bridge for u
            match bridges.get(&u) {
                | None => {
                    let _ = bridges.insert(u.clone(), (v.clone(), w, label));
                }
                | Some((_, existing_w, _)) => {
                    if w < *existing_w {
                        let _ = bridges.insert(u.clone(), (v.clone(), w, label));
                    }
                }
            }

            // Update bridge for v
            match bridges.get(&v) {
                | None => {
                    let _ = bridges.insert(v.clone(), (u.clone(), w, label));
                }
                | Some((_, existing_w, _)) => {
                    if w < *existing_w {
                        let _ = bridges.insert(v.clone(), (u.clone(), w, label));
                    }
                }
            }
        }

        bridges
    }

    /// Algorithm 66.2: Bridge star partition
    ///
    /// Performs star contraction along vertex bridges using randomized coin flips.
    /// Each vertex flips a coin (Heads/Tails). Edges from Tail→Head are contracted.
    ///
    /// APAS: Work O(n), Span O(n)
    /// claude-4-sonet: Work O(n), Span O(n) [sequential]
    ///
    /// Arguments:
    /// - vertices: Set of vertices
    /// - bridges: Vertex bridges (from vertex_bridges)
    /// - rng: Random number generator for coin flips
    ///
    /// Returns:
    /// - (remaining_vertices, partition_map) where partition_map: tail → (head, weight, label)
    pub fn bridge_star_partition<V: StT + Hash + Ord>(
        vertices: &Set<V>,
        bridges: &HashMap<V, (V, OrderedFloat<f64>, usize)>,
        rng: &mut StdRng,
    ) -> (Set<V>, HashMap<V, (V, OrderedFloat<f64>, usize)>) {
        // Coin flips for all vertices
        let mut flips: HashMap<V, bool> = HashMap::new();
        for v in vertices.iter() {
            let is_heads = rng.random::<bool>();
            let _ = flips.insert(v.clone(), is_heads);
        }

        // Select edges from Tail→Head (Tail=false, Head=true)
        let mut partition: HashMap<V, (V, OrderedFloat<f64>, usize)> = HashMap::new();
        for (u, (v, w, label)) in bridges.iter() {
            let u_heads = flips.get(u).copied().unwrap_or(false);
            let v_heads = flips.get(v).copied().unwrap_or(false);

            // Contract if u is Tail and v is Head
            if !u_heads && v_heads {
                let _ = partition.insert(u.clone(), (v.clone(), *w, *label));
            }
        }

        // Remaining vertices = all vertices minus contracted tails
        let mut remaining = SetLit![];
        for v in vertices.iter() {
            if !partition.contains_key(v) {
                let _ = remaining.insert(v.clone());
            }
        }

        (remaining, partition)
    }

    /// Algorithm 66.3: Borůvka's MST
    ///
    /// Computes the Minimum Spanning Tree using recursive bridge-based contraction.
    /// Returns the set of edge labels in the MST.
    ///
    /// APAS: Work O(m log n), Span O(m log n)
    /// claude-4-sonet: Work O(m log n), Span O(m log n) [sequential, expected O(log n) rounds]
    ///
    /// Arguments:
    /// - vertices: Set of vertices
    /// - edges: Set of labeled edges
    /// - mst_labels: Accumulated MST edge labels
    /// - rng: Random number generator
    ///
    /// Returns:
    /// - Set of edge labels in the MST
    pub fn boruvka_mst<V: StT + Hash + Ord>(
        vertices: &Set<V>,
        edges: &Set<LabeledEdge<V>>,
        mst_labels: Set<usize>,
        rng: &mut StdRng,
    ) -> Set<usize> {
        // Base case: no edges remaining
        if edges.size() == 0 {
            return mst_labels;
        }

        // Find vertex bridges
        let bridges = vertex_bridges(edges);

        // Perform bridge star partition
        let (remaining_vertices, partition) = bridge_star_partition(vertices, &bridges, rng);

        // Collect new MST labels from partition
        let mut new_mst_labels = mst_labels.clone();
        for (_, (_, _, label)) in partition.iter() {
            let _ = new_mst_labels.insert(*label);
        }

        // Build full partition map (including identity for non-contracted vertices)
        let mut full_partition: HashMap<V, V> = HashMap::new();
        for (tail, (head, _, _)) in partition.iter() {
            let _ = full_partition.insert(tail.clone(), head.clone());
        }
        for v in remaining_vertices.iter() {
            let _ = full_partition.insert(v.clone(), v.clone());
        }

        // Re-route edges to new endpoints, removing self-edges
        let mut new_edges = SetLit![];
        for (u, v, w, label) in edges.iter() {
            let new_u = full_partition.get(u).cloned().unwrap_or_else(|| u.clone());
            let new_v = full_partition.get(v).cloned().unwrap_or_else(|| v.clone());

            // Skip self-edges
            if new_u != new_v {
                let _ = new_edges.insert((new_u, new_v, *w, *label));
            }
        }

        // Recurse
        boruvka_mst(&remaining_vertices, &new_edges, new_mst_labels, rng)
    }

    /// Helper: Create Borůvka MST with a specific seed
    ///
    /// APAS: Work O(m log n), Span O(m log n)
    /// claude-4-sonet: Work O(m log n), Span O(m log n)
    ///
    /// Arguments:
    /// - vertices: Set of vertices
    /// - edges: Set of labeled edges
    /// - seed: Random seed for reproducibility
    ///
    /// Returns:
    /// - Set of edge labels in the MST
    pub fn boruvka_mst_with_seed<V: StT + Hash + Ord>(
        vertices: &Set<V>,
        edges: &Set<LabeledEdge<V>>,
        seed: u64,
    ) -> Set<usize> {
        let mut rng = StdRng::seed_from_u64(seed);
        boruvka_mst(vertices, edges, SetLit![], &mut rng)
    }

    /// Compute MST weight from edge labels
    ///
    /// APAS: Work O(m), Span O(m)
    /// claude-4-sonet: Work O(m), Span O(m)
    pub fn mst_weight<V: StT + Hash>(edges: &Set<LabeledEdge<V>>, mst_labels: &Set<usize>) -> OrderedFloat<f64> {
        let mut total = OrderedFloat(0.0);
        for (_, _, w, label) in edges.iter() {
            if mst_labels.mem(label) {
                total += *w;
            }
        }
        total
    }
}
