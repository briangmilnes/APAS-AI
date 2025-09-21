//! Chapter 5.5 ephemeral Mapping (Function) built on `Relation<A,B>`.

pub mod MappingStEphChap5_5 {
    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;
    
    use crate::Chap05::RelationStEphChap5_2::RelationStEphChap5_2::*;
    use crate::SetLit;
    use crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct Mapping<A, B> {
        rel: Relation<A, B>,
    }

    pub trait MappingStEphChap5_5Trait<
        X: StT + Hash,
        Y: StT + Hash,
    >
    {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Mapping<X, Y>;

        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y>;

        /// APAS: Work Θ(|r|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|r|), Span Θ(1)
        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;

        /// APAS: Work Θ(|m|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|m|), Span Θ(1)
        fn domain(&self) -> Set<X>;

        /// APAS: Work Θ(|m|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|m|), Span Θ(1)
        fn range(&self) -> Set<Y>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, a: &X, b: &Y) -> B;

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>>;
    }

    impl<A: Eq + Hash, B: Eq + Hash> Mapping<A, B> {
        fn unique_pairs_from_iter<I: IntoIterator<Item = Pair<A, B>>>(iter: I) -> Set<Pair<A, B>> {
            let mut m: HashMap<A, B> = HashMap::new();
            for Pair(a, b) in iter {
                m.insert(a, b);
            }
            let pairs: Vec<Pair<A, B>> = m.into_iter().map(|(a, b)| Pair(a, b)).collect();
            Set::FromVec(pairs)
        }
    }

    impl<A: StT + Hash, B: StT + Hash>
        PartialEq for Mapping<A, B>
    {
        fn eq(&self, other: &Self) -> bool { self.rel == other.rel }
    }
    impl<A: StT + Hash, B: StT + Hash> Eq
        for Mapping<A, B>
    {
    }

    impl<A: StT + Hash, B: StT + Hash> Debug for Mapping<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Debug::fmt(&self.rel, f) }
    }
    impl<A: StT + Hash, B: StT + Hash> Display for Mapping<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(&self.rel, f) }
    }

        impl<
            X: StT + Hash,
            Y: StT + Hash,
        > MappingStEphChap5_5Trait<X, Y> for Mapping<X, Y>
    {
        fn empty() -> Mapping<X, Y> {
            Mapping {
                rel: <Relation<X, Y> as RelationStEphChap5_2Trait<X, Y>>::empty(),
            }
        }

        fn FromVec(v: Vec<Pair<X, Y>>) -> Mapping<X, Y> {
            let pairs = Self::unique_pairs_from_iter(v);
            Mapping {
                rel: <Relation<X, Y> as RelationStEphChap5_2Trait<X, Y>>::FromSet(pairs),
            }
        }

        fn FromRelation(r: &Relation<X, Y>) -> Mapping<X, Y> {
            let pairs = Self::unique_pairs_from_iter(r.iter().cloned());
            Mapping {
                rel: <Relation<X, Y> as RelationStEphChap5_2Trait<X, Y>>::FromSet(pairs),
            }
        }

        fn size(&self) -> N { self.rel.size() }

        fn domain(&self) -> Set<X> { self.rel.domain() }

        fn range(&self) -> Set<Y> { self.rel.range() }

        fn mem(&self, a: &X, b: &Y) -> B { self.rel.mem(a, b) }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> { self.rel.iter() }
    }

    #[macro_export]
    macro_rules! MappingLit {
        () => {{
            < $crate::Chap05::MappingStEphChap5_5::MappingStEphChap5_5::Mapping<_, _> as $crate::Chap05::MappingStEphChap5_5::MappingStEphChap5_5::MappingStEphChap5_5Trait<_, _> >::FromRelation(& $crate::RelationLit![])
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __r = $crate::RelationLit![ $( ($a, $b) ),* ];
            < $crate::Chap05::MappingStEphChap5_5::MappingStEphChap5_5::Mapping<_, _> as $crate::Chap05::MappingStEphChap5_5::MappingStEphChap5_5::MappingStEphChap5_5Trait<_, _> >::FromRelation(&__r)
        }};
    }

    #[allow(dead_code)]
    fn _MappingLit_type_checks() {
        let _ = MappingLit![(1, "a")]; // non-empty infers (e.g., i32, &str)
        let _: Mapping<i32, &str> = MappingLit![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __mapping_macro_typecheck_exercise() {
        let _m0: Mapping<usize, char> = MappingLit![];
        let _m1 = MappingLit![(0, 'a')];
        let _m2 = MappingLit![(0, 'a'), (1, 'b')];
    }
}
