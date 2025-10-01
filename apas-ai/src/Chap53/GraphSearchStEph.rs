//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Generic Graph Search (ephemeral, single-threaded).

pub mod GraphSearchStEph {

    use crate::Types::Types::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;

    #[derive(Clone, Debug)]
    pub struct SearchResult<V: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    pub trait SelectionStrategy<V: StT + Ord> {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (AVLTreeSetStEph<V>, B);
    }

    pub struct SelectAll;
    impl<V: StT + Ord> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (AVLTreeSetStEph<V>, B) { (frontier.clone(), false) }
    }

    pub struct SelectOne;
    impl<V: StT + Ord> SelectionStrategy<V> for SelectOne {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (AVLTreeSetStEph<V>, B) {
            if frontier.size() == 0 {
                (AVLTreeSetStEph::empty(), false)
            } else {
                let seq = frontier.to_seq();
                let first = seq.nth(0).clone();
                (AVLTreeSetStEph::singleton(first), false)
            }
        }
    }

    pub trait GraphSearchStEphTrait<V: StT + Ord> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>;

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStEph<V>, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>;

        fn reachable<G>(graph: &G, source: V) -> AVLTreeSetStEph<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>;
    }

    pub struct GraphSearchStEph;

    impl<V: StT + Ord> GraphSearchStEphTrait<V> for GraphSearchStEph {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>,
        {
            let sources = AVLTreeSetStEph::singleton(source);
            Self::graph_search_multi(graph, sources, strategy)
        }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStEph<V>, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>,
        {
            fn explore<V, G, S>(
                graph: &G,
                strategy: &S,
                visited: AVLTreeSetStEph<V>,
                frontier: AVLTreeSetStEph<V>,
            ) -> AVLTreeSetStEph<V>
            where
                V: StT + Ord,
                G: Fn(&V) -> AVLTreeSetStEph<V>,
                S: SelectionStrategy<V>,
            {
                if frontier.size() == 0 {
                    return visited;
                }

                let (selected, _) = strategy.select(&frontier);
                let visited_new = visited.union(&selected);

                let mut new_neighbors = AVLTreeSetStEph::empty();
                let selected_seq = selected.to_seq();
                for i in 0..selected_seq.length() {
                    let v = selected_seq.nth(i);
                    let neighbors = graph(v);
                    new_neighbors = new_neighbors.union(&neighbors);
                }

                let frontier_new = new_neighbors.difference(&visited_new);
                explore(graph, strategy, visited_new, frontier_new)
            }

            let visited = explore(graph, strategy, AVLTreeSetStEph::empty(), sources);

            SearchResult { visited, parent: None }
        }

        fn reachable<G>(graph: &G, source: V) -> AVLTreeSetStEph<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
        {
            let result = Self::graph_search(graph, source, &SelectAll);
            result.visited
        }
    }
}
