//! Chap41Codex: multi-threaded ephemeral enumerated set stored as boolean array.

pub mod ArraySetEnumMtEph {
    use std::sync::Arc;
    use std::thread;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS;
    use crate::Types::Types::*;

    #[derive(Clone, PartialEq, Eq)]
    pub struct ArraySetEnumMtEph {
        bits: Box<[B]>,
        universe_size: N,
    }

    pub type ArraySetEnumMtEphS = ArraySetEnumMtEph;

    pub trait ArraySetEnumMtEphTrait {
        fn new(universe: N) -> Self;
        fn size(&self) -> N;
        fn to_seq(&self) -> ArraySeqMtEphS<N>;
        fn empty(universe: N) -> Self;
        fn singleton(universe: N, value: N) -> Self;
        fn from_seq(universe: N, seq: &ArraySeqMtEphS<N>) -> Self;
        fn filter<F>(&self, keep: F) -> Self
        where
            F: Fn(N) -> B + Send + Sync + 'static;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, value: N) -> B;
        fn delete(&mut self, value: N);
        fn insert(&mut self, value: N);
    }

    impl ArraySetEnumMtEphTrait for ArraySetEnumMtEph {
        fn new(universe: N) -> Self {
            ArraySetEnumMtEph {
                bits: vec![false; universe].into_boxed_slice(),
                universe_size: universe,
            }
        }

        fn size(&self) -> N {
            self.bits.iter().filter(|bit| **bit).count()
        }

        fn to_seq(&self) -> ArraySeqMtEphS<N> {
            let count = self.size();
            let mut values = Vec::with_capacity(count);
            for (index, bit) in self.bits.iter().enumerate() {
                if *bit {
                    values.push(index);
                }
            }
            ArraySeqMtEphS::from_vec(values)
        }

        fn empty(universe: N) -> Self {
            Self::new(universe)
        }

        fn singleton(universe: N, value: N) -> Self {
            let mut set = Self::new(universe);
            if value < set.universe_size {
                set.bits[value] = true;
            }
            set
        }

        fn from_seq(universe: N, seq: &ArraySeqMtEphS<N>) -> Self {
            let mut set = Self::new(universe);
            let snapshot = seq.iter_cloned();
            for value in snapshot {
                if value < set.universe_size {
                    set.bits[value] = true;
                }
            }
            set
        }

        fn filter<F>(&self, keep: F) -> Self
        where
            F: Fn(N) -> B + Send + Sync + 'static,
        {
            let predicate = Arc::new(keep);
            let mut handles: Vec<(N, thread::JoinHandle<B>)> = Vec::new();
            handles.reserve(self.size());

            for index in 0..self.universe_size {
                if self.bits[index] {
                    let predicate_clone = Arc::clone(&predicate);
                    handles.push((index, thread::spawn(move || predicate_clone(index))));
                }
            }

            let mut result_bits = vec![false; self.universe_size].into_boxed_slice();
            for (index, handle) in handles {
                if handle.join().unwrap() {
                    result_bits[index] = true;
                }
            }

            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        fn intersection(&self, other: &Self) -> Self {
            assert_eq!(self.universe_size, other.universe_size, "Universe sizes must match");
            let mut result_bits = vec![false; self.universe_size].into_boxed_slice();
            for index in 0..self.universe_size {
                result_bits[index] = self.bits[index] && other.bits[index];
            }
            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        fn difference(&self, other: &Self) -> Self {
            assert_eq!(self.universe_size, other.universe_size, "Universe sizes must match");
            let mut result_bits = vec![false; self.universe_size].into_boxed_slice();
            for index in 0..self.universe_size {
                result_bits[index] = self.bits[index] && !other.bits[index];
            }
            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        fn union(&self, other: &Self) -> Self {
            assert_eq!(self.universe_size, other.universe_size, "Universe sizes must match");
            let mut result_bits = vec![false; self.universe_size].into_boxed_slice();
            for index in 0..self.universe_size {
                result_bits[index] = self.bits[index] || other.bits[index];
            }
            ArraySetEnumMtEph {
                bits: result_bits,
                universe_size: self.universe_size,
            }
        }

        fn find(&self, value: N) -> B {
            value < self.universe_size && self.bits[value]
        }

        fn delete(&mut self, value: N) {
            if value < self.universe_size {
                self.bits[value] = false;
            }
        }

        fn insert(&mut self, value: N) {
            if value < self.universe_size {
                self.bits[value] = true;
            }
        }
    }
}
