//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Multi-threaded ephemeral set implementation using AVLTreeSetStEph as backing store.
//! 
//! Work/Span Analysis:
//! - union: Work Θ(n+m), Span Θ(n+m) (sequential, mutex-protected)
//! - intersection: Work Θ(n+m), Span Θ(n+m) (sequential, mutex-protected)
//! - filter: Work Θ(n), Span Θ(n) (sequential, mutex-protected)

pub mod AVLTreeSetMtEph {
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    use std::fmt;
    use std::sync::{Arc, Mutex};

    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord> {
        inner: Arc<Mutex<AVLTreeSetStEph<T>>>,
    }

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord> {
        fn size(&self) -> N;
        fn to_seq(&self) -> AVLTreeSeqStEphS<T>;
        fn empty() -> Self;
        fn singleton(x: T) -> Self;
        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self;
        fn filter<F: Fn(&T) -> B>(&self, f: F) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, x: &T) -> B;
        fn delete(&mut self, x: &T);
        fn insert(&mut self, x: T);
    }

    impl<T: StTInMtT + Ord> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        fn size(&self) -> N {
            let inner = self.inner.lock().unwrap();
            inner.size()
        }

        fn to_seq(&self) -> AVLTreeSeqStEphS<T> {
            let inner = self.inner.lock().unwrap();
            inner.to_seq()
        }

        fn empty() -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(AVLTreeSetStEph::empty())),
            }
        }

        fn singleton(x: T) -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(AVLTreeSetStEph::singleton(x))),
            }
        }

        fn from_seq(seq: AVLTreeSeqStEphS<T>) -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(AVLTreeSetStEph::from_seq(seq))),
            }
        }

        fn filter<F: Fn(&T) -> B>(&self, f: F) -> Self {
            let inner = self.inner.lock().unwrap();
            let filtered = inner.filter(f);
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(filtered)),
            }
        }

        fn intersection(&self, other: &Self) -> Self {
            let self_inner = self.inner.lock().unwrap();
            let other_inner = other.inner.lock().unwrap();
            let result = self_inner.intersection(&*other_inner);
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(result)),
            }
        }

        fn difference(&self, other: &Self) -> Self {
            let self_inner = self.inner.lock().unwrap();
            let other_inner = other.inner.lock().unwrap();
            let result = self_inner.difference(&*other_inner);
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(result)),
            }
        }

        fn union(&self, other: &Self) -> Self {
            let self_inner = self.inner.lock().unwrap();
            let other_inner = other.inner.lock().unwrap();
            let result = self_inner.union(&*other_inner);
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(result)),
            }
        }

        fn find(&self, x: &T) -> B {
            let inner = self.inner.lock().unwrap();
            inner.find(x)
        }

        fn delete(&mut self, x: &T) {
            let mut inner = self.inner.lock().unwrap();
            inner.delete(x);
        }

        fn insert(&mut self, x: T) {
            let mut inner = self.inner.lock().unwrap();
            inner.insert(x);
        }
    }

    impl<T: StTInMtT + Ord> Default for AVLTreeSetMtEph<T> {
        fn default() -> Self {
            Self::empty()
        }
    }

    impl<T: StTInMtT + Ord> Clone for AVLTreeSetMtEph<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.lock().unwrap();
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new((*inner).clone())),
            }
        }
    }

    impl<T: StTInMtT + Ord> fmt::Debug for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    impl<T: StTInMtT + Ord> fmt::Display for AVLTreeSetMtEph<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{{")?;
            let seq = self.to_seq();
            for i in 0..seq.length() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", seq.nth(i))?;
            }
            write!(f, "}}")
        }
    }

    #[macro_export]
    macro_rules! AVLTreeSetMtEphLit {
        () => {
            < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __set = < $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEph<_> as $crate::Chap41::AVLTreeSetMtEph::AVLTreeSetMtEph::AVLTreeSetMtEphTrait<_> >::empty();
            $( __set.insert($x); )*
            __set
        }};
    }

    #[allow(dead_code)]
    fn _AVLTreeSetMtEphLit_type_checks() {
        let _: AVLTreeSetMtEph<i32> = AVLTreeSetMtEphLit![];
        let _: AVLTreeSetMtEph<i32> = AVLTreeSetMtEphLit![1, 2, 3];
    }
}
