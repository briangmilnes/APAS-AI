//! Chapter 5.5 ephemeral Mapping (Function) built on `Relation<A,B>`.

pub mod MappingStEphChap5_5 {
    use crate::RelationStEphChap5_2::RelationStEphChap5_2::*;
    use crate::SetLit;
    use crate::SetStEphChap5_1::SetStEphChap5_1::*;
    use crate::Types::Types::*;
    use std::collections::HashMap;
    use std::hash::Hash;

    #[derive(Clone)]
    pub struct Mapping<A, B> {
        rel: Relation<A, B>,
    }

    pub trait MappingStEphChap5_5Trait<
        X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized,
        Y: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized,
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

    impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std::fmt::Debug>
        PartialEq for Mapping<A, B>
    {
        fn eq(&self, other: &Self) -> bool {
            self.rel == other.rel
        }
    }
    impl<A: Eq + Hash + std::fmt::Display + std::fmt::Debug, B: Eq + Hash + std::fmt::Display + std::fmt::Debug> Eq
        for Mapping<A, B>
    {
    }

    impl<A: std::fmt::Debug + Eq + Hash, B: std::fmt::Debug + Eq + Hash> std::fmt::Debug for Mapping<A, B> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.rel.fmt(f)
        }
    }
    impl<A: std::fmt::Display + Eq + Hash, B: std::fmt::Display + Eq + Hash> std::fmt::Display for Mapping<A, B> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.rel.fmt(f)
        }
    }

    impl<
        X: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized,
        Y: Eq + Hash + std::fmt::Display + std::fmt::Debug + Clone + Sized,
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

        fn size(&self) -> N {
            self.rel.size()
        }

        fn domain(&self) -> Set<X> {
            self.rel.domain()
        }

        fn range(&self) -> Set<Y> {
            self.rel.range()
        }

        fn mem(&self, a: &X, b: &Y) -> B {
            self.rel.mem(a, b)
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, Pair<X, Y>> {
            self.rel.iter()
        }
    }

    #[macro_export]
    macro_rules! MappingLit {
        () => {{
            < $crate::MappingStEphChap5_5::MappingStEphChap5_5::Mapping<_, _> as $crate::MappingStEphChap5_5::MappingStEphChap5_5::MappingStEphChap5_5Trait<_, _> >::FromRelation(& $crate::RelationLit![])
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __r = $crate::RelationLit![ $( ($a, $b) ),* ];
            < $crate::MappingStEphChap5_5::MappingStEphChap5_5::Mapping<_, _> as $crate::MappingStEphChap5_5::MappingStEphChap5_5::MappingStEphChap5_5Trait<_, _> >::FromRelation(&__r)
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
