//! Chapter 5.2 ephemeral Relation built on `Set<(A,B)>`.

pub mod RelationEphChap5_2 {
use crate::Types::Types::*;
use crate::SetEphChap5_1::SetEphChap5_1::*;
use crate::SetLit;

#[derive(Clone)]
pub struct Relation<A, B> {
    pairs: Set<(A, B)>,
}

pub trait RelationEphChap5_2Trait<X: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized> {
    fn empty() -> Relation<X, Y>;

    fn FromSet(pairs: Set<(X, Y)>) -> Relation<X, Y>;

    fn size(&self) -> N;

    fn domain(&self) -> Set<X>
    where
        X: Clone;

    fn range(&self) -> Set<Y>
    where
        Y: Clone;

    fn mem(&self, a: &X, b: &Y) -> B
    where
        X: Clone,
        Y: Clone;

    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)>;
}

impl<A, B> Relation<A, B> {
    pub fn FromVec(v: Vec<(A, B)>) -> Relation<A, B>
    where
        A: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug,
        B: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug,
    {
        Relation { pairs: Set::FromVec(v) }
    }
}

impl<A: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug, B: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug> PartialEq for Relation<A, B> {
    fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }
}

impl<A: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug, B: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug> Eq for Relation<A, B> {}

impl<A: std::fmt::Debug + Eq + std::hash::Hash, B: std::fmt::Debug + Eq + std::hash::Hash> std::fmt::Debug for Relation<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pairs.fmt(f)
    }
}

impl<A: std::fmt::Display + Eq + std::hash::Hash, B: std::fmt::Display + Eq + std::hash::Hash> std::fmt::Display for Relation<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (a, b) in self.pairs.iter() {
            if !first { write!(f, ", ")?; } else { first = false; }
            write!(f, "({},{}),", a, b)?;
        }
        write!(f, "}}")
    }
}

impl<X: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized, Y: Eq + std::hash::Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized> RelationEphChap5_2Trait<X, Y> for Relation<X, Y> {
    fn empty() -> Relation<X, Y> {
        Relation { pairs: SetLit![] }
    }

    fn FromSet(pairs: Set<(X, Y)>) -> Relation<X, Y> { Relation { pairs } }

    fn size(&self) -> N { self.pairs.size() }

    fn domain(&self) -> Set<X>
    where
        X: Clone,
    {
        let mut out: Set<X> = Set::empty();
        for (a, _) in self.pairs.iter() { let _ = out.insert(a.clone()); }
        out
    }

    fn range(&self) -> Set<Y>
    where
        Y: Clone,
    {
        let mut out: Set<Y> = Set::empty();
        for (_, b) in self.pairs.iter() { let _ = out.insert(b.clone()); }
        out
    }

    fn mem(&self, a: &X, b: &Y) -> B
    where
        X: Clone,
        Y: Clone,
    {
        if self.pairs.mem(&(a.clone(), b.clone())) == B::True { B::True } else { B::False }
    }

    fn iter(&self) -> std::collections::hash_set::Iter<'_, (X, Y)> { self.pairs.iter() }
}

}

pub use RelationEphChap5_2::RelationEphChap5_2Trait;

#[macro_export]
macro_rules! RelationLit {
    () => {{
        < $crate::RelationEphChap5_2::RelationEphChap5_2::Relation<_, _> as $crate::RelationEphChap5_2::RelationEphChap5_2::RelationEphChap5_2Trait<_, _> >::FromSet($crate::SetLit![])
    }};
    ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
        let __pairs = $crate::SetLit![ $( ($a, $b) ),* ];
        < $crate::RelationEphChap5_2::RelationEphChap5_2::Relation<_, _> as $crate::RelationEphChap5_2::RelationEphChap5_2::RelationEphChap5_2Trait<_, _> >::FromSet(__pairs)
    }};
}

#[allow(dead_code)]
pub fn __relation_macro_typecheck_exercise() {
    use crate::RelationEphChap5_2::RelationEphChap5_2::Relation as Rel;
    let _r0: Rel<usize, char> = RelationLit![];
    let _r1 = RelationLit![(0,'a')];
    let _r2 = RelationLit![(0,'a'), (1,'b')];
}

