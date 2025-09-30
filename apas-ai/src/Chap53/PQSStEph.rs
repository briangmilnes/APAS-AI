//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Priority-First Search (PFS) - ephemeral, single-threaded.

pub mod PFSStEph {
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct PFSResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub priorities: AVLTreeSetStEph<Pair<V, P>>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    pub trait PriorityFn<V: StT + Ord, P: StT + Ord> {
        fn priority(&self, v: &V) -> P;
    }

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

    pub trait PFSStEphTrait<V: StT + Ord, P: StT + Ord> {
        fn pfs<G, PF>(graph: &G, source: V, priority_fn: &PF) -> PFSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: PriorityFn<V, P>;

        fn pfs_multi<G, PF>(
            graph: &G,
            sources: AVLTreeSetStEph<V>,
            priority_fn: &PF,
        ) -> PFSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: PriorityFn<V, P>;
    }

    pub struct PFSStEph;

    impl<V: StT + Ord, P: StT + Ord> PFSStEphTrait<V, P> for PFSStEph {
        fn pfs<G, PF>(graph: &G, source: V, priority_fn: &PF) -> PFSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: PriorityFn<V, P>,
        {
            let sources = AVLTreeSetStEph::singleton(source);
            Self::pfs_multi(graph, sources, priority_fn)
        }

        fn pfs_multi<G, PF>(
            graph: &G,
            sources: AVLTreeSetStEph<V>,
            priority_fn: &PF,
        ) -> PFSResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: PriorityFn<V, P>,
        {
            fn find_min_priority<V: StT + Ord, P: StT + Ord>(
                frontier: &AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
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
                visited: AVLTreeSetStEph<V>,
                frontier: AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
            ) -> (AVLTreeSetStEph<V>, AVLTreeSetStEph<Pair<V, P>>)
            where
                V: StT + Ord,
                P: StT + Ord,
                G: Fn(&V) -> AVLTreeSetStEph<V>,
                PF: PriorityFn<V, P>,
            {
                if let Some(v) = find_min_priority(&frontier) {
                    let p = priority_fn.priority(&v);
                    let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                    let frontier_new = frontier.difference(&AVLTreeSetStEph::singleton(entry));

                    let visited_new = visited.union(&AVLTreeSetStEph::singleton(v.clone()));

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
                                .union(&AVLTreeSetStEph::singleton(neighbor_entry));
                        }
                    }

                    explore(graph, priority_fn, visited_new, frontier_updated)
                } else {
                    let mut priorities = AVLTreeSetStEph::empty();
                    let visited_seq = visited.to_seq();
                    for i in 0..visited_seq.length() {
                        let v = visited_seq.nth(i);
                        let p = priority_fn.priority(v);
                        priorities =
                            priorities.union(&AVLTreeSetStEph::singleton(Pair(v.clone(), p)));
                    }
                    (visited, priorities)
                }
            }

            let mut initial_frontier = AVLTreeSetStEph::empty();
            let sources_seq = sources.to_seq();
            for i in 0..sources_seq.length() {
                let v = sources_seq.nth(i);
                let p = priority_fn.priority(v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                initial_frontier =
                    initial_frontier.union(&AVLTreeSetStEph::singleton(entry));
            }

            let (visited, priorities) = explore(
                graph,
                priority_fn,
                AVLTreeSetStEph::empty(),
                initial_frontier,
            );

            PFSResult {
                visited,
                priorities,
                parent: None,
            }
        }
    }
}
