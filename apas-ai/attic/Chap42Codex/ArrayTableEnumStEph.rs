//! Enumerated (0..u) table backed by an ArraySeq, exposing only APAS array abstractions.

pub mod ArrayTableEnumStEph {
    use crate::Chap18::ArraySeqStEph::ArraySeqStEph::{ArraySeqStEphS, ArraySeqStEphTrait};
    use crate::Chap41Codex::ArraySetEnumStEph::ArraySetEnumStEph::{
        ArraySetEnumStEph, ArraySetEnumStEphTrait,
    };
    use crate::Types::Types::*;

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct ArrayTableEnumStEph<V: StT> {
        elements: ArraySeqStEphS<Option<V>>,
        size: N,
    }

    pub trait ArrayTableEnumStEphTrait<V: StT>: Sized {
        fn empty(universe: N) -> Self;
        fn singleton(universe: N, key: N, value: V) -> Self;
        fn size(&self) -> N;
        fn universe(&self) -> N;
        fn domain(&self) -> ArraySetEnumStEph;
        fn tabulate<F: Fn(N) -> V>(keys: &ArraySetEnumStEph, value_fn: &F) -> Self;
        fn map<F: Fn(&V) -> V>(&self, value_fn: &F) -> Self;
        fn filter<F: Fn(N, &V) -> B>(&self, predicate: &F) -> Self;
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> Self;
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> Self;
        fn difference(&self, other: &Self) -> Self;
        fn find(&self, key: N) -> Option<&V>;
        fn delete(&mut self, key: N) -> &mut Self;
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: N, value: V, combine: &F) -> &mut Self;
        fn restrict(&self, keys: &ArraySetEnumStEph) -> Self;
        fn subtract(&self, keys: &ArraySetEnumStEph) -> Self;
    }

    impl<V: StT> ArrayTableEnumStEph<V> {
        fn with_universe(universe: N) -> Self {
            ArrayTableEnumStEph {
                elements: ArraySeqStEphS::from_vec(vec![None; universe]),
                size: 0,
            }
        }

        fn clamp_key(&self, key: N) -> Option<N> {
            if key < self.elements.length() {
                Some(key)
            } else {
                None
            }
        }

        fn rebuild(elements: ArraySeqStEphS<Option<V>>) -> Self {
            let mut size = 0;
            for idx in 0..elements.length() {
                if elements.nth(idx).is_some() {
                    size += 1;
                }
            }
            ArrayTableEnumStEph { elements, size }
        }
    }

    impl<V: StT> ArrayTableEnumStEphTrait<V> for ArrayTableEnumStEph<V> {
        /// Codex Work: Θ(u) Span: Θ(1)
        fn empty(universe: N) -> Self {
            Self::with_universe(universe)
        }

        /// Codex Work: Θ(u) Span: Θ(1)
        fn singleton(universe: N, key: N, value: V) -> Self {
            let mut table = Self::with_universe(universe);
            let _ = table.insert(key, value, &|existing, _new| existing.clone());
            table
        }

        /// Codex Work: Θ(1) Span: Θ(1)
        fn size(&self) -> N {
            self.size
        }

        /// Codex Work: Θ(1) Span: Θ(1)
        fn universe(&self) -> N {
            self.elements.length()
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn domain(&self) -> ArraySetEnumStEph {
            let mut set = ArraySetEnumStEph::empty(self.universe());
            for idx in 0..self.universe() {
                if self.elements.nth(idx).is_some() {
                    let _ = set.insert(idx);
                }
            }
            set
        }

        /// Codex Work: Θ(u + |S|) Span: Θ(u)
        fn tabulate<F: Fn(N) -> V>(keys: &ArraySetEnumStEph, value_fn: &F) -> Self {
            let mut table = Self::with_universe(keys.universe_size());
            for key in keys.to_seq().iter() {
                let value = value_fn(*key);
                let _ = table.insert(*key, value, &|_, new_v| new_v.clone());
            }
            table
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn map<F: Fn(&V) -> V>(&self, value_fn: &F) -> Self {
            let mut new_values = Vec::with_capacity(self.universe());
            for idx in 0..self.universe() {
                let mapped = match self.elements.nth(idx) {
                    Some(value) => Some(value_fn(value)),
                    None => None,
                };
                new_values.push(mapped);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn filter<F: Fn(N, &V) -> B>(&self, predicate: &F) -> Self {
            let mut new_values = Vec::with_capacity(self.universe());
            for idx in 0..self.universe() {
                let filtered = match self.elements.nth(idx) {
                    Some(value) if predicate(idx, value) => Some(value.clone()),
                    _ => None,
                };
                new_values.push(filtered);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> Self {
            let len = self.universe().min(other.universe());
            let mut new_values = Vec::with_capacity(len);
            for idx in 0..len {
                let entry = match (self.elements.nth(idx), other.elements.nth(idx)) {
                    (Some(left), Some(right)) => Some(combine(left, right)),
                    _ => None,
                };
                new_values.push(entry);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> Self {
            let len = self.universe().max(other.universe());
            let mut new_values = Vec::with_capacity(len);
            for idx in 0..len {
                let left = if idx < self.universe() {
                    self.elements.nth(idx).clone()
                } else {
                    None
                };
                let right = if idx < other.universe() {
                    other.elements.nth(idx).clone()
                } else {
                    None
                };
                let entry = match (left, right) {
                    (Some(l), Some(r)) => Some(combine(&l, &r)),
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                };
                new_values.push(entry);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn difference(&self, other: &Self) -> Self {
            let len = self.universe();
            let mut new_values = Vec::with_capacity(len);
            for idx in 0..len {
                let keep = if idx < other.universe() {
                    other.elements.nth(idx).is_none()
                } else {
                    true
                };
                let entry = if keep {
                    self.elements.nth(idx).clone()
                } else {
                    None
                };
                new_values.push(entry);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }

        /// Codex Work: Θ(1) Span: Θ(1)
        fn find(&self, key: N) -> Option<&V> {
            self.clamp_key(key)
                .and_then(|idx| self.elements.nth(idx).as_ref())
        }

        /// Codex Work: Θ(1) Span: Θ(1)
        fn delete(&mut self, key: N) -> &mut Self {
            if let Some(idx) = self.clamp_key(key) {
                if self.elements.nth(idx).is_some() {
                    let _ = self.elements.set(idx, None);
                    self.size = self.size.saturating_sub(1);
                }
            }
            self
        }

        /// Codex Work: Θ(1) Span: Θ(1)
        fn insert<F: Fn(&V, &V) -> V>(&mut self, key: N, value: V, combine: &F) -> &mut Self {
            if let Some(idx) = self.clamp_key(key) {
                let current = self.elements.nth(idx).cloned();
                match current {
                    None => {
                        let _ = self.elements.set(idx, Some(value));
                        self.size += 1;
                    }
                    Some(existing) => {
                        let combined = combine(&existing, &value);
                        let _ = self.elements.set(idx, Some(combined));
                    }
                }
            }
            self
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn restrict(&self, keys: &ArraySetEnumStEph) -> Self {
            let len = self.universe();
            let mut new_values = Vec::with_capacity(len);
            for idx in 0..len {
                let entry = if keys.find(idx) {
                    self.elements.nth(idx).clone()
                } else {
                    None
                };
                new_values.push(entry);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }

        /// Codex Work: Θ(u) Span: Θ(u)
        fn subtract(&self, keys: &ArraySetEnumStEph) -> Self {
            let len = self.universe();
            let mut new_values = Vec::with_capacity(len);
            for idx in 0..len {
                let entry = if keys.find(idx) {
                    None
                } else {
                    self.elements.nth(idx).clone()
                };
                new_values.push(entry);
            }
            Self::rebuild(ArraySeqStEphS::from_vec(new_values))
        }
    }
}
