//! Ephemeral weight-balanced (BB[α]) binary search tree with interior locking for multi-threaded access.

pub mod BSTBBAlphaMtEph {
    use std::sync::{Arc, RwLock};

    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
    use crate::Types::Types::*;

    const ALPHA: f64 = 0.75;

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
            Node {
                key,
                size: 1,
                left: None,
                right: None,
            }
        }
    }

    #[derive(Clone)]
    pub struct BSTBBAlphaMtEph<T: StTinMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeBBAlpha<T> = BSTBBAlphaMtEph<T>;

    pub trait BSTBBAlphaMtEphTrait<T: StTinMtT + Ord>: Sized {
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

    impl<T: StTinMtT + Ord> Default for BSTBBAlphaMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StTinMtT + Ord> BSTBBAlphaMtEph<T> {
        pub fn new() -> Self {
            BSTBBAlphaMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        pub fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::size_link(&*guard)
        }

        pub fn is_empty(&self) -> B {
            if self.size() == 0 {
                B::True
            } else {
                B::False
            }
        }

        pub fn height(&self) -> N {
            fn height_rec<T: StTinMtT + Ord>(link: &Link<T>) -> N {
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
            let inserted = Self::insert_link(&mut *guard, value);
            if inserted {
                let total = Self::size_link(&*guard);
                Self::rebalance_if_needed(&mut *guard, total);
            }
        }

        pub fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::find_link(&*guard, target).cloned()
        }

        pub fn contains(&self, target: &T) -> B {
            if self.find(target).is_some() {
                B::True
            } else {
                B::False
            }
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

        fn size_link(link: &Link<T>) -> N { link.as_ref().map_or(0, |n| n.size) }

        fn update(node: &mut Node<T>) { node.size = 1 + Self::size_link(&node.left) + Self::size_link(&node.right); }

        fn insert_link(link: &mut Link<T>, value: T) -> bool {
            match link {
                | Some(node) => {
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
                | None => {
                    *link = Some(Box::new(Node::new(value)));
                    true
                }
            }
        }

        fn needs_rebuild(node: &Node<T>) -> bool {
            let total = node.size as f64;
            let left = Self::size_link(&node.left) as f64;
            let right = Self::size_link(&node.right) as f64;
            left > ALPHA * total || right > ALPHA * total
        }

        fn rebalance_if_needed(link: &mut Link<T>, total_size: N) {
            if let Some(node) = link.as_ref() {
                if Self::needs_rebuild(node) {
                    let mut values = Vec::with_capacity(total_size);
                    Self::collect_values(&Some(node.clone()), &mut values);
                    *link = Self::build_balanced(&values);
                }
            }
        }

        fn collect_values(link: &Link<T>, out: &mut Vec<T>) {
            if let Some(node) = link {
                Self::collect_values(&node.left, out);
                out.push(node.key.clone());
                Self::collect_values(&node.right, out);
            }
        }

        fn build_balanced(values: &[T]) -> Link<T> {
            if values.is_empty() {
                return None;
            }
            let mid = values.len() / 2;
            let mut node = Box::new(Node::new(values[mid].clone()));
            node.left = Self::build_balanced(&values[..mid]);
            node.right = Self::build_balanced(&values[mid + 1..]);
            Self::update(&mut node);
            Some(node)
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

    impl<T: StTinMtT + Ord> BSTBBAlphaMtEphTrait<T> for BSTBBAlphaMtEph<T> {
        fn new() -> Self { BSTBBAlphaMtEph::new() }

        fn insert(&self, value: T) { BSTBBAlphaMtEph::insert(self, value) }

        fn find(&self, target: &T) -> Option<T> { BSTBBAlphaMtEph::find(self, target) }

        fn contains(&self, target: &T) -> B { BSTBBAlphaMtEph::contains(self, target) }

        fn size(&self) -> N { BSTBBAlphaMtEph::size(self) }

        fn is_empty(&self) -> B { BSTBBAlphaMtEph::is_empty(self) }

        fn height(&self) -> N { BSTBBAlphaMtEph::height(self) }

        fn minimum(&self) -> Option<T> { BSTBBAlphaMtEph::minimum(self) }

        fn maximum(&self) -> Option<T> { BSTBBAlphaMtEph::maximum(self) }

        fn in_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::in_order(self) }

        fn pre_order(&self) -> ArrayStPerS<T> { BSTBBAlphaMtEph::pre_order(self) }
    }
}
