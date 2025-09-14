//! Chapter 5.5 ephemeral Mapping (Function) built on `Relation<A,B>`.

pub mod MappingEphChap5_5 {
use crate::Types::Types::*;
use crate::RelationEphChap5_2::RelationEphChap5_2::*;
use crate::SetEphChap5_1::SetEphChap5_1::*;
use crate::SetLit;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct Mapping<A, B> {
    rel: Relation<A, B>,
}

pub trait MappingEphChap5_5Trait<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized> {
    fn empty() -> Mapping<X, Y>;

    fn FromVec(v: Vec<(X, Y)>) -> Mapping<X, Y>;

    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;

    fn size(&self) -> N;

    fn domain(&self) -> Set<X>;

    fn range(&self) -> Set<Y>;

    fn mem(&self, a: &X, b: &Y) -> B;

    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)>;
}

impl<A, B> Mapping<A, B> {
    fn unique_pairs_from_iter<I>(iter: I) -> Set<(A, B)>
    where
        I: IntoIterator<Item = (A, B)>,
        A: Eq + Hash,
        B: Eq + Hash,
    {
        let mut m: HashMap<A, B> = HashMap::new();
        for (a, b) in iter { m.insert(a, b); }
        let pairs: Vec<(A, B)> = m.into_iter().collect();
        Set::FromVec(pairs)
    }
}

impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std::fmt::Debug> PartialEq for Mapping<A, B> {
    fn eq(&self, other: &Self) -> bool { self.rel == other.rel }
}
impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std::fmt::Debug> Eq for Mapping<A, B> {}

impl<A: std::fmt::Debug + Eq + Hash, B: std::fmt::Debug + Eq + Hash> std::fmt::Debug for Mapping<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }
}
impl<A: std::fmt::Display + Eq + Hash, B: std::fmt::Display + Eq + Hash> std::fmt::Display for Mapping<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.rel.fmt(f) }
}

impl<X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized> MappingEphChap5_5Trait<X, Y> for Mapping<X, Y> {
    fn empty() -> Mapping<X, Y> {
        Mapping { rel: <Relation<X, Y> as RelationEphChap5_2Trait<X, Y>>::empty() }
    }

    fn FromVec(v: Vec<(X, Y)>) -> Mapping<X, Y> {
        let pairs = Self::unique_pairs_from_iter(v);
        Mapping { rel: <Relation<X, Y> as RelationEphChap5_2Trait<X, Y>>::FromSet(pairs) }
    }

    fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {
        let pairs = Self::unique_pairs_from_iter(r.iter().cloned());
        Mapping { rel: <Relation<X, Y> as RelationEphChap5_2Trait<X, Y>>::FromSet(pairs) }
    }

    fn size(&self) -> N { self.rel.size() }

    fn domain(&self) -> Set<X> { self.rel.domain() }

    fn range(&self) -> Set<Y> { self.rel.range() }

    fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }

    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)> { self.rel.iter() }
}

}

pub use MappingEphChap5_5::MappingEphChap5_5Trait;

#[macro_export]
macro_rules! MappingLit {
    () => {{
        < $crate::MappingEphChap5_5::MappingEphChap5_5::Mapping<_, _> as $crate::MappingEphChap5_5::MappingEphChap5_5::MappingEphChap5_5Trait<_, _> >::FromRelation(& $crate::RelationLit![])
    }};
    ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
        let __r = $crate::RelationLit![ $( ($a, $b) ),* ];
        < $crate::MappingEphChap5_5::MappingEphChap5_5::Mapping<_, _> as $crate::MappingEphChap5_5::MappingEphChap5_5::MappingEphChap5_5Trait<_, _> >::FromRelation(&__r)
    }};
}

#[allow(dead_code)]
pub fn __mapping_macro_typecheck_exercise() {
    use crate::MappingEphChap5_5::MappingEphChap5_5::Mapping as Map;
    let _m0: Map<usize, char> = MappingLit![];
    let _m1 = MappingLit![(0,'a')];
    let _m2 = MappingLit![(0,'a'), (1,'b')];
}

