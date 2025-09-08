//! Chapter 19 algorithms for LinkedListEph (ephemeral).

use crate::LinkedListEph::{LinkedListEphS, LinkedListEphTrait};
use crate::LinkedListEphChap18::LinkedListEphChap18Trait;
use crate::Types::{B, N};

pub trait LinkedListEphChap19Trait {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;
    fn map<T, U: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;
    fn select<'a, T: Clone>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<T>;
    fn append<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;
    fn append2<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T>;
    fn filter<T: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T>;
    fn iterate<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    fn scan<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T) where F: Fn(&T, &T) -> T;
    fn flatten<T: Clone>(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;
    fn inject<T: Clone + Eq>(values: &LinkedListEphS<T>, changes: &LinkedListEphS<(N, T)>) -> LinkedListEphS<T>;
}

impl<T2> LinkedListEphChap19Trait for LinkedListEphS<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> { <LinkedListEphS<T> as LinkedListEphChap18Trait>::tabulate(f, n) }
    fn map<T, U: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::map(a, f) }
    fn select<'a, T: Clone>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<T> {
        let na = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a);
        if i < na { Some(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i).clone()) } else {
            let j = i - na; let nb = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(b);
            if j < nb { Some(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(b, j).clone()) } else { None }
        }
    }
    fn append<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::append(a, b) }
    fn append2<T: Clone>(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::append(a, b) }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T> { if f(x) == B::True { <LinkedListEphS<T> as LinkedListEphTrait<T>>::singleton(x.clone()) } else { <LinkedListEphS<T> as LinkedListEphTrait<T>>::empty() } }
    fn filter<T: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T> { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::filter(a, f) }
    fn iterate<T: Clone, A: Clone>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::iterate(a, f, x) }
    fn reduce<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::reduce(a, f, id) }
    fn scan<T: Clone, F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T) where F: Fn(&T, &T) -> T { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::scan(a, f, id) }
    fn flatten<T: Clone>(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::flatten(s) }
    fn inject<T: Clone + Eq>(values: &LinkedListEphS<T>, changes: &LinkedListEphS<(N, T)>) -> LinkedListEphS<T> { <LinkedListEphS<T2> as LinkedListEphChap18Trait>::inject(values, changes) }
}


