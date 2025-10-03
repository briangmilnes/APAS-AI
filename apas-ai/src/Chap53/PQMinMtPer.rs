//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - persistent, multi-threaded.
//!
//! Note: Parallel priority selection would require concurrent priority queues.
//! This implementation uses thread-safe sets (AVLTreeSetMtPer) which support
//! parallel set operations, but the priority selection itself remains sequential.

pub mod PQMinMtPer {

    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct PQMinResult<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static> {
        pub visited: AVLTreeSetMtPer<V>,
        pub priorities: AVLTreeSetMtPer<Pair<V, P>>,
        pub parent: Option<AVLTreeSetMtPer<Pair<V, V>>>,
    }

    pub trait PriorityFn<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static> {
        fn priority(&self, v: &V) -> P;
    }

    pub struct ClosurePriority<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, F: Fn(&V) -> P> {
        f: F,
        _phantom: std::marker::PhantomData<(V, P)>,
    }

    impl<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, F: Fn(&V) -> P> ClosurePriority<V, P, F> {
        pub fn new(f: F) -> Self {
            Self {
                f,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, F: Fn(&V) -> P> PriorityFn<V, P>
        for ClosurePriority<V, P, F>
    {
        fn priority(&self, v: &V) -> P { (self.f)(v) }
    }

    pub trait PQMinMtPerTrait<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ(|V| log |V|), Parallelism Θ(1)
        /// Priority-First Search using thread-safe persistent sets.
        /// Set operations (union, difference, filter) use parallel implementations.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            PF: PriorityFn<V, P>;

        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetMtPer<V>, priority_fn: &PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            PF: PriorityFn<V, P>;
    }

    /// Priority queue minimum search starting from single source.
    /// claude-4-sonet: Work Θ(|V| log |V| + |E|), Span Θ(|V| log |V|), Parallelism Θ(1)
    pub fn pq_min<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, G, PF>(graph: &G, source: V, priority_fn: &PF) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        PF: PriorityFn<V, P>,
    {
        let sources = AVLTreeSetMtPer::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    /// Priority queue minimum search starting from multiple sources.
    /// claude-4-sonet: Work Θ(|V| log |V| + |E|), Span Θ(|V| log |V|), Parallelism Θ(1)
    pub fn pq_min_multi<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static, G, PF>(graph: &G, sources: AVLTreeSetMtPer<V>, priority_fn: &PF) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        PF: PriorityFn<V, P>,
    {
            fn find_min_priority<V: StTInMtT + Ord + 'static, P: StTInMtT + Ord + 'static>(
                frontier: &AVLTreeSetMtPer<Pair<Pair<P, V>, V>>,
            ) -> Option<V> {
                if frontier.size() == 0 {
                    None
                } else {
                    let seq = frontier.to_seq();
                    Some(seq.nth(0).1.clone())
                }
            }

            fn explore<V, P, G, PF>(
                graph: &G,
                priority_fn: &PF,
                visited: AVLTreeSetMtPer<V>,
                frontier: AVLTreeSetMtPer<Pair<Pair<P, V>, V>>,
            ) -> (AVLTreeSetMtPer<V>, AVLTreeSetMtPer<Pair<V, P>>)
            where
                V: StTInMtT + Ord + 'static,
                P: StTInMtT + Ord + 'static,
                G: Fn(&V) -> AVLTreeSetMtPer<V>,
                PF: PriorityFn<V, P>,
            {
                if let Some(v) = find_min_priority(&frontier) {
                    let p = priority_fn.priority(&v);
                    let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                    // Parallel difference operation
                    let frontier_new = frontier.difference(&AVLTreeSetMtPer::singleton(entry));

                    // Parallel union operation
                    let visited_new = visited.union(&AVLTreeSetMtPer::singleton(v.clone()));

                    let neighbors = graph(&v);
                    let mut frontier_updated = frontier_new;
                    let neighbors_seq = neighbors.to_seq();

                    for i in 0..neighbors_seq.length() {
                        let neighbor = neighbors_seq.nth(i);
                        if !visited_new.find(neighbor) {
                            let neighbor_p = priority_fn.priority(neighbor);
                            let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                            // Parallel union for each new frontier element
                            frontier_updated = frontier_updated.union(&AVLTreeSetMtPer::singleton(neighbor_entry));
                        }
                    }

                    explore(graph, priority_fn, visited_new, frontier_updated)
                } else {
                    let mut priorities = AVLTreeSetMtPer::empty();
                    let visited_seq = visited.to_seq();
                    for i in 0..visited_seq.length() {
                        let v = visited_seq.nth(i);
                        let p = priority_fn.priority(v);
                        // Parallel union for priority set construction
                        priorities = priorities.union(&AVLTreeSetMtPer::singleton(Pair(v.clone(), p)));
                    }
                    (visited, priorities)
                }
            }

            let mut initial_frontier = AVLTreeSetMtPer::empty();
            let sources_seq = sources.to_seq();
            for i in 0..sources_seq.length() {
                let v = sources_seq.nth(i);
                let p = priority_fn.priority(v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                initial_frontier = initial_frontier.union(&AVLTreeSetMtPer::singleton(entry));
            }

            let (visited, priorities) = explore(graph, priority_fn, AVLTreeSetMtPer::empty(), initial_frontier);

            PQMinResult {
                visited,
                priorities,
                parent: None,
            }
        }
}
