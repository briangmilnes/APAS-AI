//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6 Labeled Undirected Graph (ephemeral) using Set for vertices and labeled edges.

pub mod LabUnDirGraphStEph {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct LabUnDirGraphStEph<V, L>
    where
        V: StT + Hash + Ord,
        L: StT + Hash,
    {
        vertices: Set<V>,
        labeled_edges: Set<LabEdge<V, L>>,
    }

    pub trait LabUnDirGraphStEphTrait<V, L>
    where
        V: StT + Hash + Ord,
        L: StT + Hash,
    {
        fn empty() -> Self;
        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) -> Self;
        fn vertices(&self) -> &Set<V>;
        fn labeled_edges(&self) -> &Set<LabEdge<V, L>>;
        fn edges(&self) -> Set<Edge<V>>;
        fn add_vertex(&mut self, v: V);
        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L);
        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L>;
        fn has_edge(&self, v1: &V, v2: &V) -> bool;
        fn neighbors(&self, v: &V) -> Set<V>;
        fn normalize_edge(v1: V, v2: V) -> LabEdge<V, L>;
    }

    impl<V, L> LabUnDirGraphStEphTrait<V, L> for LabUnDirGraphStEph<V, L>
    where
        V: StT + Hash + Ord,
        L: StT + Hash,
    {
        fn empty() -> Self {
            LabUnDirGraphStEph {
                vertices: Set::empty(),
                labeled_edges: Set::empty(),
            }
        }

        fn from_vertices_and_labeled_edges(vertices: Set<V>, labeled_edges: Set<LabEdge<V, L>>) -> Self {
            LabUnDirGraphStEph {
                vertices,
                labeled_edges,
            }
        }

        fn vertices(&self) -> &Set<V> { &self.vertices }

        fn labeled_edges(&self) -> &Set<LabEdge<V, L>> { &self.labeled_edges }

        fn edges(&self) -> Set<Edge<V>> {
            let mut edges = Set::empty();
            for labeled_edge in self.labeled_edges.iter() {
                edges.insert(Edge(labeled_edge.0.clone(), labeled_edge.1.clone()));
            }
            edges
        }

        fn add_vertex(&mut self, v: V) { self.vertices.insert(v); }

        fn add_labeled_edge(&mut self, v1: V, v2: V, label: L) {
            self.vertices.insert(v1.clone());
            self.vertices.insert(v2.clone());
            let normalized_edge = if v1 <= v2 {
                LabEdge(v1, v2, label)
            } else {
                LabEdge(v2, v1, label)
            };
            self.labeled_edges.insert(normalized_edge);
        }

        fn get_edge_label(&self, v1: &V, v2: &V) -> Option<&L> {
            // Check both orientations since this is undirected
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return Some(&labeled_edge.2);
                }
            }
            None
        }

        fn has_edge(&self, v1: &V, v2: &V) -> bool {
            // Check both orientations since this is undirected
            for labeled_edge in self.labeled_edges.iter() {
                if (labeled_edge.0 == *v1 && labeled_edge.1 == *v2) || (labeled_edge.0 == *v2 && labeled_edge.1 == *v1)
                {
                    return true;
                }
            }
            false
        }

        fn neighbors(&self, v: &V) -> Set<V> {
            let mut neighbors = Set::empty();
            for labeled_edge in self.labeled_edges.iter() {
                if labeled_edge.0 == *v {
                    neighbors.insert(labeled_edge.1.clone());
                } else if labeled_edge.1 == *v {
                    neighbors.insert(labeled_edge.0.clone());
                }
            }
            neighbors
        }


        fn normalize_edge(_v1: V, _v2: V) -> LabEdge<V, L> {
            // This method signature doesn't make sense for LabEdge without a label
            // This is a design issue - we need the label to create a LabEdge
            // For now, we'll panic to indicate this needs to be fixed
            panic!("normalize_edge cannot create LabEdge without a label - method signature needs revision")
        }
    }

    // DirGraphStEph-compatible interface for labeled undirected graphs
    impl<V, L> LabUnDirGraphStEph<V, L>
    where
        V: StT + Hash + Ord,
        L: StT + Hash,
    {
        /// Arc count (alias for edge count in undirected graphs)
        pub fn sizeA(&self) -> N { self.labeled_edges().size() }
        
        /// Arcs (alias for edges in undirected graphs)
        pub fn arcs(&self) -> Set<LabEdge<V, L>> { self.labeled_edges().clone() }
        
        /// Neighbors (in undirected graphs, all neighbors are both in and out)
        pub fn NPlus(&self, v: &V) -> Set<V> { self.neighbors(v) }
        
        /// Neighbors (in undirected graphs, all neighbors are both in and out)
        pub fn NMinus(&self, v: &V) -> Set<V> { self.neighbors(v) }
        
        /// Degree (in undirected graphs, in-degree equals total degree)
        pub fn InDegree(&self, v: &V) -> N { self.neighbors(v).size() }
        
        /// Degree (in undirected graphs, out-degree equals total degree)
        pub fn OutDegree(&self, v: &V) -> N { self.neighbors(v).size() }
    }

    impl<V, L> Display for LabUnDirGraphStEph<V, L>
    where
        V: StT + Hash + Ord,
        L: StT + Hash,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "LabUnDirGraph(V: {}, E: {})", self.vertices, self.labeled_edges)
        }
    }

    impl<V, L> Debug for LabUnDirGraphStEph<V, L>
    where
        V: StT + Hash + Ord,
        L: StT + Hash,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "LabUnDirGraph {{ vertices: {:?}, labeled_edges: {:?} }}",
                self.vertices, self.labeled_edges
            )
        }
    }

    #[macro_export]
    macro_rules! LabUnDirGraphStEphLit {
        () => {{
            < $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph<_, _> as $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait<_, _> >::empty()
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ($v1:expr, $v2:expr, $label:expr) ),* $(,)? ] ) => {{
            let vertices = $crate::SetLit![ $( $v ),* ];
            let labeled_edges = {
                let mut edges = $crate::Chap05::SetStEph::SetStEph::Set::empty();
                $(
                    let normalized_edge = if $v1 <= $v2 {
                        $crate::Types::Types::LabEdge($v1, $v2, $label)
                    } else {
                        $crate::Types::Types::LabEdge($v2, $v1, $label)
                    };
                    edges.insert(normalized_edge);
                )*
                edges
            };
            < $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEph<_, _> as $crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::LabUnDirGraphStEphTrait<_, _> >::from_vertices_and_labeled_edges(vertices, labeled_edges)
        }};
    }

    #[allow(dead_code)]
    fn _LabUnDirGraphStEphLit_type_checks() {
        let _ = LabUnDirGraphStEphLit!( V: [1], E: [(1, 2, "label")] ); // non-empty infers
        let _: LabUnDirGraphStEph<i32, &str> = LabUnDirGraphStEphLit!(); // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __lab_undir_graph_macro_typecheck_exercise() {
        let _g0: LabUnDirGraphStEph<usize, i32> = LabUnDirGraphStEphLit!();
        let _g1 = LabUnDirGraphStEphLit!( V: [0, 1, 2], E: [(0, 1, 10), (1, 2, 20)] );
        let _g2 = LabUnDirGraphStEphLit!( V: ["a", "b"], E: [("a", "b", 314)] );
    }
}
