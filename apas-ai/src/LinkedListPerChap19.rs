//! Chapter 19 algorithms for LinkedListPer.

pub mod LinkedListPerChap19 {
use crate::LinkedListPer::LinkedListPer::*;
use crate::LinkedListPerChap18::LinkedListPerChap18Trait;
use crate::Types::Types::*;

pub trait LinkedListPerChap19Trait<T: StT> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T>;
    fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U>;
    fn select<'a>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T>;
    fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;
    fn append2(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T>;
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T>;
    fn filter(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T>;
    fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    fn scan<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T) where F: Fn(&T, &T) -> T;
    fn flatten(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T>;
    fn inject(values: &LinkedListPerS<T>, changes: &LinkedListPerS<Pair<N, T>>,
    ) -> LinkedListPerS<T>;
}

impl<T: StT> LinkedListPerChap19Trait<T> for LinkedListPerS<T> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::tabulate(f, n)
    }
    fn map<U: StT>(a: &LinkedListPerS<T>, f: impl Fn(&T) -> U) -> LinkedListPerS<U> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::map(a, f)
    }
    fn select<'a>(a: &'a LinkedListPerS<T>, b: &'a LinkedListPerS<T>, i: N) -> Option<T> {
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
    fn append(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::append(a, b)
    }

    fn append2(a: &LinkedListPerS<T>, b: &LinkedListPerS<T>) -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::append(a, b)
    }

    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListPerS<T> {
        if f(x) == B::True {
            <LinkedListPerS<T> as LinkedListPerTrait<T>>::singleton(x.clone())
        } else {
            <LinkedListPerS<T> as LinkedListPerTrait<T>>::empty()
        }
    }

    fn filter(a: &LinkedListPerS<T>, f: impl Fn(&T) -> B) -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::filter(a, f)
    }

    fn iterate<A: StT>(a: &LinkedListPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::iterate(a, f, x)
    }

    fn reduce<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T,
    {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::reduce(a, f, id)
    }

    fn scan<F>(a: &LinkedListPerS<T>, f: &F, id: T) -> (LinkedListPerS<T>, T)
    where
        F: Fn(&T, &T) -> T,
    {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::scan(a, f, id)
    }

    fn flatten(s: &LinkedListPerS<LinkedListPerS<T>>) -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::flatten(s)
    }

    fn inject(values: &LinkedListPerS<T>, changes: &LinkedListPerS<Pair<N, T>>,) 
      -> LinkedListPerS<T> {
        <LinkedListPerS<T> as LinkedListPerChap18Trait<T>>::inject(values, changes)
    }
}

}

pub use LinkedListPerChap19::LinkedListPerChap19Trait;
