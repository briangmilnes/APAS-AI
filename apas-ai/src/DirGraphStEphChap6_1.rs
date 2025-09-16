//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

pub mod DirGraphStEphChap6_1 {
use crate::Types::Types::*;
use crate::SetStEphChap5_1::SetStEphChap5_1::*;
use crate::SetLit;
use std::hash::Hash;

#[derive(Clone)]
pub struct DirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    V: Set<V>,
    A: Set<Pair<V, V>>,
}

pub trait DirGraphStEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    /// APAS: Work Θ(1), Span Θ(1)
    /// claude-4-sonet: Work Θ(1), Span Θ(1)
    fn empty() -> DirGraphStEph<V>;
    /// APAS: Work Θ(|V| + |A|), Span Θ(1)
    /// claude-4-sonet: Work Θ(|V| + |A|), Span Θ(1)
    fn FromSets(V: Set<V>, A: Set<Pair<V, V>>) -> DirGraphStEph<V>;
    /// APAS: Work Θ(1), Span Θ(1)
    /// claude-4-sonet: Work Θ(1), Span Θ(1)
    fn vertices(&self) -> &Set<V>;
    /// APAS: Work Θ(1), Span Θ(1)
    /// claude-4-sonet: Work Θ(1), Span Θ(1)
    fn arcs(&self) -> &Set<Pair<V, V>>;
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

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphStEphChap6_1Trait<V> for DirGraphStEph<V> {
    fn empty() -> DirGraphStEph<V> { DirGraphStEph { V: SetLit![], A: SetLit![] } }
    fn FromSets(V: Set<V>, A: Set<Pair<V, V>>) -> DirGraphStEph<V> { DirGraphStEph { V, A } }
    fn vertices(&self) -> &Set<V> { &self.V }
    fn arcs(&self) -> &Set<Pair<V, V>> { &self.A }
    fn sizeV(&self) -> N { self.V.size() }
    fn sizeA(&self) -> N { self.A.size() }

    fn Neighbor(&self, u: &V, v: &V) -> B {
        // Adjacent if there is an arc either way
        if B::True == self.A.mem(&Pair(u.clone(), v.clone())) || B::True == self.A.mem(&Pair(v.clone(), u.clone())) { B::True } else { B::False }
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
        for Pair(x, y) in self.A.iter().cloned() { if x == *v { let _ = out.insert(y.clone()); } }
        out
    }

    fn NMinus(&self, v: &V) -> Set<V> {
        let mut inn: Set<V> = SetLit![];
        for Pair(x, y) in self.A.iter().cloned() { if y == *v { let _ = inn.insert(x.clone()); } }
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

    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B::False } }

    fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }
    fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }
    fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }
}

impl<V: Eq + Hash + Clone + std::fmt::Debug + std::fmt::Display> std::fmt::Debug for DirGraphStEph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirGraphStEph").field("V", &self.V).field("A", &self.A).finish()
    }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for DirGraphStEph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V={} A={:?}", self.V, self.A)
    }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphStEph<V> { fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A } }
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for DirGraphStEph<V> {}

    #[macro_export]
    macro_rules! DirGraphLit {
        ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::SetStEphChap5_1::SetStEphChap5_1::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __A: $crate::SetStEphChap5_1::SetStEphChap5_1::Set<_> = {
                let mut __s = < $crate::SetStEphChap5_1::SetStEphChap5_1::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Pair($u, $w)); )*
                __s
            };
            < $crate::DirGraphStEphChap6_1::DirGraphStEphChap6_1::DirGraphStEph<_> as $crate::DirGraphStEphChap6_1::DirGraphStEphChap6_1::DirGraphStEphChap6_1Trait<_> >::FromSets(__V, __A)
        }}}

    #[allow(dead_code)]
    pub fn __dirgraph_macro_typecheck_exercise() {
        let _g0: DirGraphStEph<usize> = DirGraphLit!( V: [], A: [] );
        let _g1 = DirGraphLit!( V: [0,1,2], A: [(0,1),(1,2)] );
    }
}

