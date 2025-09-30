//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Priority Queue Search (PQS) - persistent, single-threaded.
//!
//! Implements Algorithm 53.7 - Priority Queue Search framework.
//! PQS generalizes BFS/DFS by selecting frontier vertices based on priority.

pub mod PQSStPer {
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    /// Result of PQS containing visited vertices and parent tree.
    #[derive(Clone, Debug)]
    pub struct PQSResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStPer<V>,
        pub priorities: AVLTreeSetStPer<Pair<V, P>>, // (vertex, priority)
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent)
    }

    /// Priority function: maps vertices to their priorities.
    /// Lower priority values = higher priority (visited first).
    pub trait PriorityFn<V: StT + Ord, P: StT + Ord> {
        fn priority(&self, v: &V) -> P;
    }

    /// Simple wrapper for closure-based priority functions.
    pub struct ClosurePriority<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> {
        f: F,
        _phantom: std::marker::PhantomData<(V, P)>,
    }

    impl<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> ClosurePriority<V, P, F> {
        pub fn new(f: F) -> Self {
            Self {
                f,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> PriorityFn<V, P>
        for ClosurePriority<V, P, F>
    {
        fn priority(&self, v: &V) -> P {
            (self.f)(v)
        }
    }

    pub trait PQSStPerTrait<V: StT + Ord, P: StT + Ord> {
        /// Priority Queue Search from a single source.
        /// Work: O((|V| + |E|) log |V|) with balanced tree priority queue.
        /// Span: O(|V| log |V|) sequential rounds.
        fn pqs<G, PF>(graph: &G, source: V, priority_fn: &PF) -> PQSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>;

        /// Priority Queue Search from multiple sources.
        fn pqs_multi<G, PF>(
            graph: &G,
            sources: AVLTreeSetStPer<V>,
            priority_fn: &PF,
        ) -> PQSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>;
    }

    pub struct PQSStPer;

    impl<V: StT + Ord, P: StT + Ord> PQSStPerTrait<V, P> for PQSStPer {
        fn pqs<G, PF>(graph: &G, source: V, priority_fn: &PF) -> PQSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>,
        {
            let sources = AVLTreeSetStPer::singleton(source);
            Self::pqs_multi(graph, sources, priority_fn)
        }

        fn pqs_multi<G, PF>(
            graph: &G,
            sources: AVLTreeSetStPer<V>,
            priority_fn: &PF,
        ) -> PFSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>,
        {
            // Helper: find minimum priority vertex in frontier
            fn find_min_priority<V: StT + Ord, P: StT + Ord>(
                frontier: &AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
            ) -> Option<V> {
                if frontier.size() == 0 {
                    None
                } else {
                    let seq = frontier.to_seq();
                    // First element has minimum priority (sorted by (P, V) pair)
                    let min_entry = seq.nth(0);
                    Some(min_entry.1.clone())
                }
            }

            fn explore<V, P, G, PF>(
                graph: &G,
                priority_fn: &PF,
                visited: AVLTreeSetStPer<V>,
                frontier: AVLTreeSetStPer<Pair<Pair<P, V>, V>>, // ((priority, vertex), vertex)
            ) -> (AVLTreeSetStPer<V>, AVLTreeSetStPer<Pair<V, P>>)
            where
                V: StT + Ord,
                P: StT + Ord,
                G: Fn(&V) -> AVLTreeSetStPer<V>,
                PF: PriorityFn<V, P>,
            {
                // Select vertex with minimum priority
                if let Some(v) = find_min_priority(&frontier) {
                    // Remove selected vertex from frontier
                    let p = priority_fn.priority(&v);
                    let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                    let frontier_new = frontier.difference(&AVLTreeSetStPer::singleton(entry));

                    // Add to visited
                    let visited_new = visited.union(&AVLTreeSetStPer::singleton(v.clone()));

                    // Get neighbors and add unvisited ones to frontier
                    let neighbors = graph(&v);
                    let mut frontier_updated = frontier_new;
                    let neighbors_seq = neighbors.to_seq();

                    for i in 0..neighbors_seq.length() {
                        let neighbor = neighbors_seq.nth(i);
                        if !visited_new.find(neighbor) {
                            let neighbor_p = priority_fn.priority(neighbor);
                            let neighbor_entry = Pair(
                                Pair(neighbor_p.clone(), neighbor.clone()),
                                neighbor.clone(),
                            );
                            frontier_updated = frontier_updated
                                .union(&AVLTreeSetStPer::singleton(neighbor_entry));
                        }
                    }

                    explore(graph, priority_fn, visited_new, frontier_updated)
                } else {
                    // Build priority set from visited vertices
                    let mut priorities = AVLTreeSetStPer::empty();
                    let visited_seq = visited.to_seq();
                    for i in 0..visited_seq.length() {
                        let v = visited_seq.nth(i);
                        let p = priority_fn.priority(v);
                        priorities =
                            priorities.union(&AVLTreeSetStPer::singleton(Pair(v.clone(), p)));
                    }
                    (visited, priorities)
                }
            }

            // Initialize frontier with sources
            let mut initial_frontier = AVLTreeSetStPer::empty();
            let sources_seq = sources.to_seq();
            for i in 0..sources_seq.length() {
                let v = sources_seq.nth(i);
                let p = priority_fn.priority(v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                initial_frontier =
                    initial_frontier.union(&AVLTreeSetStPer::singleton(entry));
            }

            let (visited, priorities) = explore(
                graph,
                priority_fn,
                AVLTreeSetStPer::empty(),
                initial_frontier,
            );

            PQSResult {
                visited,
                priorities,
                parent: None,
            }
        }
    }
}
