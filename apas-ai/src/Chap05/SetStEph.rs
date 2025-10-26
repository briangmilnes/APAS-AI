//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `std::collections::HashSet`.

pub mod SetStEph {

    use std::collections::HashSet;
    use std::collections::hash_set::Iter;
    use std::fmt::{Formatter, Result, Debug, Display};
    use std::hash::{Hash, Hasher};

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct SetStEph<T> {
        data: HashSet<T>,
    }

    pub trait SetStEphTrait<T: StT + Hash> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                     -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T)                                             -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self)                                                 -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, x: &T)                                           -> B;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, other: &SetStEph<T>)                           -> Self;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn intersection(&self, other: &SetStEph<T>)                    -> Self;
        /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|parts| × |a|²), Span Θ(1)
        fn partition(&self, parts: &SetStEph<SetStEph<T>>)             -> B;

        /// APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: T)                                     -> &mut Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self)                                                 -> Iter<'_, T>;
        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        fn FromVec(v: Vec<T>)                                          -> Self;
    }

    // Provide an order-independent Hash so sets of sets can be placed in a HashSet.

    impl<T: StT + Hash> SetStEphTrait<T> for SetStEph<T> {
        fn empty() -> SetStEph<T> { SetStEph { data: HashSet::new() } }

        fn singleton(x: T) -> SetStEph<T> {
            let mut s = HashSet::with_capacity(1);
            let _ = s.insert(x);
            SetStEph { data: s }
        }

        fn size(&self) -> N { self.data.len() }

        fn mem(&self, x: &T) -> B { self.data.contains(x) }

        fn union(&self, other: &SetStEph<T>) -> SetStEph<T>
        where
            T: Clone,
        {
            let mut out = self.clone();
            for x in other.data.iter() {
                let _ = out.data.insert(x.clone());
            }
            out
        }

        fn intersection(&self, other: &SetStEph<T>) -> SetStEph<T>
        where
            T: Clone,
        {
            let mut out = HashSet::with_capacity(self.data.len().min(other.data.len()));
            for x in self.data.intersection(&other.data) {
                let _ = out.insert(x.clone());
            }
            SetStEph { data: out }
        }

        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B {
            for x in self.data.iter() {
                let mut count: N = 0;
                for subset in parts.data.iter() {
                    if subset.data.contains(x) {
                        count += 1;
                        if count > 1 {
                            return false;
                        }
                    }
                }
                if count == 0 {
                    return false;
                }
            }
            true
        }

        fn CartesianProduct<U: StT + Hash + Clone>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>
        where
            T: Clone,
        {
            let mut out = HashSet::<Pair<T, U>>::new();
            for a in self.data.iter() {
                for b in other.data.iter() {
                    let _ = out.insert(Pair(a.clone(), b.clone()));
                }
            }
            SetStEph { data: out }
        }

        fn insert(&mut self, x: T) -> &mut Self {
            let _ = self.data.insert(x);
            self
        }

        fn iter(&self) -> Iter<'_, T> { self.data.iter() }

        fn FromVec(v: Vec<T>) -> SetStEph<T> {
            let mut s = HashSet::with_capacity(v.len());
            for x in v {
                let _ = s.insert(x);
            }
            SetStEph { data: s }
        }
    }

    impl<T: Eq + Hash> PartialEq for SetStEph<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    impl<T: Eq + Hash> Eq for SetStEph<T> {}

    impl<T: Eq + Hash + Debug> Debug for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { f.debug_set().entries(self.data.iter()).finish() }
    }
    impl<T: Eq + Hash + Display> Display for SetStEph<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{{")?;
            let mut first = true;
            for x in self.data.iter() {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{x}")?;
            }
            write!(f, "}}")
        }
    }

    impl<T: Eq + Hash> Hash for SetStEph<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            use std::collections::hash_map::DefaultHasher;
            let mut element_hashes = Vec::<u64>::with_capacity(self.data.len());
            for e in self.data.iter() {
                let mut h = DefaultHasher::new();
                e.hash(&mut h);
                element_hashes.push(h.finish());
            }
            element_hashes.sort_unstable();
            self.data.len().hash(state);
            for h in element_hashes {
                h.hash(state);
            }
        }
    }

    #[macro_export]
    macro_rules! SetLit {
        () => {{
            < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetStEph::SetStEph::SetStEph<_> >::empty();
            $( let _ = __s.insert($x); )*
            __s
        }};
    }
}
