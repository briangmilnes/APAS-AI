//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral full binary tree utilities (Chapter 37).

pub mod BBTStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    /// Full binary tree nodes (`Leaf` has no stored value).
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum BBTree<T: StT> {
        Leaf,
        Node(Box<BBNode<T>>),
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct BBNode<T: StT> {
        pub(crate) left: BBTree<T>,
        pub(crate) value: T,
        pub(crate) right: BBTree<T>,
    }

    impl<T: StT> BBNode<T> {
        fn new(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {
            BBNode { left, value, right }
        }
    }

    pub trait BBTStEphTrait<T: StT> {
        fn leaf() -> Self;
        fn node(left: Self, value: T, right: Self) -> Self;
        fn is_leaf(&self) -> B;
        fn in_order(&self) -> ArraySeqStPerS<T>;
        fn pre_order(&self) -> ArraySeqStPerS<T>;
        fn height(&self) -> N;
        fn size(&self) -> N;
    }

    impl<T: StT> BBTree<T> {
        pub fn leaf() -> Self {
            BBTree::Leaf
        }

        pub fn node(left: BBTree<T>, value: T, right: BBTree<T>) -> Self {
            BBTree::Node(Box::new(BBNode::new(left, value, right)))
        }

        pub fn is_leaf(&self) -> B {
            match self {
                | BBTree::Leaf => true,
                | BBTree::Node(_) => false,
            }
        }

        pub fn in_order(&self) -> ArraySeqStPerS<T> {
            match self {
                | BBTree::Leaf => <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty(),
                | BBTree::Node(node) => {
                    let left = node.left.in_order();
                    let middle = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::singleton(node.value.clone());
                    let right = node.right.in_order();
                    let left_mid = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::append(&left, &middle);
                    <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::append(&left_mid, &right)
                }
            }
        }

        pub fn pre_order(&self) -> ArraySeqStPerS<T> {
            match self {
                | BBTree::Leaf => <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty(),
                | BBTree::Node(node) => {
                    let root = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::singleton(node.value.clone());
                    let left = node.left.pre_order();
                    let right = node.right.pre_order();
                    let root_left = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::append(&root, &left);
                    <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::append(&root_left, &right)
                }
            }
        }

        pub fn height(&self) -> N {
            match self {
                | BBTree::Leaf => 0,
                | BBTree::Node(node) => {
                    let left_h = node.left.height();
                    let right_h = node.right.height();
                    1 + left_h.max(right_h)
                }
            }
        }

        pub fn size(&self) -> N {
            match self {
                | BBTree::Leaf => 0,
                | BBTree::Node(node) => 1 + node.left.size() + node.right.size(),
            }
        }
    }

    impl<T: StT> BBTStEphTrait<T> for BBTree<T> {
        fn leaf() -> Self {
            BBTree::leaf()
        }

        fn node(left: Self, value: T, right: Self) -> Self {
            BBTree::node(left, value, right)
        }

        fn is_leaf(&self) -> B {
            BBTree::is_leaf(self)
        }

        fn in_order(&self) -> ArraySeqStPerS<T> {
            BBTree::in_order(self)
        }

        fn pre_order(&self) -> ArraySeqStPerS<T> {
            BBTree::pre_order(self)
        }

        fn height(&self) -> N {
            BBTree::height(self)
        }

        fn size(&self) -> N {
            BBTree::size(self)
        }
    }

    #[macro_export]
    macro_rules! BBNodeLit {
        ({ left: $left:expr, value: $value:expr, right: $right:expr }) => {
            $crate::Chap23::BBTStEph::BBTStEph::BBNode::new($left, $value, $right)
        };
    }

    #[allow(dead_code)]
    fn _BBNodeLit_type_checks() {
        let _: BBNode<i32> = BBNodeLit!({ left: crate::Chap23::BBTStEph::BBTStEph::BBTree::Leaf, value: 0, right: crate::Chap23::BBTStEph::BBTStEph::BBTree::Leaf });
    }
}
