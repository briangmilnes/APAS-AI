//! Chapter 5.2 ephemeral Relation built on `Set<(A,B)>`.

pub mod RelationStEphChap5_2 {

use std::fmt::{Display, Debug, Formatter, Result};
use std::hash::Hash;
use std::collections::hash_set::Iter;

use crate::Types::Types::*;
use crate::SetStEphChap5_1::SetStEphChap5_1::*;
use crate::SetLit;

#[derive(Clone)]
pub struct Relation<A, B> {
    pairs: Set<(A, B)>,
}

pub trait RelationStEphChap5_2Trait<X: Eq + Hash + Display + Debug + Clone + Sized, 
                                  Y: Eq + Hash + Display + Debug + Clone + Sized> {
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

    fn iter(&self) -> Iter<'_, (X, Y)>;
}

impl<A, B> Relation<A, B> {
    pub fn FromVec(v: Vec<(A, B)>) -> Relation<A, B>
    where
        A: Eq + Hash + Display + Debug,
        B: Eq + Hash + Display + Debug,
    {
        Relation { pairs: Set::FromVec(v) }
    }
}

impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> PartialEq for Relation<A, B> {
    fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }
}

impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> Eq for Relation<A, B> {}

impl<A: Debug + Eq + Hash, B: Debug + Eq + Hash> Debug for Relation<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.pairs.fmt(f)
    }
}

impl<A: Display + Eq + Hash, B: Display + Eq + Hash> Display for Relation<A, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{{")?;
        let mut first = true;
        for (a, b) in self.pairs.iter() {
            if !first { write!(f, ", ")?; } else { first = false; }
            write!(f, "({},{}),", a, b)?;
        }
        write!(f, "}}")
    }
}

impl<X: Eq + Hash + Display + Debug + Clone + Sized, 
     Y: Eq + Hash + Display +  Debug + Clone + Sized> RelationStEphChap5_2Trait<X, Y> for Relation<X, Y> {
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

    fn iter(&self) -> Iter<'_, (X, Y)> { self.pairs.iter() }
}

}

pub use RelationStEphChap5_2::RelationStEphChap5_2Trait;

#[macro_export]
macro_rules! RelationLit {
    () => {{
        < $crate::RelationStEphChap5_2::RelationStEphChap5_2::Relation<_, _> as $crate::RelationStEphChap5_2::RelationStEphChap5_2::RelationStEphChap5_2Trait<_, _> >::FromSet($crate::SetLit![])
    }};
    ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
        let __pairs = $crate::SetLit![ $( ($a, $b) ),* ];
        < $crate::RelationStEphChap5_2::RelationStEphChap5_2::Relation<_, _> as $crate::RelationStEphChap5_2::RelationStEphChap5_2::RelationStEphChap5_2Trait<_, _> >::FromSet(__pairs)
    }};
}

#[allow(dead_code)]
pub fn __relation_macro_typecheck_exercise() {
    use crate::RelationStEphChap5_2::RelationStEphChap5_2::Relation as Rel;
    let _r0: Rel<usize, char> = RelationLit![];
    let _r1 = RelationLit![(0,'a')];
    let _r2 = RelationLit![(0,'a'), (1,'b')];
}

