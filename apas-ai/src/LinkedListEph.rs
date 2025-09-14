//! Eph (mutable) singly linked list, duplicated from `LinkedList` with renamed types.

pub mod LinkedListEph {
use crate::Types::Types::*;

#[derive(Clone)]
pub struct NodeE<T> {
    pub value: T,
    pub next: Option<Box<NodeE<T>>>,
}

#[derive(Clone)]
pub struct LinkedListEphS<T> {
    head: Option<Box<NodeE<T>>>,
    len: N,
}

pub trait LinkedListEphTrait<T> {
    fn empty() -> LinkedListEphS<T>;
    fn new(length: N, init_value: T) -> Self
    where
        T: Clone;
    fn length(&self) -> N;
    fn nth(&self, index: N) -> &T;
    fn isEmpty(&self) -> B;
    fn isSingleton(&self) -> B;
    fn singleton(item: T) -> Self;
    fn update(&mut self, item_at: (N, T)) -> &mut Self;
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
    fn subseq_copy(&self, start: N, length: N) -> Self
    where
        T: Clone;
}

impl<T> LinkedListEphS<T> {
    fn push_front_node(&mut self, node: Box<NodeE<T>>) {
        let mut n = node;
        n.next = self.head.take();
        self.head = Some(n);
        self.len += 1;
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        let mut list = LinkedListEphS::empty();
        for value in v.into_iter().rev() {
            list.push_front_node(Box::new(NodeE { value, next: None }));
        }
        list
    }

    pub fn iter<'a>(&'a self) -> LinkedListEphIter<'a, T> {
        LinkedListEphIter { cursor: self.head.as_deref() }
    }
}

impl<T> LinkedListEphTrait<T> for LinkedListEphS<T> {
    fn empty() -> Self {
        LinkedListEphS { head: None, len: 0 }
    }
    fn new(length: N, init_value: T) -> Self
    where
        T: Clone,
    {
        let mut list = LinkedListEphS::empty();
        for _ in 0..length {
            list.push_front_node(Box::new(NodeE {
                value: init_value.clone(),
                next: None,
            }));
        }
        // reverse to maintain intuitive order
        let mut rev: Option<Box<NodeE<T>>> = None;
        while let Some(mut n) = list.head.take() {
            let next = n.next.take();
            n.next = rev.take();
            rev = Some(n);
            list.head = next;
        }
        list.head = rev;
        list
    }
    fn length(&self) -> N {
        self.len
    }
    fn nth(&self, index: N) -> &T {
        let mut i = 0;
        let mut cur = self.head.as_ref();
        while let Some(n) = cur {
            if i == index {
                return &n.value;
            }
            i += 1;
            cur = n.next.as_ref();
        }
        panic!("index out of bounds")
    }
    fn isEmpty(&self) -> B {
        if self.len == 0 {
            B::True
        } else {
            B::False
        }
    }
    fn isSingleton(&self) -> B {
        if self.len == 1 {
            B::True
        } else {
            B::False
        }
    }
    fn singleton(item: T) -> Self {
        LinkedListEphS {
            head: Some(Box::new(NodeE {
                value: item,
                next: None,
            })),
            len: 1,
        }
    }
    fn update(&mut self, (index, item): (N, T)) -> &mut Self {
        let mut i = 0;
        let mut cur = self.head.as_mut();
        while let Some(n) = cur {
            if i == index {
                n.value = item;
                break;
            }
            i += 1;
            cur = n.next.as_mut();
        }
        self
    }
    fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str> {
        let mut i = 0;
        let mut cur = self.head.as_mut();
        while let Some(n) = cur {
            if i == index {
                n.value = item;
                return Ok(self);
            }
            i += 1;
            cur = n.next.as_mut();
        }
        Err("Index out of bounds")
    }
    fn subseq_copy(&self, start: N, length: N) -> Self
    where
        T: Clone,
    {
        let n = self.len;
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s {
            return LinkedListEphS::empty();
        }
        let mut out = LinkedListEphS::empty();
        let mut i = 0;
        let mut cur = self.head.as_ref();
        while let Some(node) = cur {
            if i >= s && i < e {
                out.push_front_node(Box::new(NodeE {
                    value: node.value.clone(),
                    next: None,
                }));
            }
            if i >= e {
                break;
            }
            i += 1;
            cur = node.next.as_ref();
        }
        // reverse to maintain order
        let mut rev: Option<Box<NodeE<T>>> = None;
        while let Some(mut n) = out.head.take() {
            let next = n.next.take();
            n.next = rev.take();
            rev = Some(n);
            out.head = next;
        }
        out.head = rev;
        out
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for LinkedListEphS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut v = Vec::with_capacity(self.len);
        let mut cur = self.head.as_ref();
        while let Some(n) = cur {
            v.push(&n.value);
            cur = n.next.as_ref();
        }
        f.debug_list().entries(v).finish()
    }
}

pub struct LinkedListEphIter<'a, T> {
    cursor: Option<&'a NodeE<T>>,
}

impl<'a, T> Iterator for LinkedListEphIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(n) = self.cursor {
            self.cursor = n.next.as_deref();
            Some(&n.value)
        } else { None }
    }
}

impl<T: PartialEq> PartialEq for LinkedListEphS<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len {
            return false;
        }
        let mut a = self.head.as_ref();
        let mut b = other.head.as_ref();
        while let (Some(na), Some(nb)) = (a, b) {
            if na.value != nb.value {
                return false;
            }
            a = na.next.as_ref();
            b = nb.next.as_ref();
        }
        true
    }
}

impl<T: Eq> Eq for LinkedListEphS<T> {}

impl<T: std::fmt::Display> std::fmt::Display for LinkedListEphS<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut first = true;
        let mut cur = self.head.as_ref();
        while let Some(n) = cur {
            if !first { write!(f, ", ")?; } else { first = false; }
            write!(f, "{}", n.value)?;
            cur = n.next.as_ref();
        }
        write!(f, "]")
    }
}

#[macro_export]
macro_rules! LinkedListEph {
    ($x:expr; $n:expr) => {{
        < $crate::LinkedListEph::LinkedListEph::LinkedListEphS<_> as $crate::LinkedListEph::LinkedListEph::LinkedListEphTrait<_> >::new($n, $x)
    }};
    ($($x:expr),* $(,)?) => {{
        let __vals = vec![$($x),*];
        let __len = __vals.len();
        if __len == 0 {
            < $crate::LinkedListEph::LinkedListEph::LinkedListEphS<_> as $crate::LinkedListEph::LinkedListEph::LinkedListEphTrait<_> >::empty()
        } else {
            let mut __l = < $crate::LinkedListEph::LinkedListEph::LinkedListEphS<_> as $crate::LinkedListEph::LinkedListEph::LinkedListEphTrait<_> >::new(__len, __vals[0].clone());
            let mut __i: $crate::Types::Types::N = 0;
            for __v in __vals { let _ = < $crate::LinkedListEph::LinkedListEph::LinkedListEphS<_> as $crate::LinkedListEph::LinkedListEph::LinkedListEphTrait<_> >::set(&mut __l, __i, __v); __i += 1; }
            __l
        }
    }};
}

}
