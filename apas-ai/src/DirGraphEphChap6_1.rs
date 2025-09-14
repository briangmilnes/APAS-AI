//! Chapter 6.1 Directed Graph (ephemeral) using Set for vertices and arcs.

pub mod DirGraphEphChap6_1 {
use crate::Types::Types::*;
use crate::SetEphChap5_1::SetEphChap5_1::*;
use crate::SetLit;
use std::hash::Hash;

#[derive(Clone)]
pub struct DirGraphEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    V: Set<V>,
    A: Set<(V, V)>,
}

pub trait DirGraphEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    fn empty() -> DirGraphEph<V>;
    fn FromSets(V: Set<V>, A: Set<(V, V)>) -> DirGraphEph<V>;
    fn vertices(&self) -> &Set<V>;
    fn arcs(&self) -> &Set<(V, V)>;
    fn sizeV(&self) -> N;
    fn sizeA(&self) -> N;
    // Neighborhood and adjacency APIs
    fn Neighbor(&self, u: &V, v: &V) -> B;
    fn NG(&self, v: &V) -> Set<V>;            // Out-neighbors by convention
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;
    fn NPlus(&self, v: &V) -> Set<V>;         // Out-neighbors
    fn NMinus(&self, v: &V) -> Set<V>;        // In-neighbors
    fn NPlusOfVertices(&self, u_set: &Set<V>) -> Set<V>;
    fn NMinusOfVertices(&self, u_set: &Set<V>) -> Set<V>;
    fn Incident(&self, e: &(V, V), v: &V) -> B;
    fn Degree(&self, v: &V) -> N;             // Out-degree by convention
    fn InDegree(&self, v: &V) -> N;
    fn OutDegree(&self, v: &V) -> N;
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> DirGraphEphChap6_1Trait<V> for DirGraphEph<V> {
    fn empty() -> DirGraphEph<V> { DirGraphEph { V: SetLit![], A: SetLit![] } }
    fn FromSets(V: Set<V>, A: Set<(V, V)>) -> DirGraphEph<V> { DirGraphEph { V, A } }
    fn vertices(&self) -> &Set<V> { &self.V }
    fn arcs(&self) -> &Set<(V, V)> { &self.A }
    fn sizeV(&self) -> N { self.V.size() }
    fn sizeA(&self) -> N { self.A.size() }

    fn Neighbor(&self, u: &V, v: &V) -> B {
        // Adjacent if there is an arc either way
        if B::True == self.A.mem(&(u.clone(), v.clone())) || B::True == self.A.mem(&(v.clone(), u.clone())) { B::True } else { B::False }
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
        for (x, y) in self.A.iter().cloned() { if x == *v { let _ = out.insert(y.clone()); } }
        out
    }

    fn NMinus(&self, v: &V) -> Set<V> {
        let mut inn: Set<V> = SetLit![];
        for (x, y) in self.A.iter().cloned() { if y == *v { let _ = inn.insert(x.clone()); } }
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

    fn Incident(&self, e: &(V, V), v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B::False } }

    fn Degree(&self, v: &V) -> N { self.NPlus(v).size() }
    fn InDegree(&self, v: &V) -> N { self.NMinus(v).size() }
    fn OutDegree(&self, v: &V) -> N { self.NPlus(v).size() }
}

impl<V: Eq + Hash + Clone + std::fmt::Debug + std::fmt::Display> std::fmt::Debug for DirGraphEph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DirGraphEph").field("V", &self.V).field("A", &self.A).finish()
    }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for DirGraphEph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V={} A={:?}", self.V, self.A)
    }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for DirGraphEph<V> { fn eq(&self, other: &Self) -> bool { self.V == other.V && self.A == other.A } }
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for DirGraphEph<V> {}

}

pub use DirGraphEphChap6_1::DirGraphEphChap6_1Trait;

#[macro_export]
macro_rules! DirGraphLit {
    ( V: [ $( $v:expr ),* $(,)? ], A: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
        let __V: $crate::SetEphChap5_1::SetEphChap5_1::Set<_> = $crate::SetLit![ $( $v ),* ];
        let __A: $crate::SetEphChap5_1::SetEphChap5_1::Set<_> = $crate::SetLit![ $( ($u, $w) ),* ];
        < $crate::DirGraphEphChap6_1::DirGraphEphChap6_1::DirGraphEph<_> as $crate::DirGraphEphChap6_1::DirGraphEphChap6_1::DirGraphEphChap6_1Trait<_> >::FromSets(__V, __A)
    }}}

#[allow(dead_code)]
pub fn __dirgraph_macro_typecheck_exercise() {
    use crate::DirGraphEphChap6_1::DirGraphEphChap6_1::DirGraphEph as DG;
    let _g0: DG<usize> = DirGraphLit!( V: [], A: [] );
    let _g1 = DirGraphLit!( V: [0,1,2], A: [(0,1),(1,2)] );
}

