//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 5.1 ephemeral Set built on `std::collections::HashSet`.

pub mod SetStEph {

use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};

use crate::Types::Types::*;
    #[derive(Clone)]
    pub struct Set<T> {
        data: HashSet<T>,
    }

    pub trait SetStEphTrait<T: StT + Hash> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Set<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(x: T) -> Set<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn mem(&self, x: &T) -> B;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn union(&self, other: &Set<T>) -> Set<T>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn intersection(&self, other: &Set<T>) -> Set<T>;
        /// APAS: Work Θ(|parts| × |a|²), Span Θ(1)
        /// claude-4-sonet: Work Θ(|parts| × |a|²), Span Θ(1)
        fn partition(&self, parts: &Set<Set<T>>) -> B;

        /// APAS: Work Θ(|a| × |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| × |b|), Span Θ(1)
        fn CartesianProduct<U: StT + Hash>(&self, other: &Set<U>) -> Set<Pair<T, U>>;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn insert(&mut self, x: T) -> &mut Self;

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn iter(&self) -> std::collections::hash_set::Iter<'_, T>;
        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        fn FromVec(v: Vec<T>) -> Set<T>;
    }

    impl<T: Eq + Hash> PartialEq for Set<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    impl<T: Eq + Hash> Eq for Set<T> {}

    impl<T: Eq + Hash + std::fmt::Debug> std::fmt::Debug for Set<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_set().entries(self.data.iter()).finish()
        }
    }

    impl<T: Eq + Hash + std::fmt::Display> std::fmt::Display for Set<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{{")?;
            let mut first = true;
            for x in self.data.iter() {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", x)?;
            }
            write!(f, "}}")
        }
    }

    // Provide an order-independent Hash so sets of sets can be placed in a HashSet.
    impl<T: Eq + Hash> Hash for Set<T> {
        fn hash<H: Hasher>(&self, state: &mut H) {
            use std::collections::hash_map::DefaultHasher;
            let mut element_hashes: Vec<u64> = Vec::with_capacity(self.data.len());
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

    impl<T: Eq + Hash> Set<T> {
        pub fn empty() -> Set<T> { Set { data: HashSet::new() } }

        pub fn singleton(x: T) -> Set<T> {
            let mut s = HashSet::with_capacity(1);
            let _ = s.insert(x);
            Set { data: s }
        }

        pub fn size(&self) -> N { self.data.len() }

        pub fn mem(&self, x: &T) -> B {
            if self.data.contains(x) {
                true
            } else {
                false
            }
        }

        pub fn union(&self, other: &Set<T>) -> Set<T>
        where
            T: Clone,
        {
            let mut out = self.clone();
            for x in other.data.iter() {
                let _ = out.data.insert(x.clone());
            }
            out
        }

        pub fn intersection(&self, other: &Set<T>) -> Set<T>
        where
            T: Clone,
        {
            let mut out = HashSet::with_capacity(self.data.len().min(other.data.len()));
            for x in self.data.intersection(&other.data) {
                let _ = out.insert(x.clone());
            }
            Set { data: out }
        }

        pub fn partition(&self, parts: &Set<Set<T>>) -> B {
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

        pub fn CartesianProduct<U: StT + Hash + Clone>(&self, other: &Set<U>) -> Set<Pair<T, U>>
        where
            T: Clone,
        {
            let mut out: HashSet<Pair<T, U>> = HashSet::new();
            for a in self.data.iter() {
                for b in other.data.iter() {
                    let _ = out.insert(Pair(a.clone(), b.clone()));
                }
            }
            Set { data: out }
        }

        pub fn insert(&mut self, x: T) -> &mut Self {
            let _ = self.data.insert(x);
            self
        }

        pub fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }

        pub fn FromVec(v: Vec<T>) -> Set<T> {
            let mut s = HashSet::with_capacity(v.len());
            for x in v {
                let _ = s.insert(x);
            }
            Set { data: s }
        }
    }

    impl<T: StT + Hash> SetStEphTrait<T> for Set<T> {
        fn empty() -> Set<T> { Set { data: HashSet::new() } }

        fn singleton(x: T) -> Set<T> {
            let mut s = HashSet::with_capacity(1);
            let _ = s.insert(x);
            Set { data: s }
        }

        fn size(&self) -> N { self.data.len() }

        fn mem(&self, x: &T) -> B {
            if self.data.contains(x) {
                true
            } else {
                false
            }
        }

        fn union(&self, other: &Set<T>) -> Set<T>
        where
            T: Clone,
        {
            let mut out = self.clone();
            for x in other.data.iter() {
                let _ = out.data.insert(x.clone());
            }
            out
        }

        fn intersection(&self, other: &Set<T>) -> Set<T>
        where
            T: Clone,
        {
            let mut out = HashSet::with_capacity(self.data.len().min(other.data.len()));
            for x in self.data.intersection(&other.data) {
                let _ = out.insert(x.clone());
            }
            Set { data: out }
        }

        fn partition(&self, parts: &Set<Set<T>>) -> B {
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

        fn CartesianProduct<U: StT + Hash + Clone>(&self, other: &Set<U>) -> Set<Pair<T, U>>
        where
            T: Clone,
        {
            let mut out: HashSet<Pair<T, U>> = HashSet::new();
            for a in self.data.iter() {
                for b in other.data.iter() {
                    let _ = out.insert(Pair(a.clone(), b.clone()));
                }
            }
            Set { data: out }
        }

        fn insert(&mut self, x: T) -> &mut Self {
            let _ = self.data.insert(x);
            self
        }

        fn iter(&self) -> std::collections::hash_set::Iter<'_, T> { self.data.iter() }

        fn FromVec(v: Vec<T>) -> Set<T> {
            let mut s = HashSet::with_capacity(v.len());
            for x in v {
                let _ = s.insert(x);
            }
            Set { data: s }
        }
    }

    #[macro_export]
    macro_rules! SetLit {
        () => {{
            < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty()
        }};
        ($($x:expr),* $(,)?) => {{
            let mut __s = < $crate::Chap05::SetStEph::SetStEph::Set<_> >::empty();
            $( let _ = __s.insert($x); )*
            __s
        }};
    }

    #[allow(dead_code)]
    fn _SetLit_type_checks() {
        let _ = SetLit![1]; // non-empty infers (e.g., i32)
        let _: Set<i32> = SetLit![]; // empty form requires explicit type
    }

    #[allow(dead_code)]
    pub fn __set_macro_typecheck_exercise() {
        let _s0: Set<&'static str> = SetLit![];
        let _s1 = SetLit!("only");
        let _s2 = SetLit!("a", "b", "c");
    }
}
