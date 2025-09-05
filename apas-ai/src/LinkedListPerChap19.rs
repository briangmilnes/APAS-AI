//! Chapter 19 algorithms for LinkedListPer.

use crate::LinkedListPer::{LinkedListPerS, LinkedListPerTrait};
use crate::LinkedListPerChap18::LinkedListPerChap18Trait;
use crate::Types::{B, N};

pub trait LinkedListPerChap19Trait {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;
    fn select<'a, T>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T>
    where
        T: Clone;
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;
    fn append2<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T>;
    fn filter<T: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T>;
    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T;
    fn scan<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T)
    where
        F: Fn(&T, &T) -> T;
    fn flatten<T: Clone>(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;
    fn inject<T: Clone + Eq>(
        values: &LinkedListPerS<T>,
        changes: &LinkedListPerS<(N, T)>,
    ) -> LinkedListPerS<T>;
}

impl<T2> LinkedListPerChap19Trait for LinkedListPerS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait>::tabulate(f, n)
    }
    fn map<T, U: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::map(a, f)
    }
    fn select<'a, T>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T>
    where
        T: Clone,
    {
        let na = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(a);
        if i < na {
            Some(<LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(a, i).clone())
        } else {
            let j = i - na;
            let nb = <LinkedListPerS<T> as LinkedListPerTrait<T>>::length(b);
            if j < nb {
                Some(<LinkedListPerS<T> as LinkedListPerTrait<T>>::nth(b, j).clone())
            } else {
                None
            }
        }
    }
    fn append<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::append(a, b)
    }

    fn append2<T: Clone>(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::append(a, b)
    }

    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T> {
        if f(x) == B::True {
            <LinkedListPerS<T> as LinkedListPerTrait<T>>::singleton(x.clone())
        } else {
            <LinkedListPerS<T> as LinkedListPerTrait<T>>::empty()
        }
    }

    fn filter<T: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T> {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::filter(a, f)
    }

    fn iterate<T: Clone, A: Clone>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::iterate(a, f, x)
    }

    fn reduce<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T,
    {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::reduce(a, f, id)
    }

    fn scan<T: Clone, F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T)
    where
        F: Fn(&T, &T) -> T,
    {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::scan(a, f, id)
    }

    fn flatten<T: Clone>(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::flatten(s)
    }

    fn inject<T: Clone + Eq>(
        values: &LinkedListPerS<T>,
        changes: &LinkedListPerS<(N, T)>,
    ) -> LinkedListPerS<T> {
        <LinkedListPerS<T2> as LinkedListPerChap18Trait>::inject(values, changes)
    }
}
