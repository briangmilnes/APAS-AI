//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Directed Graph (ephemeral) using Set for vertices and labeled arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor operations.
//! Labeled arc filtering (out_neighbors, in_neighbors) are parallel.

pub mod LabDirGraphMtEph {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    use crate::ParaPair;

    #[derive(Clone)]
    pub struct LabDirGraphMtEph<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static>
    {
        vertices: Set<V>,
        labeled_arcs: Set<LabEdge<V, L>>,
    }

    pub trait LabDirGraphMtEphTrait<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static>
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

    impl<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static> LabDirGraphMtEphTrait<V, L> for LabDirGraphMtEph<V, L>
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

        fn vertices(&self) -> &Set<V> {
            &self.vertices
        }

        fn labeled_arcs(&self) -> &Set<LabEdge<V, L>> {
            &self.labeled_arcs
        }

        fn arcs(&self) -> Set<Edge<V>> {
            let mut arcs = Set::empty();
            for labeled_arc in self.labeled_arcs.iter() {
                arcs.insert(Edge(labeled_arc.0.clone_mt(), labeled_arc.1.clone_mt()));
            }
            arcs
        }

        fn add_vertex(&mut self, v: V) {
            self.vertices.insert(v);
        }

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
            // PARALLEL: filter labeled arcs using divide-and-conquer
            let arcs: Vec<LabEdge<V, L>> = self.labeled_arcs.iter().cloned().collect();
            let n = arcs.len();
            
            if n <= 8 {
                let mut neighbors = Set::empty();
                for labeled_arc in arcs {
                    if labeled_arc.0 == *v {
                        neighbors.insert(labeled_arc.1.clone_mt());
                    }
                }
                return neighbors;
            }
            
            // Parallel divide-and-conquer
            fn parallel_out<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
                arcs: Vec<LabEdge<V, L>>,
                v: V
            ) -> Set<V> {
                let n = arcs.len();
                if n == 0 {
                    return Set::empty();
                }
                if n == 1 {
                    return if arcs[0].0 == v {
                        let mut s = Set::empty();
                        s.insert(arcs[0].1.clone_mt());
                        s
                    } else {
                        Set::empty()
                    };
                }
                
                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);
                
                let v_left = v.clone_mt();
                let v_right = v;
                
                let Pair(left_result, right_result) = ParaPair!(
                    move || parallel_out(left_arcs, v_left),
                    move || parallel_out(right_arcs, v_right)
                );
                
                left_result.union(&right_result)
            }
            
            parallel_out(arcs, v.clone_mt())
        }

        fn in_neighbors(&self, v: &V) -> Set<V> {
            // PARALLEL: filter labeled arcs using divide-and-conquer
            let arcs: Vec<LabEdge<V, L>> = self.labeled_arcs.iter().cloned().collect();
            let n = arcs.len();
            
            if n <= 8 {
                let mut neighbors = Set::empty();
                for labeled_arc in arcs {
                    if labeled_arc.1 == *v {
                        neighbors.insert(labeled_arc.0.clone_mt());
                    }
                }
                return neighbors;
            }
            
            // Parallel divide-and-conquer
            fn parallel_in<V: StT + MtT + Hash + 'static, L: StTInMtT + Hash + 'static>(
                arcs: Vec<LabEdge<V, L>>,
                v: V
            ) -> Set<V> {
                let n = arcs.len();
                if n == 0 {
                    return Set::empty();
                }
                if n == 1 {
                    return if arcs[0].1 == v {
                        let mut s = Set::empty();
                        s.insert(arcs[0].0.clone_mt());
                        s
                    } else {
                        Set::empty()
                    };
                }
                
                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);
                
                let v_left = v.clone_mt();
                let v_right = v;
                
                let Pair(left_result, right_result) = ParaPair!(
                    move || parallel_in(left_arcs, v_left),
                    move || parallel_in(right_arcs, v_right)
                );
                
                left_result.union(&right_result)
            }
            
            parallel_in(arcs, v.clone_mt())
        }
    }

    impl<V: StT + MtT + Hash, L: StTInMtT + Hash> Display for LabDirGraphMtEph<V, L>
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabDirGraph(V: {}, A: {})", self.vertices, self.labeled_arcs)
        }
    }

    impl<V: StT + MtT + Hash, L: StTInMtT + Hash> Debug for LabDirGraphMtEph<V, L>
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
