//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Edge filtering (NG) and vertex map-reduce (NGOfVertices) are parallel.

pub mod UnDirGraphMtEph {
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;
    use crate::ParaPair;

    #[derive(Clone)]
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash + 'static> {
        V: Set<V>,
        E: Set<Edge<V>>,
    }

    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash + 'static> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> UnDirGraphMtEph<V>;
        /// APAS: Work Θ(|V| + |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(1)
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> &Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn edges(&self) -> &Set<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeE(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn Neighbor(&self, u: &V, v: &V) -> B;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|) - parallel divide-and-conquer filter
        fn NG(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |E|), Span Θ(log |u_set| + log |E|) - parallel map-reduce
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(log |E|) - calls parallel NG
        fn Degree(&self, v: &V) -> N;
    }

    impl<V: StT + MtT + Hash + 'static> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {
        fn empty() -> UnDirGraphMtEph<V> {
            UnDirGraphMtEph {
                V: SetLit![],
                E: SetLit![],
            }
        }
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V> {
            UnDirGraphMtEph { V, E }
        }
        fn vertices(&self) -> &Set<V> {
            &self.V
        }
        fn edges(&self) -> &Set<Edge<V>> {
            &self.E
        }
        fn sizeV(&self) -> N {
            self.V.size()
        }
        fn sizeE(&self) -> N {
            self.E.size()
        }

        fn Neighbor(&self, u: &V, v: &V) -> B {
            // Treat edges as unordered: {u,v}
            if true == self.E.mem(&Edge(u.clone_mt(), v.clone_mt()))
                || true == self.E.mem(&Edge(v.clone_mt(), u.clone_mt()))
            {
                true
            } else {
                false
            }
        }

        fn NG(&self, v: &V) -> Set<V> {
            // PARALLEL: filter edges using divide-and-conquer
            let edges: Vec<Edge<V>> = self.E.iter().cloned().collect();
            let n = edges.len();
            
            if n <= 8 {
                let mut ng: Set<V> = SetLit![];
                for Edge(a, b) in edges {
                    if a == *v {
                        let _ = ng.insert(b.clone_mt());
                    } else if b == *v {
                        let _ = ng.insert(a.clone_mt());
                    }
                }
                return ng;
            }
            
            // Parallel divide-and-conquer
            fn parallel_ng<V: StT + MtT + Hash + 'static>(
                edges: Vec<Edge<V>>,
                v: V
            ) -> Set<V> {
                let n = edges.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    let Edge(a, b) = &edges[0];
                    if a == &v {
                        let mut s = SetLit![];
                        s.insert(b.clone_mt());
                        return s;
                    } else if b == &v {
                        let mut s = SetLit![];
                        s.insert(a.clone_mt());
                        return s;
                    }
                    return SetLit![];
                }
                
                let mid = n / 2;
                let mut right_edges = edges;
                let left_edges = right_edges.split_off(mid);
                
                let v_left = v.clone_mt();
                let v_right = v;
                
                let Pair(left_result, right_result) = ParaPair!(
                    move || parallel_ng(left_edges, v_left),
                    move || parallel_ng(right_edges, v_right)
                );
                
                left_result.union(&right_result)
            }
            
            parallel_ng(edges, v.clone_mt())
        }

        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices: Vec<V> = u_set.iter().cloned().collect();
            let n = vertices.len();
            
            if n <= 8 {
                let mut result: Set<V> = SetLit![];
                for u in vertices {
                    let ng_u = self.NG(&u);
                    result = result.union(&ng_u);
                }
                return result;
            }
            
            // Parallel map-reduce
            fn parallel_ng_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: UnDirGraphMtEph<V>
            ) -> Set<V> {
                let n = vertices.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    return graph.NG(&vertices[0]);
                }
                
                let mid = n / 2;
                let mut right_verts = vertices;
                let left_verts = right_verts.split_off(mid);
                
                let graph_left = graph.clone();
                let graph_right = graph;
                
                let Pair(left_result, right_result) = ParaPair!(
                    move || parallel_ng_of_vertices(left_verts, graph_left),
                    move || parallel_ng_of_vertices(right_verts, graph_right)
                );
                
                left_result.union(&right_result)
            }
            
            parallel_ng_of_vertices(vertices, self.clone())
        }

        fn Incident(&self, e: &Edge<V>, v: &V) -> B {
            if &e.0 == v || &e.1 == v { true } else { false }
        }

        fn Degree(&self, v: &V) -> N {
            self.NG(v).size()
        }
    }

    // DirGraphStEph-compatible interface for undirected graphs
    impl<V: StT + MtT + Hash> UnDirGraphMtEph<V> {
        /// Arc count (alias for edge count in undirected graphs)
        pub fn sizeA(&self) -> N {
            self.sizeE()
        }

        /// Arcs (alias for edges in undirected graphs)
        pub fn arcs(&self) -> &Set<Edge<V>> {
            self.edges()
        }

        /// Neighbors (in undirected graphs, all neighbors are both in and out)
        pub fn NPlus(&self, v: &V) -> Set<V> {
            self.NG(v)
        }

        /// Neighbors (in undirected graphs, all neighbors are both in and out)
        pub fn NMinus(&self, v: &V) -> Set<V> {
            self.NG(v)
        }

        /// Neighbors of vertex set
        pub fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            self.NGOfVertices(u_set)
        }

        /// Neighbors of vertex set
        pub fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            self.NGOfVertices(u_set)
        }

        /// Degree (in undirected graphs, in-degree equals total degree)
        pub fn InDegree(&self, v: &V) -> N {
            self.Degree(v)
        }

        /// Degree (in undirected graphs, out-degree equals total degree)
        pub fn OutDegree(&self, v: &V) -> N {
            self.Degree(v)
        }
    }

    impl<V: StT + MtT + Hash + 'static> std::fmt::Debug for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UnDirGraphMtEph")
                .field("V", &self.V)
                .field("E", &self.E)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash + 'static> std::fmt::Display for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "V={} E={:?}", self.V, self.E)
        }
    }

    impl<V: StT + MtT + Hash + 'static> PartialEq for UnDirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool {
            self.V == other.V && self.E == other.E
        }
    }
    impl<V: StT + MtT + Hash + 'static> Eq for UnDirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! UnDirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::Set<_> = $crate::SetLit![];
            let __E: $crate::Chap05::SetStEph::SetStEph::Set<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::FromSets(__V, __E)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::Chap05::SetStEph::SetStEph::Set<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::FromSets(__V, __E)
        }};
    }

    #[allow(dead_code)]
    fn _UnDirGraphMtEphLit_type_checks() {
        let _ = UnDirGraphMtEphLit!( V: [1], E: [(1, 2)] ); // non-empty infers
        let _: UnDirGraphMtEph<i32> = UnDirGraphMtEphLit![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __undirgraph_mt_macro_typecheck_exercise() {
        let _g0: UnDirGraphMtEph<usize> = UnDirGraphMtEphLit!( V: [], E: [] );
        let _g1 = UnDirGraphMtEphLit!( V: [0, 1, 2], E: [(0, 1), (1, 2)] );
    }
}
