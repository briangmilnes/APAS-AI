//! Chapter 19 algorithms for LinkedListStEph (ephemeral).

pub mod LinkedListStEphChap19 {
    use crate::LinkedListStEph::LinkedListStEph::*;
    use crate::LinkedListStEphChap18::LinkedListStEphChap18Trait;
    use crate::Types::Types::*;

    pub trait LinkedListStEphChap19Trait<T: StT> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T>;
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U>;
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T>;
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T>;
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T>;
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T>;
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T;
        fn scan<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (LinkedListStEphS<T>, T)
        where
            F: Fn(&T, &T) -> T;
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T>;
        fn inject(
            values: &LinkedListStEphS<T>,
            changes: &LinkedListStEphS<Pair<N, T>>,
        ) -> LinkedListStEphS<T>;
    }

    impl<T: StT> LinkedListStEphChap19Trait<T> for LinkedListStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> U) -> LinkedListStEphS<U> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::map(a, f)
        }
        fn select<'a>(a: &'a LinkedListStEphS<T>, b: &'a LinkedListStEphS<T>, i: N) -> Option<T> {
            let na = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(a);
            if i < na {
                Some(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(a, i).clone())
            } else {
                let j = i - na;
                let nb = <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::length(b);
                if j < nb {
                    Some(<LinkedListStEphS<T> as LinkedListStEphTrait<T>>::nth(b, j).clone())
                } else {
                    None
                }
            }
        }
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::append(a, b)
        }
        fn append2(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> LinkedListStEphS<T> {
            if f(x) == B::True {
                <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::singleton(x.clone())
            } else {
                <LinkedListStEphS<T> as LinkedListStEphTrait<T>>::empty()
            }
        }
        fn filter(a: &LinkedListStEphS<T>, f: impl Fn(&T) -> B) -> LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::filter(a, f)
        }
        fn iterate<A: StT>(a: &LinkedListStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::iterate(a, f, x)
        }
        fn reduce<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> T
        where
            F: Fn(&T, &T) -> T,
        {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::reduce(a, f, id)
        }
        fn scan<F>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (LinkedListStEphS<T>, T)
        where
            F: Fn(&T, &T) -> T,
        {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::scan(a, f, id)
        }
        fn flatten(s: &LinkedListStEphS<LinkedListStEphS<T>>) -> LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::flatten(s)
        }
        fn inject(values: &LinkedListStEphS<T>,changes: &LinkedListStEphS<Pair<N, T>>,) 
         -> LinkedListStEphS<T> {
            <LinkedListStEphS<T> as LinkedListStEphChap18Trait<T>>::inject(values, changes)
        }
    }
}

pub use LinkedListStEphChap19::LinkedListStEphChap19Trait;
