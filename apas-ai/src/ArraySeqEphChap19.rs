//! Chapter 19 algorithms for `ArrayEphS<T>`.

use crate::ArraySeqEph::{ArraySeqEphS, ArraySeqEphTrait};
use crate::ArraySeqEphChap18::ArraySeqEphChap18Trait;
use crate::Types::{B, N};

pub trait ArraySeqEphChap19Trait {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;
    fn select<'a, T: Clone>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T>;
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;
    fn append2<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T>;
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T>;
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T;
    fn scan<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)
    where
        F: Fn(&T, &T) -> T;
    fn flatten<T: Clone + Eq>(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;
}

impl<T2> ArraySeqEphChap19Trait for ArraySeqEphS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait>::tabulate(f, n)
    }
    fn map<T, U: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::map(a, f)
    }
    fn select<'a, T: Clone>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T> {
        if i < a.length() {
            Some(a.nth(i).clone())
        } else {
            let off = i - a.length();
            if off < b.length() {
                Some(b.nth(off).clone())
            } else {
                None
            }
        }
    }
    fn append<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::append(a, b)
    }
    fn append2<T: Clone + Eq>(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::append(a, b)
    }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T> {
        if f(x) == B::True {
            <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::singleton(x.clone())
        } else {
            <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::empty()
        }
    }
    fn filter<T: Clone + Eq>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::filter(a, f)
    }
    fn iterate<T: Clone + Eq, A: Clone>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::iterate(a, f, x)
    }
    fn reduce<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T,
    {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::reduce(a, f, id)
    }
    fn scan<T: Clone + Eq, F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)
    where
        F: Fn(&T, &T) -> T,
    {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::scan(a, f, id)
    }
    fn flatten<T: Clone + Eq>(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T2> as ArraySeqEphChap18Trait>::flatten(s)
    }
}
