//! Chapter 19 algorithms for `ArraySeqEph<T>`.

pub mod ArraySeqEphChap19 {
use crate::ArraySeqEph::ArraySeqEph::*;
use crate::ArraySeqEphChap18::ArraySeqEphChap18Trait;
use crate::Types::Types::*;

pub trait ArraySeqEphChap19Trait<T: StT> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T>;
    fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U>;
    fn select<'a>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T>;
    fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;
    fn append2(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T>;
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T>;
    fn filter(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T>;
    fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T;
    fn scan<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)
    where
        F: Fn(&T, &T) -> T;
    fn flatten(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T>;
}

impl<T: StT> ArraySeqEphChap19Trait<T> for ArraySeqEphS<T> {
    fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::tabulate(f, n)
    }
    fn map<U: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqEphS<U> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::map(a, f)
    }
    fn select<'a>(a: &'a ArraySeqEphS<T>, b: &'a ArraySeqEphS<T>, i: N) -> Option<T> {
        if i <a.length() {
            Some(a.nth(i).clone())
        } else {
            let off = i - a.length();
            if off <b.length() {
                Some(b.nth(off).clone())
            } else {
                None
            }
        }
    }
    fn append(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::append(a, b)
    }
    fn append2(a: &ArraySeqEphS<T>, b: &ArraySeqEphS<T>) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::append(a, b)
    }
    fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqEphS<T> {
        if f(x) == B::True {
            <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::singleton(x.clone())
        } else {
            <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::empty()
        }
    }
    fn filter(a: &ArraySeqEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::filter(a, f)
    }
    fn iterate<A: StT>(a: &ArraySeqEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::iterate(a, f, x)
    }
    fn reduce<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> T
    where
        F: Fn(&T, &T) -> T,
    {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::reduce(a, f, id)
    }
    fn scan<F>(a: &ArraySeqEphS<T>, f: &F, id: T) -> (ArraySeqEphS<T>, T)
    where
        F: Fn(&T, &T) -> T,
    {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::scan(a, f, id)
    }
    fn flatten(s: &ArraySeqEphS<ArraySeqEphS<T>>) -> ArraySeqEphS<T> {
        <ArraySeqEphS<T> as ArraySeqEphChap18Trait<T>>::flatten(s)
    }
}

}

pub use ArraySeqEphChap19::ArraySeqEphChap19Trait;
