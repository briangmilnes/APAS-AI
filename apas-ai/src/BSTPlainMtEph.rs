//! Ephemeral binary search tree built on `BBTEph` primitives with fine-grained locking.

pub mod BSTPlainMtEph {
    use std::sync::{Arc, RwLock};

    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    type Link<T> = Arc<RwLock<Option<Node<T>>>>;

    #[derive(Clone)]
    struct Node<T: StTinMtT + Ord> {
        key: T,
        height: i32,
        size: N,
        left: Link<T>,
        right: Link<T>,
    }

    impl<T: StTinMtT + Ord> Node<T> {
        fn new(key: T) -> Self {
            Node { key, height: 1, size: 1, left: Arc::new(RwLock::new(None)), right: Arc::new(RwLock::new(None)) }
        }

        fn update(&mut self) {
            let left = self.left.read().unwrap();
            let right = self.right.read().unwrap();
            self.height = 1 + height_of(&left).max(height_of(&right));
            self.size = 1 + size_of(&left) + size_of(&right);
        }
    }

    pub struct BSTPlainMtEph<T: StTinMtT + Ord> {
        root: Link<T>,
    }

    pub type BSTree<T> = BSTPlainMtEph<T>;

    pub trait BSTPlainMtEphTrait<T: StTinMtT + Ord>: Sized {
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
    }

    impl<T: StTinMtT + Ord> BSTPlainMtEph<T> {
        pub fn new() -> Self {
            Self { root: Arc::new(RwLock::new(None)) }
        }

        pub fn insert(&self, value: T) {
            fn descend<T: StTinMtT + Ord>(link: &Link<T>, value: T) -> bool {
                let mut guard = link.write().unwrap();
                match guard.as_mut() {
                    Some(node) => {
                        if value == node.key {
                            return false;
                        }

                        let branch = if value < node.key { node.left.clone() } else { node.right.clone() };

                        drop(guard);
                        let inserted = descend(&branch, value);
                        if inserted {
                            let mut guard = link.write().unwrap();
                            if let Some(node) = guard.as_mut() {
                                node.update();
                            }
                        }
                        inserted
                    }
                    None => {
                        *guard = Some(Node::new(value));
                        true
                    }
                }
            }

            descend(&self.root, value);
        }

        pub fn find(&self, target: &T) -> Option<T> {
            fn find_rec<T: StTinMtT + Ord>(link: &Link<T>, target: &T) -> Option<T> {
                let guard = link.read().unwrap();
                match guard.as_ref() {
                    Some(node) if target == &node.key => Some(node.key.clone()),
                    Some(node) => {
                        let branch = if target < &node.key { node.left.clone() } else { node.right.clone() };
                        drop(guard);
                        find_rec(&branch, target)
                    }
                    None => None,
                }
            }
            find_rec(&self.root, target)
        }

        pub fn contains(&self, target: &T) -> B {
            if self.find(target).is_some() { B::True } else { B::False }
        }
        pub fn size(&self) -> N {
            let guard = self.root.read().unwrap();
            guard.as_ref().map_or(0, |node| node.size)
        }

        pub fn is_empty(&self) -> B {
            if self.size() == 0 { B::True } else { B::False }
        }

        pub fn height(&self) -> N {
            let guard = self.root.read().unwrap();
            guard.as_ref().map_or(0, |node| node.height as N)
        }

        pub fn minimum(&self) -> Option<T> {
            fn leftmost<T: StTinMtT + Ord>(link: &Link<T>) -> Option<T> {
                let guard = link.read().unwrap();
                if let Some(node) = guard.as_ref() {
                    let child = node.left.clone();
                    let key = node.key.clone();
                    drop(guard);
                    let left_guard = child.read().unwrap();
                    if left_guard.is_none() {
                        Some(key)
                    } else {
                        drop(left_guard);
                        leftmost(&child)
                    }
                } else {
                    None
                }
            }

            leftmost(&self.root)
        }

        pub fn maximum(&self) -> Option<T> {
            fn rightmost<T: StTinMtT + Ord>(link: &Link<T>) -> Option<T> {
                let guard = link.read().unwrap();
                if let Some(node) = guard.as_ref() {
                    let child = node.right.clone();
                    let key = node.key.clone();
                    drop(guard);
                    let right_guard = child.read().unwrap();
                    if right_guard.is_none() {
                        Some(key)
                    } else {
                        drop(right_guard);
                        rightmost(&child)
                    }
                } else {
                    None
                }
            }

            rightmost(&self.root)
        }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            fn traverse<T: StTinMtT + Ord>(link: &Link<T>, out: &mut Vec<T>) {
                let guard = link.read().unwrap();
                if let Some(node) = guard.as_ref() {
                    let left = node.left.clone();
                    let right = node.right.clone();
                    let key = node.key.clone();
                    drop(guard);
                    traverse(&left, out);
                    out.push(key);
                    traverse(&right, out);
                }
            }

            let mut values = Vec::new();
            traverse(&self.root, &mut values);
            ArrayStPerS::from_vec(values)
        }
    }

    fn height_of<T: StTinMtT + Ord>(link: &Option<Node<T>>) -> i32 {
        link.as_ref().map_or(0, |n| n.height)
    }

    fn size_of<T: StTinMtT + Ord>(link: &Option<Node<T>>) -> N {
        link.as_ref().map_or(0, |n| n.size)
    }

    impl<T: StTinMtT + Ord> BSTPlainMtEphTrait<T> for BSTPlainMtEph<T> {
        fn new() -> Self {
            BSTPlainMtEph::new()
        }

        fn insert(&self, value: T) {
            BSTPlainMtEph::insert(self, value)
        }

        fn find(&self, target: &T) -> Option<T> {
            BSTPlainMtEph::find(self, target)
        }

        fn contains(&self, target: &T) -> B {
            BSTPlainMtEph::contains(self, target)
        }

        fn size(&self) -> N {
            BSTPlainMtEph::size(self)
        }

        fn is_empty(&self) -> B {
            BSTPlainMtEph::is_empty(self)
        }

        fn height(&self) -> N {
            BSTPlainMtEph::height(self)
        }

        fn minimum(&self) -> Option<T> {
            BSTPlainMtEph::minimum(self)
        }

        fn maximum(&self) -> Option<T> {
            BSTPlainMtEph::maximum(self)
        }

        fn in_order(&self) -> ArrayStPerS<T> {
            BSTPlainMtEph::in_order(self)
        }
    }
}
