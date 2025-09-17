//! Ephemeral binary search tree built on `BBTEph` primitives.

pub mod BSTEph {
    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::BBTEph::BBTEph::BBTree;
    use crate::Types::Types::*;

    pub struct BSTree<T: StT + Ord> {
        root: BBTree<T>,
    }

    impl<T: StT + Ord> BSTree<T> {
        pub fn new() -> Self {
            BSTree { root: BBTree::leaf() }
        }

        pub fn is_empty(&self) -> B {
            self.root.is_leaf()
        }

        pub fn height(&self) -> N {
            self.root.height()
        }

        pub fn size(&self) -> N {
            self.root.size()
        }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            self.root.in_order()
        }

        pub fn pre_order(&self) -> ArrayStPerS<T> {
            self.root.pre_order()
        }

        pub fn insert(&mut self, value: T) {
            insert_node(&mut self.root, value);
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

        pub fn root(&self) -> &BBTree<T> {
            &self.root
        }
    }

    fn insert_node<T: StT + Ord>(node: &mut BBTree<T>, value: T) {
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

    fn contains_node<'a, T: StT + Ord>(node: &'a BBTree<T>, target: &T) -> B {
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

    fn min_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {
        match node {
            BBTree::Leaf => None,
            BBTree::Node(inner) => match &inner.left {
                BBTree::Leaf => Some(&inner.value),
                _ => min_node(&inner.left),
            },
        }
    }

    fn max_node<'a, T: StT + Ord>(node: &'a BBTree<T>) -> Option<&'a T> {
        match node {
            BBTree::Leaf => None,
            BBTree::Node(inner) => match &inner.right {
                BBTree::Leaf => Some(&inner.value),
                _ => max_node(&inner.right),
            },
        }
    }
}
