//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ephemeral binary search tree built on `BBTEph` primitives.

pub mod BSTPlainStEph {
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap23::BalBinTreeStEph::BalBinTreeStEph::BalBinTree;
    use crate::Types::Types::*;

    #[derive(Debug, Clone)]
    pub struct BSTPlainStEph<T: StT + Ord> {
        root: BalBinTree<T>,
    }

    pub type BSTree<T> = BSTPlainStEph<T>;

    pub trait BSTPlainStEphTrait<T: StT + Ord> {
        fn new() -> Self;
        fn size(&self) -> N;
        fn is_empty(&self) -> B;
        fn height(&self) -> N;
        fn insert(&mut self, value: T);
        fn find(&self, target: &T) -> Option<&T>;
        fn contains(&self, target: &T) -> B;
        fn minimum(&self) -> Option<&T>;
        fn maximum(&self) -> Option<&T>;
        fn in_order(&self) -> ArraySeqStPerS<T>;
        fn pre_order(&self) -> ArraySeqStPerS<T>;
    }

    impl<T: StT + Ord> BSTPlainStEph<T> {
        // Private helper methods only - no public delegation
    }

    impl<T: StT + Ord> BSTPlainStEphTrait<T> for BSTPlainStEph<T> {
        fn new() -> Self {
            BSTPlainStEph {
                root: BalBinTree::leaf(),
            }
        }

        fn size(&self) -> N { self.root.size() }

        fn is_empty(&self) -> B { self.root.is_leaf() }

        fn height(&self) -> N { self.root.height() }

        fn insert(&mut self, value: T) { insert_node(&mut self.root, value); }

        fn find(&self, target: &T) -> Option<&T> { find_node(&self.root, target) }

        fn contains(&self, target: &T) -> B { contains_node(&self.root, target) }

        fn minimum(&self) -> Option<&T> { min_node(&self.root) }

        fn maximum(&self) -> Option<&T> { max_node(&self.root) }

        fn in_order(&self) -> ArraySeqStPerS<T> { self.root.in_order() }

        fn pre_order(&self) -> ArraySeqStPerS<T> { self.root.pre_order() }
    }

    fn insert_node<T: StT + Ord>(node: &mut BalBinTree<T>, value: T) {
        match node {
            | BalBinTree::Leaf => {
                *node = BalBinTree::node(BalBinTree::leaf(), value, BalBinTree::leaf());
            }
            | BalBinTree::Node(inner) => {
                if value < inner.value {
                    insert_node(&mut inner.left, value);
                } else if value > inner.value {
                    insert_node(&mut inner.right, value);
                }
            }
        }
    }

    fn contains_node<'a, T: StT + Ord>(node: &'a BalBinTree<T>, target: &T) -> B {
        match node {
            | BalBinTree::Leaf => false,
            | BalBinTree::Node(inner) => {
                if target == &inner.value {
                    true
                } else if target < &inner.value {
                    contains_node(&inner.left, target)
                } else {
                    contains_node(&inner.right, target)
                }
            }
        }
    }

    fn find_node<'a, T: StT + Ord>(node: &'a BalBinTree<T>, target: &T) -> Option<&'a T> {
        match node {
            | BalBinTree::Leaf => None,
            | BalBinTree::Node(inner) => {
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

    fn min_node<'a, T: StT + Ord>(node: &'a BalBinTree<T>) -> Option<&'a T> {
        match node {
            | BalBinTree::Leaf => None,
            | BalBinTree::Node(inner) => match &inner.left {
                | BalBinTree::Leaf => Some(&inner.value),
                | _ => min_node(&inner.left),
            },
        }
    }

    fn max_node<'a, T: StT + Ord>(node: &'a BalBinTree<T>) -> Option<&'a T> {
        match node {
            | BalBinTree::Leaf => None,
            | BalBinTree::Node(inner) => match &inner.right {
                | BalBinTree::Leaf => Some(&inner.value),
                | _ => max_node(&inner.right),
            },
        }
    }

    #[macro_export]
    macro_rules! BSTPlainStEphLit {
        () => { $crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph::new() };
        ($x:expr; $n:expr) => {{
            let mut __tree = $crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph::new();
            for _ in 0..$n { __tree.insert($x.clone()); }
            __tree
        }};
        ($($x:expr),+ $(,)?) => {{
            let mut __tree = $crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph::new();
            $( __tree.insert($x); )*
            __tree
        }};
    }

    #[allow(dead_code)]
    fn _BSTPlainStEphLit_type_checks() {
        use crate::Chap37::BSTPlainStEph::BSTPlainStEph::BSTPlainStEph;
        let _: BSTPlainStEph<i32> = BSTPlainStEphLit![];
        let _ = BSTPlainStEphLit![1, 2, 3];
        let _ = BSTPlainStEphLit![0; 2];
    }
}
