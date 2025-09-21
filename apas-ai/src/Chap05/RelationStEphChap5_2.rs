//! Chapter 5.2 ephemeral Relation built on `Set<Pair<A,B>>`.

pub mod RelationStEphChap5_2 {

    use std::collections::hash_set::Iter;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::hash::Hash;

    use crate::SetLit;
    use crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct Relation<A, B> {
        pairs: Set<Pair<A, B>>,
    }

    pub trait RelationStEphChap5_2Trait<
        X: Eq + Hash + Display + Debug + Clone + Sized,
        Y: Eq + Hash + Display + Debug + Clone + Sized,
    >
    {
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

    impl<A, B> Relation<A, B> {
        pub fn FromVec(v: Vec<Pair<A, B>>) -> Relation<A, B>
        where
            A: Eq + Hash + Display + Debug + Clone,
            B: Eq + Hash + Display + Debug + Clone,
        {
            Relation { pairs: Set::FromVec(v) }
        }
    }

    impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> PartialEq for Relation<A, B> {
        fn eq(&self, other: &Self) -> bool { self.pairs == other.pairs }
    }

    impl<A: Eq + Hash + Display + Debug, B: Eq + Hash + Display + Debug> Eq for Relation<A, B> {}

    impl<A: Debug + Eq + Hash, B: Debug + Eq + Hash> Debug for Relation<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { self.pairs.fmt(f) }
    }

    impl<A: Display + Eq + Hash, B: Display + Eq + Hash> Display for Relation<A, B> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{{")?;
            let mut first = true;
            for Pair(a, b) in self.pairs.iter() {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "({},{}),", a, b)?;
            }
            write!(f, "}}")
        }
    }

    impl<X: StT + Hash, Y: StT + Hash>
        RelationStEphChap5_2Trait<X, Y> for Relation<X, Y>
    {
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
            if self.pairs.mem(&Pair(a.clone(), b.clone())) == B::True {
                B::True
            } else {
                B::False
            }
        }

        fn iter(&self) -> Iter<'_, Pair<X, Y>> { self.pairs.iter() }
    }

    #[macro_export]
    macro_rules! RelationLit {
        () => {{
            let __pairs: $crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::Set<$crate::Types::Types::Pair<_, _>> = < $crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::Set<_> >::empty();
            < $crate::Chap05::RelationStEphChap5_2::RelationStEphChap5_2::Relation<_, _> as $crate::Chap05::RelationStEphChap5_2::RelationStEphChap5_2::RelationStEphChap5_2Trait<_, _> >::FromSet(__pairs)
        }};
        ( $( ($a:expr, $b:expr) ),* $(,)? ) => {{
            let __pairs = {
                let mut __s = < $crate::Chap05::SetStEphChap5_1::SetStEphChap5_1::Set<_> >::empty();
                $( let _ = __s.insert($crate::Types::Types::Pair($a, $b)); )*
                __s
            };
            < $crate::Chap05::RelationStEphChap5_2::RelationStEphChap5_2::Relation<_, _> as $crate::Chap05::RelationStEphChap5_2::RelationStEphChap5_2::RelationStEphChap5_2Trait<_, _> >::FromSet(__pairs)
        }};
    }

    #[allow(dead_code)]
    fn _RelationLit_type_checks() {
        let _ = RelationLit![(1, "a")]; // non-empty infers (e.g., i32, &str)
        let _: Relation<i32, &str> = RelationLit![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __relation_macro_typecheck_exercise() {
        let _r0: Relation<usize, char> = RelationLit![];
        let _r1 = RelationLit![(0, 'a')];
        let _r2 = RelationLit![(0, 'a'), (1, 'b')];
    }
}
