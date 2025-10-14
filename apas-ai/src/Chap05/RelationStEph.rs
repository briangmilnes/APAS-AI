//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.2 ephemeral Relation built on `Set<Pair<A,B>>`.

pub mod RelationStEph {

    use std::collections::hash_set::Iter;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct Relation<A, B> {
        pairs: Set<Pair<A, B>>,
    }

    pub trait RelationStEphTrait<X: StT + Hash, Y: StT + Hash> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Relation<X, Y>;

        /// APAS: Work Θ(|pairs|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|pairs|), Span Θ(1)
        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;

        /// APAS: Work Θ(|R|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|R|), Span Θ(1)
        fn domain(&self) -> Set<X>
        where
            X: Clone;

        /// APAS: Work Θ(|R|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|R|), Span Θ(1)
        fn range(&self) -> Set<Y>
        where
            Y: Clone;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, a: &X, b: &Y) -> B
        where
            X: Clone,
            Y: Clone;

        fn iter(&self) -> Iter<'_, Pair<X, Y>>;
    }

    impl<A: StT + Hash, B: StT + Hash> Relation<A, B> {
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B> { Relation { pairs: Set::FromVec(v) } }
    }

    impl<A: StT + Hash, B: StT + Hash> PartialEq for Relation<A, B> {
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for Relation<A, B> {}

    impl<A: StT + Hash, B: StT + Hash> Debug for Relation<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Debug::fmt(&self.pairs, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for Relation<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { std::fmt::Display::fmt(&self.pairs, f) }
    }

    impl<X: StT + Hash, Y: StT + Hash> RelationStEphTrait<X, Y> for Relation<X, Y> {
        fn empty() -> Relation<X, Y> { Relation { pairs: SetLit![] } }

        fn FromSet(pairs: Set<Pair<X, Y>>) -> Relation<X, Y> { Relation { pairs } }

        fn size(&self) -> N { self.pairs.size() }

        fn domain(&self) -> Set<X>
        where
            X: Clone,
        {
            let mut out: Set<X> = Set::empty();
            for Pair(a, _) in self.pairs.iter() {
                let _ = out.insert(a.clone());
            }
            out
        }

        fn range(&self) -> Set<Y>
        where
            Y: Clone,
        {
            let mut out: Set<Y> = Set::empty();
            for Pair(_, b) in self.pairs.iter() {
                let _ = out.insert(b.clone());
            }
            out
        }

        fn mem(&self, a: &X, b: &Y) -> B
        where
            X: Clone,
            Y: Clone,
        {
            self.pairs.mem(&Pair(a.clone(), b.clone()))
        }

        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }
    }

    #[macro_export]
    macro_rules! RelationLit {
        () => {{
            let __pairs: $crate::Chap05::SetStEph::SetStEph::Set<$crate::Types::Types::Pair<_, _>> = < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty();
            < $crate::Chap05::RelationStEph::RelationStEph::Relation<_, _> as $crate::Chap05::RelationStEph::RelationStEph::RelationStEphTrait<_, _> >::FromSet(__pairs)
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __pairs = {
                let mut __s = < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Pair($a, $b)); )*
                __s
            };
            < $crate::Chap05::RelationStEph::RelationStEph::Relation<_, _> as $crate::Chap05::RelationStEph::RelationStEph::RelationStEphTrait<_, _> >::FromSet(__pairs)
        }};
    }

}
