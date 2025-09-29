//! Chap41Codex: multi-threaded ephemeral AVL tree set with coarse-grained synchronization.

pub mod AVLTreeSetMtEph {
    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
    use crate::Chap41Codex::AVLTreeSetStEph::AVLTreeSetStEph::{AVLTreeSetStEph, AVLTreeSetStEphTrait};
    use crate::Types::Types::*;

    #[derive(Clone, Default)]
    pub struct AVLTreeSetMtEph<T: StTInMtT + Ord + 'static> {
        inner: Arc<Mutex<AVLTreeSetStEph<T>>>,
    }

    pub trait AVLTreeSetMtEphTrait<T: StTInMtT + Ord + 'static> {
        fn size(&self) -> N;
        fn to_seq(&self) -> ArraySeqMtEphS<T>;
        fn empty() -> Self;
        fn singleton(value: T) -> Self;
        fn from_seq(seq: &ArraySeqMtEphS<T>) -> Self;
        fn filter<F>(&self, predicate: F) -> Self
        where
            F: Fn(&T) -> B + Send + Sync + 'static;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, value: &T) -> B;
        fn delete(&self, value: &T);
        fn insert(&self, value: T);
    }

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEph<T> {
        fn from_inner(inner: AVLTreeSetStEph<T>) -> Self {
            AVLTreeSetMtEph {
                inner: Arc::new(Mutex::new(inner)),
            }
        }
    }

    impl<T: StTInMtT + Ord + 'static> AVLTreeSetMtEphTrait<T> for AVLTreeSetMtEph<T> {
        fn size(&self) -> N {
            let guard = self.inner.lock().unwrap();
            guard.size()
        }

        fn to_seq(&self) -> ArraySeqMtEphS<T> {
            let values = {
                let guard = self.inner.lock().unwrap();
                guard.inorder_vec()
            };
            ArraySeqMtEphS::from_vec(values)
        }

        fn empty() -> Self {
            Self::from_inner(AVLTreeSetStEph::empty())
        }

        fn singleton(value: T) -> Self {
            let mut set = AVLTreeSetStEph::empty();
            set.insert(value);
            Self::from_inner(set)
        }

        fn from_seq(seq: &ArraySeqMtEphS<T>) -> Self {
            let snapshot = seq.iter_cloned();
            let mut set = AVLTreeSetStEph::empty();
            for value in snapshot {
                set.insert(value);
            }
            Self::from_inner(set)
        }

        fn filter<F>(&self, predicate: F) -> Self
        where
            F: Fn(&T) -> B + Send + Sync + 'static,
        {
            let values = {
                let guard = self.inner.lock().unwrap();
                guard.inorder_vec()
            };
            let predicate = Arc::new(predicate);
            let mut handles = Vec::with_capacity(values.len());
            for value in values.into_iter() {
                let predicate_clone = Arc::clone(&predicate);
                handles.push(thread::spawn(move || {
                    let keep = predicate_clone(&value);
                    (value, keep)
                }));
            }
            let mut result = AVLTreeSetStEph::empty();
            for handle in handles {
                let (value, keep) = handle.join().unwrap();
                if keep {
                    result.insert(value);
                }
            }
            Self::from_inner(result)
        }

        fn intersection(&self, other: &Self) -> Self {
            let lhs = self.inner.lock().unwrap().clone();
            let rhs = other.inner.lock().unwrap().clone();
            Self::from_inner(lhs.intersection(&rhs))
        }

        fn difference(&self, other: &Self) -> Self {
            let lhs = self.inner.lock().unwrap().clone();
            let rhs = other.inner.lock().unwrap().clone();
            Self::from_inner(lhs.difference(&rhs))
        }

        fn union(&self, other: &Self) -> Self {
            let lhs = self.inner.lock().unwrap().clone();
            let rhs = other.inner.lock().unwrap().clone();
            Self::from_inner(lhs.union(&rhs))
        }

        fn find(&self, value: &T) -> B {
            let guard = self.inner.lock().unwrap();
            guard.find(value)
        }

        fn delete(&self, value: &T) {
            let mut guard = self.inner.lock().unwrap();
            guard.delete(value);
        }

        fn insert(&self, value: T) {
            let mut guard = self.inner.lock().unwrap();
            guard.insert(value);
        }
    }
}

