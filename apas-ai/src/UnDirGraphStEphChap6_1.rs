//! Chapter 6.1 Undirected Graph (ephemeral) using Set for vertices and edges.

pub mod UnDirGraphStEphChap6_1 {
use crate::Types::Types::*;
use crate::SetStEphChap5_1::SetStEphChap5_1::*;
use crate::SetLit;
use std::hash::Hash;

#[derive(Clone)]
pub struct UnDirGraphStEph<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    V: Set<V>,
    E: Set<Pair<V, V>>,
}

pub trait UnDirGraphStEphChap6_1Trait<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> {
    fn empty() -> UnDirGraphStEph<V>;
    fn FromSets(V: Set<V>, E: Set<Pair<V, V>>) -> UnDirGraphStEph<V>;
    fn vertices(&self) -> &Set<V>;
    fn edges(&self) -> &Set<Pair<V, V>>;
    fn sizeV(&self) -> N;
    fn sizeE(&self) -> N;
    // Neighborhood and adjacency APIs (undirected)
    fn Neighbor(&self, u: &V, v: &V) -> B;
    fn NG(&self, v: &V) -> Set<V>;
    fn NGOfVertices(&self, u_set: &Set<V>) -> Set<V>;
    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B;
    fn Degree(&self, v: &V) -> N;
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> UnDirGraphStEphChap6_1Trait<V> for UnDirGraphStEph<V> {
    fn empty() -> UnDirGraphStEph<V> { UnDirGraphStEph { V: SetLit![], E: SetLit![] } }
    fn FromSets(V: Set<V>, E: Set<Pair<V, V>>) -> UnDirGraphStEph<V> { UnDirGraphStEph { V, E } }
    fn vertices(&self) -> &Set<V> { &self.V }
    fn edges(&self) -> &Set<Pair<V, V>> { &self.E }
    fn sizeV(&self) -> N { self.V.size() }
    fn sizeE(&self) -> N { self.E.size() }

    fn Neighbor(&self, u: &V, v: &V) -> B {
        // Treat edges as unordered: {u,v}
        if B::True == self.E.mem(&Pair(u.clone(), v.clone())) || B::True == self.E.mem(&Pair(v.clone(), u.clone())) { B::True } else { B::False }
    }

    fn NG(&self, v: &V) -> Set<V> {
        let mut ng: Set<V> = SetLit![];
        for Pair(a, b) in self.E.iter().cloned() {
            if a == *v { let _ = ng.insert(b.clone()); }
            else if b == *v { let _ = ng.insert(a.clone()); }
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

    fn Incident(&self, e: &Pair<V, V>, v: &V) -> B { if &e.0 == v || &e.1 == v { B::True } else { B::False } }

    fn Degree(&self, v: &V) -> N { self.NG(v).size() }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Debug for UnDirGraphStEph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnDirGraphStEph").field("V", &self.V).field("E", &self.E).finish()
    }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> std::fmt::Display for UnDirGraphStEph<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V={} E={:?}", self.V, self.E)
    }
}

impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> PartialEq for UnDirGraphStEph<V> { fn eq(&self, other: &Self) -> bool { self.V == other.V && self.E == other.E } }
impl<V: Eq + Hash + Clone + std::fmt::Display + std::fmt::Debug> Eq for UnDirGraphStEph<V> {}

    #[macro_export]
    macro_rules! UnDirGraphLit {
        ( V: [ $( $v:expr ),* $(,)? ], E: [ $( ( $u:expr , $w:expr ) ),* $(,)? ] ) => {{
            let __V: $crate::SetStEphChap5_1::SetStEphChap5_1::Set<_> = $crate::SetLit![ $( $v ),* ];
            let __E: $crate::SetStEphChap5_1::SetStEphChap5_1::Set<_> = {
                let mut __s = < $crate::SetStEphChap5_1::SetStEphChap5_1::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Pair($u, $w)); )*
                __s
            };
            < $crate::UnDirGraphStEphChap6_1::UnDirGraphStEphChap6_1::UnDirGraphStEph<_> as $crate::UnDirGraphStEphChap6_1::UnDirGraphStEphChap6_1::UnDirGraphStEphChap6_1Trait<_> >::FromSets(__V, __E)
        }};
    }

    #[allow(dead_code)]
    pub fn __undirgraph_macro_typecheck_exercise() {
        let _g0: UnDirGraphStEph<usize> = UnDirGraphLit!( V: [], E: [] );
        let _g1 = UnDirGraphLit!( V: [0,1,2], E: [(0,1),(1,2)] );
    }
}

