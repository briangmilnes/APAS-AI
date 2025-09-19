//! Chapter 19 algorithms for `ArraySeqStEph<T>`.

pub mod ArraySeqStEphChap19 {
    use crate::ArraySeqStEph::ArraySeqStEph::*;
    use crate::ArraySeqStEphChap18::ArraySeqStEphChap18::*;
    use crate::Types::Types::*;

    pub trait ArraySeqStEphChap19Trait<T: StT> {
        /// APAS: Work Θ(n), Span Θ(1)
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a| + |b|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a| + |b|), Span Θ(1)
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T>;
        /// APAS: Work Θ(|a|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|a|), Span Θ(1)
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T>;
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A;
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T;
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T);
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T>;
    }

    impl<T: StT> ArraySeqStEphChap19Trait<T> for ArraySeqStEphS<T> {
        fn tabulate(f: impl Fn(N) -> T, n: N) -> ArraySeqStEphS<T> {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::tabulate(f, n)
        }
        fn map<U: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> U) -> ArraySeqStEphS<U> {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::map(a, f)
        }
        fn select<'a>(a: &'a ArraySeqStEphS<T>, b: &'a ArraySeqStEphS<T>, i: N) -> Option<T> {
            if i < a.length() {
                Some(a.nth(i).clone())
            } else {
                let off = i - a.length();
                if off < b.length() { Some(b.nth(off).clone()) } else { None }
            }
        }
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::append(a, b)
        }
        fn append2(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> ArraySeqStEphS<T> {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::append(a, b)
        }
        fn deflate(f: impl Fn(&T) -> B, x: &T) -> ArraySeqStEphS<T> {
            if f(x) == B::True {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::singleton(x.clone())
            } else {
                <ArraySeqStEphS<T> as ArraySeqStEphTrait<T>>::empty()
            }
        }
        fn filter(a: &ArraySeqStEphS<T>, f: impl Fn(&T) -> B) -> ArraySeqStEphS<T> {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::filter(a, f)
        }
        fn iterate<A: StT>(a: &ArraySeqStEphS<T>, f: impl Fn(&A, &T) -> A, x: A) -> A {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::iterate(a, f, x)
        }
        fn reduce(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> T {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::reduce(a, f, id)
        }
        fn scan(a: &ArraySeqStEphS<T>, f: &impl Fn(&T, &T) -> T, id: T) -> (ArraySeqStEphS<T>, T) {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::scan(a, f, id)
        }
        fn flatten(s: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> ArraySeqStEphS<T> {
            <ArraySeqStEphS<T> as ArraySeqStEphChap18Trait<T>>::flatten(s)
        }
    }
}
