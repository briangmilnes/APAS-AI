//! Ephemeral full binary tree utilities (Chapter 37).

pub mod BBTEph {
    use crate::ArraySeqStPer::ArraySeqStPer::*;
    use crate::ArraySeqStPerChap18::ArraySeqStPerChap18::*;
    use crate::Types::Types::*;

    /// Full binary tree nodes (`Leaf` has no stored value).
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum BBTree<T: StT> {
        Leaf,
        Node(Box<BBNode<T>>),
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct BBNode<T: StT> {
        pub left: BBTree<T>,
        pub value: T,
        pub right: BBTree<T>,
    }

    impl<T: StT> BBTree<T> {
        pub fn leaf() -> Self {
            BBTree::Leaf
        }

        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {
            BBTree::Node(Box::new(BBNode { left, value, right }))
        }

        pub fn is_leaf(&self) -> B {
            match self {
                BBTree::Leaf => B::True,
                BBTree::Node(_) => B::False,
            }
        }

        pub fn in_order(&self) -> ArrayStPerS<T> {
            match self {
                BBTree::Leaf => <ArrayStPerS<T> as ArraySeqStPerTrait<T>>::empty(),
                BBTree::Node(node) => {
                    let left = node.left.in_order();
                    let middle = <ArrayStPerS<T> as ArraySeqStPerTrait<T>>::singleton(node.value.clone());
                    let right = node.right.in_order();
                    let left_mid = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::append(&left, &middle);
                    <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::append(&left_mid, &right)
                }
            }
        }

        pub fn pre_order(&self) -> ArrayStPerS<T> {
            match self {
                BBTree::Leaf => <ArrayStPerS<T> as ArraySeqStPerTrait<T>>::empty(),
                BBTree::Node(node) => {
                    let root = <ArrayStPerS<T> as ArraySeqStPerTrait<T>>::singleton(node.value.clone());
                    let left = node.left.pre_order();
                    let right = node.right.pre_order();
                    let root_left = <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::append(&root, &left);
                    <ArrayStPerS<T> as ArraySeqStPerChap18Trait<T>>::append(&root_left, &right)
                }
            }
        }

        pub fn height(&self) -> N {
            match self {
                BBTree::Leaf => 0,
                BBTree::Node(node) => {
                    let left_h = node.left.height();
                    let right_h = node.right.height();
                    1 + left_h.max(right_h)
                }
            }
        }

        pub fn size(&self) -> N {
            match self {
                BBTree::Leaf => 0,
                BBTree::Node(node) => 1 + node.left.size() + node.right.size(),
            }
        }
    }
}
