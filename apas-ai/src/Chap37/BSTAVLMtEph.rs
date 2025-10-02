//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral AVL-balanced binary search tree with interior locking for multi-threaded access.

pub mod BSTAVLMtEph {

use std::sync::{Arc, RwLock};

use crate::Types::Types::*;
use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    type Link<T> = Option<Box<Node<T>>>;

    #[derive(Debug, Clone)]
    struct Node<T: StTInMtT + Ord> {
        key: T,
        height: i32,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTInMtT + Ord> Node<T> {
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
    pub struct BSTAVLMtEph<T: StTInMtT + Ord> {
        root: Arc<RwLock<Link<T>>>,
    }

    pub type BSTreeAVL<T> = BSTAVLMtEph<T>;

    pub trait BSTAVLMtEphTrait<T: StTInMtT + Ord>: Sized {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn new() -> Self;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn insert(&self, value: T);
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn find(&self, target: &T) -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn contains(&self, target: &T) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn size(&self) -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> B;
        /// claude-4-sonet: Work Θ(n), Span Θ(n)
        fn height(&self) -> N;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn minimum(&self) -> Option<T>;
        /// claude-4-sonet: Work Θ(log n), Span Θ(log n) with locking
        fn maximum(&self) -> Option<T>;
        fn in_order(&self) -> ArraySeqStPerS<T>;
        fn pre_order(&self) -> ArraySeqStPerS<T>;
    }

    impl<T: StTInMtT + Ord> Default for BSTAVLMtEph<T> {
        fn default() -> Self { Self::new() }
    }

    impl<T: StTInMtT + Ord> BSTAVLMtEph<T> {
        // Private helper methods only - no public delegation

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
                | Some(node) => match &node.left {
                    | None => Some(&node.key),
                    | Some(_) => Self::min_link(&node.left),
                },
            }
        }

        fn max_link<'a>(link: &'a Link<T>) -> Option<&'a T> {
            match link {
                | None => None,
                | Some(node) => match &node.right {
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

    impl<T: StTInMtT + Ord> BSTAVLMtEphTrait<T> for BSTAVLMtEph<T> {
        fn new() -> Self {
            BSTAVLMtEph {
                root: Arc::new(RwLock::new(None)),
            }
        }

        fn insert(&self, value: T) {
            let mut guard = self.root.write().unwrap();
            Self::insert_link(&mut *guard, value);
        }

        fn find(&self, target: &T) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::find_link(&*guard, target).cloned()
        }

        fn contains(&self, target: &T) -> B { self.find(target).is_some() }

        fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::size_link(&*guard)
        }

        fn is_empty(&self) -> B {
            if self.size() == 0 {
                true
            } else {
                false
            }
        }

        fn height(&self) -> N {
            let guard = self.root.read().unwrap();
            Self::height_link(&*guard) as N
        }

        fn minimum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::min_link(&*guard).cloned()
        }

        fn maximum(&self) -> Option<T> {
            let guard = self.root.read().unwrap();
            Self::max_link(&*guard).cloned()
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::in_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            let guard = self.root.read().unwrap();
            let mut out = Vec::with_capacity(Self::size_link(&*guard));
            Self::pre_order_collect(&*guard, &mut out);
            ArraySeqStPerS::from_vec(out)
        }
    }

    #[macro_export]
    macro_rules! BSTAVLMtEphLit {
        () => {
            < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> as $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEphTrait<_> >::new()
        };
        ( $( $x:expr ),* $(,)? ) => {{
            let __tree = < $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEph<_> as $crate::Chap37::BSTAVLMtEph::BSTAVLMtEph::BSTAVLMtEphTrait<_> >::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTAVLMtEphLit_type_checks() {
        let _ = BSTAVLMtEphLit![1, 2, 3]; // non-empty infers (e.g., i32)
        let _: BSTAVLMtEph<i32> = BSTAVLMtEphLit![]; // empty form requires explicit type
    }
}
