//! Ephemeral (mutable) Array sequence variants. Mirrors `ArraySeq` under distinct names.

pub use crate::Types::{B, N, O};

/// Fixed-length sequence backed by `Box<[T]>` (ephemeral variant).
#[derive(Clone)]
pub struct ArraySeqEphS<T> {
    pub data: Box<[T]>,
}

/// Sequence trait for `ArraySeqEphS<T>`.
pub trait ArraySeqEphTrait<T> {
    /// Work Θ(length), Span Θ(1).
    fn new(length: N, init_value: T) -> Self
    where
        T: Clone;
    /// Work Θ(1), Span Θ(1).
    fn length(&self) -> N;
    /// Work Θ(1), Span Θ(1).
    fn nth(&self, index: N) -> &T;
    /// Work Θ(1), Span Θ(1).
    fn empty() -> Self;
    /// Work Θ(1), Span Θ(1).
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
    /// Work Θ(1), Span Θ(1).
    fn singleton(item: T) -> Self;
    /// Work Θ(1), Span Θ(1).
    fn isEmpty(&self) -> B;
    /// Work Θ(1), Span Θ(1).
    fn isSingleton(&self) -> B;
    /// Work Θ(length) to allocate/clone.
    fn subseq_copy(&self, start: N, length: N) -> Self
    where
        T: Clone + Eq;
}

impl<T> ArraySeqEphS<T> {
    pub fn subseq(&self, start: N, length: N) -> &[T] {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        &self.data[s..e]
    }
    pub fn subseq_copy(&self, start: N, length: N) -> Self
    where
        T: Clone + Eq,
    {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s {
            return <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::empty();
        }
        let len = e - s;
        let first = self.nth(s).clone();
        let mut out = <ArraySeqEphS<T> as ArraySeqEphTrait<T>>::new(len, first.clone());
        let _ = out.set(0, first);
        for i in 1..len {
            let _ = out.set(i, self.nth(s + i).clone());
        }
        out
    }
    pub fn from_vec(v: Vec<T>) -> Self {
        ArraySeqEphS {
            data: v.into_boxed_slice(),
        }
    }
    pub fn update(&mut self, (index, item): (N, T)) -> &mut ArraySeqEphS<T> {
        if index < self.data.len() {
            self.data[index] = item;
        }
        self
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> { self.data.iter() }
}

impl<T: Eq> PartialEq for ArraySeqEphS<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length() != other.length() {
            return false;
        }
        for i in 0..self.length() {
            if self.nth(i) != other.nth(i) {
                return false;
            }
        }
        true
    }
}
impl<T: Eq> Eq for ArraySeqEphS<T> {}

impl<T: std::fmt::Debug> std::fmt::Debug for ArraySeqEphS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elts = (0..self.length()).map(|i| self.nth(i));
        f.debug_list().entries(elts).finish()
    }
}

impl<T> ArraySeqEphTrait<T> for ArraySeqEphS<T> {
    fn new(length: N, init_value: T) -> Self
    where
        T: Clone,
    {
        ArraySeqEphS::from_vec(vec![init_value; length])
    }
    fn length(&self) -> N {
        self.data.len()
    }
    fn nth(&self, index: N) -> &T {
        &self.data[index]
    }
    fn empty() -> Self {
        ArraySeqEphS::from_vec(Vec::new())
    }
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
        if index < self.data.len() {
            self.data[index] = item;
            Ok(self)
        } else {
            Err("Index out of bounds")
        }
    }
    fn singleton(item: T) -> Self {
        ArraySeqEphS::from_vec(vec![item])
    }
    fn isEmpty(&self) -> B {
        if self.data.len() == 0 {
            B::True
        } else {
            B::False
        }
    }
    fn isSingleton(&self) -> B {
        if self.data.len() == 1 {
            B::True
        } else {
            B::False
        }
    }
    fn subseq_copy(&self, start: N, length: N) -> Self
    where
        T: Clone + Eq,
    {
        self.subseq_copy(start, length)
    }
}

#[macro_export]
macro_rules! ArraySeqEph {
    () => { $crate::ArraySeqEph::ArraySeqEphS::from_vec(Vec::new()) };
    ($x:expr; $n:expr) => { $crate::ArraySeqEph::ArraySeqEphS::from_vec(vec![$x; $n]) };
    ($($x:expr),* $(,)?) => { $crate::ArraySeqEph::ArraySeqEphS::from_vec(vec![$($x),*]) };
}
