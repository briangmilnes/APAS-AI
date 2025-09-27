//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree with `find` support and public traversal helpers.

pub mod BSTAVLStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone)]
    #[derive(Debug)]
    struct Node<T: StT + Ord> {
        key: T,
        height: i32,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StT + Ord> Node<T> {
        fn new(key: T) -> Self {
            Node {
                key,
                height: 1,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTAVLStEph<T: StT + Ord> {
        root: Link<T>,
    }

    pub type BSTreeAVL<T> = BSTAVLStEph<T>;

    pub trait BSTAVLStEphTrait<T: StT + Ord> {
        fn new() -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn insert(&mut self, value: T);
        fn find(&self, target: &T) -> Option<&T>;
        fn contains(&self, target: &T) -> B;
        fn minimum(&self) -> Option<&T>;
        fn maximum(&self) -> Option<&T>;
        fn in_order(&self) -> ArraySeqStPerS<T>;
        fn pre_order(&self) -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> Default for BSTAVLStEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StT + Ord> BSTAVLStEph<T> {
        pub fn new() -> Self { BSTAVLStEph { root: None } }

        pub fn size(&self) -> N { Self::size_link(&self.root) }

        pub fn is_empty(&self) -> B { if self.size() == 0 { true } else { false } }

        pub fn height(&self) -> N { Self::height_link(&self.root) as N }

        pub fn insert(&mut self, value: T) { Self::insert_link(&mut self.root, value); }

        pub fn find(&self, target: &T) -> Option<&T> { Self::find_link(&self.root, target) }

        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { true } else { false } }

        pub fn minimum(&self) -> Option<&T> { Self::min_link(&self.root) }

        pub fn maximum(&self) -> Option<&T> { Self::max_link(&self.root) }

        pub fn in_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::in_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        pub fn pre_order(&self) -> ArraySeqStPerS<T> {
            let mut out = Vec::with_capacity(self.size());
            Self::pre_order_collect(&self.root, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn height_link(link: &Link<T>) -> i32 { link.as_ref().map_or(0, |n| n.height) }

        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update(node: &mut Node<T>) {
            node.height = 1 + Self::height_link(&node.left).max(Self::height_link(&node.right));
            node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right);
        }

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut y) = link.take() {
                if let Some(mut x) = y.left.take() {
                    y.left = x.right.take();
                    Self::update(&mut y);
                    Self::update(&mut x);
                    x.right = Some(y);
                    *link = Some(x);
                } else {
                    *link = Some(y);
                }
            }
        }

        fn rotate_left(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.right.take() {
                    x.right = y.left.take();
                    Self::update(&mut x);
                    Self::update(&mut y);
                    y.left = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        fn rebalance(link: &mut Link<T>) {
            if let Some(node) = link.as_mut() {
                Self::update(node);
                let bf = Self::height_link(&node.left) - Self::height_link(&node.right);
                if bf > 1 {
                    if let Some(left) = node.left.as_mut() {
                        if Self::height_link(&left.right) > Self::height_link(&left.left) {
                            Self::rotate_left(&mut node.left);
                        }
                    }
                    Self::rotate_right(link);
                } else if bf < -1 {
                    if let Some(right) = node.right.as_mut() {
                        if Self::height_link(&right.left) > Self::height_link(&right.right) {
                            Self::rotate_right(&mut node.right);
                        }
                    }
                    Self::rotate_left(link);
                }
            }
            if let Some(node) = link.as_mut() {
                Self::update(node);
            }
        }

        fn insert_link(link: &mut Link<T>, value: T) {
            match link {
                | Some(node) => {
                    if value < node.key {
                        Self::insert_link(&mut node.left, value);
                    } else if value > node.key {
                        Self::insert_link(&mut node.right, value);
                    } else {
                        return;
                    }
                }
                | None => {
                    *link = Some(Box::new(Node::new(value)));
                    return;
                }
            }
            Self::rebalance(link);
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => {
                    if target == &node.key {
                        Some(&node.key)
                    } else if target < &node.key {
                        Self::find_link(&node.left, target)
                    } else {
                        Self::find_link(&node.right, target)
                    }
                }
            }
        }

        fn min_link<'a>(link: &'a Link<T>) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => match node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => match node.right {
                    | None => Some(&node.key),
                    | Some(_) => Self::max_link(&node.right),
                },
            }
        }

        fn in_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                Self::in_order_collect(&node.left, out);
                out.push(node.key.clone());
                Self::in_order_collect(&node.right, out);
            }
        }

        fn pre_order_collect(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                out.push(node.key.clone());
                Self::pre_order_collect(&node.left, out);
                Self::pre_order_collect(&node.right, out);
            }
        }
    }

    impl<T: StT + Ord> BSTAVLStEphTrait<T> for BSTAVLStEph<T> {
        fn new() -> Self { BSTAVLStEph::new() }

        fn size(&self) -> N { BSTAVLStEph::size(self) }

        fn is_empty(&self) -> B { BSTAVLStEph::is_empty(self) }

        fn height(&self) -> N { BSTAVLStEph::height(self) }

        fn insert(&mut self, value: T) { BSTAVLStEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<&T> { BSTAVLStEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTAVLStEph::contains(self, target) }

        fn minimum(&self) -> Option<&T> { BSTAVLStEph::minimum(self) }

        fn maximum(&self) -> Option<&T> { BSTAVLStEph::maximum(self) }

        fn in_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::in_order(self) }

        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTAVLStEph::pre_order(self) }
    }

    #[macro_export]
    macro_rules! BSTAVLStEphLit {
        () => {
            < $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEph<_> as $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let mut __tree = < $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEph<_> as $crate::Chap37::BSTAVLStEph::BSTAVLStEph::BSTAVLStEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTAVLStEphLit_type_checks() {
        let _ = BSTAVLStEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTAVLStEph<i32> = BSTAVLStEphLit![]; // empty form requires explicit type
    }
}
