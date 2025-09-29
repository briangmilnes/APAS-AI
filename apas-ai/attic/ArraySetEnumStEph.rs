//! Chap41Codex: single-threaded ephemeral enumerated set stored as boolean array.

pub mod ArraySetEnumStEph {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Types::Types::*;

    #[derive(Clone, PartialEq, Eq)]
    pub struct ArraySetEnumStEph {
        bits: Box<[B]>,
        universe_size: N,
    }

    pub type ArraySetEnumStEphS = ArraySetEnumStEph;

    pub trait ArraySetEnumStEphTrait {
        fn new(universe: N) -> Self;
        fn size(&self) -> N;
        fn to_seq(&self) -> ArraySeqStEphS<N>;
        fn empty(universe: N) -> Self;
        fn singleton(universe: N, value: N) -> Self;
        fn from_seq(universe: N, seq: &ArraySeqStEphS<N>) -> Self;
        fn filter<F>(&self, keep: F) -> Self
        where
            F: Fn(N) -> B;
        fn intersection(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn find(&self, value: N) -> B;
        fn delete(&mut self, value: N);
        fn insert(&mut self, value: N);
    }

    impl ArraySetEnumStEph {
        fn checked_clone_bits(&self) -> Box<[B]> {
            let mut clone = vec![false; self.universe_size].into_boxed_slice();
            clone.copy_from_slice(&self.bits);
            clone
        }
    }

    impl ArraySetEnumStEphTrait for ArraySetEnumStEph {
        fn new(universe: N) -> Self {
            ArraySetEnumStEph {
                bits: vec![false; universe].into_boxed_slice(),
                universe_size: universe,
            }
        }

        fn size(&self) -> N {
            self.bits.iter().filter(|bit| **bit).count()
        }

        fn to_seq(&self) -> ArraySeqStEphS<N> {
            let count = self.size();
            let mut values = Vec::with_capacity(count);
            for (index, bit) in self.bits.iter().enumerate() {
                if *bit {
                    values.push(index);
                }
            }
            ArraySeqStEphS::from_vec(values)
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

        fn from_seq(universe: N, seq: &ArraySeqStEphS<N>) -> Self {
            let mut set = Self::new(universe);
            for index in 0..seq.length() {
                let value = *seq.nth(index);
                if value < set.universe_size {
                    set.bits[value] = true;
                }
            }
            set
        }

        fn filter<F>(&self, keep: F) -> Self
        where
            F: Fn(N) -> B,
        {
            let mut result_bits = self.checked_clone_bits();
            for index in 0..self.universe_size {
                if result_bits[index] && !keep(index) {
                    result_bits[index] = false;
                }
            }
            ArraySetEnumStEph {
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
            ArraySetEnumStEph {
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
            ArraySetEnumStEph {
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
            ArraySetEnumStEph {
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
