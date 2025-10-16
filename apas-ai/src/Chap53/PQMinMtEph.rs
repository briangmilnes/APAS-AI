//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - ephemeral, multi-threaded.
//!
//! Note: This uses AVLTreeSetMtEph which wraps mutable state with Arc<Mutex<...>>.
//! Priority selection is sequential, but set operations benefit from thread-safe structures.

pub mod PQMinMtEph {

    use std::marker::PhantomData;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct PQMinResult<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static> {
        pub visited: AVLTreeSetMtEph<V>,
        pub priorities: AVLTreeSetMtEph<Pair<V, P>>,
        pub parent: Option<AVLTreeSetMtEph<Pair<V, V>>>,
    }

    /// Priority function: maps vertices to their priorities.
    /// Lower priority values = higher priority (visited first).
    pub trait PriorityFn<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static>: Send + Sync + 'static {
        fn priority(&self, v: &V) -> P;
    }

    /// Simple wrapper for closure-based priority functions.
    pub struct ClosurePriority<
        V: StTInMtT + Ord + 'static,
        P: StTInMtT + Ord + 'static,
        F: Fn(&V) -> P + Send + Sync + 'static,
    > {
        f: F,
        _phantom: PhantomData<(V, P)>,
    }

    impl<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, F: Fn(&V) -> P + Send + Sync + 'static>
        ClosurePriority<V, P, F>
    {
        pub fn new(f: F) -> Self {
            Self {
                f,
                _phantom: PhantomData,
            }
        }
    }

    impl<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, F: Fn(&V) -> P + Send + Sync + 'static>
        PriorityFn<V, P> for ClosurePriority<V, P, F>
    {
        fn priority(&self, v: &V) -> P { (self.f)(v) }
    }

    pub trait PQMinMtEphTrait<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static> {
        /// Min-Priority Queue Search from a single source.
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ(|V| log |V|), Parallelism Θ(1)
        /// Work: O((|V| + |E|) log |V|), Span: O(|V| log |V|) sequential rounds.
        /// Set operations use thread-safe structures.
        fn pq_min<G, PF>(graph: G, source: V, priority_fn: PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetMtEph<V> + Send + Sync + 'static,
            PF: PriorityFn<V, P>;

        /// Min-Priority Queue Search from multiple sources.
        fn pq_min_multi<G, PF>(graph: G, sources: AVLTreeSetMtEph<V>, priority_fn: PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetMtEph<V> + Send + Sync + 'static,
            PF: PriorityFn<V, P>;
    }

    /// Priority queue minimum search starting from single source.
    /// claude-4-sonet: Work Θ(|V| log |V| + |E|), Span Θ(|V| log |V|), Parallelism Θ(1)
    pub fn pq_min<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, G, PF>(graph: G, source: V, priority_fn: PF) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetMtEph<V> + Send + Sync + 'static,
        PF: PriorityFn<V, P>,
    {
        let sources = AVLTreeSetMtEph::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    /// Priority queue minimum search starting from multiple sources.
    /// claude-4-sonet: Work Θ(|V| log |V| + |E|), Span Θ(|V| log |V|), Parallelism Θ(1)
    pub fn pq_min_multi<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, G, PF>(graph: G, sources: AVLTreeSetMtEph<V>, priority_fn: PF) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetMtEph<V> + Send + Sync + 'static,
        PF: PriorityFn<V, P>,
    {
            // Helper: find minimum priority vertex in frontier
            fn find_min_priority<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static>(
                frontier: &AVLTreeSetMtEph<Pair<Pair<P, V>, V>>,
            ) -> Option<V> {
                if frontier.size() == 0 {
                    None
                } else {
                    let seq = frontier.to_seq();
                    Some(seq.nth(0).1.clone())
                }
            }

            let mut visited = AVLTreeSetMtEph::empty();

            // Initialize frontier with sources
            let mut frontier = AVLTreeSetMtEph::empty();
            let sources_seq = sources.to_seq();
            for i in 0..sources_seq.length() {
                let v = sources_seq.nth(i);
                let p = priority_fn.priority(v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                frontier.insert(entry);
            }

            // Main search loop
            while let Some(v) = find_min_priority(&frontier) {
                // Remove selected vertex from frontier
                let p = priority_fn.priority(&v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                frontier.delete(&entry);

                // Add to visited
                visited.insert(v.clone());

                // Get neighbors and add unvisited ones to frontier
                let neighbors = graph(&v);
                let neighbors_seq = neighbors.to_seq();

                for i in 0..neighbors_seq.length() {
                    let neighbor = neighbors_seq.nth(i);
                    if !visited.find(neighbor) {
                        let neighbor_p = priority_fn.priority(neighbor);
                        let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                        frontier.insert(neighbor_entry);
                    }
                }
            }

            // Build priority set from visited vertices
            let mut priorities = AVLTreeSetMtEph::empty();
            let visited_seq = visited.to_seq();
            for i in 0..visited_seq.length() {
                let v = visited_seq.nth(i);
                let p = priority_fn.priority(v);
                priorities.insert(Pair(v.clone(), p));
            }

            PQMinResult {
                visited,
                priorities,
                parent: None,
            }
        }
}
