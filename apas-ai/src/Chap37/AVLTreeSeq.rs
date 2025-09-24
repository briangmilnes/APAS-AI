//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Implicit-order AVL tree providing O(lg(n)) nth and set by maintaining subtree sizes.
//!
//! Abstract:
//! - `AVLTreeRoot<T>` stores a balanced binary tree; in-order traversal defines the sequence order.
//! - `AVLTreeNode<T>` stores `value`, `height`, `left_size`, `right_size`, and children.
//! - Supports sequence-like operations similar to `ArraySeq` plus `from_vec`, iterator, and conversion to `ArrayS`.
//! - Duplicates are supported naturally; a `dup_key` monotonically increases to distinguish insertion order if needed.

pub mod AVLTreeSeq {
    use std::fmt::Debug;

    use crate::Chap18::ArraySeq::ArraySeq::*;
    pub use crate::Types::Types::*;

    type Link<T> = Option<Box<AVLTreeNode<T>>>;

    pub struct AVLTreeNode<T: Copy + Debug> {
        pub value: T,
        pub height: N,
        pub left_size: N,
        pub right_size: N,
        pub left: Link<T>,
        pub right: Link<T>,
        pub index: N,
    }

    impl<T: Copy + Debug> AVLTreeNode<T> {
        fn new(value: T, index: N) -> Self {
            AVLTreeNode {
                value,
                height: 1,
                left_size: 0,
                right_size: 0,
                left: None,
                right: None,
                index,
            }
        }
    }

    pub struct AVLTreeS<T: Copy + Debug> {
        pub root: Link<T>,
        pub next_key: N,
    }

    pub trait AVLTreeSeq<T: Copy + Debug> {
        /// Construct an empty tree.
        /// APAS: Work Θ(1), Span Θ(1).
        fn empty() -> AVLTreeS<T>;

        /// Construct an empty tree (alias).
        /// APAS: Work Θ(1), Span Θ(1).
        fn new() -> AVLTreeS<T>;

        /// Return number of elements.
        /// APAS: Work Θ(1), Span Θ(1).
        fn length(&self) -> N;

        /// Return a reference to the in-order `index`-th element. Panics if out of bounds.
        /// APAS: Work Θ(lg(n)), Span Θ(lg(n)).
        fn nth(&self, index: N) -> &T;

        /// Set the in-order `index`-th element to `item`.
        /// APAS: Work Θ(lg(n)), Span Θ(lg(n)).
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str>;

        /// Construct a singleton sequence.
        /// APAS: Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> AVLTreeS<T>;

        /// Predicates.
        /// APAS: Work Θ(1), Span Θ(1).
        fn isEmpty(&self) -> B;
        fn isSingleton(&self) -> B;

        /// Return subsequence [start, start+length) as a new tree.
        /// APAS: Work Θ(1 + lg(|a|)), Span Θ(1 + lg(|a|)).
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>
        where
            T: Clone + Eq;
    }

    impl<T: Copy + Debug> AVLTreeS<T> {
        pub fn new_root() -> Self {
            AVLTreeS {
                root: None,
                next_key: 0,
            }
        }
        pub fn new() -> Self { Self::new_root() }

        pub fn update(&mut self, (index, item): (N, T)) -> &mut AVLTreeS<T> {
            let _ = <AVLTreeS<T> as AVLTreeSeq<T>>::set(self, index, item);
            self
        }

        pub fn from_vec(values: Vec<T>) -> AVLTreeS<T>
        where
            T: Clone,
        {
            let length = values.len();
            let mut t = AVLTreeS::new_root();
            for (i, v) in values.into_iter().enumerate() {
                t.root = insert_at_link(t.root.take(), i, v, &mut t.next_key);
            }
            debug_assert!(t.length() == length);
            t
        }

        pub fn to_arrayseq(&self) -> ArrayS<T>
        where
            T: Clone,
        {
            let len = self.length();
            if len == 0 {
                return <ArrayS<T> as ArraySeq<T>>::empty();
            }
            let mut it = self.iter();
            let first = it.next().expect("length > 0 but iter was empty").clone();
            let mut out = <ArrayS<T> as ArraySeq<T>>::new(len, first);
            let mut index: N = 1;
            for v in it {
                let _ = out.set(index, v.clone());
                index += 1;
            }
            out
        }

        pub fn iter<'a>(&'a self) -> AVLTreeSeqIter<'a, T> { AVLTreeSeqIter::new(&self.root) }

        pub fn push_back(&mut self, value: T) {
            let len = self.length();
            let node = insert_at_link(self.root.take(), len, value, &mut self.next_key);
            self.root = node;
        }

        // Convenience APIs kept for older tests (set-like operations over sequence storage)
        pub fn contains_value(&self, target: &T) -> B
        where
            T: PartialEq,
        {
            for v in self.iter() {
                if v == target {
                    return B::True;
                }
            }
            B::False
        }

        pub fn insert_value(&mut self, value: T) { self.push_back(value); }

        pub fn delete_value(&mut self, target: &T) -> bool
        where
            T: Clone + PartialEq,
        {
            let len = self.length();
            let mut found_index: Option<N> = None;
            for i in 0..len {
                if self.nth(i) == target {
                    found_index = Some(i);
                    break;
                }
            }
            if let Some(idx) = found_index {
                // Rebuild without the element at idx, using ArraySeq preallocation
                let mut out_vec: Vec<T> = Vec::with_capacity(len - 1);
                for i in 0..idx {
                    out_vec.push(self.nth(i).clone());
                }
                for i in (idx + 1)..len {
                    out_vec.push(self.nth(i).clone());
                }
                *self = AVLTreeS::from_vec(out_vec);
                true
            } else {
                false
            }
        }

        pub fn is_tree_empty(&self) -> bool { self.length() == 0 }

        pub fn values_in_order(&self) -> Vec<T>
        where
            T: Clone,
        {
            let mut out = Vec::with_capacity(self.length());
            push_inorder(&self.root, &mut out);
            out
        }
    }

    impl<T: Copy + Debug> AVLTreeSeq<T> for AVLTreeS<T> {
        /// APAS: Work Θ(1), Span Θ(1).
        fn empty() -> AVLTreeS<T> { AVLTreeS::new_root() }

        /// APAS: Work Θ(1), Span Θ(1).
        fn new() -> AVLTreeS<T> { AVLTreeS::new_root() }

        /// APAS: Work Θ(1), Span Θ(1).
        fn length(&self) -> N { size_link(&self.root) }

        /// APAS: Work Θ(lg(n)), Span Θ(lg(n)).
        fn nth(&self, index: N) -> &T { nth_link(&self.root, index) }

        /// APAS: Work Θ(lg(n)), Span Θ(lg(n)).
        fn set(&mut self, index: N, item: T) -> Result<&mut AVLTreeS<T>, &'static str> {
            set_link(&mut self.root, index, item)?;
            Ok(self)
        }

        /// APAS: Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> AVLTreeS<T> {
            let mut t = AVLTreeS::new_root();
            t.root = insert_at_link(t.root.take(), 0, item, &mut t.next_key);
            t
        }

        /// APAS: Work Θ(1), Span Θ(1).
        fn isEmpty(&self) -> B { if self.length() == 0 { B::True } else { B::False } }
        /// APAS: Work Θ(1), Span Θ(1).
        fn isSingleton(&self) -> B { if self.length() == 1 { B::True } else { B::False } }

        /// APAS: Work Θ(1 + lg(|a|)), Span Θ(1 + lg(|a|)).
        fn subseq_copy(&self, start: N, length: N) -> AVLTreeS<T>
        where
            T: Clone + Eq,
        {
            let n = self.length();
            let s = start.min(n);
            let e = start.saturating_add(length).min(n);
            if e <= s {
                return <AVLTreeS<T> as AVLTreeSeq<T>>::empty();
            }
            let mut vals: Vec<T> = Vec::with_capacity(e - s);
            for i in s..e {
                vals.push(self.nth(i).clone());
            }
            AVLTreeS::from_vec(vals)
        }
    }

    impl<T: Eq + Copy + Debug> PartialEq for AVLTreeS<T> {
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

    impl<T: Eq + Copy + Debug> Eq for AVLTreeS<T> {}

    impl<T: Debug + Copy> std::fmt::Debug for AVLTreeS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let elts = (0..self.length()).map(|i| self.nth(i));
            f.debug_list().entries(elts).finish()
        }
    }

    impl<T: std::fmt::Display + Copy + Debug> std::fmt::Display for AVLTreeS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            for v in self.iter() {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{}", v)?;
            }
            write!(f, "]")
        }
    }

    // ---- Iterator ----

    pub struct AVLTreeSeqIter<'a, T: Copy + Debug> {
        stack: Vec<&'a AVLTreeNode<T>>,
        current: Option<&'a AVLTreeNode<T>>,
    }

    impl<'a, T: Copy + Debug> AVLTreeSeqIter<'a, T> {
        fn new(root: &'a Link<T>) -> Self {
            let mut it = AVLTreeSeqIter {
                stack: Vec::new(),
                current: None,
            };
            it.push_left(root);
            it
        }

        fn push_left(&mut self, link: &'a Link<T>) {
            let mut cursor = link;
            while let Some(ref node) = cursor.as_ref() {
                self.stack.push(node);
                cursor = &node.left;
            }
        }
    }

    impl<'a, T: Copy + Debug> Iterator for AVLTreeSeqIter<'a, T> {
        type Item = &'a T;
        fn next(&mut self) -> Option<Self::Item> {
            let node = self.stack.pop()?;
            let value_ref: &T = &node.value;
            self.push_left(&node.right);
            Some(value_ref)
        }
    }

    // ---- Internal helpers ----

    fn h<T: Copy + Debug>(n: &Link<T>) -> N { n.as_ref().map_or(0, |b| b.height) }
    fn size_link<T: Copy + Debug>(n: &Link<T>) -> N {
        if let Some(b) = n {
            1 + b.left_size + b.right_size
        } else {
            0
        }
    }

    fn update_meta<T: Copy + Debug>(n: &mut Box<AVLTreeNode<T>>) {
        n.left_size = size_link(&n.left);
        n.right_size = size_link(&n.right);
        let hl = h(&n.left);
        let hr = h(&n.right);
        n.height = 1 + hl.max(hr);
    }

    fn rotate_right<T: Copy + Debug>(mut y: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        let mut x = y.left.take().expect("rotate_right requires left child");
        let t2 = x.right.take();

        y.left = t2;
        update_meta(&mut y);

        x.right = Some(y);
        update_meta(x.right.as_mut().unwrap());
        update_meta(&mut x);
        x
    }

    fn rotate_left<T: Copy + Debug>(mut x: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        let mut y = x.right.take().expect("rotate_left requires right child");
        let t2 = y.left.take();

        x.right = t2;
        update_meta(&mut x);

        y.left = Some(x);
        update_meta(y.left.as_mut().unwrap());
        update_meta(&mut y);
        y
    }

    fn rebalance<T: Copy + Debug>(mut n: Box<AVLTreeNode<T>>) -> Box<AVLTreeNode<T>> {
        update_meta(&mut n);
        let hl = h(&n.left);
        let hr = h(&n.right);
        if hl > hr.saturating_add(1) {
            // Left heavy: check for Left-Right case
            if h(&n.left.as_ref().unwrap().right) > h(&n.left.as_ref().unwrap().left) {
                let left = n.left.take().unwrap();
                n.left = Some(rotate_left(left));
            }
            return rotate_right(n);
        }
        if hr > hl.saturating_add(1) {
            // Right heavy: check for Right-Left case
            if h(&n.right.as_ref().unwrap().left) > h(&n.right.as_ref().unwrap().right) {
                let right = n.right.take().unwrap();
                n.right = Some(rotate_right(right));
            }
            return rotate_left(n);
        }
        n
    }

    #[doc(hidden)]
    pub(crate) fn insert_at_link<T: Copy + Debug>(node: Link<T>, index: N, value: T, next_key: &mut N) -> Link<T> {
        match node {
            | None => {
                debug_assert!(index == 0, "insert_at_link reached None with index > 0");
                let key = *next_key;
                *next_key += 1;
                Some(Box::new(AVLTreeNode::new(value, key)))
            }
            | Some(mut n) => {
                let left_size = n.left_size;
                if index <= left_size {
                    n.left = insert_at_link(n.left.take(), index, value, next_key);
                } else {
                    n.right = insert_at_link(n.right.take(), index - left_size - 1, value, next_key);
                }
                Some(rebalance(n))
            }
        }
    }

    fn nth_link<'a, T: Copy + Debug>(node: &'a Link<T>, index: N) -> &'a T {
        let n = node.as_ref().expect("index out of bounds");
        let left_size = n.left_size;
        if index < left_size {
            return nth_link(&n.left, index);
        }
        if index == left_size {
            return &n.value;
        }
        nth_link(&n.right, index - left_size - 1)
    }

    fn set_link<T: Copy + Debug>(node: &mut Link<T>, index: N, value: T) -> Result<(), &'static str> {
        match node {
            | None => Err("Index out of bounds"),
            | Some(n) => {
                let left_size = n.left_size;
                if index < left_size {
                    set_link(&mut n.left, index, value)
                } else if index == left_size {
                    n.value = value;
                    Ok(())
                } else {
                    set_link(&mut n.right, index - left_size - 1, value)
                }
            }
        }
    }

    fn push_inorder<T: Copy + Debug>(link: &Link<T>, out: &mut Vec<T>)
    where
        T: Clone,
    {
        if let Some(n) = link {
            push_inorder(&n.left, out);
            out.push(n.value);
            push_inorder(&n.right, out);
        }
    }
}
