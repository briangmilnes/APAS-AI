//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

pub mod DirGraphStEph {
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct DirGraphStEph<V: StT + Hash> {
        V: Set<V>,
        A: Set<Edge<V>>,
    }

    pub trait DirGraphStEphTrait<V: StT + Hash> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> DirGraphStEph<V>;
        /// APAS: Work Θ(|V| + |A|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|V| + |A|), Span Θ(1)
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V>;
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
        fn Incident(&self, e: &Edge<V>, v: &V) -> B;
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

    impl<V: StT + Hash> DirGraphStEphTrait<V> for DirGraphStEph<V> {
        fn empty() -> DirGraphStEph<V> {
            DirGraphStEph {
                V: SetLit![],
                A: SetLit![],
            }
        }
        fn FromSets(V: Set<V>, A: Set<Edge<V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }
        fn vertices(&self) -> &Set<V> { &self.V }
        fn arcs(&self) -> &Set<Edge<V>> { &self.A }
        fn sizeV(&self) -> N { self.V.size() }
        fn sizeA(&self) -> N { self.A.size() }

        fn Neighbor(&self, u: &V, v: &V) -> B {
            // Adjacent if there is an arc either way
            if true == self.A.mem(&Edge(u.clone(), v.clone())) {
                true
            } else {
                false
            }
        }

        fn NG(&self, v: &V) -> Set<V> { self.NPlus(v).union(&self.NMinus(v)) }

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
                    let _ = out.insert(y.clone());
                }
            }
            out
        }

        fn NMinus(&self, v: &V) -> Set<V> {
            let mut inn: Set<V> = SetLit![];
            for Edge(x, y) in self.A.iter().cloned() {
                if y == *v {
                    let _ = inn.insert(x.clone());
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

        fn Incident(&self, e: &Edge<V>, v: &V) -> B {
            if &e.0 == v || &e.1 == v {
                true
            } else {
                false
            }
        }

        fn Degree(&self, v: &V) -> N { self.InDegree(v) + self.OutDegree(v) }
        fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }
        fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }
    }

    impl<V: StT + Hash> Debug for DirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("DirGraphStEph")
                .field("V", &self.V)
                .field("A", &self.A)
                .finish()
        }
    }

    impl<V: StT + Hash> Display for DirGraphStEph<V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "V={} A={:?}", self.V, self.A) }
    }

    impl<V: StT + Hash> PartialEq for DirGraphStEph<V> {
        fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A }
    }
    impl<V: StT + Hash> Eq for DirGraphStEph<V> {}

    #[macro_export]
    macro_rules! DirGraphStEphLit {
        () => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::Set<_> = $crate::SetLit![];
            let __A: $crate::Chap05::SetStEph::SetStEph::Set<$crate::Types::Types::Edge<_>> = $crate::SetLit![];
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::FromSets(__V, __A)
        }};
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::Chap05::SetStEph::SetStEph::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::Chap05::SetStEph::SetStEph::Set<_> = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Edge($u, $w)); )*
                __s
            };
            < $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEph<_> as $crate::Chap06::DirGraphStEph::DirGraphStEph::DirGraphStEphTrait<_> >::FromSets(__V, __A)
        }}}

    #[allow(dead_code)]
    fn _DirGraphStEphLit_type_checks() {
        let _ = DirGraphStEphLit!( V: [1], A: [(1,2)] ); // non-empty infers (e.g., i32)
        let _: DirGraphStEph<i32> = DirGraphStEphLit![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __dirgraph_macro_typecheck_exercise() {
        let _g0: DirGraphStEph<usize> = DirGraphStEphLit!( V: [], A: [] );
        let _g1 = DirGraphStEphLit!( V: [0,1,2], A: [(0,1),(1,2)] );
    }
}
