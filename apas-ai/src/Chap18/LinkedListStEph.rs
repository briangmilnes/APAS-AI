//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStEph (ephemeral).

pub mod LinkedListStEph {
    use std::collections::HashSet;

    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct NodeE<T: StT> {
        pub value: T,
        pub next: Option<Box<NodeE<T>>>,
    }

    #[derive(Clone)]
    pub struct LinkedListStEphS<T: StT> {
        head: Option<Box<NodeE<T>>>,
        len: N,
    }

    impl<T: StT> LinkedListStEphS<T> {
        pub fn empty() -> Self { LinkedListStEphS { head: None, len: 0 } }

        pub fn new(length: N, init_value: T) -> Self
        where
            T: Clone,
        {
            LinkedListStEphS::from_vec(vec![init_value; length])
        }

        pub fn singleton(item: T) -> Self { LinkedListStEphS::from_vec(vec![item]) }

        pub fn from_vec(mut elts: Vec<T>) -> Self {
            let len = elts.len();
            let mut head: Option<Box<NodeE<T>>> = None;
            while let Some(value) = elts.pop() {
                head = Some(Box::new(NodeE { value, next: head }));
            }
            LinkedListStEphS { head, len }
        }

        pub fn length(&self) -> N { self.len }

        pub fn nth(&self, index: N) -> &T {
            self.node_at(index)
                .map(|node| &node.value)
                .expect("Index out of bounds")
        }

        pub fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            match self.node_at_mut(index) {
                | Some(node) => {
                    node.value = item;
                    Ok(self)
                }
                | None => Err("Index out of bounds"),
            }
        }

        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            if length == 0 || start >= self.len {
                return LinkedListStEphS::empty();
            }
            let mut current = self.head.as_deref();
            let mut skipped = 0usize;
            while skipped < start {
                match current {
                    | Some(node) => {
                        current = node.next.as_deref();
                        skipped += 1;
                    }
                    | None => return LinkedListStEphS::empty(),
                }
            }
            let mut out: Vec<T> = Vec::with_capacity(length);
            let mut taken = 0usize;
            while taken < length {
                match current {
                    | Some(node) => {
                        out.push(node.value.clone());
                        current = node.next.as_deref();
                        taken += 1;
                    }
                    | None => break,
                }
            }
            LinkedListStEphS::from_vec(out)
        }

        fn node_at(&self, index: N) -> Option<&NodeE<T>> {
            if index >= self.len {
                return None;
            }
            let mut current = self.head.as_deref();
            let mut i = 0usize;
            while let Some(node) = current {
                if i == index {
                    return Some(node);
                }
                current = node.next.as_deref();
                i += 1;
            }
            None
        }

        fn node_at_mut(&mut self, index: N) -> Option<&mut NodeE<T>> {
            if index >= self.len {
                return None;
            }
            let mut current = self.head.as_deref_mut();
            let mut i = 0usize;
            while let Some(node) = current {
                if i == index {
                    return Some(node);
                }
                current = node.next.as_deref_mut();
                i += 1;
            }
            None
        }
    }

    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            let mut current = self.head.as_deref();
            while let Some(node) = current {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", node.value)?;
                current = node.next.as_deref();
            }
            write!(f, "]")
        }
    }

    impl<T: StT> std::fmt::Debug for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            let mut current = self.head.as_deref();
            while let Some(node) = current {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", node.value)?;
                current = node.next.as_deref();
            }
            write!(f, "]")
        }
    }

    impl<T: StT> PartialEq for LinkedListStEphS<T> {
        fn eq(&self, other: &Self) -> bool {
            if self.len != other.len {
                return false;
            }
            let mut left = self.head.as_deref();
            let mut right = other.head.as_deref();
            while let (Some(a), Some(b)) = (left, right) {
                if a.value != b.value {
                    return false;
                }
                left = a.next.as_deref();
                right = b.next.as_deref();
            }
            true
        }
    }

    impl<T: StT> Eq for LinkedListStEphS<T> {}

    pub trait LinkedListStEphTrait<T: StT> {
        fn new(length: N, init_value: T) -> Self
        where
            T: Clone;

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;

        fn length(&self) -> N;

        fn nth(&self, index: N) -> &T;

        fn empty() -> Self;

        fn singleton(item: T) -> Self;

        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U>;
        fn subseq_copy(&self, start: N, length: N) -> Self;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &Self, b: &Self) -> Self;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn filter<F: Fn(&T) -> B>(a: &Self, pred: &F) -> Self;
        /// Helper for filter: deflate f x = if f(x) then [x] else []
        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> Self;
        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
        fn update(a: &mut Self, item_at: Pair<N, T>) -> &mut Self;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;
        /// APAS: Work Θ(|a| + |updates|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |updates|), Span Θ(1)
        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self;
        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStEphS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStEphS<Pair<A, LinkedListStEphS<Bv>>>;
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A;
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEphS<A>, A);
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T;
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T);
    }

    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {
        fn new(length: N, init_value: T) -> Self
        where
            T: Clone,
        {
            LinkedListStEphS::new(length, init_value)
        }

        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
            LinkedListStEphS::set(self, index, item)
        }

        fn length(&self) -> N { LinkedListStEphS::length(self) }

        fn nth(&self, index: N) -> &T { LinkedListStEphS::nth(self, index) }

        fn empty() -> Self { LinkedListStEphS::empty() }

        fn singleton(item: T) -> Self { LinkedListStEphS::singleton(item) }

        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> Self {
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            LinkedListStEphS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U> {
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            LinkedListStEphS::from_vec(values)
        }

        fn subseq_copy(&self, start: N, length: N) -> Self { LinkedListStEphS::subseq_copy(self, start, length) }

        fn append(a: &Self, b: &Self) -> Self {
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            LinkedListStEphS::from_vec(values)
        }

        fn filter<F: Fn(&T) -> B>(a: &Self, pred: &F) -> Self {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) == B::True {
                    kept.push(value.clone());
                }
            }
            LinkedListStEphS::from_vec(kept)
        }

        fn deflate<F: Fn(&T) -> B>(f: &F, x: &T) -> Self {
            // Helper for filter: deflate f x = if f(x) then [x] else []
            if f(x) == B::True {
                LinkedListStEphS::from_vec(vec![x.clone()])
            } else {
                LinkedListStEphS::empty()
            }
        }

        fn flatten(ss: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            LinkedListStEphS::from_vec(values)
        }

        fn update(a: &mut Self, Pair(index, item): Pair<N, T>) -> &mut Self {
            let _ = a.set(index, item);
            a
        }

        fn inject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {
            let mut out = a.clone();
            let mut applied: HashSet<N> = HashSet::new();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                if applied.insert(idx) {
                    let _ = out.set(idx, val);
                }
            }
            out
        }

        fn ninject(a: &Self, updates: &LinkedListStEphS<Pair<N, T>>) -> Self {
            let mut out = a.clone();
            for i in 0..updates.length() {
                let Pair(idx, val) = updates.nth(i).clone();
                let _ = out.set(idx, val);
            }
            out
        }

        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStEphS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStEphS<Pair<A, LinkedListStEphS<Bv>>> {
            let mut groups: Vec<Pair<A, Vec<Bv>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(k, v) = a.nth(i).clone();
                if let Some(Pair(_, existing)) = groups.iter_mut().find(|Pair(gk, _)| cmp(&k, gk) == O::Equal) {
                    existing.push(v);
                } else {
                    groups.push(Pair(k, vec![v]));
                }
            }
            let pairs: Vec<Pair<A, LinkedListStEphS<Bv>>> = groups
                .into_iter()
                .map(|Pair(k, vs)| Pair(k, LinkedListStEphS::from_vec(vs)))
                .collect();
            LinkedListStEphS::from_vec(pairs)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> (LinkedListStEphS<A>, A) {
            let mut acc = x.clone();
            let mut prefixes: Vec<A> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                prefixes.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (LinkedListStEphS::from_vec(prefixes), acc)
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T {
            let len = a.length();
            if len == 0 {
                return id;
            }
            if len == 1 {
                return a.nth(0).clone();
            }
            let mid = len / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, len - mid);
            let l = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (LinkedListStEphS<T>, T) {
            let len = a.length();
            if len == 0 {
                return (LinkedListStEphS::empty(), id);
            }
            let mut prefixes: Vec<T> = Vec::with_capacity(len);
            for i in 0..len {
                let prefix = a.subseq_copy(0, i);
                let red = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(&prefix, f, id.clone());
                prefixes.push(red);
            }
            let total = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::reduce(a, f, id);
            (LinkedListStEphS::from_vec(prefixes), total)
        }
    }
}
