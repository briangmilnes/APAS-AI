//! Chapter 19 algorithms for LinkedListEph (ephemeral).

pub mod LinkedListEphChap19 {
    use crate::LinkedListEph::LinkedListEph::*;
    use crate::LinkedListEphChap18::LinkedListEphChap18Trait;
    use crate::Types::Types::*;

    pub trait LinkedListEphChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T>;
        fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U>;
        fn select<'a>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<T>;
        fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;
        fn append2(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T>;
        fn filter(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T>;
        fn iterate<A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce<F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T;
        fn scan<F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T)
        where
            F: Fn(&T, &T) -> T;
        fn flatten(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T>;
        fn inject(
            values: &LinkedListEphS<T>,
            changes: &LinkedListEphS<Pair<N, T>>,
        ) -> LinkedListEphS<T>;
    }

    impl<T: StT> LinkedListEphChap19Trait<T> for LinkedListEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListEphS<T> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &LinkedListEphS<T>, f: impl Fn(&T) -> U) -> LinkedListEphS<U> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::map(a, f)
        }
        fn select<'a>(a: &'a LinkedListEphS<T>, b: &'a LinkedListEphS<T>, i: N) -> Option<T> {
            let na = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(a);
            if i < na {
                Some(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(a, i).clone())
            } else {
                let j = i - na;
                let nb = <LinkedListEphS<T> as LinkedListEphTrait<T>>::length(b);
                if j < nb {
                    Some(<LinkedListEphS<T> as LinkedListEphTrait<T>>::nth(b, j).clone())
                } else {
                    None
                }
            }
        }
        fn append(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::append(a, b)
        }
        fn append2(a: &LinkedListEphS<T>, b: &LinkedListEphS<T>) -> LinkedListEphS<T> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListEphS<T> {
            if f(x) == B::True {
                <LinkedListEphS<T> as LinkedListEphTrait<T>>::singleton(x.clone())
            } else {
                <LinkedListEphS<T> as LinkedListEphTrait<T>>::empty()
            }
        }
        fn filter(a: &LinkedListEphS<T>, f: impl Fn(&T) -> B) -> LinkedListEphS<T> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::filter(a, f)
        }
        fn iterate<A: StT>(a: &LinkedListEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::iterate(a, f, x)
        }
        fn reduce<F>(a: &LinkedListEphS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T,
        {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::reduce(a, f, id)
        }
        fn scan<F>(a: &LinkedListEphS<T>, f: &F, id: T) -> (LinkedListEphS<T>, T)
        where
            F: Fn(&T, &T) -> T,
        {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::scan(a, f, id)
        }
        fn flatten(s: &LinkedListEphS<LinkedListEphS<T>>) -> LinkedListEphS<T> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::flatten(s)
        }
        fn inject(values: &LinkedListEphS<T>,changes: &LinkedListEphS<Pair<N, T>>,) 
         -> LinkedListEphS<T> {
            <LinkedListEphS<T> as LinkedListEphChap18Trait<T>>::inject(values, changes)
        }
    }
}

pub use LinkedListEphChap19::LinkedListEphChap19Trait;
