//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version.

pub mod LabDirGraphMtEph {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabDirGraphMtEph<V, L>
    where
        V: StT + MtT + Hash,
        L: StTInMtT + Hash,
    {
        vertices: Set<V>,
        labeled_arcs: Set<LabEdge<V, L>>,
    }

    pub trait LabDirGraphMtEphTrait<V, L>
    where
        V: StT + MtT + Hash,
        L: StTInMtT + Hash,
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

    impl<V, L> LabDirGraphMtEphTrait<V, L> for LabDirGraphMtEph<V, L>
    where
        V: StT + MtT + Hash,
        L: StTInMtT + Hash,
    {
        fn empty() -> Self {
            LabDirGraphMtEph {
                vertices: Set::empty(),
                labeled_arcs: Set::empty(),
            }
        }

        fn from_vertices_and_labeled_arcs(vertices: Set<V>, labeled_arcs: Set<LabEdge<V, L>>) -> Self {
            LabDirGraphMtEph { vertices, labeled_arcs }
        }

        fn vertices(&self) -> &Set<V> { &self.vertices }

        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> { &self.labeled_arcs }

        fn arcs(&self) -> Set<Edge<V>> {
            let mut arcs = Set::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                arcs.insert(Edge(labeled_arc.0.clone_mt(), labeled_arc.1.clone_mt()));
            }
            arcs
        }

        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn add_labeled_arc(&mut self, from: V, to: V, label: L) {
            self.vertices.insert(from.clone_mt());
            self.vertices.insert(to.clone_mt());
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
                    neighbors.insert(labeled_arc.1.clone_mt());
                }
            }
            neighbors
        }

        fn in_neighbors(&self, v: &V) -> Set<V> {
            let mut neighbors = Set::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                if labeled_arc.1 == *v {
                    neighbors.insert(labeled_arc.0.clone_mt());
                }
            }
            neighbors
        }

    }

    impl<V, L> Display for LabDirGraphMtEph<V, L>
    where
        V: StT + MtT + Hash,
        L: StTInMtT + Hash,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V, L> Debug for LabDirGraphMtEph<V, L>
    where
        V: StT + MtT + Hash,
        L: StTInMtT + Hash,
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
    macro_rules! LabDirGraphMtEphLit {
        () => {{
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ($from:expr, $to:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_arcs = $crate::SetLit![ $( $crate::Types::Types::LabEdge($from, $to, $label) ),* ];
            < $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEph<_, _> as $crate::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::LabDirGraphMtEphTrait<_, _> >::from_vertices_and_labeled_arcs(vertices, labeled_arcs)
        }};
    }

    #[allow(dead_code)]
    fn _LabDirGraphMtEphLit_type_checks() {
        let _ = LabDirGraphMtEphLit!( V: [1], A: [(1, 2, "label")] ); // non-empty infers
        let _: LabDirGraphMtEph<i32, &str> = LabDirGraphMtEphLit!(); // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __lab_dir_graph_mt_macro_typecheck_exercise() {
        let _g0: LabDirGraphMtEph<usize, &str> = LabDirGraphMtEphLit!();
        let _g1 = LabDirGraphMtEphLit!( V: [0, 1, 2], A: [(0, 1, "edge1"), (1, 2, "edge2")] );
        let _g2 = LabDirGraphMtEphLit!( V: ["a", "b"], A: [("a", "b", 314)] );
    }
}
