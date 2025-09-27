//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs.

pub mod LabDirGraphStEph {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabDirGraphStEph<V, L>
    where
        V: StT + Hash,
        L: StT + Hash,
    {
        vertices: Set<V>,
        labeled_arcs: Set<LabEdge<V, L>>,
    }

    pub trait LabDirGraphStEphTrait<V, L>
    where
        V: StT + Hash,
        L: StT + Hash,
    {
        fn empty() -> Self;
        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) -> Self;
        fn vertices(&self) -> &Set<V>;
        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>>;
        fn arcs(&self) -> Set<Edge<V>>;
        fn add_vertex(&mut self, v: V);
        fn add_labeled_arc(&mut self, from: V, to: V, label: L);
        fn get_arc_label(&self, from: &V, to: &V) -> Option<&L>;
        fn has_arc(&self, from: &V, to: &V) -> bool;
        fn out_neighbors(&self, v: &V) -> Set<V>;
        fn in_neighbors(&self, v: &V) -> Set<V>;
    }

    impl<V, L> LabDirGraphStEphTrait<V, L> for LabDirGraphStEph<V, L>
    where
        V: StT + Hash,
        L: StT + Hash,
    {
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

        fn FromSets(vertices: Set<V>, arcs: Set<Pair<V, V>>) -> Self
        where
            L: Default,
        {
            let labeled_arcs = arcs.iter()
                .map(|pair| LabEdge(pair.0.clone(), pair.1.clone(), L::default()))
                .collect::<Set<_>>();
            Self::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }

        fn Incident(&self, arc: &Pair<V, V>, v: &V) -> B {
            if &arc.0 == v || &arc.1 == v { B::True } else { B::False }
        }
    }

    impl<V, L> Display for LabDirGraphStEph<V, L>
    where
        V: StT + Hash,
        L: Clone + Display + Debug + Eq + Hash,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V, L> Debug for LabDirGraphStEph<V, L>
    where
        V: StT + Hash,
        L: Clone + Display + Debug + Eq + Hash,
    {
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

    #[allow(dead_code)]
    fn _LabDirGraphStEphLit_type_checks() {
        let _ = LabDirGraphStEphLit!( V: [1], A: [(1, 2, "label")] ); // non-empty infers
        let _: LabDirGraphStEph<i32, &str> = LabDirGraphStEphLit!(); // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __lab_dir_graph_macro_typecheck_exercise() {
        let _g0: LabDirGraphStEph<usize, i32> = LabDirGraphStEphLit!();
        let _g1 = LabDirGraphStEphLit!( V: [0, 1, 2], A: [(0, 1, 10), (1, 2, 20)] );
        let _g2 = LabDirGraphStEphLit!( V: ["a", "b"], A: [("a", "b", 314)] );
    }
}
