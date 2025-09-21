//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges - Multi-threaded version.

pub mod UnDirGraphMtEph {
    use crate::SetLit;
    use crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use crate::Types::Types::*;
    use std::hash::Hash;

    #[derive(Clone)]
    pub struct UnDirGraphMtEph<V: StT + MtT + Hash> {
        V: Set<V>,
        E: Set<Edge<V>>,
    }

    pub trait UnDirGraphMtEphTrait<V: StT + MtT + Hash> {
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
        /// claude-4-sonet: Work Θ(|E|), Span Θ(1)
        fn NG(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |E|), Span Θ(1)
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;
        /// APAS: Work Θ(|E|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|E|), Span Θ(1)
        fn Degree(&self, v: &V) -> N;
    }

    impl<V: StT + MtT + Hash> UnDirGraphMtEphTrait<V> for UnDirGraphMtEph<V> {
        fn empty() -> UnDirGraphMtEph<V> {
            UnDirGraphMtEph {
                V: SetLit![],
                E: SetLit![],
            }
        }
        fn FromSets(V: Set<V>, E: Set<Edge<V>>) -> UnDirGraphMtEph<V> { UnDirGraphMtEph { V, E } }
        fn vertices(&self) -> &Set<V> { &self.V }
        fn edges(&self) -> &Set<Edge<V>> { &self.E }
        fn sizeV(&self) -> N { self.V.size() }
        fn sizeE(&self) -> N { self.E.size() }

        fn Neighbor(&self, u: &V, v: &V) -> B {
            // Treat edges as unordered: {u,v}
            if B::True == self.E.mem(&Edge(u.clone_mt(), v.clone_mt())) || B::True == self.E.mem(&Edge(v.clone_mt(), u.clone_mt()))
            {
                B::True
            } else {
                B::False
            }
        }

        fn NG(&self, v: &V) -> Set<V> {
            let mut ng: Set<V> = SetLit![];
            for Edge(a, b) in self.E.iter().cloned() {
                if a == *v {
                    let _ = ng.insert(b.clone_mt());
                } else if b == *v {
                    let _ = ng.insert(a.clone_mt());
                }
            }
            ng
        }

        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            let mut result: Set<V> = SetLit![];
            for u in u_set.iter() {
                let ng_u = self.NG(u);
                result = result.union(&ng_u);
            }
            result
        }

        fn Incident(&self, e: &Edge<V>, v: &V) -> B {
            if &e.0 == v || &e.1 == v {
                B::True
            } else {
                B::False
            }
        }

        fn Degree(&self, v: &V) -> N { self.NG(v).size() }
    }

    impl<V: StT + MtT + Hash> std::fmt::Debug for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UnDirGraphMtEph")
                .field("V", &self.V)
                .field("E", &self.E)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash> std::fmt::Display for UnDirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} E={:?}", self.V, self.E) }
    }

    impl<V: StT + MtT + Hash> PartialEq for UnDirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E }
    }
    impl<V: StT + MtT + Hash> Eq for UnDirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! UnDirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> = $crate::SetLit![];
            let __E: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap6::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap6::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::FromSets(__V, __E)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> = {
                let mut __s = < $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap6::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEph<_> as $crate::Chap6::UnDirGraphMtEph::UnDirGraphMtEph::UnDirGraphMtEphTrait<_> >::FromSets(__V, __E)
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
