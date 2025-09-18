//! Ephemeral binary search tree built on `BBTEph` primitives.

pub mod BSTPlainMtEph {
    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::BBTEph::BBTEph::BBTree;
    use crate::Types::Types::*;

    pub struct BSTree<T: StT + Ord + Send> {
        root: BBTree<T>,
    }

    pub trait BSTPlainMtEphTrait<T: StT + Ord + Send> {
        fn new() -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn insert(&mut self, value: T);
        fn find(&self, target: &T) -> Option<&T>;
        fn contains(&self, target: &T) -> B;
        fn minimum(&self) -> Option<&T>;
        fn maximum(&self) -> Option<&T>;
        fn in_order(&self) -> ArrayStPerS<T>;
        fn pre_order(&self) -> ArrayStPerS<T>;
    }

    impl<T: StT + Ord + Send> BSTree<T> {
        pub fn new() -> Self {
            BSTree { root: BBTree::leaf() }
        }

        pub fn size(&self) -> N {
            self.root.size()
        }

        pub fn is_empty(&self) -> B {
            self.root.is_leaf()
        }

        pub fn height(&self) -> N {
            self.root.height()
        }

        pub fn insert(&mut self, value: T) {
            insert_node(&mut self.root, value);
        }

        pub fn find(&self, target: &T) -> Option<&T> {
            find_node(&self.root, target)
        }

        pub fn contains(&self, target: &T) -> B {
            contains_node(&self.root, target)
        }

        pub fn minimum(&self) -> Option<&T> {
            min_node(&self.root)
        }

        pub fn maximum(&self) -> Option<&T> {
            max_node(&self.root)
        }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            self.root.in_order()
        }

        pub fn pre_order(&self) -> ArrayStPerS<T> {
            self.root.pre_order()
        }
    }

    impl<T: StT + Ord + Send> BSTPlainMtEphTrait<T> for BSTree<T> {
        fn new() -> Self {
            BSTree::new()
        }

        fn size(&self) -> N {
            BSTree::size(self)
        }

        fn is_empty(&self) -> B {
            BSTree::is_empty(self)
        }

        fn height(&self) -> N {
            BSTree::height(self)
        }

        fn insert(&mut self, value: T) {
            BSTree::insert(self, value)
        }

        fn find(&self, target: &T) -> Option<&T> {
            BSTree::find(self, target)
        }

        fn contains(&self, target: &T) -> B {
            BSTree::contains(self, target)
        }

        fn minimum(&self) -> Option<&T> {
            BSTree::minimum(self)
        }

        fn maximum(&self) -> Option<&T> {
            BSTree::maximum(self)
        }

        fn in_order(&self) -> ArrayStPerS<T> {
            BSTree::in_order(self)
        }

        fn pre_order(&self) -> ArrayStPerS<T> {
            BSTree::pre_order(self)
        }
    }

    fn insert_node<T: StT + Ord + Send>(node: &mut BBTree<T>, value: T) {
        match node {
            BBTree::Leaf => {
                *node = BBTree::node(BBTree::leaf(), value, BBTree::leaf());
            }
            BBTree::Node(inner) => {
                if value < inner.value {
                    insert_node(&mut inner.left, value);
                } else if value > inner.value {
                    insert_node(&mut inner.right, value);
                }
            }
        }
    }

    fn contains_node<'a, T: StT + Ord + Send>(node: &'a BBTree<T>, target: &T) -> B {
        match node {
            BBTree::Leaf => B::False,
            BBTree::Node(inner) => {
                if target == &inner.value {
                    B::True
                } else if target < &inner.value {
                    contains_node(&inner.left, target)
                } else {
                    contains_node(&inner.right, target)
                }
            }
        }
    }

    fn find_node<'a, T: StT + Ord + Send>(node: &'a BBTree<T>, target: &T) -> Option<&'a T> {
        match node {
            BBTree::Leaf => None,
            BBTree::Node(inner) => {
                if target == &inner.value {
                    Some(&inner.value)
                } else if target < &inner.value {
                    find_node(&inner.left, target)
                } else {
                    find_node(&inner.right, target)
                }
            }
        }
    }

    fn min_node<'a, T: StT + Ord + Send>(node: &'a BBTree<T>) -> Option<&'a T> {
        match node {
            BBTree::Leaf => None,
            BBTree::Node(inner) => match &inner.left {
                BBTree::Leaf => Some(&inner.value),
                _ => min_node(&inner.left),
            },
        }
    }

    fn max_node<'a, T: StT + Ord + Send>(node: &'a BBTree<T>) -> Option<&'a T> {
        match node {
            BBTree::Leaf => None,
            BBTree::Node(inner) => match &inner.right {
                BBTree::Leaf => Some(&inner.value),
                _ => max_node(&inner.right),
            },
        }
    }
}
