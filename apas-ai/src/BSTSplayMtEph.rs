//! Ephemeral splay-style (simple BST) structure with interior locking for multi-threaded access.

pub mod BSTSplayMtEph {
    use std::sync::{Arc, RwLock};

    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
    use crate::Types::Types::*;

    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Clone)]
    struct Node<T: StTinMtT + Ord> {
        key: T,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTinMtT + Ord> Node<T> {
        fn new(key: T) -> Self {
            Node { key, size: 1, left: None, right: None }
        }
    }

    #[derive(Clone)]
    pub struct BSTSplayMtEph<T: StTinMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeSplay<T> = BSTSplayMtEph<T>;

    pub trait BSTSplayMtEphTrait<T: StTinMtT + Ord>: Sized {
        fn new() -> Self;
        fn insert(&self, value: T);
        fn find(&self, target: &T) -> Option<T>;
        fn contains(&self, target: &T) -> B;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn minimum(&self) -> Option<T>;
        fn maximum(&self) -> Option<T>;
        fn in_order(&self) -> ArrayStPerS<T>;
        fn pre_order(&self) -> ArrayStPerS<T>;
    }

    impl<T: StTinMtT + Ord> Default for BSTSplayMtEph<T> {
        fn default() -> Self {
            Self::new()
        }
    }

    impl<T: StTinMtT + Ord> BSTSplayMtEph<T> {
        pub fn new() -> Self {
            BSTSplayMtEph { root: Arc::new(RwLock::new(None)) }
        }

        pub fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::size_link(&*guard)
        }

        pub fn is_empty(&self) -> B {
            if self.size() == 0 { B::True } else { B::False }
        }

        pub fn height(&self) -> N {
            fn height_rec<T: StTinMtT + Ord>(link: &Link<T>) -> N {
                match link {
                    None => 0,
                    Some(node) => 1 + height_rec(&node.left).max(height_rec(&node.right)),
                }
            }

            let guard = self.root.read().unwrap();
            height_rec(&*guard)
        }

        pub fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            Self::insert_link(&mut *guard, value);
        }

        pub fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::find_link(&*guard, target).cloned()
        }

        pub fn contains(&self, target: &T) -> B {
            if self.find(target).is_some() { B::True } else { B::False }
        }

        pub fn minimum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::min_link(&*guard).cloned()
        }

        pub fn maximum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::max_link(&*guard).cloned()
        }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::in_order_collect(&*guard, &mut out);
            ArrayStPerS::from_vec(out)
        }

        pub fn pre_order(&self) -> ArrayStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::pre_order_collect(&*guard, &mut out);
            ArrayStPerS::from_vec(out)
        }

        fn size_link(link: &Link<T>) -> N {
            link.as_ref().map_or(0, |n| n.size)
        }

        fn update(node: &mut Node<T>) {
            node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right);
        }

        fn insert_link(link: &mut Link<T>, value: T) -> bool {
            match link {
                Some(node) => {
                    let inserted = if value < node.key {
                        Self::insert_link(&mut node.left, value)
                    } else if value > node.key {
                        Self::insert_link(&mut node.right, value)
                    } else {
                        false
                    };
                    if inserted {
                        Self::update(node);
                    }
                    inserted
                }
                None => {
                    *link = Some(Box::new(Node::new(value)));
                    true
                }
            }
        }

        fn find_link<'a>(link: &'a Link<T>, target: &T) -> Option<&'a T> {
            match link {
                None => None,
                Some(node) => {
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
                None => None,
                Some(node) => match node.left {
                    None => Some(&node.key),
                    Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {
            match link {
                None => None,
                Some(node) => match node.right {
                    None => Some(&node.key),
                    Some(_) => Self::max_link(&node.right),
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

    impl<T: StTinMtT + Ord> BSTSplayMtEphTrait<T> for BSTSplayMtEph<T> {
        fn new() -> Self {
            BSTSplayMtEph::new()
        }

        fn insert(&self, value: T) {
            BSTSplayMtEph::insert(self, value)
        }

        fn find(&self, target: &T) -> Option<T> {
            BSTSplayMtEph::find(self, target)
        }

        fn contains(&self, target: &T) -> B {
            BSTSplayMtEph::contains(self, target)
        }

        fn size(&self) -> N {
            BSTSplayMtEph::size(self)
        }

        fn is_empty(&self) -> B {
            BSTSplayMtEph::is_empty(self)
        }

        fn height(&self) -> N {
            BSTSplayMtEph::height(self)
        }

        fn minimum(&self) -> Option<T> {
            BSTSplayMtEph::minimum(self)
        }

        fn maximum(&self) -> Option<T> {
            BSTSplayMtEph::maximum(self)
        }

        fn in_order(&self) -> ArrayStPerS<T> {
            BSTSplayMtEph::in_order(self)
        }

        fn pre_order(&self) -> ArrayStPerS<T> {
            BSTSplayMtEph::pre_order(self)
        }
    }
}
