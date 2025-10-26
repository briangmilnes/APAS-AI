//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.5 ephemeral Mapping (Function) built on `RelationStEph<A,B>`.

pub mod MappingStEph {

    use std::collections::HashMap;
    use std::collections::hash_set::Iter;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::Chap05::RelationStEph::RelationStEph::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::SetLit;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct MappingStEph<A, B> {
        rel: RelationStEph<A, B>,
    }

    pub trait MappingStEphTrait<X: StT + Hash, Y: StT + Hash> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                               -> Self;

        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        fn FromVec(v: Vec<Pair<X, Y>>)           -> Self;

        /// APAS: Work Θ(|r|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|r|), Span Θ(1)
        fn FromRelation(r: &RelationStEph<X, Y>) -> Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                           -> N;

        /// APAS: Work Θ(|m|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|m|), Span Θ(1)
        fn domain(&self)                         -> SetStEph<X>;

        /// APAS: Work Θ(|m|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|m|), Span Θ(1)
        fn range(&self)                          -> SetStEph<Y>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, a: &X, b: &Y)              -> B;

        fn iter(&self)                           -> Iter<'_, Pair<X, Y>>;
    }

    // ensure unique pairs (mapping property: each domain element maps to at most one range element)
    fn unique_pairs_from_iter<A: StT + Hash, B: StT + Hash, I: IntoIterator<Item = Pair<A, B>>>(
        iter: I,
    ) -> SetStEph<Pair<A, B>> {
        let mut m = HashMap::<A, B>::new();
        for Pair(a, b) in iter {
            m.insert(a, b);
        }
        let pairs = m.into_iter().map(|(a, b)| Pair(a, b)).collect::<Vec<Pair<A, B>>>();
        SetStEph::FromVec(pairs)
    }

    impl<X: StT + Hash, Y: StT + Hash> MappingStEphTrait<X, Y> for MappingStEph<X, Y> {
        fn empty() -> MappingStEph<X, Y> {
            MappingStEph {
                rel: <RelationStEph<X, Y> as RelationStEphTrait<X, Y>>::empty(),
            }
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> MappingStEph<X, Y> {
            let pairs = unique_pairs_from_iter(v);
            MappingStEph {
                rel: <RelationStEph<X, Y> as RelationStEphTrait<X, Y>>::FromSet(pairs),
            }
        }

        fn FromRelation(r: &RelationStEph<X, Y>) -> MappingStEph<X, Y> {
            let pairs = unique_pairs_from_iter(r.iter().cloned());
            MappingStEph {
                rel: <RelationStEph<X, Y> as RelationStEphTrait<X, Y>>::FromSet(pairs),
            }
        }

        fn size(&self) -> N { self.rel.size() }

        fn domain(&self) -> SetStEph<X> { self.rel.domain() }

        fn range(&self) -> SetStEph<Y> { self.rel.range() }

        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }

        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.rel.iter() }
    }

    impl<A: StT + Hash, B: StT + Hash> PartialEq for MappingStEph<A, B> {
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }
    }

    impl<A: StT + Hash, B: StT + Hash> Eq for MappingStEph<A, B> {}

    impl<A: StT + Hash, B: StT + Hash> Debug for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }
    }

    impl<A: StT + Hash, B: StT + Hash> Display for MappingStEph<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }
    }

    #[macro_export]
    macro_rules! MappingLit {
        () => {{
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> as $crate::Chap05::MappingStEph::MappingStEph::MappingStEphTrait<_, _> >::FromRelation(& $crate::RelationLit![])
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __pairs = vec![ $( $crate::Types::Types::Pair($a, $b) ),* ];
            // Check for duplicate domain elements
            let mut __seen_keys = std::collections::HashSet::new();
            for pair in &__pairs {
                let key = &pair.0;
                if !__seen_keys.insert(key) {
                    panic!("MappingLit!: duplicate domain element {:?}", key);
                }
            }
            < $crate::Chap05::MappingStEph::MappingStEph::MappingStEph<_, _> as $crate::Chap05::MappingStEph::MappingStEph::MappingStEphTrait<_, _> >::FromVec(__pairs)
        }};
    }
}
