//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.
//!
//! Note: NOW uses true parallelism via ParaPair! for neighbor/degree operations.
//! Arc filtering (NPlus, NMinus) and vertex map-reduce (NGOfVertices, etc.) are parallel.

pub mod DirGraphMtEph {

    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::ParaPair;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct DirGraphMtEph<V: StT + MtT + Hash + 'static> {
        V: Set<V>,
        A: Set<Edge<V>>,
    }

    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash + 'static> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> DirGraphMtEph<V>;
        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |A|), Span Θ(1)
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn vertices(&self) -> &Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn arcs(&self) -> &Set<Edge<V>>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeV(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn sizeA(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn Neighbor(&self, u: &V, v: &V) -> B;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer in NPlus+NMinus
        fn NG(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|), Parallelism Θ((|u_set| × |A|)/(log |u_set| + log |A|)) - parallel map-reduce
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn NPlus(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - parallel divide-and-conquer filter
        fn NMinus(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|), Parallelism Θ((|u_set| × |A|)/(log |u_set| + log |A|)) - parallel map-reduce
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(log |u_set| + log |A|), Parallelism Θ((|u_set| × |A|)/(log |u_set| + log |A|)) - parallel map-reduce
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - calls parallel InDegree + OutDegree
        fn Degree(&self, v: &V) -> N;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - calls parallel NMinus
        fn InDegree(&self, v: &V) -> N;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(log |A|), Parallelism Θ(|A|/log |A|) - calls parallel NPlus
        fn OutDegree(&self, v: &V) -> N;
    }

    impl<V: StT + MtT + Hash + 'static> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {
        fn empty() -> DirGraphMtEph<V> {
            DirGraphMtEph {
                V: SetLit![],
                A: SetLit![],
            }
        }
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphMtEph<V> { DirGraphMtEph { V, A } }
        fn vertices(&self) -> &Set<V> { &self.V }
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }
        fn sizeV(&self) -> N { self.V.size() }
        fn sizeA(&self) -> N { self.A.size() }

        fn Neighbor(&self, u: &V, v: &V) -> B {
            // Adjacent if there is an arc either way
            if true == self.A.mem(&Edge(u.clone_mt(), v.clone_mt())) {
                true
            } else {
                false
            }
        }

        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v).union(&self.NMinus(v)) }

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
                graph: DirGraphMtEph<V>,
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

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_ng_of_vertices(left_verts, graph_left), move || {
                        parallel_ng_of_vertices(right_verts, graph_right)
                    });

                left_result.union(&right_result)
            }

            parallel_ng_of_vertices(vertices, self.clone())
        }

        fn NPlus(&self, v: &V) -> Set<V> {
            // PARALLEL: filter arcs using divide-and-conquer
            let arcs: Vec<Edge<V>> = self.A.iter().cloned().collect();
            let n = arcs.len();

            if n <= 8 {
                let mut out: Set<V> = SetLit![];
                for Edge(x, y) in arcs {
                    if x == *v {
                        let _ = out.insert(y.clone_mt());
                    }
                }
                return out;
            }

            // Parallel divide-and-conquer
            fn parallel_nplus<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> Set<V> {
                let n = arcs.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    let Edge(x, y) = &arcs[0];
                    return if x == &v {
                        let mut s = SetLit![];
                        s.insert(y.clone_mt());
                        s
                    } else {
                        SetLit![]
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_nplus(left_arcs, v_left), move || parallel_nplus(
                        right_arcs, v_right
                    ));

                left_result.union(&right_result)
            }

            parallel_nplus(arcs, v.clone_mt())
        }

        fn NMinus(&self, v: &V) -> Set<V> {
            // PARALLEL: filter arcs using divide-and-conquer
            let arcs: Vec<Edge<V>> = self.A.iter().cloned().collect();
            let n = arcs.len();

            if n <= 8 {
                let mut inn: Set<V> = SetLit![];
                for Edge(x, y) in arcs {
                    if y == *v {
                        let _ = inn.insert(x.clone_mt());
                    }
                }
                return inn;
            }

            // Parallel divide-and-conquer
            fn parallel_nminus<V: StT + MtT + Hash + 'static>(arcs: Vec<Edge<V>>, v: V) -> Set<V> {
                let n = arcs.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    let Edge(x, y) = &arcs[0];
                    return if y == &v {
                        let mut s = SetLit![];
                        s.insert(x.clone_mt());
                        s
                    } else {
                        SetLit![]
                    };
                }

                let mid = n / 2;
                let mut right_arcs = arcs;
                let left_arcs = right_arcs.split_off(mid);

                let v_left = v.clone_mt();
                let v_right = v;

                let Pair(left_result, right_result) = ParaPair!(
                    move || parallel_nminus(left_arcs, v_left),
                    move || parallel_nminus(right_arcs, v_right)
                );

                left_result.union(&right_result)
            }

            parallel_nminus(arcs, v.clone_mt())
        }

        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices: Vec<V> = u_set.iter().cloned().collect();
            let n = vertices.len();

            if n <= 8 {
                let mut result: Set<V> = SetLit![];
                for u in vertices {
                    let plus_u = self.NPlus(&u);
                    result = result.union(&plus_u);
                }
                return result;
            }

            // Parallel map-reduce
            fn parallel_nplus_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: DirGraphMtEph<V>,
            ) -> Set<V> {
                let n = vertices.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    return graph.NPlus(&vertices[0]);
                }

                let mid = n / 2;
                let mut right_verts = vertices;
                let left_verts = right_verts.split_off(mid);

                let graph_left = graph.clone();
                let graph_right = graph;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_nplus_of_vertices(left_verts, graph_left), move || {
                        parallel_nplus_of_vertices(right_verts, graph_right)
                    });

                left_result.union(&right_result)
            }

            parallel_nplus_of_vertices(vertices, self.clone())
        }

        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            // PARALLEL: map-reduce over vertices using divide-and-conquer
            let vertices: Vec<V> = u_set.iter().cloned().collect();
            let n = vertices.len();

            if n <= 8 {
                let mut result: Set<V> = SetLit![];
                for u in vertices {
                    let minus_u = self.NMinus(&u);
                    result = result.union(&minus_u);
                }
                return result;
            }

            // Parallel map-reduce
            fn parallel_nminus_of_vertices<V: StT + MtT + Hash + 'static>(
                vertices: Vec<V>,
                graph: DirGraphMtEph<V>,
            ) -> Set<V> {
                let n = vertices.len();
                if n == 0 {
                    return SetLit![];
                }
                if n == 1 {
                    return graph.NMinus(&vertices[0]);
                }

                let mid = n / 2;
                let mut right_verts = vertices;
                let left_verts = right_verts.split_off(mid);

                let graph_left = graph.clone();
                let graph_right = graph;

                let Pair(left_result, right_result) =
                    ParaPair!(move || parallel_nminus_of_vertices(left_verts, graph_left), move || {
                        parallel_nminus_of_vertices(right_verts, graph_right)
                    });

                left_result.union(&right_result)
            }

            parallel_nminus_of_vertices(vertices, self.clone())
        }

        fn Incident(&self, e: &Edge<V>, v: &V) -> B { if &e.0 == v || &e.1 == v { true } else { false } }

        fn Degree(&self, v: &V) -> N { self.InDegree(v) + self.OutDegree(v) }
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }
    }

    impl<V: StT + MtT + Hash + 'static> std::fmt::Debug for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DirGraphMtEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash + 'static> std::fmt::Display for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<V: StT + MtT + Hash + 'static> PartialEq for DirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }
    impl<V: StT + MtT + Hash + 'static> Eq for DirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! DirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::Set<_> = $crate::SetLit![];
            let __A: $crate::Chap05::SetStEph::SetStEph::Set<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::FromSets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::Chap05::SetStEph::SetStEph::Set<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap06::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::FromSets(__V, __A)
        }}}

}
