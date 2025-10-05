//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs.

pub mod LabDirGraphStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabDirGraphStEph<V: StT + Hash, L: StT + Hash> {
        vertices: Set<V>,
        labeled_arcs: Set<LabEdge<V, L>>,
    }

    pub trait LabDirGraphStEphTrait<V: StT + Hash, L: StT + Hash> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn empty() -> Self;
        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |A|), Span Θ(|V| + |A|), Parallelism Θ(1) - sequential
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn vertices(&self) -> &Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential map
        fn arcs(&self) -> Set<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_vertex(&mut self, v: V);
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential search
        fn has_arc(&self, from: &V, to: &V) -> bool;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        fn out_neighbors(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(|A|), Parallelism Θ(1) - sequential filter
        fn in_neighbors(&self, v: &V) -> Set<V>;
    }

    impl<V: StT + Hash, L: StT + Hash> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L> {
        fn empty() -> Self {
            LabDirGraphStEph {
                vertices: Set::empty(),
                labeled_arcs: Set::empty(),
            }
        }

        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) -> Self {
            LabDirGraphStEph { vertices, labeled_arcs }
        }

        fn vertices(&self) -> &Set<V> { &self.vertices }

        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }

        fn arcs(&self) -> Set<Edge<V>> {
            let mut arcs = Set::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                arcs.insert(Edge(labeled_arc.0.clone(), labeled_arc.1.clone()));
            }
            arcs
        }

        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            self.vertices.insert(from.clone());
            self.vertices.insert(to.clone());
            self.labeled_arcs.insert(LabEdge(from, to, label));
        }

        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L> {
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *from && labeled_arc.1 == *to {
                    return Some(&labeled_arc.2);
                }
            }
            None
        }

        fn has_arc(&self, from: &V, to: &V) -> bool {
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *from && labeled_arc.1 == *to {
                    return true;
                }
            }
            false
        }

        fn out_neighbors(&self, v: &V) -> Set<V> {
            let mut neighbors = Set::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.0 == *v {
                    neighbors.insert(labeled_arc.1.clone());
                }
            }
            neighbors
        }

        fn in_neighbors(&self, v: &V) -> Set<V> {
            let mut neighbors = Set::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.1 == *v {
                    neighbors.insert(labeled_arc.0.clone());
                }
            }
            neighbors
        }
    }

    impl<V: StT + Hash, L: Clone + Display + Debug + Eq + Hash> Display for LabDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V: StT + Hash, L: Clone + Display + Debug + Eq + Hash> Debug for LabDirGraphStEph<V, L> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabDirGraph {{ vertices: {:?}, labeled_arcs: {:?} }}",
                self.vertices, self.labeled_arcs
            )
        }
    }

    #[macro_export]
    macro_rules! LabDirGraphStEphLit {
        () => {{
            < $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph<_, _> as $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ($from:expr, $to:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_arcs = $crate::SetLit![ $( $crate::Types::Types::LabEdge($from, $to, $label) ),* ];
            < $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEph<_, _> as $crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait<_, _> >::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }};
    }

}
