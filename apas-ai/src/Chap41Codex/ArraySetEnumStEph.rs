//! Enumerated set backed by a dense bitvector for single-threaded mutation.

pub mod ArraySetEnumStEph {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
    use crate::Types::Types::*;

    /// Bit-vector set storing presence bits for the universe `0..u`.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ArraySetEnumStEph {
        bits: Vec<B>,
        size: N,
    }

    /// Trait (typeclass interface) describing the enumerated set API.
    pub trait ArraySetEnumStEphTrait {
        fn empty(universe: N) -> Self;
        fn singleton(universe: N, element: N) -> Self;
        fn size(&self) -> N;
        fn universe_size(&self) -> N;
        fn to_seq(&self) -> ArraySeqStEphS<N>;
        fn from_seq(universe: N, values: &ArraySeqStEphS<N>) -> Self;
        fn filter<F: Fn(N) -> B>(&self, predicate: &F) -> Self;
        fn intersection(&self, other: &Self) -> Self;
        fn union(&self, other: &Self) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn find(&self, element: N) -> B;
        fn insert(&mut self, element: N) -> &mut Self;
        fn delete(&mut self, element: N) -> &mut Self;
    }

    impl ArraySetEnumStEph {
        fn zeroed(universe: N) -> Self {
            ArraySetEnumStEph {
                bits: vec![false; universe],
                size: 0,
            }
        }

        fn clamp_index(&self, index: N) -> Option<N> {
            if index < self.bits.len() {
                Some(index)
            } else {
                None
            }
        }

        fn rebuild(bits: Vec<B>) -> Self {
            let size = bits.iter().filter(|&&flag| flag).count();
            ArraySetEnumStEph { bits, size }
        }

        fn combine<F: Fn(B, B) -> B>(&self, other: &Self, op: F) -> Self {
            let len = self.bits.len().min(other.bits.len());
            let mut bits = Vec::with_capacity(len);
            for i in 0..len {
                bits.push(op(self.bits[i], other.bits[i]));
            }
            Self::rebuild(bits)
        }

        fn filter_inner<F: Fn(N) -> B>(&self, predicate: &F) -> Vec<B> {
            let mut out = self.bits.clone();
            for (index, flag) in out.iter_mut().enumerate() {
                if *flag && !predicate(index) {
                    *flag = false;
                }
            }
            out
        }
    }

    impl ArraySetEnumStEphTrait for ArraySetEnumStEph {
        fn empty(universe: N) -> Self {
            Self::zeroed(universe)
        }

        fn singleton(universe: N, element: N) -> Self {
            let mut set = Self::zeroed(universe);
            let _ = set.insert(element);
            set
        }

        fn size(&self) -> N {
            self.size
        }

        fn universe_size(&self) -> N {
            self.bits.len()
        }

        fn to_seq(&self) -> ArraySeqStEphS<N> {
            let mut values: Vec<N> = Vec::with_capacity(self.size());
            for (index, flag) in self.bits.iter().enumerate() {
                if *flag {
                    values.push(index);
                }
            }
            ArraySeqStEphS::from_vec(values)
        }

        fn from_seq(universe: N, values: &ArraySeqStEphS<N>) -> Self {
            let mut bits = vec![false; universe];
            let mut size: N = 0;
            for value in values.iter() {
                if *value < universe {
                    let slot = &mut bits[*value];
                    if !*slot {
                        *slot = true;
                        size += 1;
                    }
                }
            }
            ArraySetEnumStEph { bits, size }
        }

        fn filter<F: Fn(N) -> B>(&self, predicate: &F) -> Self {
            let bits = self.filter_inner(predicate);
            Self::rebuild(bits)
        }

        fn intersection(&self, other: &Self) -> Self {
            self.combine(other, |a, b| a && b)
        }

        fn union(&self, other: &Self) -> Self {
            self.combine(other, |a, b| a || b)
        }

        fn difference(&self, other: &Self) -> Self {
            self.combine(other, |a, b| a && !b)
        }

        fn find(&self, element: N) -> B {
            self.clamp_index(element)
                .map(|index| self.bits[index])
                .unwrap_or(false)
        }

        fn insert(&mut self, element: N) -> &mut Self {
            if let Some(index) = self.clamp_index(element) {
                if !self.bits[index] {
                    self.bits[index] = true;
                    self.size += 1;
                }
            }
            self
        }

        fn delete(&mut self, element: N) -> &mut Self {
            if let Some(index) = self.clamp_index(element) {
                if self.bits[index] {
                    self.bits[index] = false;
                    self.size = self.size.saturating_sub(1);
                }
            }
            self
        }
    }

    #[macro_export]
    macro_rules! ArraySetEnumStEphCodexLit {
        ($u:expr;) => {
            <$crate::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::ArraySetEnumStEph as $crate::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::ArraySetEnumStEphTrait>::empty($u)
        };
        ($u:expr; $($x:expr),+ $(,)?) => {{
            let mut set = <$crate::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::ArraySetEnumStEph as $crate::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::ArraySetEnumStEphTrait>::empty($u);
            $( { let _ = set.insert($x); } )+
            set
        }};
    }
}
