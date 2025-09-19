//! StEph (mutable) singly linked list, duplicated from `LinkedList` with renamed types.

pub mod LinkedListStEph {
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct NodeE<T: StT> {
        pub value: T,
        pub next: Option<Box<NodeE<T>>>,
    }

    #[derive(Clone)]
    pub struct LinkedListStEphS<T: StT> {
        head: Option<Box<NodeE<T>>>,
        len: N,
    }

    pub trait LinkedListStEphTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> LinkedListStEphS<T>;
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
        fn nth(&self, index: N) -> &T;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
        fn update(&mut self, item_at: Pair<N, T>) -> &mut Self;
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
        fn set(&mut self, index: N, item: T) -> Result<&mut Self, &'static str>;
        /// APAS: Work Θ(start + length), Span Θ(start + length)
        /// claude-4-sonet: Work Θ(start + length), Span Θ(start + length)
        fn subseq_copy(&self, start: N, length: N) -> Self;
    }

    impl<T: StT> LinkedListStEphS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn push_front_node(&mut self, node: Box<NodeE<T>>) {
            let mut n = node;
            n.next = self.head.take();
            self.head = Some(n);
            self.len += 1;
        }

        /// APAS: Work Θ(|v|), Span Θ(1)
        /// claude-4-sonet: Work Θ(|v|), Span Θ(1)
        pub fn from_vec(v: Vec<T>) -> Self {
            let mut list = LinkedListStEphS::empty();
            for value in v.into_iter().rev() {
                list.push_front_node(Box::new(NodeE { value, next: None }));
            }
            list
        }

        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        pub fn iter<'a>(&'a self) -> LinkedListStEphIter<'a, T> {
            LinkedListStEphIter { cursor: self.head.as_deref() }
        }
    }

    impl<T: StT> LinkedListStEphTrait<T> for LinkedListStEphS<T> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Self {
            LinkedListStEphS { head: None, len: 0 }
        }
        /// APAS: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn new(length: N, init_value: T) -> Self {
            let mut list = LinkedListStEphS::empty();
            for _ in 0..length {
                list.push_front_node(Box::new(NodeE { value: init_value.clone(), next: None }));
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
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn length(&self) -> N {
            self.len
        }
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
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
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isEmpty(&self) -> B {
            if self.len == 0 { B::True } else { B::False }
        }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn isSingleton(&self) -> B {
            if self.len == 1 { B::True } else { B::False }
        }
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self {
            LinkedListStEphS { head: Some(Box::new(NodeE { value: item, next: None })), len: 1 }
        }
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
        fn update(&mut self, Pair(index, item): Pair<N, T>) -> &mut Self {
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
        /// APAS: Work Θ(index), Span Θ(index)
        /// claude-4-sonet: Work Θ(index), Span Θ(index)
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
        /// APAS: Work Θ(start + length), Span Θ(start + length)
        /// claude-4-sonet: Work Θ(start + length), Span Θ(start + length)
        fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.len;
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return LinkedListStEphS::empty();
            }
            let mut out = LinkedListStEphS::empty();
            let mut i = 0;
            let mut cur = self.head.as_ref();
            while let Some(node) = cur {
                if i >= s && i < e {
                    out.push_front_node(Box::new(NodeE { value: node.value.clone(), next: None }));
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

    impl<T: StT> std::fmt::Debug for LinkedListStEphS<T> {
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

    pub struct LinkedListStEphIter<'a, T: StT> {
        cursor: Option<&'a NodeE<T>>,
    }

    impl<'a, T: StT> Iterator for LinkedListStEphIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            if let Some(n) = self.cursor {
                self.cursor = n.next.as_deref();
                Some(&n.value)
            } else {
                None
            }
        }
    }

    impl<T: StT> PartialEq for LinkedListStEphS<T> {
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

    impl<T: StT> Eq for LinkedListStEphS<T> {}

    impl<T: StT> std::fmt::Display for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            let mut cur = self.head.as_ref();
            while let Some(n) = cur {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", n.value)?;
                cur = n.next.as_ref();
            }
            write!(f, "]")
        }
    }

    #[macro_export]
    macro_rules! LinkedListStEph {
        () => {
            < $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphS<_> as $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphTrait<_> >::empty()
        };
        ($x:expr; $n:expr) => {{
            < $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphS<_> as $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphTrait<_> >::new($n, $x)
        }};
        ($($x:expr),* $(,)?) => {{
            let __vals = vec![$($x),*];
            let __len = __vals.len();
            if __len == 0 {
                < $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphS<_> as $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphTrait<_> >::empty()
            } else {
                let mut __l = < $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphS<_> as $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphTrait<_> >::new(__len, __vals[0].clone());
                let mut __i: $crate::Types::Types::N = 0;
                for __v in __vals { let _ = < $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphS<_> as $crate::LinkedListStEph::LinkedListStEph::LinkedListStEphTrait<_> >::set(&mut __l, __i, __v); __i += 1; }
                __l
            }
        }};
    }

    #[allow(dead_code)]
    fn _LinkedListStEph_type_checks() {
        let _ = LinkedListStEph![1]; // non-empty infers (e.g., i32)
        let _: LinkedListStEphS<i32> = LinkedListStEph![]; // empty form requires explicit type
    }
}
