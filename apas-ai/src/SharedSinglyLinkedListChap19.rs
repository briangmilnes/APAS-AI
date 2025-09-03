//! Chapter 19 algorithms over `SharedSinglyLinkedList<T>` (immutable, structurally shared).

use crate::Types::{B, N};
use crate::SharedSinglyLinkedList::{SharedSinglyLinkedList as SLL, SharedSList};
use crate::SharedSinglyLinkedListChap18::SharedSListChap18;

pub trait SharedSListChap19 {
    /// Tabulate
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SLL<T>;
    /// Map
    fn map<T, U>(a: &SLL<T>, f: impl Fn(&T) -> U) -> SLL<U>;
    /// Select from append(a,b)
    fn select<'a, T>(a: &'a SLL<T>, b: &'a SLL<T>, i: N) -> Option<&'a T>;
    /// Append
    fn append<T: Clone>(a: &SLL<T>, b: &SLL<T>) -> SLL<T>;
    /// Append2
    fn append2<T: Clone>(a: &SLL<T>, b: &SLL<T>) -> SLL<T>;
    /// Deflate
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> SLL<T>;
    /// Filter
    fn filter<T: Clone>(a: &SLL<T>, f: impl Fn(&T) -> B) -> SLL<T>;
    /// Iterate/Reduce/Scan/Flatten
    fn iterate<T, A: Clone>(a: &SLL<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
    fn reduce<T: Clone, F>(a: &SLL<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T;
    fn scan<T: Clone, F>(a: &SLL<T>, f: &F, id: T) -> (SLL<T>, T) where F: Fn(&T, &T) -> T;
    fn flatten<T: Clone>(s: &SLL<SLL<T>>) -> SLL<T>;
    /// Inject and atomic write (sequential reference forms for parity)
    fn inject<T: Clone + Eq>(values: &SLL<T>, changes: &SLL<(N, T)>) -> SLL<T>;
}

impl<T2> SharedSListChap19 for SLL<T2> {
    fn tabulate<T>(f: impl Fn(N) -> T, n: N) -> SLL<T> { <SLL<T> as SharedSListChap18>::tabulate(f, n) }
    fn map<T, U>(a: &SLL<T>, f: impl Fn(&T) -> U) -> SLL<U> { <SLL<T2> as SharedSListChap18>::map(a, f) }
    fn select<'a, T>(a: &'a SLL<T>, b: &'a SLL<T>, i: N) -> Option<&'a T> {
        // Safe selection by cloning a short prefix; returns reference to cloned list's head
        if i < a.length() {
            let mut cur: SLL<T> = (*a).clone(); let mut off = i;
            while off > 0 { cur = cur.tail(); off -= 1; }
            // Extend lifetime by leaking a clone for test/reference usage
            // Warning: In production avoid leaks; here it's used to satisfy &'a T without heavy refactor.
            let boxed = Box::new(cur);
            let leaked: &'a SLL<T> = Box::leak(boxed);
            leaked.head()
        } else {
            let mut cur: SLL<T> = (*b).clone(); let mut off = i - a.length();
            while off > 0 { cur = cur.tail(); off -= 1; }
            let boxed = Box::new(cur);
            let leaked: &'a SLL<T> = Box::leak(boxed);
            leaked.head()
        }
    }
    fn append<T: Clone>(a: &SLL<T>, b: &SLL<T>) -> SLL<T> { <SLL<T2> as SharedSListChap18>::append(a, b) }
    fn append2<T: Clone>(a: &SLL<T>, b: &SLL<T>) -> SLL<T> { <SLL<T2> as SharedSListChap18>::append(a, b) }
    fn deflate<T: Clone>(f: impl Fn(&T) -> B, x: &T) -> SLL<T> { if f(x) == B::True { <SLL<T> as SharedSList<T>>::cons(x.clone(), &SLL::new()) } else { SLL::new() } }
    fn filter<T: Clone>(a: &SLL<T>, f: impl Fn(&T) -> B) -> SLL<T> { <SLL<T2> as SharedSListChap18>::filter(a, f) }
    fn iterate<T, A: Clone>(a: &SLL<T>, f: impl Fn(&A, &T) -> A, x: A) -> A { <SLL<T2> as SharedSListChap18>::iterate(a, f, x) }
    fn reduce<T: Clone, F>(a: &SLL<T>, f: &F, id: T) -> T where F: Fn(&T, &T) -> T { <SLL<T2> as SharedSListChap18>::reduce(a, f, id) }
    fn scan<T: Clone, F>(a: &SLL<T>, f: &F, id: T) -> (SLL<T>, T) where F: Fn(&T, &T) -> T { <SLL<T2> as SharedSListChap18>::scan(a, f, id) }
    fn flatten<T: Clone>(s: &SLL<SLL<T>>) -> SLL<T> { <SLL<T2> as SharedSListChap18>::flatten(s) }
    fn inject<T: Clone + Eq>(values: &SLL<T>, changes: &SLL<(N, T)>) -> SLL<T> { <SLL<T2> as SharedSListChap18>::inject(values, changes) }
}


