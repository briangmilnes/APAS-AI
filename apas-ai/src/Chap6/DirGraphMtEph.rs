//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs - Multi-threaded version.

pub mod DirGraphMtEph {
    use crate::SetLit;
    use crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::*;
    use crate::Types::Types::*;
    use std::hash::Hash;

    #[derive(Clone)]
    pub struct DirGraphMtEph<V: StT + MtT + Hash> {
        V: Set<V>,
        A: Set<Edge<V>>,
    }

    pub trait DirGraphMtEphTrait<V: StT + MtT + Hash> {
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
        /// claude-4-sonet: Work Θ(|A|), Span Θ(1)
        fn NG(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(1)
        fn NPlus(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(1)
        fn NMinus(&self, v: &V) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(|u_set| × |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|u_set| × |A|), Span Θ(1)
        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(1)
        fn Degree(&self, v: &V) -> N;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(1)
        fn InDegree(&self, v: &V) -> N;
        /// APAS: Work Θ(|A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|A|), Span Θ(1)
        fn OutDegree(&self, v: &V) -> N;
    }

    impl<V: StT + MtT + Hash> DirGraphMtEphTrait<V> for DirGraphMtEph<V> {
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
            if B::True == self.A.mem(&Edge(u.clone_mt(), v.clone_mt()))
            {
                B::True
            } else {
                B::False
            }
        }

        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v) }

        fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            let mut result: Set<V> = SetLit![];
            for u in u_set.iter() {
                let ng_u = self.NG(u);
                result = result.union(&ng_u);
            }
            result
        }

        fn NPlus(&self, v: &V) -> Set<V> {
            let mut out: Set<V> = SetLit![];
            for Edge(x, y) in self.A.iter().cloned() {
                if x == *v {
                    let _ = out.insert(y.clone_mt());
                }
            }
            out
        }

        fn NMinus(&self, v: &V) -> Set<V> {
            let mut inn: Set<V> = SetLit![];
            for Edge(x, y) in self.A.iter().cloned() {
                if y == *v {
                    let _ = inn.insert(x.clone_mt());
                }
            }
            inn
        }

        fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            let mut result: Set<V> = SetLit![];
            for u in u_set.iter() {
                let plus_u = self.NPlus(u);
                result = result.union(&plus_u);
            }
            result
        }

        fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V> {
            let mut result: Set<V> = SetLit![];
            for u in u_set.iter() {
                let minus_u = self.NMinus(u);
                result = result.union(&minus_u);
            }
            result
        }

        fn Incident(&self, e: &Pair<V, V>, v: &V) -> B {
            if &e.0 == v || &e.1 == v {
                B::True
            } else {
                B::False
            }
        }

        fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }
    }

    impl<V: StT + MtT + Hash> std::fmt::Debug for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("DirGraphMtEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StT + MtT + Hash> std::fmt::Display for DirGraphMtEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<V: StT + MtT + Hash> PartialEq for DirGraphMtEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }
    impl<V: StT + MtT + Hash> Eq for DirGraphMtEph<V> {}

    #[macro_export]
    macro_rules! DirGraphMtEphLit {
        () => {{
            let __V: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> = $crate::SetLit![];
            let __A: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap6::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap6::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::FromSets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> = {
                let mut __s = < $crate::Chap5::SetStEphChap5_1::SetStEphChap5_1::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap6::DirGraphMtEph::DirGraphMtEph::DirGraphMtEph<_> as $crate::Chap6::DirGraphMtEph::DirGraphMtEph::DirGraphMtEphTrait<_> >::FromSets(__V, __A)
        }}}

    #[allow(dead_code)]
    fn _DirGraphMtEphLit_type_checks() {
        let _ = DirGraphMtEphLit!( V: [1], A: [(1, 2)] ); // non-empty infers
        let _: DirGraphMtEph<i32> = DirGraphMtEphLit![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __dirgraph_mt_macro_typecheck_exercise() {
        let _g0: DirGraphMtEph<usize> = DirGraphMtEphLit!( V: [], A: [] );
        let _g1 = DirGraphMtEphLit!( V: [0, 1, 2], A: [(0, 1), (1, 2)] );
    }
}
