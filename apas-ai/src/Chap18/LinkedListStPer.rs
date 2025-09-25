//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStPer.

pub mod LinkedListStPer {
    use std::collections::HashSet;

    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct NodeP<T: StT> {
        pub value: T,
        pub next: Option<Box<NodeP<T>>>,
    }

    #[derive(Debug, Clone)]
    pub struct LinkedListStPerS<T: StT> {
        head: Option<Box<NodeP<T>>>,
        len: N,
    }

    pub trait LinkedListStPerTrait<T: StT> {
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>
        where
            T: Clone;
        fn empty() -> LinkedListStPerS<T>;
        fn singleton(item: T) -> LinkedListStPerS<T>;
        fn length(&self) -> N;
        fn nth(&self, index: N) -> &T;
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T>;
        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T>;
        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U>;
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;
        fn filter<F: Fn(&T) -> B>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T>;
        fn update(a: &LinkedListStPerS<T>, item_at: Pair<N, T>) -> LinkedListStPerS<T>;
        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T>;
        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T>;
        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A;
        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> (LinkedListStPerS<A>, A);
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T;

        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T);

        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;

        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStPerS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStPerS<Pair<A, LinkedListStPerS<Bv>>>;
    }

    impl<T: StT> LinkedListStPerS<T> {
        pub fn empty() -> Self { LinkedListStPerS { head: None, len: 0 } }

        pub fn new(length: N, init_value: T) -> Self
        where
            T: Clone,
        {
            LinkedListStPerS::from_vec(vec![init_value; length])
        }

        pub fn singleton(item: T) -> Self { LinkedListStPerS::from_vec(vec![item]) }

        pub fn from_vec(elts: Vec<T>) -> Self {
            let mut head: Option<Box<NodeP<T>>> = None;
            let mut len = 0usize;
            for value in elts.into_iter().rev() {
                head = Some(Box::new(NodeP { value, next: head }));
                len += 1;
            }
            LinkedListStPerS { head, len }
        }

        pub fn length(&self) -> N { self.len }

        pub fn nth(&self, index: N) -> &T {
            self.node_at(index)
                .map(|node| &node.value)
                .expect("Index out of bounds")
        }

        pub fn subseq_copy(&self, start: N, length: N) -> Self {
            if length == 0 || start >= self.len {
                return LinkedListStPerS::empty();
            }
            let mut current = self.head.as_deref();
            let mut skipped = 0usize;
            while skipped < start {
                match current {
                    | Some(node) => {
                        current = node.next.as_deref();
                        skipped += 1;
                    }
                    | None => return LinkedListStPerS::empty(),
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
            LinkedListStPerS::from_vec(out)
        }

        fn node_at(&self, index: N) -> Option<&NodeP<T>> {
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
    }

    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {
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

    impl<T: StT> PartialEq for LinkedListStPerS<T> {
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

    impl<T: StT> Eq for LinkedListStPerS<T> {}

    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {
        fn new(length: N, init_value: T) -> LinkedListStPerS<T>
        where
            T: Clone,
        {
            LinkedListStPerS::new(length, init_value)
        }

        fn empty() -> LinkedListStPerS<T> { LinkedListStPerS::empty() }
        fn singleton(item: T) -> LinkedListStPerS<T> { LinkedListStPerS::singleton(item) }
        fn length(&self) -> N { LinkedListStPerS::length(self) }
        fn nth(&self, index: N) -> &T { LinkedListStPerS::nth(self, index) }
        fn subseq_copy(&self, start: N, length: N) -> LinkedListStPerS<T> {
            LinkedListStPerS::subseq_copy(self, start, length)
        }

        fn tabulate<F: Fn(N) -> T>(f: &F, n: N) -> LinkedListStPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(n);
            for i in 0..n {
                values.push(f(i));
            }
            LinkedListStPerS::from_vec(values)
        }

        fn map<U: StT, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U> {
            let mut values: Vec<U> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                values.push(f(a.nth(i)));
            }
            LinkedListStPerS::from_vec(values)
        }

        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(a.length() + b.length());
            for i in 0..a.length() {
                values.push(a.nth(i).clone());
            }
            for j in 0..b.length() {
                values.push(b.nth(j).clone());
            }
            LinkedListStPerS::from_vec(values)
        }

        fn filter<F: Fn(&T) -> B>(a: &LinkedListStPerS<T>, pred: &F) -> LinkedListStPerS<T> {
            let mut kept: Vec<T> = Vec::new();
            for i in 0..a.length() {
                let value = a.nth(i);
                if pred(value) == B::True {
                    kept.push(value.clone());
                }
            }
            LinkedListStPerS::from_vec(kept)
        }

        fn update(a: &LinkedListStPerS<T>, Pair(index, item): Pair<N, T>) -> LinkedListStPerS<T> {
            let mut values: Vec<T> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                let current = a.nth(i).clone();
                if i == index {
                    values.push(item.clone());
                } else {
                    values.push(current);
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn inject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T> {
            let mut values: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            let mut seen = std::collections::HashSet::new();
            for k in 0..updates.length() {
                let Pair(idx, val) = updates.nth(k).clone();
                if idx < values.len() && !seen.contains(&idx) {
                    values[idx] = val;
                    seen.insert(idx);
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn ninject(a: &LinkedListStPerS<T>, updates: &LinkedListStPerS<Pair<N, T>>) -> LinkedListStPerS<T> {
            let mut values: Vec<T> = (0..a.length()).map(|i| a.nth(i).clone()).collect();
            for k in 0..updates.length() {
                let Pair(idx, val) = updates.nth(k).clone();
                if idx < values.len() {
                    values[idx] = val;
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn iterate<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            for i in 0..a.length() {
                acc = f(&acc, a.nth(i));
            }
            acc
        }

        fn iteratePrefixes<A: StT, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, x: A) -> (LinkedListStPerS<A>, A) {
            let mut acc = x.clone();
            let mut prefixes: Vec<A> = Vec::with_capacity(a.length());
            for i in 0..a.length() {
                prefixes.push(acc.clone());
                acc = f(&acc, a.nth(i));
            }
            (LinkedListStPerS::from_vec(prefixes), acc)
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T {
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
            let l = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(&left, f, id.clone());
            let r = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(&right, f, id);
            f(&l, &r)
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T) {
            let len = a.length();
            if len == 0 {
                return (LinkedListStPerS::empty(), id);
            }
            let mut prefixes: Vec<T> = Vec::with_capacity(len);
            for i in 0..len {
                let prefix = a.subseq_copy(0, i);
                let red = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(&prefix, f, id.clone());
                prefixes.push(red);
            }
            let total = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::reduce(a, f, id);
            (LinkedListStPerS::from_vec(prefixes), total)
        }

        fn flatten(ss: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {
            let mut values: Vec<T> = Vec::new();
            for i in 0..ss.length() {
                let inner = ss.nth(i);
                for j in 0..inner.length() {
                    values.push(inner.nth(j).clone());
                }
            }
            LinkedListStPerS::from_vec(values)
        }

        fn collect<A: StT, Bv: StT>(
            a: &LinkedListStPerS<Pair<A, Bv>>,
            cmp: fn(&A, &A) -> O,
        ) -> LinkedListStPerS<Pair<A, LinkedListStPerS<Bv>>> {
            let mut groups: Vec<Pair<A, Vec<Bv>>> = Vec::new();
            for i in 0..a.length() {
                let Pair(k, v) = a.nth(i).clone();
                if let Some(Pair(_, existing)) = groups.iter_mut().find(|Pair(gk, _)| cmp(&k, gk) == O::Equal) {
                    existing.push(v);
                } else {
                    groups.push(Pair(k, vec![v]));
                }
            }
            let pairs: Vec<Pair<A, LinkedListStPerS<Bv>>> = groups
                .into_iter()
                .map(|Pair(k, vs)| Pair(k, LinkedListStPerS::from_vec(vs)))
                .collect();
            LinkedListStPerS::from_vec(pairs)
        }
    }
}
