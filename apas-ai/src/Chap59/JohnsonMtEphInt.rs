//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson's Algorithm - Multi-threaded Ephemeral Integer Weights

pub mod JohnsonMtEphInt {
    use std::thread;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphMtEphInt::WeightedDirGraphMtEphInt::*;
    use crate::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphInt::AllPairsResultStEphInt;
    use crate::Chap57::DijkstraStEphInt::DijkstraStEphInt::dijkstra;
    use crate::Chap58::BellmanFordStEphInt::BellmanFordStEphInt::bellman_ford;
    use crate::Types::Types::*;

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths (Parallel)
    ///
    /// APAS: Work O(mn log n), Span O(m log n), Parallelism Θ(n)
    pub fn johnson_apsp(graph: &WeightedDirGraphMtEphInt<usize>) -> AllPairsResultStEphInt {
        let n = graph.vertices().size();

        // Phase 1: Add dummy source and run Bellman-Ford
        let (graph_with_dummy, dummy_idx) = add_dummy_source(graph, n);

        let bellman_ford_result = match bellman_ford(&graph_with_dummy, dummy_idx) {
            | Ok(res) => res,
            | Err(_) => {
                // Negative cycle detected - return infinity matrix
                return create_negative_cycle_result(n);
            }
        };

        // Extract potentials
        let potentials = ArraySeqStEphS::tabulate(&|i| bellman_ford_result.get_distance(i), n);

        // Phase 2: Reweight edges to eliminate negative weights
        let reweighted_graph = reweight_graph(graph, &potentials, n);

        // Phase 3: Run Dijkstra from each vertex IN PARALLEL and adjust distances
        // Unconditionally parallel using recursive divide-and-conquer with ParaPair!
        let (all_distances, all_predecessors) = parallel_dijkstra_all(&reweighted_graph, &potentials, 0, n, n);

        AllPairsResultStEphInt {
            distances: all_distances,
            predecessors: all_predecessors,
            n,
        }
    }

    /// Parallel Dijkstra execution using recursive divide-and-conquer with ParaPair!
    /// Returns sequences of distance and predecessor rows
    fn parallel_dijkstra_all(
        graph: &WeightedDirGraphStEphInt<usize>,
        potentials: &ArraySeqStEphS<i64>,
        start: usize,
        end: usize,
        n: usize,
    ) -> (
        ArraySeqStEphS<ArraySeqStEphS<i64>>,
        ArraySeqStEphS<ArraySeqStEphS<usize>>,
    ) {
        let range_size = end - start;

        // Base case: empty range
        if range_size == 0 {
            return (ArraySeqStEphS::empty(), ArraySeqStEphS::empty());
        }

        // Base case: single vertex
        if range_size == 1 {
            let u = start;
            let dijkstra_result = dijkstra(graph, u);

            // Adjust distances: δG(u,v) = δG'(u,v) - p(u) + p(v)
            let p_u = *potentials.nth(u);
            let adjusted_row = ArraySeqStEphS::tabulate(
                &|v| {
                    let d_prime = dijkstra_result.get_distance(v);
                    if d_prime == i64::MAX {
                        i64::MAX
                    } else {
                        let p_v = *potentials.nth(v);
                        d_prime - p_u + p_v
                    }
                },
                n,
            );

            let dist_seq = ArraySeqStEphS::singleton(adjusted_row);
            let pred_seq = ArraySeqStEphS::singleton(dijkstra_result.predecessors.clone());
            return (dist_seq, pred_seq);
        }

        // Recursive case: split in half and use ParaPair! for unconditional parallelism
        let mid = start + range_size / 2;
        let graph_left = graph.clone();
        let graph_right = graph.clone();
        let potentials_left = potentials.clone();
        let potentials_right = potentials.clone();

        let Pair((left_dist, left_pred), (right_dist, right_pred)) = crate::ParaPair!(
            move || parallel_dijkstra_all(&graph_left, &potentials_left, start, mid, n),
            move || parallel_dijkstra_all(&graph_right, &potentials_right, mid, end, n)
        );

        // Combine results
        let combined_dist = ArraySeqStEphS::append(&left_dist, &right_dist);
        let combined_pred = ArraySeqStEphS::append(&left_pred, &right_pred);

        (combined_dist, combined_pred)
    }

    /// Add dummy source with zero-weight edges to all vertices
    fn add_dummy_source(graph: &WeightedDirGraphMtEphInt<usize>, n: usize) -> (WeightedDirGraphStEphInt<usize>, usize) {
        // Convert MtEph graph to StEph for Bellman-Ford
        let mut vertices = Set::empty();
        for i in 0..n {
            vertices.insert(i);
        }

        // Add dummy vertex
        vertices.insert(n);

        let mut edges = Set::empty();

        // Copy all original edges
        for u in 0..n {
            for v_w in graph.out_neighbors_weighted(&u).iter() {
                let (v, w) = v_w;
                edges.insert((u, *v, *w));
            }
        }

        // Add edges from dummy source to all original vertices
        for v in 0..n {
            edges.insert((n, v, 0));
        }

        (WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges), n)
    }

    /// Reweight edges: w'(u,v) = w(u,v) + p(u) - p(v)
    fn reweight_graph(
        graph: &WeightedDirGraphMtEphInt<usize>,
        potentials: &ArraySeqStEphS<i64>,
        n: usize,
    ) -> WeightedDirGraphStEphInt<usize> {
        let mut vertices = Set::empty();
        for i in 0..n {
            vertices.insert(i);
        }

        let mut reweighted_edges = Set::empty();
        for u in 0..n {
            let p_u = *potentials.nth(u);
            for v_w in graph.out_neighbors_weighted(&u).iter() {
                let (v, w) = v_w;
                let p_v = *potentials.nth(*v);
                let w_prime = (*w as i64) + p_u - p_v;
                reweighted_edges.insert((u, *v, w_prime as i32));
            }
        }

        WeightedDirGraphStEphInt::from_weighted_edges(vertices, reweighted_edges)
    }

    /// Create result for negative cycle case
    fn create_negative_cycle_result(n: usize) -> AllPairsResultStEphInt {
        let distances = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| i64::MAX, n), n);
        let predecessors = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| 0, n), n);
        AllPairsResultStEphInt {
            distances,
            predecessors,
            n,
        }
    }
}
