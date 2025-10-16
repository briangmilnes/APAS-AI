//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `std::collections::HashSet` - CLEAN pattern.

pub mod SetStEphClean {

    use std::collections::hash_set::Iter;
    use std::collections::HashSet;
    use std::fmt::Formatter;
    use std::fmt::Result;
    use std::fmt::{Debug, Display};
    use std::hash::{Hash, Hasher};

    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct SetStEph<T> {
        data: HashSet<T>,
    }

    pub trait SetStEphCleanTrait<T: StT + Hash>: Sized {
        // Method signatures (implementation in impl block)
        fn empty()                                                     -> Self;
        fn singleton(x: T)                                             -> Self;
        fn size(&self)                                                 -> N;
        fn mem(&self, x: &T)                                           -> B;
        fn iter(&self)                                                 -> Iter<'_, T>;
        fn insert(&mut self, x: T)                                     -> &mut Self;

        // Multi-line methods (signature only, implementation in impl block)
        fn union(&self, other: &Self)                                  -> Self
        where
            T: Clone;
        fn intersection(&self, other: &Self)                           -> Self
        where
            T: Clone;
        fn partition(&self, parts: &SetStEph<SetStEph<T>>)             -> B
        where
            T: Clone;
        fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>
        where
            T: Clone;
    }

    impl<T: StT + Hash> SetStEphCleanTrait<T> for SetStEph<T> {
        // One-line implementations (≤120 chars)
        fn empty() -> Self { SetStEph { data: HashSet::new() } }
        fn singleton(x: T) -> Self {
            let mut s = HashSet::with_capacity(1);
            s.insert(x);
            SetStEph { data: s }
        }
        fn size(&self) -> N { self.data.len() }
        fn mem(&self, x: &T) -> B { self.data.contains(x) }
        fn iter(&self) -> Iter<'_, T> { self.data.iter() }
        fn insert(&mut self, x: T) -> &mut Self {
            self.data.insert(x);
            self
        }

        // Multi-line default implementations
        fn union(&self, other: &Self) -> Self
        where
            T: Clone,
        {
            let mut out = self.clone();
            for x in other.data.iter() {
                out.data.insert(x.clone());
            }
            out
        }

        fn intersection(&self, other: &Self) -> Self
        where
            T: Clone,
        {
            let mut out = HashSet::with_capacity(self.data.len().min(other.data.len()));
            for x in self.data.intersection(&other.data) {
                out.insert(x.clone());
            }
            SetStEph { data: out }
        }

        fn partition(&self, parts: &SetStEph<SetStEph<T>>) -> B
        where
            T: Clone,
        {
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

        fn CartesianProduct<U: StT + Hash>(&self, other: &SetStEph<U>) -> SetStEph<Pair<T, U>>
        where
            T: Clone,
        {
            let mut out = HashSet::with_capacity(self.data.len() * other.data.len());
            for x in self.data.iter() {
                for y in other.data.iter() {
                    out.insert(Pair(x.clone(), y.clone()));
                }
            }
            SetStEph { data: out }
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
                }
                write!(f, "{}", x)?;
                first = false;
            }
            write!(f, "}}")
        }
    }

    impl<T: Eq + Hash> Hash for SetStEph<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            use std::collections::hash_map::DefaultHasher;
            let mut element_hashes: Vec<u64> = Vec::with_capacity(self.data.len());
            for e in self.data.iter() {
                let mut h = DefaultHasher::new();
                e.hash(&mut h);
                element_hashes.push(h.finish());
            }
            element_hashes.sort_unstable();
            for hash in element_hashes {
                hash.hash(state);
            }
        }
    }
}
