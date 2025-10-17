//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral full binary tree utilities (Chapter 23).

pub mod BalBinTreeStEph {

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub enum BalBinTree<T: StT> {
        Leaf,
        Node(Box<BalBinNode<T>>),
    }

    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct BalBinNode<T: StT> {
        pub(crate) left: BalBinTree<T>,
        pub(crate) value: T,
        pub(crate) right: BalBinTree<T>,
    }

    pub trait BalBinTreeStEphTrait<T: StT> {
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn leaf()                                  -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn node(left: Self, value: T, right: Self) -> Self;
        /// APAS: Work Θ(1), Span Θ(1)
        /// claude-4-sonet: Work Θ(1), Span Θ(1), Parallelism Θ(1)
        fn is_leaf(&self)                          -> B;
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential traversal with append
        fn in_order(&self)                         -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential traversal with append
        fn pre_order(&self)                        -> ArraySeqStPerS<T>;
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential tree traversal
        fn height(&self)                           -> N;
        /// APAS: Work Θ(n), Span Θ(n)
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1) - sequential tree traversal
        fn size(&self)                             -> N;
    }


    impl<T: StT> BalBinNode<T> {
        fn new(left: BalBinTree<T>, value: T, right: BalBinTree<T>) -> Self { BalBinNode { left, value, right } }
    }

    impl<T: StT> BalBinTree<T> {
        pub fn leaf() -> Self { BalBinTree::Leaf }

        pub fn node(left: BalBinTree<T>, value: T, right: BalBinTree<T>) -> Self {
            BalBinTree::Node(Box::new(BalBinNode::new(left, value, right)))
        }

        pub fn is_leaf(&self) -> B {
            match self {
                | BalBinTree::Leaf => true,
                | BalBinTree::Node(_) => false,
            }
        }

        pub fn in_order(&self) -> ArraySeqStPerS<T> {
            match self {
                | BalBinTree::Leaf => <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty(),
                | BalBinTree::Node(node) => {
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
                | BalBinTree::Leaf => <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty(),
                | BalBinTree::Node(node) => {
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
                | BalBinTree::Leaf => 0,
                | BalBinTree::Node(node) => {
                    let left_h = node.left.height();
                    let right_h = node.right.height();
                    1 + left_h.max(right_h)
                }
            }
        }

        pub fn size(&self) -> N {
            match self {
                | BalBinTree::Leaf => 0,
                | BalBinTree::Node(node) => 1 + node.left.size() + node.right.size(),
            }
        }
    }

    impl<T: StT> BalBinTreeStEphTrait<T> for BalBinTree<T> {
        fn leaf() -> Self { BalBinTree::leaf() }

        fn node(left: Self, value: T, right: Self) -> Self { BalBinTree::node(left, value, right) }

        fn is_leaf(&self) -> B { BalBinTree::is_leaf(self) }

        fn in_order(&self) -> ArraySeqStPerS<T> { BalBinTree::in_order(self) }

        fn pre_order(&self) -> ArraySeqStPerS<T> { BalBinTree::pre_order(self) }

        fn height(&self) -> N { BalBinTree::height(self) }

        fn size(&self) -> N { BalBinTree::size(self) }
    }

    #[macro_export]
    macro_rules! BalBinNodeLit {
        ({ left: $left:expr, value: $value:expr, right: $right:expr }) => {
            $crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::BalBinNode::new($left, $value, $right)
        };
    }
}
