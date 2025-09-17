//! StPersistent (immutable by rebuilding) singly linked list.

pub mod LinkedListStPer {
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct NodeP<T: StT> {
        pub value: T,
        pub next: Option<Box<NodeP<T>>>,
    }

    #[derive(Clone)]
    pub struct LinkedListStPerS<T: StT> {
        head: Option<Box<NodeP<T>>>,
        len: N,
    }

    pub trait LinkedListStPerTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty() -> Self;
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
        /// APAS (ephemeral set Θ(1 + i)); rebuilding here: Work Θ(length), Span Θ(1)
        /// claude-4-sonet: Work Θ(length), Span Θ(1)
        fn set(&self, index: N, item: T) -> Result<Self, &'static str>
        where
            Self: Sized;
        /// APAS: Work Θ(start + length), Span Θ(start + length)
        /// claude-4-sonet: Work Θ(start + length), Span Θ(start + length)
        fn subseq_copy(&self, start: N, length: N) -> Self;
    }

    impl<T: StT> LinkedListStPerS<T> {
        fn push_front_node(&mut self, node: Box<NodeP<T>>) {
            let mut n = node;
            n.next = self.head.take();
            self.head = Some(n);
            self.len += 1;
        }

        pub fn from_vec(v: Vec<T>) -> Self {
            let mut list = LinkedListStPerS::empty();
            for value in v.into_iter().rev() {
                list.push_front_node(Box::new(NodeP { value, next: None }));
            }
            list
        }

        pub fn iter<'a>(&'a self) -> LinkedListStPerIter<'a, T> {
            LinkedListStPerIter {
                cursor: self.head.as_deref(),
            }
        }
    }

    impl<T: StT> LinkedListStPerTrait<T> for LinkedListStPerS<T> {
        fn empty() -> Self {
            LinkedListStPerS { head: None, len: 0 }
        }
        fn new(length: N, init_value: T) -> Self {
            let mut list = LinkedListStPerS::empty();
            for _ in 0..length {
                list.push_front_node(Box::new(NodeP {
                    value: init_value.clone(),
                    next: None,
                }));
            }
            // reverse to maintain intuitive order
            let mut rev: Option<Box<NodeP<T>>> = None;
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
            if self.len == 0 { B::True } else { B::False }
        }
        fn isSingleton(&self) -> B {
            if self.len == 1 { B::True } else { B::False }
        }
        fn singleton(item: T) -> Self {
            LinkedListStPerS {
                head: Some(Box::new(NodeP {
                    value: item,
                    next: None,
                })),
                len: 1,
            }
        }
        fn set(&self, index: N, item: T) -> Result<Self, &'static str> {
            if index >= self.len {
                return Err("Index out of bounds");
            }
            // rebuild list with the changed value
            let mut out = LinkedListStPerS::empty();
            let mut i = 0;
            let mut cur = self.head.as_ref();
            while let Some(node) = cur {
                let v = if i == index { item.clone() } else { node.value.clone() };
                out.push_front_node(Box::new(NodeP { value: v, next: None }));
                i += 1;
                cur = node.next.as_ref();
            }
            // reverse to preserve order
            let mut rev: Option<Box<NodeP<T>>> = None;
            while let Some(mut n) = out.head.take() {
                let next = n.next.take();
                n.next = rev.take();
                rev = Some(n);
                out.head = next;
            }
            out.head = rev;
            Ok(out)
        }
        fn subseq_copy(&self, start: N, length: N) -> Self {
            let n = self.len;
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return LinkedListStPerS::empty();
            }
            let mut out = LinkedListStPerS::empty();
            let mut i = 0;
            let mut cur = self.head.as_ref();
            while let Some(node) = cur {
                if i >= s && i < e {
                    out.push_front_node(Box::new(NodeP {
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
            let mut rev: Option<Box<NodeP<T>>> = None;
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

    impl<T: StT> std::fmt::Debug for LinkedListStPerS<T> {
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

    pub struct LinkedListStPerIter<'a, T: StT> {
        cursor: Option<&'a NodeP<T>>,
    }

    impl<'a, T: StT> Iterator for LinkedListStPerIter<'a, T> {
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

    impl<T: StT> PartialEq for LinkedListStPerS<T> {
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

    impl<T: StT> Eq for LinkedListStPerS<T> {}

    impl<T: StT> std::fmt::Display for LinkedListStPerS<T> {
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
    macro_rules! LinkedListStPer {
        () => { $crate::LinkedListStPer::LinkedListStPer::LinkedListStPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => {{
            < $crate::LinkedListStPer::LinkedListStPer::LinkedListStPerS<_> as $crate::LinkedListStPer::LinkedListStPer::LinkedListStPerTrait<_> >::new($n, $x)
        }};
        ($($x:expr),* $(,)?) => { $crate::LinkedListStPer::LinkedListStPer::LinkedListStPerS::from_vec(vec![$($x),*]) };
    }

    #[allow(dead_code)]
    fn _LinkedListStPer_type_checks() {
        use crate::LinkedListStPer::LinkedListStPer::LinkedListStPerS;
        let _ = LinkedListStPer![1]; // non-empty infers (e.g., i32)
        let _: LinkedListStPerS<i32> = LinkedListStPer![]; // empty form requires explicit type
    }
}
