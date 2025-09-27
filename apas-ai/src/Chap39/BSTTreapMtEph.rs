//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral Treap (randomized heap-ordered BST) with interior locking for multi-threaded access.

pub mod BSTTreapMtEph {
    use std::sync::{Arc, RwLock};

    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;
    use rand::{Rng, rng};

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        priority: u64,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTInMtT + Ord> Node<T> {
        fn new(key: T, priority: u64) -> Self {
            Node {
                key,
                priority,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct BSTTreapMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeTreap<T> = BSTTreapMtEph<T>;

    pub trait BSTTreapMtEphTrait<T: StTInMtT + Ord>: Sized {
        fn new() -> Self;
        fn insert(&self, value: T);
        fn find(&self, target: &T) -> Option<T>;
        fn contains(&self, target: &T) -> B;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn minimum(&self) -> Option<T>;
        fn maximum(&self) -> Option<T>;
        fn in_order(&self) -> ArraySeqStPerS<T>;
        fn pre_order(&self) -> ArraySeqStPerS<T>;
    }

    impl<T: StTInMtT + Ord> Default for BSTTreapMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StTInMtT + Ord> BSTTreapMtEph<T> {
        pub fn new() -> Self {
            BSTTreapMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        pub fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::size_link(&*guard)
        }

        pub fn is_empty(&self) -> B { if self.size() == 0 { true } else { false } }

        pub fn height(&self) -> N {
            fn height_rec<T: StTInMtT + Ord>(link: &Link<T>) -> N {
                match link {
                    | None => 0,
                    | Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }

            let guard = self.root.read().unwrap();
            height_rec(&*guard)
        }

        pub fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            let mut rng = rng();
            Self::insert_link(&mut *guard, value, &mut rng);
        }

        pub fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::find_link(&*guard, target).cloned()
        }

        pub fn contains(&self, target: &T) -> B { if self.find(target).is_some() { true } else { false } }

        pub fn minimum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::min_link(&*guard).cloned()
        }

        pub fn maximum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::max_link(&*guard).cloned()
        }

        pub fn in_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::in_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        pub fn pre_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::pre_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right); }

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

        fn rotate_right(link: &mut Link<T>) {
            if let Some(mut x) = link.take() {
                if let Some(mut y) = x.left.take() {
                    x.left = y.right.take();
                    Self::update(&mut x);
                    Self::update(&mut y);
                    y.right = Some(x);
                    *link = Some(y);
                } else {
                    *link = Some(x);
                }
            }
        }

        fn insert_link(link: &mut Link<T>, value: T, rng: &mut impl Rng) {
            if let Some(node) = link.as_mut() {
                if value < node.key {
                    Self::insert_link(&mut node.left, value, rng);
                    if node.left.as_ref().map_or(false, |left| left.priority < node.priority) {
                        Self::rotate_right(link);
                    }
                } else if value > node.key {
                    Self::insert_link(&mut node.right, value, rng);
                    if node
                        .right
                        .as_ref()
                        .map_or(false, |right| right.priority < node.priority)
                    {
                        Self::rotate_left(link);
                    }
                }
                if let Some(node) = link.as_mut() {
                    Self::update(node);
                }
            } else {
                *link = Some(Box::new(Node::new(value, rng.random())));
            }
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

    impl<T: StTInMtT + Ord> BSTTreapMtEphTrait<T> for BSTTreapMtEph<T> {
        fn new() -> Self { BSTTreapMtEph::new() }

        fn insert(&self, value: T) { BSTTreapMtEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<T> { BSTTreapMtEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTTreapMtEph::contains(self, target) }

        fn size(&self) -> N { BSTTreapMtEph::size(self) }

        fn is_empty(&self) -> B { BSTTreapMtEph::is_empty(self) }

        fn height(&self) -> N { BSTTreapMtEph::height(self) }

        fn minimum(&self) -> Option<T> { BSTTreapMtEph::minimum(self) }

        fn maximum(&self) -> Option<T> { BSTTreapMtEph::maximum(self) }

        fn in_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::in_order(self) }

        fn pre_order(&self) -> ArraySeqStPerS<T> { BSTTreapMtEph::pre_order(self) }
    }

    #[macro_export]
    macro_rules! BSTTreapMtEphLit {
        () => {
            < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEph<_> as $crate::Chap39::BSTTreapMtEph::BSTTreapMtEph::BSTTreapMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTTreapMtEphLit_type_checks() {
        let _ = BSTTreapMtEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTTreapMtEph<i32> = BSTTreapMtEphLit![]; // empty form requires explicit type
    }
}
