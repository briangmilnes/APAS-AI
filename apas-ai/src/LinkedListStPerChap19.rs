//! Chapter 19 algorithms for LinkedListStPer.

pub mod LinkedListStPerChap19 {
use crate::LinkedListStPer::LinkedListStPer::*;
use crate::LinkedListStPerChap18::LinkedListStPerChap18Trait;
use crate::Types::Types::*;

pub trait LinkedListStPerChap19Trait<T: StT> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T>;
    fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U>;
    fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T>;
    fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;
    fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T>;
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T>;
    fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T>;
    fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    fn scan<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T) where F: Fn(&T, &T) -> T;
    fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T>;
    fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>,
    ) -> LinkedListStPerS<T>;
}

impl<T: StT> LinkedListStPerChap19Trait<T> for LinkedListStPerS<T> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStPerS<T> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::tabulate(f, n)
    }
    fn map<U: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> U) -> LinkedListStPerS<U> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::map(a, f)
    }
    fn select<'a>(a: &'a LinkedListStPerS<T>, b: &'a LinkedListStPerS<T>, i: N) -> Option<T> {
        let na = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(a);
        if i < na {
            Some(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(a, i).clone())
        } else {
            let j = i - na;
            let nb = <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::length(b);
            if j < nb {
                Some(<LinkedListStPerS<T> as LinkedListStPerTrait<T>>::nth(b, j).clone())
            } else {
                None
            }
        }
    }
    fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::append(a, b)
    }

    fn append2(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> LinkedListStPerS<T> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::append(a, b)
    }

    fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStPerS<T> {
        if f(x) == B::True {
            <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::singleton(x.clone())
        } else {
            <LinkedListStPerS<T> as LinkedListStPerTrait<T>>::empty()
        }
    }

    fn filter(a: &LinkedListStPerS<T>, f: impl Fn(&T) -> B) -> LinkedListStPerS<T> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::filter(a, f)
    }

    fn iterate<A: StT>(a: &LinkedListStPerS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::iterate(a, f, x)
    }

    fn reduce<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T,
    {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::reduce(a, f, id)
    }

    fn scan<F>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T)
    where
        F: Fn(&T, &T) -> T,
    {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::scan(a, f, id)
    }

    fn flatten(s: &LinkedListStPerS<LinkedListStPerS<T>>) -> LinkedListStPerS<T> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::flatten(s)
    }

    fn inject(values: &LinkedListStPerS<T>, changes: &LinkedListStPerS<Pair<N, T>>,) 
      -> LinkedListStPerS<T> {
        <LinkedListStPerS<T> as LinkedListStPerChap18Trait<T>>::inject(values, changes)
    }
}

}

pub use LinkedListStPerChap19::LinkedListStPerChap19Trait;
